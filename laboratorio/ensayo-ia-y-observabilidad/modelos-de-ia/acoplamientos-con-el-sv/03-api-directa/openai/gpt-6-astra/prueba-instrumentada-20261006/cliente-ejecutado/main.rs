#![forbid(unsafe_code)]
mod prueba;
mod respuesta;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::{Read, Write}, net::{TcpListener, TcpStream}, path::Path, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use subtle::ConstantTimeEq;
use url::Url;
use zeroize::{Zeroize, Zeroizing};

type R<T> = Result<T, String>;
const AUTH: &str = "https://auth.openai.com/api/accounts/authorize";
const TOKEN: &str = "https://auth.openai.com/api/accounts/oauth/token";
const JWKS: &str = "https://auth.openai.com/.well-known/jwks.json";
const ISSUER: &str = "https://auth.openai.com";
const MODELS: &str = "https://api.openai.com/v1/models";
const RESOURCE: &str = "https://api.openai.com/v1";
const SCOPES: &str = "openid profile email offline_access resource.invoke chatgpt.tokens.use.direct";
const APP: &str = "SV - Comprobacion de acceso OpenAI";
const MAX_REQUEST: usize = 16384;

#[derive(Default, Serialize, Deserialize)]
struct Registration {
    host: String,
    client: Option<String>,
    subject_hash: Option<String>,
}
#[derive(Deserialize)]
struct TokenSet {
    access_token: String,
    id_token: String,
    #[serde(default)] refresh_token: String,
    token_type: String,
    #[serde(default)] scope: String,
}
impl Drop for TokenSet {
    fn drop(&mut self) {
        self.access_token.zeroize();
        self.id_token.zeroize();
        self.refresh_token.zeroize();
    }
}
#[derive(Clone, Deserialize)]
struct Claims { sub: String, nonce: String, exp: u64, iat: u64, iss: String, aud: Value, #[serde(default)] azp: Option<String> }
struct Pending { state: Zeroizing<String>, nonce: Zeroizing<String>, verifier: Zeroizing<String>, redirect: String, existing: Option<String> }

fn now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn random() -> R<String> {
    let mut b = [0u8;32];
    getrandom::getrandom(&mut b).map_err(|_| "No se obtiene aleatoriedad del sistema".to_string())?;
    let s = URL_SAFE_NO_PAD.encode(b); b.zeroize(); Ok(s)
}
fn ct(a: &str, b: &str) -> bool { a.as_bytes().ct_eq(b.as_bytes()).into() }
fn sha(s: &[u8]) -> String { format!("{:x}",Sha256::digest(s)) }
fn save(path: &str, value: &Value) -> R<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| "No se codifica el registro")?;
    let tmp = format!("{path}.tmp");
    let mut f = fs::OpenOptions::new().write(true).create_new(true).open(&tmp).map_err(|_| "No se abre registro temporal")?;
    f.write_all(&bytes).and_then(|_|f.sync_all()).map_err(|_|"No se conserva registro")?;
    drop(f);
    // Renombrado tras sincronización; las rutas están confinadas al directorio privado de la aplicación.
    fs::rename(&tmp,path).map_err(|_|"No se sustituye registro".to_string())
}
fn report_path() -> String { if std::env::args().nth(1).is_some_and(|s|s.starts_with("prueba")) { format!("{}/resultado.json",prueba::dir()) } else { "sesion/resultado.json".into() } }
fn write_report(value: Value) -> R<()> { save(&report_path(),&value) }
fn client() -> R<Client> {
    Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(30))
        .user_agent("SV-Comprobacion-Acceso/0.1.0")
        .build().map_err(|_|"No se configura HTTPS".to_string())
}
fn body(mut response: reqwest::blocking::Response, max: usize) -> R<Zeroizing<Vec<u8>>> {
    if !response.status().is_success() { return Err(format!("HTTP {}", response.status().as_u16())); }
    if response.content_length().is_some_and(|n| n > max as u64) { return Err("Respuesta excesiva".into()); }
    let mut bytes = Zeroizing::new(Vec::new());
    response.by_ref().take((max+1) as u64).read_to_end(&mut bytes).map_err(|_|"Lectura HTTP interrumpida")?;
    if bytes.len()>max { return Err("Respuesta excesiva".into()); }
    Ok(bytes)
}
fn query(target: &str) -> R<BTreeMap<String,String>> {
    if !target.starts_with("/auth/callback?") { return Err("Ruta de retorno incorrecta".into()); }
    let u = Url::parse(&format!("http://127.0.0.1{target}")).map_err(|_|"Retorno inválido")?;
    let mut m = BTreeMap::new();
    for (k,v) in u.query_pairs() {
        if m.insert(k.into_owned(),v.into_owned()).is_some() { return Err("Parámetros repetidos".into()); }
    }
    Ok(m)
}
fn validate_callback(m: &BTreeMap<String,String>, p: &Pending) -> R<(String,Zeroizing<String>)> {
    let state = m.get("state").ok_or("Falta estado de retorno")?;
    if !ct(state,&p.state) { return Err("Estado de retorno discordante".into()); }
    if m.contains_key("error") { return Err("Autorización no concedida".into()); }
    let client_id = match (&p.existing,m.get("client_id")) {
        (Some(old),Some(new)) if old!=new => return Err("Identificador de cliente discordante".into()),
        (Some(old),_)=>old.clone(),
        (None,Some(new)) if new.starts_with("oaiapp_")=>new.clone(),
        _=>return Err("Registro incompleto: falta cliente emitido".into()),
    };
    if client_id=="dynamic_agent_client" || client_id.len()>512 { return Err("Identificador no admisible".into()); }
    let code = m.get("code").filter(|v|!v.is_empty()).ok_or("Falta código de autorización")?;
    Ok((client_id,Zeroizing::new(code.clone())))
}
fn verify_identity(token: &str, keys: &JwkSet, client_id: &str, nonce: &str) -> R<Claims> {
    let h=decode_header(token).map_err(|_|"Cabecera de identidad inválida")?;
    if h.alg!=Algorithm::RS256 { return Err("Algoritmo de identidad no admitido".into()); }
    let kid=h.kid.ok_or("Falta identificador de clave de firma")?;
    let jwk=keys.find(&kid).ok_or("Clave de firma desconocida")?;
    let key=DecodingKey::from_jwk(jwk).map_err(|_|"Clave pública no admisible")?;
    let mut v=Validation::new(Algorithm::RS256);
    v.set_issuer(&[ISSUER]); v.set_audience(&[client_id]); v.set_required_spec_claims(&["exp","iss","aud","sub"]);
    v.leeway=0; v.validate_nbf=true;
    let claims=decode::<Claims>(token,&key,&v).map_err(|_|"Firma o identidad no verificadas")?.claims;
    if !ct(&claims.nonce,nonce)||claims.sub.is_empty()||claims.exp<=now()||claims.iat>now()+60||claims.iss!=ISSUER||claims.aud.is_null() {
        return Err("Identidad no vinculada a esta autorización".into());
    }
    if claims.azp.as_deref().is_some_and(|v|v!=client_id)
       || (claims.aud.as_array().is_some_and(|a|a.len()>1) && claims.azp.as_deref()!=Some(client_id)) {
        return Err("Parte autorizada discordante".into());
    }
    Ok(claims)
}
fn catalog(http: &Client, token: &str) -> R<Value> {
    let t=Instant::now();
    let response=http.get(MODELS).bearer_auth(token).send().map_err(|_|"No se obtiene catálogo")?;
    let status=response.status().as_u16();
    let bytes=body(response,4*1024*1024)?;
    let j: Value=serde_json::from_slice(&bytes).map_err(|_|"Catálogo no interpretable")?;
    let models=j.get("models").and_then(Value::as_array).ok_or("Formato de catálogo no reconocido")?;
    let mut selected=Vec::new();
    let mut all=Vec::new();
    for m in models {
        all.push(json!({"slug":m.get("slug"),"display_name":m.get("display_name"),
            "visibility":m.get("visibility"),"supported_in_api":m.get("supported_in_api"),
            "campos_declarados":m.as_object().map(|o|o.keys().collect::<Vec<_>>())}));
        if m.get("visibility").and_then(Value::as_str)==Some("list") {
            let slug=m.get("slug").and_then(Value::as_str).ok_or("Modelo sin identificador")?;
            let name=m.get("display_name").and_then(Value::as_str).ok_or("Modelo sin denominación")?;
            selected.push(json!({"slug":slug,"display_name":name}));
        }
    }
    Ok(json!({"fecha_unix":now(),"operacion":"GET /v1/models","http":status,"duracion_ms":t.elapsed().as_millis(),
        "bytes":bytes.len(),"sha256":sha(&bytes),"modelos":selected,"entradas_catalogo":all,
        "total_entradas":models.len(),
        "gpt_6_1_sol_presente_en_catalogo":models.iter().any(|m|m.get("slug").and_then(Value::as_str)==Some("gpt-6.1-sol")),
        "gpt_6_1_sol_listado":selected.iter().any(|m|m["slug"]=="gpt-6.1-sol"),
        "inferencia_ejecutada":false,"acceso_de_inferencia":"no comprobado mediante ejecución"}))
}
fn respond(stream: &mut TcpStream, code: &str, html: &str, location: Option<&str>) {
    let style=html.split_once("<style>").and_then(|(_,s)|s.split_once("</style>").map(|(s,_)|s));
    let style_policy=style.map(|s|format!("style-src 'sha256-{}'; ",base64::engine::general_purpose::STANDARD.encode(Sha256::digest(s.as_bytes())))).unwrap_or_default();
    let loc=location.map(|x|format!("Location: {x}\r\n")).unwrap_or_default();
    let header=format!("HTTP/1.1 {code}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nPragma: no-cache\r\nReferrer-Policy: no-referrer\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: default-src 'none'; {style_policy}frame-ancestors 'none'; base-uri 'none'; form-action 'none'\r\nConnection: close\r\n{loc}\r\n",html.len());
    let _=stream.write_all(header.as_bytes()); let _=stream.write_all(html.as_bytes()); let _=stream.flush();
}
fn request(stream: &mut TcpStream, host: &str) -> R<Zeroizing<String>> {
    stream.set_read_timeout(Some(Duration::from_secs(3))).map_err(|_|"Tiempo de lectura")?;
    stream.set_write_timeout(Some(Duration::from_secs(3))).map_err(|_|"Tiempo de escritura")?;
    let mut raw=Zeroizing::new(Vec::new());
    let mut b=[0u8;1024]; let started=Instant::now();
    loop {
        if started.elapsed()>Duration::from_secs(5) { return Err("Solicitud lenta".into()); }
        let n=stream.read(&mut b).map_err(|_|"Solicitud interrumpida")?;
        if n==0 { return Err("Solicitud incompleta".into()); }
        raw.extend_from_slice(&b[..n]); b.zeroize();
        if raw.len()>MAX_REQUEST { return Err("Solicitud excesiva".into()); }
        if raw.windows(4).any(|w|w==b"\r\n\r\n") {break;}
    }
    let raw=Zeroizing::new(String::from_utf8(raw.to_vec()).map_err(|_|"Solicitud no textual")?);
    let mut lines=raw.split("\r\n");
    let first=lines.next().ok_or("Solicitud sin inicio")?.split_whitespace().collect::<Vec<_>>();
    if first.len()!=3||first[0]!="GET"||first[2]!="HTTP/1.1" {return Err("Método no permitido".into());}
    let hosts:Vec<_>=lines.filter_map(|l|l.split_once(':')).filter(|(k,_)|k.eq_ignore_ascii_case("host")).map(|(_,v)|v.trim()).collect();
    if hosts!=[host] {return Err("Host de retorno discordante".into());}
    Ok(Zeroizing::new(first[1].to_string()))
}
fn main_run() -> R<()> {
    let trial=std::env::args().nth(1).is_some_and(|s|s.starts_with("prueba"));
    if trial { prueba::prepare()?; }
    for name in ["privado","sesion"] { fs::create_dir_all(name).map_err(|_|"No se crea directorio")?; }
    if Path::new(&report_path()).exists() {return Err("Ya existe resultado; conservarlo antes de otra sesión".into());}
    let mut reg:Registration=if Path::new("privado/registro.json").exists() {
        serde_json::from_slice(&fs::read("privado/registro.json").map_err(|_|"No se lee registro")?).map_err(|_|"Registro local inválido")?
    } else {
        let mut r=[0u8;16];
        getrandom::getrandom(&mut r).map_err(|_|"No se obtiene identidad aleatoria del anfitrión")?;
        r[6]=(r[6]&0x0f)|0x40; r[8]=(r[8]&0x3f)|0x80;
        let h:String=r.iter().map(|b|format!("{b:02x}")).collect();
        let host=format!("urn:uuid:{}-{}-{}-{}-{}",&h[0..8],&h[8..12],&h[12..16],&h[16..20],&h[20..32]);
        let reg=Registration{host,client:None,subject_hash:None};
        save("privado/registro.json",&serde_json::to_value(&reg).unwrap())?; reg
    };
    let http=client()?;
    let discovery_bytes=body(http.get("https://auth.openai.com/.well-known/openid-configuration").send().map_err(|_|"No se obtiene configuración oficial")?,256*1024)?;
    let d:Value=serde_json::from_slice(&discovery_bytes).map_err(|_|"Configuración oficial no interpretable")?;
    if d["issuer"]!=ISSUER||d["authorization_endpoint"]!=AUTH||d["token_endpoint"]!=TOKEN||d["jwks_uri"]!=JWKS {
        return Err("La configuración oficial no coincide con los destinos previstos".into());
    }
    let listener=TcpListener::bind("127.0.0.1:0").map_err(|_|"No se abre retorno local")?;
    listener.set_nonblocking(true).map_err(|_|"No se configura retorno")?;
    let addr=listener.local_addr().map_err(|_|"Dirección local")?;
    let p=Pending{state:Zeroizing::new(random()?),nonce:Zeroizing::new(random()?),verifier:Zeroizing::new(random()?),redirect:format!("http://{addr}/auth/callback"),existing:reg.client.clone()};
    let mut auth=Url::parse(AUTH).unwrap();
    {
        let mut pairs=auth.query_pairs_mut();
        pairs.append_pair("client_id",reg.client.as_deref().unwrap_or("dynamic_agent_client"));
        if reg.client.is_none(){pairs.append_pair("agent_name_hint",APP);}
        pairs.append_pair("ext_agent_host_id",&reg.host).append_pair("response_type","code")
            .append_pair("redirect_uri",&p.redirect).append_pair("scope",SCOPES).append_pair("resource",RESOURCE)
            .append_pair("state",&p.state).append_pair("nonce",&p.nonce).append_pair("code_challenge_method","S256")
            .append_pair("code_challenge",&URL_SAFE_NO_PAD.encode(Sha256::digest(p.verifier.as_bytes())));
    }
    println!("NAVEGADOR_LOCAL=http://{addr}/");
    std::io::stdout().flush().map_err(|_|"Salida local")?;
    let mut result:Option<String>=None;
    let mut finish_at:Option<Instant>=None;
    let started=Instant::now();
    while started.elapsed()<Duration::from_secs(1800) {
        if finish_at.is_some_and(|x|x.elapsed()>Duration::from_secs(1800)){break;}
        let (mut stream,peer)=match listener.accept(){Ok(v)=>v,Err(e)if e.kind()==std::io::ErrorKind::WouldBlock=>{std::thread::sleep(Duration::from_millis(80));continue},Err(_)=>return Err("Error en retorno local".into())};
        if !peer.ip().is_loopback(){continue;}
        let target=match request(&mut stream,&addr.to_string()){Ok(v)=>v,Err(_)=>{respond(&mut stream,"400 Bad Request","Solicitud no admitida.",None);continue}};
        match target.as_str() {
            "/" if trial => respond(&mut stream,"200 OK",&prueba::start_page(),None),
            "/resultado" if trial => respond(&mut stream,"200 OK",&prueba::result_page(result.as_deref()),None),
            "/" => respond(&mut stream,"200 OK","<!doctype html><html lang='es'><meta charset='utf-8'><title>SV · Acceso OpenAI</title><h1>Comprobación de acceso a OpenAI</h1><p>Aplicación local escrita en Rust. Registro e identidad mediante el flujo oficial de ChatGPT.</p><p>La única operación posterior será consultar el catálogo de modelos; no se enviarán casos ni se ejecutará inferencia.</p><p>Se solicitará permiso de uso del plan. Las credenciales permanecen sólo en memoria en esta comprobación. El permiso podrá revocarse desde ChatGPT.</p><p><a href='/start'>Continue with ChatGPT</a></p></html>",None),
            "/start" if result.is_none()=>respond(&mut stream,"303 See Other","Continuar con ChatGPT.",Some(auth.as_str())),
            "/resultado"=>respond(&mut stream,"200 OK",&format!("<!doctype html><html lang='es'><meta charset='utf-8'><title>SV · Resultado de acceso</title><h1>Comprobación de acceso</h1><p>{}</p><p>No se ha ejecutado inferencia.</p></html>",result.as_deref().unwrap_or("Comprobación pendiente.")),None),
            x if x.starts_with("/auth/callback?") && result.is_none()=> {
                let mut params=match query(x){Ok(v)=>v,Err(_)=>{respond(&mut stream,"400 Bad Request","Retorno no admitido.",None);continue}};
                let validated=validate_callback(&params,&p);
                for value in params.values_mut(){value.zeroize();}
                let (id,code)=match validated {Ok(v)=>v,Err(e)=>{result=Some(e.clone());write_report(json!({"fecha_unix":now(),"estado":"no_autorizado","motivo":e,"inferencia_ejecutada":false}))?;respond(&mut stream,"303 See Other","Retorno recibido.",Some("/resultado"));finish_at=Some(Instant::now());continue}};
                reg.client=Some(id.clone()); save("privado/registro.json",&serde_json::to_value(&reg).unwrap())?;
                respond(&mut stream,"303 See Other","Retorno recibido.",Some("/resultado"));
                let check=(||->R<Value>{
                    let bytes=body(http.post(TOKEN).form(&[("grant_type","authorization_code"),("client_id",id.as_str()),("code",code.as_str()),("code_verifier",p.verifier.as_str()),("redirect_uri",p.redirect.as_str()),("resource",RESOURCE)]).send().map_err(|_|"Intercambio de autorización no completado")?,256*1024)?;
                    let tokens:TokenSet=serde_json::from_slice(&bytes).map_err(|_|"Credenciales no interpretables")?;
                    if !tokens.token_type.eq_ignore_ascii_case("bearer") {return Err("Tipo de credencial no admitido".into());}
                    let keybytes=body(http.get(JWKS).send().map_err(|_|"No se obtienen claves públicas")?,512*1024)?;
                    let keys:JwkSet=serde_json::from_slice(&keybytes).map_err(|_|"Claves públicas inválidas")?;
                    let claims=verify_identity(&tokens.id_token,&keys,&id,&p.nonce)?;
                    let subject=sha(format!("{ISSUER}\0{id}\0{}",claims.sub).as_bytes());
                    if reg.subject_hash.as_ref().is_some_and(|old|old!=&subject){return Err("Cuenta distinta de la registrada".into());}
                    reg.subject_hash=Some(subject); save("privado/registro.json",&serde_json::to_value(&reg).unwrap())?;
                    let granted:Vec<&str>=tokens.scope.split_whitespace().collect();
                    if !granted.contains(&"chatgpt.tokens.use.direct")||!granted.contains(&"resource.invoke"){return Err("Permiso de uso del plan no concedido".into());}
                    let mut c=catalog(&http,&tokens.access_token)?;
                    c["identidad_verificada"]=json!(true);c["permisos"]=json!(granted);c["cliente"]=json!(APP);
                    if trial {
                        if !c["modelos"].as_array().is_some_and(|a|a.iter().any(|m|m["slug"]==prueba::MODEL)) {return Err("Astra no figura en el catálogo autorizado actual".into());}
                        save(&format!("{}/catalogo.json",prueba::dir()),&c)?;
                        return prueba::infer(&http,&tokens.access_token);
                    }
                    Ok(c)
                })();
                match check {
                    Ok(v) if trial=>{write_report(v.clone())?;let html=prueba::page(&v);fs::write(format!("{}/resultado.html",prueba::dir()),html).map_err(|_|"No se conserva pantalla")?;result=Some("Prueba finalizada; consulte el resultado registrado.".into());println!("PRUEBA_CONCLUIDA={}",v["estado"]);}
                    Ok(v)=>{result=Some(if v["gpt_6_1_sol_listado"]==true{"Identidad y permisos verificados. GPT-6.1 Sol aparece en el catálogo autorizado."}else{"Identidad y permisos verificados. GPT-6.1 Sol no aparece en el catálogo autorizado."}.into());write_report(v)?;println!("COMPROBACION_CONCLUIDA");}
                    Err(e)=>{result=Some(e.clone());let sent=trial && Path::new(&format!("{}/envio-unico.json",prueba::dir())).exists();write_report(json!({"fecha_unix":now(),"estado":"impedimento","motivo":e,"inferencia_ejecutada":if sent {Value::Null}else{json!(false)},"envio_iniciado_o_incierto":sent,"reintentos":0}))?;println!("COMPROBACION_CON_IMPEDIMENTO");}
                }
                finish_at=Some(Instant::now());
            }
            _=>respond(&mut stream,"404 Not Found","Recurso no disponible.",None),
        }
    }
    if result.is_none(){return Err("Venció el plazo de autorización local".into());}
    Ok(())
}
fn select_model(slug: &str) -> R<()> {
    let source=fs::read("sesion/resultado.json").map_err(|_|"Falta comprobación conservada")?;
    let record:Value=serde_json::from_slice(&source).map_err(|_|"Comprobación no interpretable")?;
    if record["identidad_verificada"]!=true||record["http"]!=200 {
        return Err("La comprobación no acredita identidad y catálogo".into());
    }
    let models=record["modelos"].as_array().ok_or("Falta catálogo comprobado")?;
    let m=models.iter().find(|m|m["slug"].as_str()==Some(slug)).ok_or("El candidato no figura en el catálogo comprobado")?;
    save("sesion/seleccion-modelo.json",&json!({
        "fecha_unix":now(),"proveedor":"OpenAI","modelo":m,
        "origen":"selección humana expresa posterior a la consulta del catálogo",
        "comprobacion_sha256":sha(&source),"inferencia_ejecutada":false,
        "cambio_de_nombre_de_carpeta":"pendiente de la continuación acordada"
    }))?;
    println!("SELECCION_CONFIRMADA={slug}");
    Ok(())
}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    if args.get(1).map(String::as_str)==Some("seleccionar") {
        if let Some(slug)=args.get(2) { if let Err(e)=select_model(slug) { eprintln!("{e}"); std::process::exit(1); } return; }
        eprintln!("Falta identificador del modelo"); std::process::exit(1);
    }
    if let Err(e)=main_run(){
        eprintln!("Acceso no completado: {e}");
        if !Path::new(&report_path()).exists() { let _=write_report(json!({"fecha_unix":now(),"estado":"impedimento","motivo":e,"inferencia_ejecutada":false})); }
        std::process::exit(1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn firmas_identidad_y_contexto() {
        use rsa::{RsaPrivateKey, pkcs1::EncodeRsaPrivateKey, traits::PublicKeyParts};
        use jsonwebtoken::{encode, Header, EncodingKey};
        let private=RsaPrivateKey::new(&mut rand::thread_rng(),2048).unwrap();
        let public=private.to_public_key();
        let keys:JwkSet=serde_json::from_value(json!({"keys":[{
            "kty":"RSA","kid":"prueba","alg":"RS256","use":"sig",
            "n":URL_SAFE_NO_PAD.encode(public.n().to_bytes_be()),
            "e":URL_SAFE_NO_PAD.encode(public.e().to_bytes_be())}]})).unwrap();
        let der=private.to_pkcs1_der().unwrap();
        let key=EncodingKey::from_rsa_der(der.as_bytes());
        let mut header=Header::new(Algorithm::RS256); header.kid=Some("prueba".into());
        let base=json!({"sub":"sujeto-artificial","nonce":"nonce","exp":now()+3600,"iat":now(),"iss":ISSUER,"aud":"oaiapp_prueba"});
        let sign=|v:&Value|encode(&header,v,&key).unwrap();
        assert!(verify_identity(&sign(&base),&keys,"oaiapp_prueba","nonce").is_ok());
        assert!(verify_identity(&sign(&base),&keys,"oaiapp_otro","nonce").is_err());
        assert!(verify_identity(&sign(&base),&keys,"oaiapp_prueba","otro").is_err());
        let mut expired=base.clone();expired["exp"]=json!(now()-60);
        assert!(verify_identity(&sign(&expired),&keys,"oaiapp_prueba","nonce").is_err());
        let mut issuer=base.clone();issuer["iss"]=json!("https://example.invalid");
        assert!(verify_identity(&sign(&issuer),&keys,"oaiapp_prueba","nonce").is_err());
        let signed=sign(&base);
        let mut sections=signed.split('.').map(str::to_string).collect::<Vec<_>>();
        let mut altered=base.clone(); altered["sub"]=json!("otro-sujeto-artificial");
        sections[1]=URL_SAFE_NO_PAD.encode(serde_json::to_vec(&altered).unwrap());
        assert!(verify_identity(&sections.join("."),&keys,"oaiapp_prueba","nonce").is_err());
    }
    fn p()->Pending{Pending{state:Zeroizing::new("estado".into()),nonce:Zeroizing::new("nonce".into()),verifier:Zeroizing::new("verificador".into()),redirect:"http://127.0.0.1:3000/auth/callback".into(),existing:None}}
    #[test]fn rechazo_estado(){assert!(validate_callback(&query("/auth/callback?state=otro&code=x&client_id=oaiapp_prueba").unwrap(),&p()).is_err());}
    #[test]fn rechazo_duplicado(){assert!(query("/auth/callback?state=x&state=x").is_err());}
    #[test]fn rechazo_denegado(){assert!(validate_callback(&query("/auth/callback?state=estado&error=access_denied").unwrap(),&p()).is_err());}
    #[test]fn cliente_emitido_obligatorio(){assert!(validate_callback(&query("/auth/callback?state=estado&code=x").unwrap(),&p()).is_err());}
    #[test]fn cliente_previo_no_sustituible(){let mut v=p();v.existing=Some("oaiapp_a".into());assert!(validate_callback(&query("/auth/callback?state=estado&code=x&client_id=oaiapp_b").unwrap(),&v).is_err());}
    #[test]fn retorno_valido(){assert_eq!(validate_callback(&query("/auth/callback?state=estado&code=x&client_id=oaiapp_prueba").unwrap(),&p()).unwrap().0,"oaiapp_prueba");}
    #[test]fn rechazo_token_sin_firma(){let keys:JwkSet=serde_json::from_value(json!({"keys":[]})).unwrap();assert!(verify_identity("eyJhbGciOiJub25lIn0.e30.",&keys,"oaiapp_prueba","nonce").is_err());drop(keys);}
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

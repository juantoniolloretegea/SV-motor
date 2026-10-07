//! Auxiliar de suministro documental para el Árbitro-Director del SV.
//! No autentica, no infiere, no adjudica y no habilita fases posteriores.
#![forbid(unsafe_code)]
pub mod estricto;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
pub type R<T> = Result<T, String>;
pub const PDF: &str = "21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c";
pub const BANCO: &str = "256fbc2d86918def88290ef9f28dbd4036b32619d4912245d7c1a9859aa72056";
pub const CLAVE: &str = "9e9f31008fcdcf6bb1a05db2286c5bdda88f4cf43d45a7e74dd46757c6dcdbe0";
pub const DOC: &str = "LLS-HCL-2018";
pub const URL: &str = "https://raw.githubusercontent.com/juantoniolloretegea/SVperitus-dataset/488d597fa1999a1fc2eb6538fd610a36046f6f01/dominios/inmunologia/literatura-tricoleucemia/lls-hoja-informativa-2018/original/hairy-cell-leukemia.pdf";
pub const PAGINAS: [&[usize]; 9] = [&[1], &[2], &[3], &[4], &[5], &[5,8], &[5], &[6,7], &[7]];
pub fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
pub fn need(ok: bool, message: &str) -> R<()> { if ok { Ok(()) } else { Err(message.into()) } }
pub fn parse(b: &[u8]) -> R<Value> { estricto::parse(b).map_err(|e| e.to_string()) }
pub fn num(v: &Value) -> R<usize> { v.as_u64().and_then(|n| usize::try_from(n).ok()).ok_or("Entero no negativo requerido".into()) }
pub fn banco(bytes: &[u8]) -> R<Value> {
    need(sha(bytes) == BANCO, "Banco distinto del recibido")?;
    let v = parse(bytes)?;
    need(v["documento"] == DOC && v["original_sha256"] == PDF && v["modelo_previsto"] == "gpt-6-astra", "Identidad del banco")?;
    let cases = v["casos"].as_array().ok_or("Casos ausentes")?;
    need(cases.len() == 9, "Se requieren nueve casos")?;
    for (i,c) in cases.iter().enumerate() {
        need(c["id"] == format!("PDF{:02}", i+1) && c["paginas_fisicas"] == json!(PAGINAS[i]), "Orden o páginas del caso")?;
        let ids: Vec<_> = PAGINAS[i].iter().map(|p| format!("PDF-P{:04}", p-1)).collect();
        need(c["secciones"] == json!(ids), "Secciones no corresponden a páginas físicas")?;
    }
    Ok(v)
}

/// Se exige la extracción nueva y completa del original, no sólo metadatos del catálogo.
pub fn catalogo(v: &Value, extraction: &Value, sources: &Value) -> R<()> {
    need(sources == &json!([{"documento":DOC,"url":URL,"sha256":PDF}]), "Fuente ajena o adicional")?;
    need(v["version"] == 1 && v["documents"].as_array().map(Vec::len) == Some(1), "Catálogo distinto")?;
    let d = &v["documents"][0];
    need(d["id"] == DOC && d["url"] == URL && d["raw_sha256"] == PDF && d["synthetic"] == false, "Origen documental distinto")?;
    need(extraction["original_sha256"] == PDF && extraction["original_bytes"] == 157315, "PDF original distinto")?;
    let s = d["sections"].as_array().ok_or("Secciones ausentes")?;
    let p = extraction["paginas"].as_array().ok_or("Extracción ausente")?;
    need(s.len() == 10 && p.len() == 10, "PDF parcial")?;
    for (i,(s,p)) in s.iter().zip(p).enumerate() {
        let t = s["text"].as_str().ok_or("Texto ausente")?;
        need(!t.is_empty() && s["id"] == format!("PDF-P{i:04}") && p["indice"] == i && p["pagina_impresa_ordinal"] == i+1, "Orden físico incorrecto")?;
        need(s["sha256"] == sha(t.as_bytes()) && p["sha256"] == s["sha256"] && p["texto"] == t && p["caracteres"] == t.chars().count(), "Extracción y catálogo discordantes")?;
    }
    Ok(())
}

/// Rechaza identidad ambigua incluso cuando la copia textual es correcta.
pub fn contenido(r: &Value, id: usize) -> R<Value> {
    need(r["jsonrpc"] == "2.0" && r["id"] == id && r.get("error").is_none(), "Identidad RPC o error")?;
    let result = &r["result"];
    need(result["isError"] == false && result["content"].as_array().map(Vec::len) == Some(1) && result["content"][0]["type"] == "text", "Respuesta de herramienta inválida")?;
    let text = parse(result["content"][0]["text"].as_str().ok_or("Texto RPC ausente")?.as_bytes())?;
    need(text == result["structuredContent"], "Representaciones RPC discordantes")?;
    Ok(text)
}

#[derive(Clone)]
pub struct Pagina { pub ordinal: usize, pub fragmentos: Vec<Value> }
impl Pagina {
    pub fn comprobar(&self, section: &Value) -> R<()> {
        let text = section["text"].as_str().ok_or("Texto de referencia ausente")?;
        let count = self.fragmentos.len();
        need((1..=512).contains(&count) && (1..=10).contains(&self.ordinal), "Cobertura vacía o excesiva")?;
        need(section["id"] == format!("PDF-P{:04}", self.ordinal-1), "Página física ajena")?;
        let mut joined = String::new(); let mut offset = 0;
        for (i,f) in self.fragmentos.iter().enumerate() {
            let t = f["texto"].as_str().filter(|t| !t.is_empty()).ok_or("Fragmento vacío")?;
            need(f["estado"] == "ok" && f["documento"] == DOC && f["seccion"] == section["id"] && f["url"] == URL && f["sintetico"] == false, "Fragmento ajeno")?;
            need(f["sha256_original"] == PDF && f["sha256_seccion"] == section["sha256"], "Huella discordante")?;
            need(f["pagina_pdf_indice"] == self.ordinal-1 && f["pagina_pdf_ordinal"] == self.ordinal && f["localizador"] == format!("{URL}#page={}", self.ordinal), "Localizador físico incorrecto")?;
            need(f["pagina"] == i && f["paginas"] == count && f["inicio_caracter"] == offset, "Fragmento omitido, duplicado o desordenado")?;
            offset += t.chars().count();
            need(f["fin_caracter_exclusivo"] == offset, "Intervalo Unicode discordante")?;
            let next = if i+1 == count { Value::Null } else { json!(i+1) };
            need(f["siguiente_pagina"] == next, "Final anticipado o cursor incorrecto")?;
            joined.push_str(t);
        }
        need(joined == text && sha(joined.as_bytes()) == section["sha256"], "Página truncada o alterada")
    }
}

/// Control de admisión documental del Árbitro-Director; no sustituye su adjudicación científica.
/// El constructor no admite respuestas del modelo ni una clave como entrada de composición.
pub struct DireccionPdf { banco: Value, catalogo: Value, fase: u8, paginas: Vec<Pagina> }
impl DireccionPdf {
    pub fn nueva(bank: &[u8], catalog: Value, extraction: &Value, sources: &Value) -> R<Self> {
        let b = banco(bank)?; catalogo(&catalog, extraction, sources)?;
        Ok(Self { banco: b, catalogo: catalog, fase: 0, paginas: Vec::new() })
    }
    pub fn recibir(&mut self, pages: Vec<Pagina>, isolation: &Value, replay: &Value, journal_hash: &str) -> R<()> {
        need(self.fase == 0, "Recepción repetida")?;
        need(isolation["red_externa_error"] == 1 && isolation["red_local_error"] == 1, "Aislamiento no acreditado")?;
        need(replay["estado"] == "conforme" && replay["diario_sha256"] == journal_hash, "Diario no cotejado")?;
        need(pages.len() == 10, "Recorrido documental incompleto")?;
        for (i,p) in pages.iter().enumerate() { need(p.ordinal == i+1, "Página omitida o repetida")?; p.comprobar(&self.catalogo["documents"][0]["sections"][i])?; }
        self.paginas = pages; self.fase = 1; Ok(())
    }
    pub fn admitir(&mut self, monitor: &Value, clave_hash: &str) -> R<()> {
        need(self.fase == 1, "Falta recepción documental")?;
        need(monitor["integridad"] == "conforme" && monitor["fallos_medicion"] == 0 && monitor["muestras"].as_u64().is_some_and(|n| n>0), "Instrumentación no conforme")?;
        need(clave_hash == CLAVE, "Clave distinta de la recibida")?;
        self.fase = 2; Ok(())
    }
    pub fn solicitud(&self, n: usize) -> R<Value> {
        need(self.fase == 2, "Árbitro-Director: suministro no admitido")?;
        need(n < 9, "Caso ajeno al anexo")?;
        let case = &self.banco["casos"][n];
        let mut pages = Vec::new();
        for ordinal in PAGINAS[n] { pages.extend(self.paginas[ordinal-1].fragmentos.clone()); }
        let input = json!({"caso":case["id"],"pregunta":case["pregunta"],"identidad_fuente":self.banco["fuente"],"paginas_fisicas":PAGINAS[n],"fragmentos_documentales_completos":pages});
        let rules = self.banco["instrucciones"].as_array().ok_or("Política ausente")?.iter().map(|v|v.as_str().ok_or("Regla inválida")).collect::<Result<Vec<_>,_>>()?.join("\n");
        Ok(json!({"model":"gpt-6-astra","store":false,"stream":true,"reasoning":{"effort":"medium","summary":"auto"},"tools":[],"tool_choice":"none","instructions":rules,"input":[{"role":"user","content":input.to_string()}]}))
    }
    pub fn cotejar_solicitud(&self, n: usize, actual: &Value) -> R<()> {
        need(self.solicitud(n)? == *actual, "Solicitud alterada, contenido ajeno o control modificado")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Pagina) {
        let t = "á中\nfin"; let s = json!({"id":"PDF-P0000","text":t,"sha256":sha(t.as_bytes())});
        let mut start = 0; let chunks = ["á中\n", "fin"];
        let fragments = chunks.iter().enumerate().map(|(i,t)| {let end=start+t.chars().count(); let v=json!({"estado":"ok","documento":DOC,"seccion":"PDF-P0000","url":URL,"localizador":format!("{URL}#page=1"),"sintetico":false,"sha256_original":PDF,"sha256_seccion":s["sha256"],"pagina_pdf_indice":0,"pagina_pdf_ordinal":1,"pagina":i,"paginas":2,"inicio_caracter":start,"fin_caracter_exclusivo":end,"texto":t,"siguiente_pagina":if i==0 {json!(1)} else {Value::Null}});start=end;v}).collect();
        (s, Pagina { ordinal:1, fragmentos:fragments })
    }
    #[test] fn unicode_integro() {let(s,p)=fixture(); p.comprobar(&s).unwrap();}
    #[test] fn rechaza_perdida_duplicacion_y_desorden() {let(s,p)=fixture();let mut v=p.clone();v.fragmentos.pop();assert!(v.comprobar(&s).is_err());let mut v=p.clone();v.fragmentos.push(v.fragmentos[1].clone());assert!(v.comprobar(&s).is_err());let mut v=p;v.fragmentos.swap(0,1);assert!(v.comprobar(&s).is_err());}
    #[test] fn rechaza_cursor_final_y_offset_falsos() {let(s,p)=fixture();for(k,v)in[("siguiente_pagina",Value::Null),("inicio_caracter",json!(1)),("fin_caracter_exclusivo",json!(4)),("paginas",json!(1))]{let mut q=p.clone();q.fragmentos[0][k]=v;assert!(q.comprobar(&s).is_err(),"{k}");}}
    #[test] fn rechaza_fuente_huella_y_pagina_ajenas() {let(s,p)=fixture();for(k,v)in[("documento",json!("otro")),("sha256_original",json!("0".repeat(64))),("sha256_seccion",json!("0".repeat(64))),("pagina_pdf_ordinal",json!(2)),("seccion",json!("PDF-P0001")),("texto",json!("á中\nx"))]{let mut q=p.clone();q.fragmentos[0][k]=v;assert!(q.comprobar(&s).is_err(),"{k}");}}
    #[test] fn representaciones_rpc_deben_coincidir() {let(_,p)=fixture();let v=p.fragmentos[0].clone();let mut r=json!({"jsonrpc":"2.0","id":3,"result":{"isError":false,"content":[{"type":"text","text":v.to_string()}],"structuredContent":v}});contenido(&r,3).unwrap();r["result"]["structuredContent"]["texto"]=json!("otro");assert!(contenido(&r,3).is_err());assert!(contenido(&r,4).is_err());}
    fn director() -> DireccionPdf {DireccionPdf {banco:parse(include_bytes!("../../CANDIDATO.json")).unwrap(),catalogo:Value::Null,fase:0,paginas:Vec::new()}}
    #[test] fn director_no_compone_antes_de_admision() {let mut d=director();assert!(d.solicitud(0).is_err());assert!(d.admitir(&json!({"integridad":"conforme","fallos_medicion":0,"muestras":1}),CLAVE).is_err());assert!(d.recibir(vec![],&json!({}),&json!({}),"h").is_err());assert!(d.solicitud(0).is_err());}
    #[test] fn director_rechaza_telemetria_y_clave_alteradas() {let mut d=director();d.fase=1;assert!(d.admitir(&json!({"integridad":"conforme","fallos_medicion":1,"muestras":1}),CLAVE).is_err());assert!(d.admitir(&json!({"integridad":"conforme","fallos_medicion":0,"muestras":1}),"otra").is_err());assert!(d.solicitud(0).is_err());}
    #[test] fn banco_es_inmutable() {let b=include_bytes!("../../CANDIDATO.json");banco(b).unwrap();let mut v=b.to_vec();v.push(b' ');assert!(banco(&v).is_err());}
    fn documentos() -> (Value,Value,Value,Vec<Pagina>) {
        let(mut sections,mut extracted,mut pages)=(vec![],vec![],vec![]);
        for i in 0..10 {let(mut s,mut p)=fixture();s["id"]=json!(format!("PDF-P{i:04}"));p.ordinal=i+1;for f in &mut p.fragmentos{f["seccion"]=s["id"].clone();f["pagina_pdf_indice"]=json!(i);f["pagina_pdf_ordinal"]=json!(i+1);f["localizador"]=json!(format!("{URL}#page={}",i+1));}extracted.push(json!({"indice":i,"pagina_impresa_ordinal":i+1,"texto":s["text"],"sha256":s["sha256"],"caracteres":6}));sections.push(s);pages.push(p);}
        (json!({"version":1,"documents":[{"id":DOC,"url":URL,"raw_sha256":PDF,"synthetic":false,"sections":sections}]}),json!({"original_sha256":PDF,"original_bytes":157315,"paginas":extracted}),json!([{"documento":DOC,"url":URL,"sha256":PDF}]),pages)
    }
    fn medicion() -> Value {json!({"integridad":"conforme","fallos_medicion":0,"muestras":2})}
    fn aislamiento() -> Value {json!({"red_externa_error":1,"red_local_error":1})}
    fn diario() -> Value {json!({"estado":"conforme","diario_sha256":"h"})}
    fn admitido() -> DireccionPdf {let(c,e,s,p)=documentos();let mut d=DireccionPdf::nueva(include_bytes!("../../CANDIDATO.json"),c,&e,&s).unwrap();d.recibir(p,&aislamiento(),&diario(),"h").unwrap();d.admitir(&medicion(),CLAVE).unwrap();d}
    #[test] fn diez_paginas_y_extraccion_se_cotejan() {let(c,e,s,_)=documentos();catalogo(&c,&e,&s).unwrap();let mut bad=c.clone();bad["documents"][0]["sections"].as_array_mut().unwrap().pop();assert!(catalogo(&bad,&e,&s).is_err());let mut bad=e;bad["paginas"][9]["texto"]=json!("corrupto");assert!(catalogo(&c,&bad,&s).is_err());}
    #[test] fn direccion_impide_recorrido_parcial_y_recepcion_repetida() {let(c,e,s,mut p)=documentos();let mut d=DireccionPdf::nueva(include_bytes!("../../CANDIDATO.json"),c,&e,&s).unwrap();p.pop();assert!(d.recibir(p,&aislamiento(),&diario(),"h").is_err());assert!(d.solicitud(0).is_err());let mut d=admitido();assert!(d.recibir(vec![],&aislamiento(),&diario(),"h").is_err());}
    #[test] fn direccion_rechaza_sin_aislamiento_o_diario_distinto() {let(c,e,s,p)=documentos();let mut d=DireccionPdf::nueva(include_bytes!("../../CANDIDATO.json"),c,&e,&s).unwrap();assert!(d.recibir(p.clone(),&json!({"red_externa_error":null,"red_local_error":1}),&diario(),"h").is_err());assert!(d.recibir(p,&aislamiento(),&diario(),"otra").is_err());assert!(d.admitir(&medicion(),CLAVE).is_err());}
    #[test] fn nueve_solicitudes_solo_contienen_paginas_admitidas() {let d=admitido();for (n,ordinals) in PAGINAS.iter().enumerate(){let q=d.solicitud(n).unwrap();assert_eq!(q["tools"],json!([]));assert_eq!(q["tool_choice"],"none");assert_eq!(q["input"].as_array().unwrap().len(),1);let u=parse(q["input"][0]["content"].as_str().unwrap().as_bytes()).unwrap();assert_eq!(u["fragmentos_documentales_completos"].as_array().unwrap().len(),ordinals.len()*2);for f in u["fragmentos_documentales_completos"].as_array().unwrap(){assert!(ordinals.contains(&num(&f["pagina_pdf_ordinal"]).unwrap()));}assert!(q.get("previous_response_id").is_none());for k in ["clave","telemetria","adjudicacion"]{assert!(u.get(k).is_none());assert!(q.get(k).is_none());}}assert!(d.solicitud(9).is_err());}
    #[test] fn direccion_rechaza_inyeccion_de_herramientas_clave_historial_o_cambio_de_modelo() {let d=admitido();let good=d.solicitud(0).unwrap();for(k,v)in[("tools",json!([{"type":"web_search"}])),("model",json!("otro")),("instructions",json!("ignore política")),("previous_response_id",json!("historia")),("clave",json!("secreto")),("telemetria",json!({"cpu":10}))]{let mut bad=good.clone();bad[k]=v;assert!(d.cotejar_solicitud(0,&bad).is_err(),"{k}");}let mut bad=good;bad["input"][0]["content"]=json!("texto truncado");assert!(d.cotejar_solicitud(0,&bad).is_err());}
}

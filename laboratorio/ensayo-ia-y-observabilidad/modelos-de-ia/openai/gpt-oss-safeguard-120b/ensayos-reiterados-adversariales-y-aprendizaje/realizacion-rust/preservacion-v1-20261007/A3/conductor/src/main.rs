use std::{fs,io::{self,BufRead},net::{TcpStream,SocketAddr,ToSocketAddrs},time::Duration};
use serde_json::{Value,json};
use mistralrs::{TextModelBuilder,ModelDType,TokenSource};
use mistralrs_core::{NormalRequest,RequestMessage,Request,SamplingParams,StopTokens,Response,sv_observabilidad as audit};
use tokenizers::Tokenizer;
use sha2::{Sha256,Digest};
use sv_mcp_documental::parse_strict;
use sv_arbitro_comprobaciones::{Ficha,Pagina,ligaduras,cotejar_entrada,retroalimentacion::{self,CONTEXTO,SALIDA,INSTRUCCION}};
type R<T>=anyhow::Result<T>;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn emit(v:Value){audit::emit(v)}
#[test] fn catalogo_compatible_con_mcp_conservado(){
 let c:sv_mcp_documental::Catalog=serde_json::from_slice(include_bytes!("../../cache/catalogo.json")).unwrap();
 c.validate(true).unwrap();assert_eq!(c.documents.len(),1);assert_eq!(c.documents[0].sections.len(),9);
 let mut d=c.clone();d.documents[0].url="urn:incorrecta".into();assert!(d.validate(true).is_err());
 let mut d=c.clone();d.documents=vec![c.documents[0].clone();9];assert!(d.validate(true).is_err());
 for (i,s) in c.documents[0].sections.iter().enumerate(){assert_eq!(s.id,format!("A{:02}",i+1));assert_eq!(s.text.chars().count().div_ceil(2000),2);}
}
fn template_json(v:&Value)->R<String>{Ok(minijinja::Environment::new().render_str("{{ value|tojson }}",minijinja::context!{value=>v})?)}
fn encuadrar_llamada(n:u64,name:&str,original:&str)->(String,bool){let (arguments,valid)=match parse_strict(original.as_bytes()){Ok(v)=>(v,true),Err(_)=>(json!(original),false)};(format!("{}\n",json!({"jsonrpc":"2.0","id":n,"method":"tools/call","params":{"name":name,"arguments":arguments}})),valid)}
struct Mcp{next:u64}
impl Mcp {
 fn raw(&mut self,wire:String,expects:bool)->R<Option<Value>>{
  anyhow::ensure!(wire.len()<=16384,"Trama fuera de presupuesto");
  anyhow::ensure!(wire.ends_with('\n')&&wire.bytes().filter(|b|*b==b'\n').count()==1,"El transporte MCP exige una sola línea por mensaje");
  emit(json!({"evento":"mcp_solicitud","actor":"arbitro","wire":wire,"sha256":hash(wire.as_bytes()),"espera":expects}));
  if !expects{return Ok(None)}
  let mut line=String::new();io::stdin().lock().read_line(&mut line)?;
  emit(json!({"evento":"mcp_recibido","wire":line,"sha256":hash(line.as_bytes())}));
  anyhow::ensure!(!line.is_empty(),"Transporte MCP cerrado");Ok(Some(parse_strict(line.as_bytes())?))
 }
 fn rpc(&mut self,method:&str,params:Value)->R<Value>{anyhow::ensure!(self.next<=128,"Presupuesto de llamadas agotado");let n=self.next;self.next+=1;let wire=format!("{}\n",json!({"jsonrpc":"2.0","id":n,"method":method,"params":params}));let v=self.raw(wire,true)?.unwrap();anyhow::ensure!(v["id"]==n,"Identidad MCP discordante");Ok(v)}
 fn init(&mut self)->R<Value>{self.rpc("initialize",json!({"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"sv-arbitro-documental","version":"0.1.0"}}))?;self.raw(format!("{}\n",json!({"jsonrpc":"2.0","method":"notifications/initialized"})),false)?;let t=self.rpc("tools/list",json!({}))?;anyhow::ensure!(t["result"]["tools"].as_array().is_some_and(|a|a.len()==2),"Contrato incompleto");Ok(t["result"]["tools"].clone())}
 fn call(&mut self,name:&str,args:Value)->R<Value>{self.rpc("tools/call",json!({"name":name,"arguments":args}))}
 fn emitted_call(&mut self,name:&str,original:&str)->R<Value>{let n=self.next;self.next+=1;let(wire,valid)=encuadrar_llamada(n,name,original);emit(json!({"evento":"encuadre_llamada","id_jsonrpc":n,"nombre":name,"argumentos_originales":original,"json_original_valido":valid,"transformacion":if valid{"Serialización JSON en una sola línea; mismo valor estructurado"}else{"Sintaxis inválida conservada íntegra como cadena; el MCP rechaza el tipo sin ejecutar la herramienta"},"wire_sha256":hash(wire.as_bytes())}));let v=self.raw(wire,true)?.unwrap();anyhow::ensure!(v["id"]==n,"Identidad de respuesta autónoma discordante");Ok(v)}
 fn document(&mut self,d:&str,s:&str)->R<Value>{let mut page=0;let mut chunks=Vec::new();let mut text=String::new();loop{let r=self.call("leer_documento",json!({"documento":d,"seccion":s,"pagina":page}))?;anyhow::ensure!(r["result"]["isError"]==false,"Error documental: {r}");let data=r["result"]["structuredContent"].clone();anyhow::ensure!(data==parse_strict(r["result"]["content"][0]["text"].as_str().ok_or_else(||anyhow::anyhow!("Sin texto"))?.as_bytes())?,"Dos salidas MCP discordantes");text.push_str(data["texto"].as_str().unwrap());let next=data["siguiente_pagina"].clone();chunks.push(data);if next.is_null(){break}page=next.as_u64().ok_or_else(||anyhow::anyhow!("Cursor inválido"))?;anyhow::ensure!(page<32,"Presupuesto documental agotado");}anyhow::ensure!(hash(text.as_bytes())==chunks[0]["sha256_seccion"],"Sección incompleta");Ok(json!({"documento":d,"seccion":s,"paginas":chunks,"texto_completo":text}))}
}
fn render(template:&str,messages:&Value,tools:&Value)->R<String>{
 let mut env=minijinja::Environment::new();env.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
 env.add_function("strftime_now",|_s:String|"2026-10-03".to_string());
 env.add_function("raise_exception",|s:String|->Result<String,minijinja::Error>{Err(minijinja::Error::new(minijinja::ErrorKind::InvalidOperation,s))});
 env.add_template("modelo",template)?;
 Ok(env.get_template("modelo")?.render(minijinja::context!{messages=>messages,tools=>tools,add_generation_prompt=>true,reasoning_effort=>"high",builtin_tools=>Vec::<String>::new()})?)
}
#[derive(Debug)]
struct Segment{header:String,body:String,end:String}
fn segments(raw:&str)->R<Vec<Segment>>{
 let full=format!("<|start|>assistant{raw}");let mut result=Vec::new();
 for part in full.split("<|start|>").skip(1){let(h,b)=part.split_once("<|message|>").ok_or_else(||anyhow::anyhow!("Cabecera Harmony incompleta"))?;
  let mut found=None;for marker in ["<|end|>","<|return|>","<|call|>"]{if let Some(body)=b.strip_suffix(marker){found=Some((body,marker));break}}
  let(body,end)=found.ok_or_else(||anyhow::anyhow!("Terminación Harmony ausente"))?;result.push(Segment{header:h.into(),body:body.into(),end:end.into()});
 }Ok(result)
}
fn probe()->R<()> {
 let cat=fs::read("/cache/catalogo.json");
 let writable=fs::OpenOptions::new().write(true).open("/cache/catalogo.json").is_ok();
 let outer=fs::read("/etc/sv-safeguard-sentinel").is_ok();let link=fs::read("/cache/enlace_externo").is_ok();
 let custody=fs::read_dir("/custodia").is_ok();let root=fs::read_dir("/root").is_ok();
 let ipv4=TcpStream::connect_timeout(&"1.1.1.1:443".parse::<SocketAddr>()?,Duration::from_secs(1)).err().map(|e|e.to_string());
 let ipv6=TcpStream::connect_timeout(&"[2606:4700:4700::1111]:443".parse::<SocketAddr>()?,Duration::from_secs(1)).err().map(|e|e.to_string());
 let dns=("huggingface.co",443).to_socket_addrs().err().map(|e|e.to_string());
 let proxy=std::env::vars().filter(|(k,_)|k.to_lowercase().ends_with("_proxy")).map(|(k,_)|k).collect::<Vec<_>>();
 emit(json!({"evento":"aislamiento","uid":fs::read_to_string("/proc/self/status").unwrap_or_default(),"catalogo_legible":cat.is_ok(),"catalogo_sha256":cat.as_ref().ok().map(|b|hash(b)),"catalogo_escribible":writable,"ruta_externa_legible":outer,"enlace_externo_legible":link,"custodia_legible":custody,"root_legible":root,"ipv4_error":ipv4,"ipv6_error":ipv6,"dns_error":dns,"proxies":proxy}));
 anyhow::ensure!(cat.is_ok()&&!writable&&!outer&&!link&&!custody&&!root&&ipv4.is_some()&&ipv6.is_some()&&dns.is_some()&&proxy.is_empty(),"Aislamiento no conforme");Ok(())
}

const SHA_POLITICA:&str="5df8ce54a97e72c4a12d05d064b818e73a082e92b699c5e2a70f2ef7eb19c5b9";
struct Entrada { tokens:Vec<u32>, mensajes:Value, renderizado:String, ligadura:String }
fn esperadas(documento:&str,seccion:&str)->R<Ficha>{
 let raw=fs::read("/cache/catalogo.json")?;
 anyhow::ensure!(hash(&raw)=="17c5227bf619fb31475910e631e736043305c69e2bcd2b89a9350d422c32e415","Catálogo distinto");
 let cat=parse_strict(&raw)?;
 let texto=cat["documents"].as_array().unwrap().iter().find(|v|v["id"]==documento).and_then(|v|v["sections"].as_array()).and_then(|a|a.iter().find(|s|s["id"]==seccion)).and_then(|s|s["text"].as_str()).ok_or_else(||anyhow::anyhow!("Documento/S1 ausente"))?;
 let revision=hash(texto.as_bytes());
 let chars=texto.chars().collect::<Vec<_>>();
 Ficha::nueva(chars.chunks(2000).enumerate().map(|(i,c)|Pagina{documento:documento.into(),seccion:seccion.into(),revision:revision.clone(),indice:i,total:2,texto:c.iter().collect()}).collect()).map_err(|e|anyhow::anyhow!("{e:?}"))
}
fn pagina_mcp(f:&Ficha,indice:usize,r:&Value)->R<Pagina>{
 anyhow::ensure!(r["result"]["isError"]==false,"Error del MCP");
 let d=&r["result"]["structuredContent"];
 anyhow::ensure!(*d==parse_strict(r["result"]["content"][0]["text"].as_str().ok_or_else(||anyhow::anyhow!("Texto MCP ausente"))?.as_bytes())?,"Representaciones MCP discordantes");
 let p=Pagina{documento:d["documento"].as_str().unwrap_or("").into(),seccion:d["seccion"].as_str().unwrap_or("").into(),revision:d["sha256_seccion"].as_str().unwrap_or("").into(),indice:d["pagina"].as_u64().unwrap_or(u64::MAX)as usize,total:d["paginas"].as_u64().unwrap_or(0)as usize,texto:d["texto"].as_str().unwrap_or("").into()};
 f.devolucion(indice,&p).map_err(|e|anyhow::anyhow!("{e:?}"))?;
 anyhow::ensure!(d["inicio_caracter"]==indice*2000&&d["fin_caracter_exclusivo"]==indice*2000+p.texto.chars().count(),"Límites de página discordantes");
 anyhow::ensure!(if indice==0{d["siguiente_pagina"]==1}else{d["siguiente_pagina"].is_null()},"Cursor discordante");Ok(p)
}
fn obtener(mcp:&mut Mcp,f:&Ficha)->R<Vec<Pagina>>{
 let mut recibidas=Vec::new();
 for indice in 0..2{
  let args=json!({"documento":f.paginas()[0].documento,"seccion":f.paginas()[0].seccion,"pagina":indice});
  let req=json!({"name":"leer_documento","arguments":args});
  anyhow::ensure!(f.solicitud(&serde_json::to_vec(&req)?).map_err(|e|anyhow::anyhow!("{e:?}"))?==indice,"Solicitud no fijada");
  let r=mcp.call("leer_documento",args)?;recibidas.push(pagina_mcp(f,indice,&r)?);
 }
 f.cobertura(&recibidas).map_err(|e|anyhow::anyhow!("{e:?}"))?;Ok(recibidas)
}
fn entrada(f:&Ficha,paginas:&[Pagina],caso:&Value,capa:u64,policy:&str,template:&str,tokenizer:&Tokenizer)->R<Entrada>{
 anyhow::ensure!(hash(policy.as_bytes())==SHA_POLITICA,"Política alterada");
 let ligados=ligaduras::recibir(f,paginas).map_err(anyhow::Error::msg)?;
 let v=ligados.ligaduras();
 let datos=ligados.paginas().iter().map(|p|json!({"documento":p.documento,"seccion":p.seccion,"pagina":p.indice,"paginas":p.total,"texto":p.texto})).collect::<Vec<_>>();
 // Orden textual explícito: instrucción, afirmación, páginas 0 y 1.
 let user=retroalimentacion::contenido(caso,capa,&json!(datos)).map_err(anyhow::Error::msg)?;
 let mensajes=json!([{"role":"system","content":policy},{"role":"user","content":user}]);
 let renderizado=render(template,&mensajes,&json!([]))?;
 anyhow::ensure!(renderizado.contains("Reasoning: high"),"Esfuerzo alto ausente");
 let tokens=tokenizer.encode(renderizado.clone(),false).map_err(anyhow::Error::msg)?.get_ids().to_vec();
 let vuelta=tokenizer.decode(&tokens,false).map_err(anyhow::Error::msg)?;
 cotejar_entrada(renderizado.as_bytes(),vuelta.as_bytes(),tokens.len()).map_err(|e|anyhow::anyhow!("{e:?}"))?;
 Ok(Entrada{tokens,mensajes,renderizado,ligadura:format!("{v:#?}")})
}
fn acuerdo(etapa:&str,id:&str,hash_entrada:&str)->R<bool>{
 emit(json!({"evento":"solicitud_custodia","etapa":etapa,"id":id,"entrada_sha256":hash_entrada,"actor":"arbitro"}));
 let mut respuesta=String::new();io::stdin().lock().read_line(&mut respuesta)?;
 let v=parse_strict(respuesta.as_bytes())?;
 let mut esperado=json!({"custodia":"conservada","etapa":etapa,"id":id,"entrada_sha256":hash_entrada});
 if etapa=="adjudicacion"{anyhow::ensure!(v["continuar"].is_boolean(),"Sin decisión externa de continuidad");esperado["continuar"]=v["continuar"].clone();}
 anyhow::ensure!(v==esperado,"Custodia no acreditada");Ok(v["continuar"]==true)
}
fn contexto(evento:&str,caso:&Value,e:&Entrada,capa:u64)->Value{
 json!({"evento":evento,"id":caso["id"],"documento":caso["documento"],"seccion":caso["seccion"],"ronda":capa,
 "mensajes":e.mensajes,"funciones":[],"plantilla_efectiva":e.renderizado,"tokens":e.tokens,
 "tokens_sha256":hash(&serde_json::to_vec(&e.tokens).unwrap()),"max_salida":SALIDA,"paginas":[0,1],
 "reserva_tras_salida":CONTEXTO-e.tokens.len()-SALIDA})
}
async fn run()->R<()> {
 let mode=std::env::args().nth(1).unwrap_or_default();
 if mode=="probe"{return probe()}
 anyhow::ensure!(mode=="instrumental"||mode=="contraste","Modo no autorizado");
 probe()?;
 let template=fs::read_to_string("/modelo/chat_template.jinja")?;
 let tokenizer=Tokenizer::from_file("/modelo/tokenizer.json").map_err(anyhow::Error::msg)?;
 let policy=fs::read_to_string("/config/politica.txt")?;
 let plan_raw=fs::read("/config/plan.json")?;
 anyhow::ensure!(hash(&plan_raw)=="ad49bd52e3b29daf6437f2d02405e7379700490b154cfa5e5b0356dc1616c92a","Plan distinto");
 let plan=parse_strict(&plan_raw)?;retroalimentacion::validar_plan(&plan).map_err(anyhow::Error::msg)?;let capa=plan["capa"].as_u64().unwrap();
 let casos=plan["casos"].as_array().ok_or_else(||anyhow::anyhow!("Sin casos"))?;
 anyhow::ensure!(casos.len()==9,"Número de casos distinto");
 let mut mcp=Mcp{next:1};let contrato=mcp.init()?;
 anyhow::ensure!(contrato.as_array().unwrap().iter().any(|t|t["name"]=="leer_documento"),"Falta herramienta");
 emit(json!({"evento":"contrato_adaptado","mcp":contrato,"funciones":[],
   "plantilla_sha256":hash(template.as_bytes()),"tokenizador_sha256":hash(&fs::read("/modelo/tokenizer.json")?),
   "politica":policy,"plan":plan,"instruccion":INSTRUCCION,"actor":"arbitro"}));
 let mut preparadas=Vec::new();
 for (i,caso) in casos.iter().enumerate(){
  let id=caso["id"].as_str().unwrap();
  anyhow::ensure!(id==format!("{}{:02}",plan["bloque"].as_str().unwrap(),i+1)&&caso["documento"]==format!("BANCO-{}",plan["bloque"].as_str().unwrap())&&caso["seccion"]==id,"Correspondencia alterada");
  let f=esperadas(caso["documento"].as_str().unwrap(),caso["seccion"].as_str().unwrap())?;let paginas=obtener(&mut mcp,&f)?;
  let preparada=entrada(&f,&paginas,caso,capa,&policy,&template,&tokenizer)?;
  let sha=hash(&serde_json::to_vec(&preparada.tokens)?);
  emit(json!({"evento":"ligaduras_documentales","id":id,"nucleo_revision":"1c6b84238e62e867c4d5c5ab8069c9ae8f67dccc",
    "gramatica":sv_core::GRAMMAR_VERSION,"ir":sv_core::IR_VERSION,"registro":preparada.ligadura,
    "autoridad_r1":false,"evaluacion_terna":false}));
  emit(contexto("contexto_previsto",caso,&preparada,capa));acuerdo("preparacion",id,&sha)?;preparadas.push(preparada);
 }
 if mode=="instrumental" {
  for args in [json!({"documento":"BANCO-A","seccion":"A01","pagina":-1}),json!({"documento":"BANCO-A","seccion":"A01","pagina":999}),json!({"documento":"../externo","seccion":"S1","pagina":0})] {
   let r=mcp.call("leer_documento",args)?;
   anyhow::ensure!(r["result"]["isError"]==true||r["error"].is_object(),"MCP no rechaza petición prohibida");
  }
  let r=mcp.call("herramienta_no_autorizada",json!({}))?;
  anyhow::ensure!(r["result"]["isError"]==true||r["error"].is_object(),"MCP no rechaza herramienta ajena");
  let f=esperadas("BANCO-A","A09")?;let _=obtener(&mut mcp,&f)?;
  emit(json!({"evento":"pruebas_instrumentales_fin","conforme":true,"rutas_completas":9,"paginas":[0,1],"inferencia":false}));
  emit(json!({"evento":"fin_conductor","modo":mode}));return Ok(())
 }
 acuerdo("carga",casos[0]["id"].as_str().unwrap(),&hash(&serde_json::to_vec(&preparadas[0].tokens)?))?;
 emit(json!({"evento":"carga_inicio","modelo":"openai/gpt-oss-safeguard-120b","revision":"3c7391182603991a904031244e7822488c67796d","representacion":"MXFP4 original; restantes F32; sin recuantizacion"}));
 let model=TextModelBuilder::new("/modelo").with_force_cpu().with_dtype(ModelDType::F32).with_token_source(TokenSource::None).with_max_num_seqs(1).with_prefix_cache_n(None).build().await?;
 emit(json!({"evento":"carga_fin","prefix_cache":false,"max_secuencias_simultaneas":1}));
 for (i,caso) in casos.iter().enumerate(){
  let id=caso["id"].as_str().unwrap();audit::begin(i+1);
  emit(json!({"evento":"caso_inicio","id":id,"orden":i+1,"historial":caso["antecedentes"],"capa":capa,"contexto_nuevo":true}));
  let f=esperadas(caso["documento"].as_str().unwrap(),caso["seccion"].as_str().unwrap())?;let paginas=obtener(&mut mcp,&f)?;
  let efectiva=entrada(&f,&paginas,caso,capa,&policy,&template,&tokenizer)?;
  anyhow::ensure!(efectiva.tokens==preparadas[i].tokens&&efectiva.mensajes==preparadas[i].mensajes,"Entrada efectiva distinta");
  let sha=hash(&serde_json::to_vec(&efectiva.tokens)?);
  emit(contexto("contexto_conductor",caso,&efectiva,capa));acuerdo("generacion",id,&sha)?;
  let(tx,mut rx)=tokio::sync::mpsc::channel(16);let mut sampling=SamplingParams::deterministic();
  sampling.max_len=Some(SALIDA);sampling.stop_toks=Some(StopTokens::Ids(vec![200002,200012]));
  let mut request=NormalRequest::new_simple(RequestMessage::CompletionTokens(efectiva.tokens),sampling,tx,100+i,None,None);request.seed=Some(42);
  model.inner().get_sender(None)?.send(Request::Normal(Box::new(request))).await?;
  let response=rx.recv().await.ok_or_else(||anyhow::anyhow!("Motor sin cierre"))?;
  let emitted=audit::finish();let raw=tokenizer.decode(&emitted,false).map_err(anyhow::Error::msg)?;
  emit(json!({"evento":"emision_integra","id":id,"ronda":capa,"tokens":emitted,"texto":raw}));
  let mut completed=false;
  match response {
   Response::CompletionDone(r)=>{
    emit(json!({"evento":"respuesta_motor","id":id,"respuesta":r}));
    match segments(&raw) {
     Ok(parsed) if emitted.last()==Some(&200002)=>{
      let finals=parsed.iter().filter(|s|s.header.contains("<|channel|>final")&&s.end=="<|return|>").collect::<Vec<_>>();
      if finals.len()==1{emit(json!({"evento":"respuesta_final","id":id,"contenido":finals[0].body,"salida_total_tokens":emitted.len()}));completed=true;}
     }, _=>{}
    }
   },
   other=>emit(json!({"evento":"impedimento","id":id,"causa":format!("{:?}",other.as_result())}))
  }
  emit(json!({"evento":"caso_fin","id":id,"completo":completed,"tokens":emitted.len()}));
  anyhow::ensure!(completed,"Salida incompleta: parada definitiva sin reintento");
  if !acuerdo("adjudicacion",id,&sha)?{emit(json!({"evento":"cierre_por_adjudicacion","id":id}));break}
 }
 emit(json!({"evento":"fin_conductor","modo":mode}));Ok(())
}
#[tokio::main]
async fn main(){if let Err(e)=run().await{emit(json!({"evento":"fallo_conductor","causa":format!("{e:#}")}));eprintln!("{e:#}");std::process::exit(1)}}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn harmony_preserva_analisis_y_final(){let s=segments("<|channel|>analysis<|message|>ñ 漢字<|end|><|start|>assistant<|channel|>final<|message|>{\"decision\":\"RESPALDADA\"}<|return|>").unwrap();assert_eq!(s.len(),2);assert_eq!(s[0].body,"ñ 漢字");assert_eq!(s[1].end,"<|return|>");}
 #[test]fn harmony_no_inventa_cierre(){assert!(segments("<|channel|>final<|message|>incompleta").is_err());}
 #[test]fn harmony_preserva_argumentos_invalidos(){let s=segments(" to=functions.leer_documento<|channel|>commentary json<|message|>{\"pagina\":-1}<|call|>").unwrap();assert_eq!(s[0].body,"{\"pagina\":-1}");}

 #[test]fn mcp_multilinea_conserva_valor_y_un_solo_marco(){let original="{\n\"consulta\":\"línea\\nsiguiente\",\n\"desplazamiento\":0\n}";let(w,valid)=encuadrar_llamada(7,"buscar_documentos",original);assert!(valid);assert_eq!(w.bytes().filter(|b|*b==b'\n').count(),1);let v=parse_strict(w.as_bytes()).unwrap();assert_eq!(v["params"]["arguments"],parse_strict(original.as_bytes()).unwrap());assert_eq!(v["id"],7);}
 #[test]fn mcp_sintaxis_invalida_no_se_repara(){let original="{\n\"documento\":\"D1\"";let(w,valid)=encuadrar_llamada(8,"leer_documento",original);assert!(!valid);assert_eq!(w.bytes().filter(|b|*b==b'\n').count(),1);assert_eq!(parse_strict(w.as_bytes()).unwrap()["params"]["arguments"],original);}
 #[test]fn mcp_duplicados_no_se_normalizan(){let original="{\"pagina\":0,\"pagina\":1}";let(w,valid)=encuadrar_llamada(9,"leer_documento",original);assert!(!valid);assert_eq!(parse_strict(w.as_bytes()).unwrap()["params"]["arguments"],original);}
}

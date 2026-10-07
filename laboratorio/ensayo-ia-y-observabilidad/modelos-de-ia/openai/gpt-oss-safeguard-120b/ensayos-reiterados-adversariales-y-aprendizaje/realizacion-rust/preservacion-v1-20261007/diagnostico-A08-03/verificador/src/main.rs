use std::{fs,path::Path};use serde_json::{Value,json};use tokenizers::Tokenizer;use sha2::{Sha256,Digest};
use sv_arbitro_comprobaciones::retroalimentacion::{self,CONTEXTO,SALIDA,RESERVA};
type R<T>=Result<T,Box<dyn std::error::Error+Send+Sync>>;
fn ck(v:bool,s:&str)->R<()>{if v{Ok(())}else{Err(s.into())}}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn ids(v:&Value)->Vec<u32>{v.as_array().unwrap().iter().map(|x|x.as_u64().unwrap()as u32).collect()}
fn render(template:&str,messages:&Value,tools:&Value)->R<String>{let mut e=minijinja::Environment::new();e.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);e.add_function("strftime_now",|_s:String|"2026-10-03".to_string());e.add_function("raise_exception",|s:String|->Result<String,minijinja::Error>{Err(minijinja::Error::new(minijinja::ErrorKind::InvalidOperation,s))});e.add_template("t",template)?;Ok(e.get_template("t")?.render(minijinja::context!{messages=>messages,tools=>tools,add_generation_prompt=>true,reasoning_effort=>"high",builtin_tools=>Vec::<String>::new()})?)}
fn main()->R<()>{
 let a=std::env::args().collect::<Vec<_>>();let p=Path::new(a.get(1).ok_or("Recorrido requerido")?);let parcial=a.get(2).is_some_and(|s|s=="parcial");
 let tok=Tokenizer::from_file("/opt/sv-safeguard/modelo/tokenizer.json")?;
 let template=fs::read_to_string("/opt/sv-safeguard/modelo/chat_template.jinja")?;
 let planraw=fs::read("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-03/config/plan.json")?;ck(sha(&planraw)=="82a8bdd419860d631c941d4c45424a23947990ef6c0f17c0507d51d816117af8","Plan no prefijado")?;
 let plan:Value=serde_json::from_slice(&planraw)?;retroalimentacion::validar_plan(&plan)?;
 let catraw=fs::read("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-03/cache/catalogo.json")?;ck(sha(&catraw)=="17c5227bf619fb31475910e631e736043305c69e2bcd2b89a9350d422c32e415","Catálogo no prefijado")?;let cat:Value=serde_json::from_slice(&catraw)?;
 let original=fs::read(p.join("modelo.stdout"))?;
 let ev=std::str::from_utf8(&original)?.lines().filter_map(|l|l.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;
 let reales=ev.iter().filter(|v|v["datos"]["evento"]=="contexto_conductor").map(|v|&v["datos"]).collect::<Vec<_>>();
 let previas=ev.iter().filter(|v|v["datos"]["evento"]=="contexto_previsto").map(|v|&v["datos"]).collect::<Vec<_>>();
 ck(previas.len()==1&&reales.len()<=1,"Número de entradas no autorizado")?;
 let recibos=ev.iter().filter(|v|v["datos"]["evento"]=="mcp_recibido").map(|v|serde_json::from_str::<Value>(v["datos"]["wire"].as_str().unwrap())).collect::<Result<Vec<_>,_>>()?;
 let mut detalle=Vec::new();
 for (i,d) in previas.iter().enumerate(){
  let caso=&plan["casos"][i];let doc=caso["documento"].as_str().ok_or("Documento")?;let sec=caso["seccion"].as_str().ok_or("Sección")?;
  ck(d["id"]==caso["id"]&&d["documento"]==doc&&d["seccion"]==sec,"Caso discordante")?;
  ck(d["funciones"]==json!([])&&d["max_salida"]==SALIDA&&d["paginas"]==json!([0,1]),"Herramientas, cobertura o salida alteradas")?;
  let mensajes=d["mensajes"].as_array().ok_or("Mensajes")?;
  ck(mensajes.len()==2&&mensajes[0]["role"]=="system"&&mensajes[1]["role"]=="user","Contexto contaminado")?;
  let texto=d["plantilla_efectiva"].as_str().ok_or("Sin plantilla")?;
  ck(render(&template,&d["mensajes"],&d["funciones"])?==texto,"Plantilla discordante")?;
  let tokens=tok.encode(texto,false)?.get_ids().to_vec();
  ck(tokens==ids(&d["tokens"])&&sha(&serde_json::to_vec(&tokens)?)==d["tokens_sha256"],"Tokenización discordante")?;
  ck(tok.decode(&tokens,false)?==texto,"Pérdida tras tokenización")?;
  ck(tokens.len()+SALIDA+RESERVA<=CONTEXTO,"Reserva insuficiente después de salida")?;
  ck(sha(mensajes[0]["content"].as_str().ok_or("Política ausente")?.as_bytes())=="5df8ce54a97e72c4a12d05d064b818e73a082e92b699c5e2a70f2ef7eb19c5b9","Política alterada")?;
  let user=mensajes[1]["content"].as_str().ok_or("Mensaje ausente")?;
  let prefijo=format!("{}\n\nDatos autorizados:\n",retroalimentacion::INSTRUCCION);
  let datos:Value=serde_json::from_str(user.strip_prefix(&prefijo).ok_or("Instrucción u orden alterados")?)?;
  ck(datos["afirmacion"]==caso["afirmacion"]&&datos["capa"]==plan["capa"]&&datos["antecedentes_propios"]==caso["antecedentes"],"Afirmación o antecedentes alterados")?;
  let paginas=datos["documentacion"].clone();
  ck(user==retroalimentacion::contenido(caso,plan["capa"].as_u64().unwrap(),&paginas)?,"Entrada no idéntica al contrato")?;
  let paginas=paginas.as_array().ok_or("Páginas ausentes")?;ck(paginas.len()==2,"Conjunto incompleto")?;
  let mut completo=String::new();
  for(j,pagina)in paginas.iter().enumerate(){
   ck(pagina["documento"]==doc&&pagina["seccion"]==sec&&pagina["pagina"]==j&&pagina["paginas"]==2,"Identidad de página discordante")?;
   let r=recibos.iter().find(|r|r["result"]["isError"]==false&&r["result"]["structuredContent"]["documento"]==doc&&r["result"]["structuredContent"]["seccion"]==sec&&r["result"]["structuredContent"]["pagina"]==j).ok_or("Página sin recibo")?;
   let actual=&r["result"]["structuredContent"];for k in ["documento","seccion","pagina","paginas","texto"]{ck(pagina[k]==actual[k],"Página distinta del MCP")?;}
   ck(serde_json::from_str::<Value>(r["result"]["content"][0]["text"].as_str().ok_or("Texto dual")?)?==*actual,"Representaciones MCP discordantes")?;
   completo.push_str(pagina["texto"].as_str().ok_or("Texto de página")?);
  }
  let docf=cat["documents"].as_array().unwrap().iter().find(|v|v["id"]==doc).ok_or("Documento no encontrado")?;
  let secf=docf["sections"].as_array().unwrap().iter().find(|s|s["id"]==sec).ok_or("Sección no encontrada")?; ck(completo==secf["text"].as_str().ok_or("Texto catalogado")?&&sha(completo.as_bytes())==secf["sha256"],"Sección incompleta o distinta")?;
  if let Some(real)=reales.iter().find(|r|r["id"]==d["id"]){for k in ["id","documento","seccion","mensajes","funciones","plantilla_efectiva","tokens","tokens_sha256","max_salida"]{ck(real[k]==d[k],"Entrada efectiva distinta de la admitida")?;}}
  detalle.push(json!({"id":caso["id"],"entrada_tokens":tokens.len(),"salida_maxima":SALIDA,"reserva_tras_salida":CONTEXTO-tokens.len()-SALIDA,"paginas":2,"tokens_sha256":d["tokens_sha256"],"ejecutada":reales.iter().any(|r|r["id"]==d["id"])}));
 }
 let emisiones=ev.iter().filter(|v|v["datos"]["evento"]=="emision_integra").map(|v|&v["datos"]).collect::<Vec<_>>();
 let finales=ev.iter().filter(|v|v["datos"]["evento"]=="respuesta_final").map(|v|&v["datos"]).collect::<Vec<_>>();
 let cierres=ev.iter().filter(|v|v["datos"]["evento"]=="caso_fin").map(|v|&v["datos"]).collect::<Vec<_>>();
 ck(emisiones.len()==reales.len()&&finales.len()==emisiones.len()&&cierres.len()==emisiones.len(),"Original incompleto")?;
 for (i,v) in emisiones.iter().enumerate(){
  ck(v["id"]==plan["casos"][i]["id"]&&finales[i]["id"]==v["id"]&&cierres[i]["id"]==v["id"]&&cierres[i]["completo"]==true,"Identidad o cierre de emisión")?;
  ck(tok.decode(&ids(&v["tokens"]),false)?==v["texto"],"Decodificación discordante")?;
 }
 if parcial{
  let punto:Value=serde_json::from_slice(&fs::read(p.join("PUNTO.json"))?)?;let ultima=emisiones.last().ok_or("Punto sin emisión")?;
  ck(punto["adjudicacion_pendiente"]==true&&punto["inferencia_siguiente_impedida"]==true&&punto["id"]==ultima["id"]&&punto["salida_sha256"]==sha(ultima["texto"].as_str().unwrap().as_bytes()),"Punto sin original concordante")?;
 }else{ck(ev.iter().any(|v|v["datos"]["evento"]=="fin_conductor"),"Sin cierre")?;}
 let informe=json!({"conforme":true,"entradas_prefijadas":1,"detalle":detalle,"emisiones_decodificadas":emisiones.len(),"inferencia":!reales.is_empty(),"punto_previo_adjudicacion":parcial,"modelo_stdout_sha256":sha(&original),"criterio":"Recibos MCP reales, política, afirmaciones y páginas exactas; plantilla oficial, tokenización, decodificación y reserva posterior a salida cotejadas. Antecedentes propios completos y fijados para la capa. No adjudica semántica ni constituye autoridad R1."});
 let dest=std::path::PathBuf::from(a.get(3).ok_or("Salida externa requerida")?); ck(dest.parent().ok_or("Directorio")?.canonicalize()?!=p.canonicalize()?,"No escribir en originales")?;ck(!dest.exists(),"Original preservado")?;fs::write(dest,serde_json::to_vec_pretty(&informe)?)?;println!("{informe}");Ok(())
}

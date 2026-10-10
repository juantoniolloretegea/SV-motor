#![forbid(unsafe_code)]
//! Banco documental: suministro, contrato, transporte e instrumentación comunes.
use sv_cliente_api::{self as api,need,parse,save,sha,Perfil,R};
use serde_json::{json,Value};
use std::{fs,path::{Path,PathBuf},time::Instant};
use zeroize::Zeroizing;
#[path="../manual/contrato.rs"] mod contrato;
#[path="../manual/localizadores.rs"] mod localizadores;
#[path="../manual/suministro.rs"] mod suministro;
mod suministro_pdf {
 pub use sv_cliente_api::{parse,need,sha,R};use serde_json::Value;
 pub fn num(v:&Value)->R<usize>{v.as_u64().and_then(|x|usize::try_from(x).ok()).ok_or("Entero requerido".into())}
 pub fn contenido(v:&Value,id:usize)->R<Value>{need(v["jsonrpc"]=="2.0"&&v["id"]==id&&v.get("error").is_none(),"RPC discordante")?;let r=&v["result"];need(r["isError"]==false&&r["content"].as_array().map(Vec::len)==Some(1)&&r["content"][0]["type"]=="text","Recepción MCP inválida")?;let t=parse(r["content"][0]["text"].as_str().ok_or("Texto ausente")?.as_bytes())?;need(t==r["structuredContent"],"Representaciones RPC discordantes")?;Ok(t)}
}
const WORK:&str="C:/SV-LABORATORIO";
fn admite_gasto(spent:u64,reserve:u64,limit:u64)->bool{spent.checked_add(reserve).is_some_and(|v|v<=limit)}
fn load(p:&Path)->R<Value>{api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn ident(p:&Path)->R<Value>{api::guard(p)?;let b=fs::read(p).map_err(|e|e.to_string())?;Ok(json!({"ruta":p.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn config(root:&Path)->R<(Perfil,suministro::ContratoLibro)>{
 let p:Perfil=serde_json::from_value(load(&root.join("PERFIL.json"))?).map_err(|e|e.to_string())?;p.comprobar()?;
 need(p.proveedor=="Z.ai"&&p.modelo=="glm-5.3"&&p.presupuesto_ticks==30_000_000_000&&!p.exigir_zdr&&p.cuota_gratuita_tokens.is_none(),"Perfil distinto del alcance recibido")?;
 let c:suministro::ContratoLibro=serde_json::from_value(load(&root.join("CONTRATO-LIBRO.json"))?).map_err(|e|e.to_string())?;c.comprobar()?;
 need(c.documentos==5&&c.preguntas==16&&c.prefijo=="C"&&c.modelo.as_deref()==Some("glm-5.3"),"Banco distinto")?;Ok((p,c))
}
fn compose(base:&Value,p:&Perfil,stage:usize,history:&[String])->R<(Value,Value)>{
 let mut q=contrato::compose_documental(base,stage,history)?;
 q["max_output_tokens"]=json!(16384);q["reasoning"]=json!({"effort":"high"});
 for message in q["input"].as_array_mut().ok_or("Entrada")?{
  if message["role"]!="user"{continue;}let Some(t)=message["content"].as_str()else{continue;};
  if let Ok(mut v)=parse(t.as_bytes()){if v["tipo"]=="registro_de_instrucciones_historicas"{
   let s=v["etapa_historica"].as_u64().ok_or("Etapa histórica")? as usize;need(s<stage,"Etapa futura")?;
   let (_,wire)=compose(base,p,s,&history[..s])?;
   v["instrucciones_aplicadas"]=wire["messages"][0]["content"].clone();message["content"]=json!(v.to_string());
  }}
 }
 let mut wire=q.clone();api::proteger(&mut wire,p)?;api::chat::validar(&wire,&p.modelo)?;
 need(wire["messages"][1]["content"]==base["input"][0]["content"],"Fuente modificada")?;Ok((q,wire))
}
fn frozen(root:&Path)->R<Value>{
 let mut items=vec![];
 for f in ["ADMISION.md","CRITERIOS.md","SELECCION.json","ANTECEDENTE.json","PERFIL.json","CONTRATO-LIBRO.json","fuentes/BANCO.json","reservado/CLAVE.json","fuentes/preparado/CATALOGO.json","fuentes/preparado/FUENTES.json","suministro/CONTROL-ARBITRO.json"]{items.push(ident(&root.join(f))?);}
 for i in 1..=16{items.push(ident(&root.join(format!("fuentes-admitidas/C{i:02}.json")))?);}
 let common=Path::new(WORK).join("desarrollo/acoplamientos-con-el-sv/cliente-api-rust");
 for f in ["Cargo.toml","Cargo.lock","src/lib.rs","src/estricto.rs","src/presupuesto.rs","src/chat.rs","src/manual/contrato.rs","src/manual/contrato-base.rs","src/manual/localizadores.rs","src/manual/suministro.rs","src/bin/sv-banco-cyb16-api.rs"]{items.push(ident(&common.join(f))?);}
 for f in ["Cargo.toml","src/lib.rs"]{items.push(ident(&Path::new(WORK).join("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust").join(f))?);}
 items.push(ident(&std::env::current_exe().map_err(|e|e.to_string())?)?);
 for f in ["sv-mcp-documental","verificar-diario"]{items.push(ident(&Path::new(WORK).join("compilacion/mcp-mdbook/debug").join(f))?);}
 Ok(json!(items))
}
fn check(root:&Path,c:&suministro::ContratoLibro)->R<()>{need(load(&root.join("PREVIA.json"))?["archivos"]==frozen(root)?,"Controlador o fuentes alterados")?;suministro::verificar_contrato(root,c)}
fn prepare(root:&Path,p:&Perfil,c:&suministro::ContratoLibro)->R<Value>{
 suministro::preparar_contrato(root,c)?;
 let mut rows=vec![];
 for i in 1..=16{let base=load(&root.join(format!("fuentes-admitidas/C{i:02}.json")))?;
  for stage in 0..3{let h=(0..stage).map(|_|"Antecedente sintético de comprobación, no respuesta del candidato".to_string()).collect::<Vec<_>>();let(_,w)=compose(&base,p,stage,&h)?;rows.push(json!({"caso":format!("C{i:02}"),"etapa":stage,"reserva_sintetica_ticks":api::reserva(&w,p)?}));}
 }
 let v=json!({"conforme":true,"inferencias":0,"archivos":frozen(root)?,"composiciones":rows,"licencia":api::LICENCIA});save(&root.join("PREVIA.json"),&v)?;Ok(v)
}
fn ledger(root:&Path)->R<Vec<Value>>{
 let dir=root.join("hitos");if !dir.exists(){return Ok(vec![]);}
 let mut paths=fs::read_dir(dir).map_err(|e|e.to_string())?.map(|e|e.map(|e|e.path())).collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;paths.sort();paths.iter().map(|p|load(p)).collect()
}
fn history(root:&Path,id:&str,stage:usize,rows:&[Value])->R<Vec<String>>{
 let mut h=vec![];for s in 0..stage{
  let r=rows.iter().find(|r|r["caso"]==id&&r["etapa"]==s&&r["completa"]==true).ok_or("Antecedente completo ausente")?;
  let dir=root.join(r["directorio"].as_str().ok_or("Directorio")?);
  need(r["resultado"]==ident(&dir.join("RESULTADO.json"))?&&r["texto"]==ident(&dir.join("FINAL.txt"))?,"Antecedente alterado")?;
  h.push(fs::read_to_string(dir.join("FINAL.txt")).map_err(|e|e.to_string())?);
 }Ok(h)
}
fn execute(root:&Path,p:&Perfil,c:&suministro::ContratoLibro)->R<Value>{
 check(root,c)?;need(!root.join("INICIO.json").exists(),"Banco ya iniciado; no se permite repetición automática")?;
 let secret_path=PathBuf::from(std::env::var("SV_API_KEY_FILE").map_err(|_|"Falta ruta de clave")?);api::guard(&secret_path)?;
 need(secret_path.starts_with(Path::new(WORK).join("privado")),"Credencial fuera de sede privada")?;
 let secret=Zeroizing::new(fs::read_to_string(secret_path).map_err(|_|"Clave no accesible")?);
 need(secret.len()>=24&&!secret.chars().any(char::is_whitespace),"Clave inválida")?;
 save(&root.join("INICIO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"proveedor":p.proveedor,"modelo":p.modelo,"limite_ticks":p.presupuesto_ticks,"preguntas":16,"etapas":3,"licencia":api::LICENCIA}))?;
 let prior=load(&root.join("ANTECEDENTE.json"))?;
 need(prior["tipo"]=="campana_nueva_sin_consumo_previo"&&prior["consumo_previo_ticks"]==0,"Antecedente impropio")?;
 let prior_cost=0u64;
 let start=Instant::now();let mut rows=vec![];let mut spent=prior_cost;let mut status="completo";let mut progress=[0usize;16];let mut interruption:Option<Instant>=None;
 'bank:for pass in 0..2{
  for (i,next) in progress.iter_mut().enumerate(){let id=format!("C{:02}",i+1);
   while *next<3{
    if start.elapsed().as_secs()>=5400{status="suspendido_por_plazo_global";break 'bank;}
    if interruption.is_some_and(|t|t.elapsed().as_secs()>300){status="no_valido_por_interrupcion_proveedor_mayor_5_minutos";break 'bank;}
    check(root,c)?;let stage=*next;let base=load(&root.join(format!("fuentes-admitidas/{id}.json")))?;
    let h=history(root,&id,stage,&rows)?;let(canonical,wire)=compose(&base,p,stage,&h)?;let reserve=api::reserva(&wire,p)?;
    let allowed=admite_gasto(spent,reserve,p.presupuesto_ticks);
    let n=rows.len()+1;let relative=format!("intentos/I{n:03}-{id}-R{stage}");let dir=root.join(&relative);
    save(&root.join(format!("reservas/I{n:03}.json")),&json!({"caso":id,"etapa":stage,"consumo_o_reserva_previo_ticks":spent,"reserva_ticks":reserve,"limite_ticks":p.presupuesto_ticks,"admision":allowed,"solicitud_sha256":sha(&serde_json::to_vec_pretty(&wire).map_err(|e|e.to_string())?)}))?;
    if !allowed{status="suspendido_antes_del_envio_por_presupuesto";break 'bank;}
    let r=api::enviar(p,&secret,&wire,&dir,300000)?;
    let complete=r["completa"]==true&&r["telemetria_conforme"]==true;
    let est=if r["completa"]==true{api::presupuesto::estimacion_chat(&r["entrega"]["uso_proveedor"],p)?}else{Value::Null};
    let consumed=est["estimacion_sin_descuento_ticks"].as_u64().unwrap_or(reserve);spent=spent.checked_add(consumed).ok_or("Desbordamiento")?;
    save(&dir.join("ESTIMACION-TARIFARIA.json"),&est)?;
    let formal=if complete{parse(r["entrega"]["texto_original"].as_str().ok_or("Texto")?.as_bytes()).and_then(|v|contrato::formal(&v,&canonical)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}))}else{Value::Null};save(&dir.join("AUDITORIA-FORMAL.json"),&formal)?;
    let http=load(&dir.join("HTTP.json")).unwrap_or(Value::Null);
    let row=json!({"intento":n,"caso":id,"etapa":stage,"pasada":pass,"directorio":relative,"completa":complete,"resultado":ident(&dir.join("RESULTADO.json"))?,"texto":if complete{ident(&dir.join("FINAL.txt"))?}else{Value::Null},"auditoria_formal":formal,"estimacion_tarifaria":est,"cargo_comunicado_ticks":null,"reserva_ticks":reserve,"consumo_o_reserva_ticks":consumed,"http":http["status"],"error":r["error"],"duracion_operacion_ms":r["duracion_operacion_ms"]});
    save(&root.join(format!("hitos/I{n:03}.json")),&row)?;rows.push(row);
    println!("{id} R{stage}: completa={complete}; formal={}; estimacion/reserva USD={:.6}",formal["conforme"],spent as f64/1e10);
    if consumed>reserve||spent>p.presupuesto_ticks{status="suspendido_por_desviacion_presupuestaria";break 'bank;}
    if r["telemetria_conforme"]!=true{status="suspendido_por_instrumentacion";break 'bank;}
    if complete{*next+=1;interruption=None;}else{
     let provider=http["status"].as_u64().is_some_and(|s|s==429||s>=500)||r["error"].as_str().is_some_and(|s|s.contains("Transporte interrumpido")||s.contains("Lectura interrumpida"));
     if provider{interruption.get_or_insert_with(Instant::now);if r["duracion_operacion_ms"].as_u64().unwrap_or(0)>300000{status="no_valido_por_interrupcion_proveedor_mayor_5_minutos";break 'bank;}break;}
     status="suspendido_por_incidencia_de_recepcion";break 'bank;
    }
   }
  }
  if progress.iter().all(|n|*n==3){break;}
 }
 if status=="completo"&&!progress.iter().all(|n|*n==3){status="incompleto_por_interrupciones_del_proveedor";}
 let v=json!({"estado":status,"duracion_ms":start.elapsed().as_millis(),"entregas_completas":progress.iter().sum::<usize>(),"progreso":progress,"consumo_estimado_o_reservado_ticks":spent,"limite_ticks":p.presupuesto_ticks,"intentos":rows,"adjudicacion":"pendiente de revisión exterior","licencia":api::LICENCIA});save(&root.join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
fn receive(root:&Path,p:&Perfil,c:&suministro::ContratoLibro)->R<Value>{
 check(root,c)?;let bank=load(&root.join("RESULTADO-BANCO.json"))?;let rows=ledger(root)?;need(bank["intentos"]==json!(rows),"Diario de intentos discordante")?;
 let mut entries=vec![];let(mut input,mut output,mut total,mut cost,mut samples,mut gap)=(0u64,0u64,0u64,0u64,0u64,0u64);
 for row in &rows{
  let dir=root.join(row["directorio"].as_str().ok_or("Directorio")?);need(row["resultado"]==ident(&dir.join("RESULTADO.json"))?,"Resultado alterado")?;
  let r=load(&dir.join("RESULTADO.json"))?;let t=sv_instrumentacion::verify(&dir.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Telemetría discordante")?;
  let id=row["caso"].as_str().ok_or("Caso")?;let stage=row["etapa"].as_u64().ok_or("Etapa")? as usize;
  let base=load(&root.join(format!("fuentes-admitidas/{id}.json")))?;let h=history(root,id,stage,&rows)?;let(canonical,wire)=compose(&base,p,stage,&h)?;need(load(&dir.join("SOLICITUD.json"))?==wire,"Solicitud no reproducida")?;
  let mut normalized=Value::Null;
  if row["completa"]==true{
   let mut stream=api::chat::Flujo::default();stream.feed(&fs::read(dir.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?)?;let received=stream.recibir(&p.modelo)?;
   need(received["texto_original"]==fs::read_to_string(dir.join("FINAL.txt")).map_err(|e|e.to_string())?&&received["uso_proveedor"]==r["entrega"]["uso_proveedor"]&&received["razonamiento_comunicado"]==r["entrega"]["razonamiento_comunicado"],"Entrega reconstruida discordante")?;
   let estimate=api::presupuesto::estimacion_chat(&received["uso_proveedor"],p)?;need(estimate==row["estimacion_tarifaria"],"Valoración discordante")?;normalized=estimate["uso_normalizado"].clone();input+=normalized["input_tokens"].as_u64().unwrap();output+=normalized["output_tokens"].as_u64().unwrap();total+=normalized["total_tokens"].as_u64().unwrap();cost+=estimate["estimacion_sin_descuento_ticks"].as_u64().unwrap();
   let formal=parse(received["texto_original"].as_str().unwrap().as_bytes()).and_then(|v|contrato::formal(&v,&canonical)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));need(formal==row["auditoria_formal"],"Auditoría formal discordante")?;
  }
  samples+=t["muestras"].as_u64().ok_or("Muestras")?;gap=gap.max(t["intervalo_maximo_ms"].as_u64().ok_or("Intervalo")?);
  entries.push(json!({"caso":id,"etapa":stage,"intento":row["intento"],"directorio":row["directorio"],"completa":row["completa"],"uso":normalized,"estimacion_tarifaria":row["estimacion_tarifaria"],"telemetria":t,"duracion_operacion_ms":r["duracion_operacion_ms"],"primer_texto_ms":r["entrega"]["primer_texto_ms"],"auditoria_formal":row["auditoria_formal"],"final_sha256":row["texto"]["sha256"]}));
 }
 need(input+output==total,"Suma incoherente")?;
 let v=json!({"conforme":true,"estado_banco":bank["estado"],"entregas_completas":bank["entregas_completas"],"casos":entries,"tokens_entrada":input,"tokens_salida":output,"tokens_totales":total,"estimacion_sin_cache_ticks":cost,"cargo_comunicado_ticks":null,"duracion_ms":bank["duracion_ms"],"muestras":samples,"intervalo_maximo_ms":gap,"suministro":load(&root.join("suministro/CONTROL-ARBITRO.json"))?,"limite":"Recepción técnica; no adjudica verdad ni acredita infraestructura interna del proveedor","licencia":api::LICENCIA});save(&root.join("RECEPCION-RUST.json"),&v)?;Ok(v)
}
fn main(){let result=(||->R<Value>{let args=std::env::args().collect::<Vec<_>>();need(args.len()==3,"Uso: preparar|ejecutar|recibir RAIZ")?;let root=PathBuf::from(&args[2]);api::guard(&root)?;let(p,c)=config(&root)?;match args[1].as_str(){"preparar"=>prepare(&root,&p,&c),"ejecutar"=>execute(&root,&p,&c),"recibir"=>receive(&root,&p,&c),_=>Err("Acción no recibida".into())}})();match result{Ok(v)=>println!("Estado: {}",v["estado"].as_str().or(v["estado_banco"].as_str()).unwrap_or("conforme")),Err(e)=>{eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}}
#[cfg(test)]mod tests{use super::*;
 #[test]fn presupuesto_no_admite_exceso_ni_desbordamiento(){assert!(admite_gasto(29_000_000_000,1_000_000_000,30_000_000_000));assert!(!admite_gasto(29_000_000_000,1_000_000_001,30_000_000_000));assert!(!admite_gasto(u64::MAX,1,u64::MAX));}
 #[test]fn historia_conserva_instruccion_chat_real(){let p=Perfil{proveedor:"Z.ai".into(),modelo:"glm-5.3".into(),endpoint:"https://api.z.ai/api/paas/v4/chat/completions".into(),presupuesto_ticks:30000000000,exigir_zdr:false,entrada_ticks_por_token:14000,salida_ticks_por_token:44000,cuota_gratuita_tokens:None};let b=json!({"input":[{"role":"user","content":json!({"caso":"MD02","pregunta":"Q","fragmentos_documentales_completos":[]}).to_string()}],"instructions":"Fuente exclusiva","tools":[],"tool_choice":"none","store":false});let(_,r0)=compose(&b,&p,0,&[]).unwrap();let(c,r1)=compose(&b,&p,1,&["original".into()]).unwrap();let h=parse(c["input"][1]["content"].as_str().unwrap().as_bytes()).unwrap();assert_eq!(h["instrucciones_aplicadas"],r0["messages"][0]["content"]);assert_eq!(r1["messages"][3]["content"],"original");assert!(r1.get("tools").is_none());}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

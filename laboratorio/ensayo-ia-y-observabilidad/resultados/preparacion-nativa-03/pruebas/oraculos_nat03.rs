//! Oráculos precomprometidos. Esta fuente no ha sido ejecutada.
use crate::nativa::*;
use serde::Deserialize;
use serde_json::{Value,json};
use std::time::{Instant,Duration};
#[derive(Clone,Debug,Deserialize)]
pub struct Caso {
 pub caso:String,pub modo:String,pub contrato:String,pub estado:String,
 pub admisible:bool,pub revocada:bool,pub sello:bool,pub laguna:Option<bool>,
 pub error:Option<String>,pub codigo:Option<i64>,pub senal:Option<i64>,pub parada:bool,
 pub frames:Option<usize>,pub error_recuperacion:Option<String>,
}
pub fn caso(nombre:&str)->Result<Caso,Error>{
 let casos:Vec<Caso>=serde_json::from_str(include_str!("CASOS-NAT03.json"))?;
 casos.into_iter().find(|c|c.caso==nombre).ok_or_else(||"CASO_DESCONOCIDO".into())
}
pub fn identidad(c:&Caso,v:&Value)->Result<(),Error>{
 let e=&v["estimulo"];
 if e["caso"]!=c.caso||e["modo"]!=c.modo||e["contrato"]!=c.contrato{return Err("ESTIMULO_DISTINTO".into())}Ok(())
}
pub fn terminal(c:&Caso,v:&Value)->Result<(),Error>{
 identidad(c,v)?;
 if v["estado"]!=c.estado||v["admisible"]!=c.admisible||v["condiciones"]["revocada"]!=c.revocada{return Err("TERMINAL_DISTINTO".into())}
 if !c.admisible&&!v["resultado"].is_null(){return Err("RESULTADO_INADMISIBLE".into())}
 if v["sello"].as_str().is_some()!=c.sello{return Err("SELLO_EXIGIDO_O_PROHIBIDO".into())}
 if !c.sello&&!v["sello"].is_null(){return Err("SELLO_PROHIBIDO".into())}
 if c.sello && !v["sello"].as_str().is_some_and(|s|s.len()==64&&s.bytes().all(|x|x.is_ascii_hexdigit())){return Err("SELLO_FORMA".into())}
 if c.sello{
  let a=&v["condiciones"];
  if a["eof_completos"]!=json!(c.laguna.map(|x|!x))||a["secuencia_completa"]!=(c.caso!="omision")||
     a["oraculo_ok"]!=(c.caso!="omision")||a["proceso_ok"]!=!["no_cero","tardio"].contains(&c.caso.as_str())||
     a["fallo_observacion"]!=false||a["sellado_ok"]!=true||v["evidencia_parcial"]!=!c.admisible{return Err("CONDICIONES_DEL_PREFIJO".into())}
 }
 if c.caso=="escritura"{
  match (v["salida_proceso"]["codigo"].as_i64(),v["salida_proceso"]["senal"].as_i64()){
   (Some(0),None)=>{},
   (None,Some(9)) if v["parada_solicitada"]==true=>{},
   _=>return Err("SALIDA_ESCRITURA_DISTINTA".into())
  }
 }
 if v["reaped"]!=true{return Err("RECOGIDA_NO_OBSERVADA".into())}
 if let Some(x)=c.codigo{if v["salida_proceso"]["codigo"]!=x||!v["salida_proceso"]["senal"].is_null(){return Err("SALIDA_PROCESO".into())}}
 if let Some(x)=c.senal{if v["salida_proceso"]["senal"]!=x||!v["salida_proceso"]["codigo"].is_null(){return Err("SENAL_PROCESO".into())}}
 if c.parada{
  if v["parada_solicitada"]!=true{return Err("PARADA_NO_OBSERVADA".into())}
  let senales=v["senales"].as_array().ok_or("SENALES")?;
  for nombre in ["SIGTERM","SIGKILL"]{
   if !senales.iter().any(|s|s["tipo"]==nombre&&s["resultado"]=="Ok(())"){return Err("ACTUACION_SENAL_NO_OBSERVADA".into())}
  }
 }
 if v["error_custodia"]!=json!(c.error)||v["condiciones"]["fallo_custodia"]!=!c.sello{return Err("CAUSA_CUSTODIA".into())}
 if c.caso=="escritor_desconectado"{
  if v["registro_waitpid_fallido"]!=true{return Err("FALLO_WAITPID_NO_INYECTADO".into())}
  if !v["diagnosticos_volatiles"].as_array().ok_or("DIAGNOSTICOS")?.iter().any(|x|x["fase"]=="registro_waitpid"&&x["error"]=="COLA_CUSTODIA"){return Err("DIAGNOSTICO_WAITPID".into())}
 }
 Ok(())
}
pub const MAX_BYTES:usize=42*1024*1024;
pub const MAX_FRAGMENTOS:usize=1600;
pub struct Presupuesto{pub fin:Instant,pub fragmentos:usize,pub bytes:usize}
impl Presupuesto{
 pub fn nuevo(fin_sesion:Instant)->Self{Self{fin:fin_sesion.min(Instant::now()+Duration::from_secs(20)),fragmentos:0,bytes:0}}
 pub fn restante(&self)->Result<Duration,Error>{self.fin.checked_duration_since(Instant::now()).filter(|d|!d.is_zero()).ok_or_else(||"RECUPERACION_PLAZO_GLOBAL".into())}
 pub fn llamada(&mut self)->Result<(),Error>{self.restante()?;self.fragmentos+=1;if self.fragmentos>MAX_FRAGMENTOS{return Err("RECUPERACION_FRAGMENTOS".into())}Ok(())}
}
pub fn fragmento(v:&Value,nombre:&str,sello:&str,offset:usize,total:&mut Option<usize>,p:&mut Presupuesto)->Result<(Vec<u8>,bool),Error>{
 p.restante()?;
 if v.get("error").is_some(){return Err("RECUPERACION_ERROR_INESPERADO".into())}
 if v["archivo"]!=nombre||v["sello"]!=sello||v["offset"].as_u64()!=Some(offset as u64){return Err("IDENTIDAD_FRAGMENTO".into())}
 let n=usize::try_from(v["total"].as_u64().ok_or("TOTAL")?)?;
 if n>8*1024*1024||total.is_some_and(|t|t!=n){return Err("TOTAL_CAMBIANTE_O_LIMITE".into())}*total=Some(n);
 let s=v["hex"].as_str().ok_or("HEX")?;if s.len()%2!=0||s.len()>65536{return Err("HEX_TAMANO".into())}
 let mut b=Vec::with_capacity(s.len()/2);
 for h in s.as_bytes().chunks_exact(2){b.push(u8::from_str_radix(std::str::from_utf8(h)?,16)?);}
 let fin=v["fin"].as_bool().ok_or("FIN")?;
 let siguiente=offset.checked_add(b.len()).ok_or("OFFSET_OVERFLOW")?;
 if b.is_empty()&&!(fin&&offset==0&&n==0){return Err("FRAGMENTO_SIN_PROGRESO".into())}
 if siguiente>n||fin!=(siguiente==n){return Err("FINAL_INCOHERENTE".into())}
 p.bytes=p.bytes.checked_add(b.len()).ok_or("BYTES_OVERFLOW")?;
 if p.bytes>MAX_BYTES{return Err("RECUPERACION_BYTES".into())}Ok((b,fin))
}
pub fn frames(c:&Caso,b:&[u8],id:&str)->Result<(),Error>{
 let mut esperados=Vec::new();
 for n in ["inicio","pesos.leidos","modelo.antes","modelo.despues","forward.antes","forward.despues"]{esperados.push(("marca",json!({"nombre":n})))}
 for n in ["consulta.A","consulta.B","generacion.inicio","generacion.fin","peticion"]{
  if c.caso=="omision"&&n=="consulta.B"{continue}
  esperados.push(("otel",json!({"nombre":n,"trace":"testigo","span":n,"padre":"testigo","atributos_perdidos":0,"eventos_perdidos":0})));
 }
 let texto=if c.caso=="hostil"{"<script>fetch('https://example.invalid/')</script><img src=x onerror=alert(1)>"}else{r#"{"peticion":"sintetica-01","referencias":[{"id":"A","version":1},{"id":"B","version":1}],"accion":"ninguna","respuesta":"A y B."}"#};
 esperados.push(("resultado",json!({"salida_original":{"texto":texto,"tokens":[]},"juicio_verificador":if c.caso=="hostil"{"ESTRUCTURA"}else{"OK"},"efectos_ejecutados":0,"sintetico":true})));
 let lineas:Vec<_>=b.split_inclusive(|x|*x==b'\n').collect();
 if Some(lineas.len())!=c.frames||lineas.len()!=esperados.len(){return Err("CONTENIDO_CANTIDAD".into())}
 let mut previo=0;
 for (i,(l,(tipo,datos))) in lineas.iter().zip(esperados).enumerate(){
  if l.last()!=Some(&b'\n'){return Err("LINEA_INCOMPLETA".into())}
  let f:Frame=serde_json::from_slice(l)?;
  if f.id!=id||f.contrato!="EIO-NAT/2"||f.seq!=i as u64+1||f.tipo!=tipo||f.datos!=datos||f.mono_ns<previo{return Err("CONTENIDO_ORDINAL".into())}previo=f.mono_ns;
 }Ok(())
}
pub fn ordinales(journal:&[u8],corte:u64,entrada:&[u8],out:&[u8],err:&[u8])->Result<(),Error>{
 let mut ordinal=0;let mut offsets=[0usize;2];let mut entrada_vista=false;let mut anterior=Value::Null;let mut ultima=Value::Null;
 for (i,l) in journal.split_inclusive(|x|*x==b'\n').enumerate(){
  if l.last()!=Some(&b'\n'){return Err("JOURNAL_INCOMPLETO".into())}
  let v:Value=serde_json::from_slice(l)?;
  if v["seq"].as_u64()!=Some(i as u64+1){return Err("JOURNAL_ORDINAL".into())}
  if v["tipo"]=="trabajo"{
   let d=&v["datos"];ordinal+=1;
   if d["ordinal"].as_u64()!=Some(ordinal){return Err("TRABAJO_ORDINAL".into())}
   match d["clase"].as_str(){
    Some("datos")=>{
     let (i,b)=match d["archivo"].as_str(){Some("inferidor.jsonl")=>(0,out),Some("inferidor.stderr")=>(1,err),_=>return Err("TRABAJO_ARCHIVO".into())};
     let n=usize::try_from(d["bytes"].as_u64().ok_or("TRABAJO_BYTES")?)?;
     let fin=offsets[i].checked_add(n).ok_or("TRABAJO_LIMITE")?;
     if n==0||fin>b.len()||d["sha256"]!=hash(&b[offsets[i]..fin]){return Err("TRABAJO_CONTENIDO".into())}offsets[i]=fin;
    },
    Some("entrada")=>{
     if entrada_vista||d["archivo"]!="entrada.txt"||d["bytes"].as_u64()!=Some(entrada.len() as u64)||d["sha256"]!=hash(entrada){return Err("TRABAJO_ENTRADA".into())}entrada_vista=true;
    },
    Some("evento")=>{if d["tipo"]!=anterior["tipo"]||d["datos"]!=anterior["datos"]||anterior["tipo"]=="trabajo"{return Err("TRABAJO_EVENTO".into())}},
    _=>return Err("TRABAJO_CLASE".into())
   }
  }
  anterior=v.clone();ultima=v;
 }
 if ordinal!=corte||!entrada_vista||offsets!=[out.len(),err.len()]||ultima["tipo"]!="cierre"||ultima["datos"]["corte_custodia"]!=corte{return Err("TRABAJO_AUSENTE".into())}Ok(())
}
pub fn conjunto(c:&Caso,archivos:&[(&str,Vec<u8>)],id:&str,sello:&str)->Result<(),Error>{
 let get=|n:&str|->Result<&[u8],Error>{archivos.iter().find(|(p,_)|*p==n).map(|(_,b)|b.as_slice()).ok_or_else(||"ARCHIVO_AUSENTE".into())};
 let m=get("MANIFIESTO.json")?;if hash(m)!=sello{return Err("SELLO_SHA".into())}
 let man:Value=serde_json::from_slice(m)?;
 if man["contrato"]!="EIO-NAT/2"||man["estado"]!=c.estado||man["completa"]!=c.admisible||man["laguna_tardia_posible"]!=json!(c.laguna){return Err("MANIFIESTO_ORACULO".into())}
 let items=man["archivos"].as_array().ok_or("INVENTARIO")?;
 if items.len()!=4{return Err("INVENTARIO_CANTIDAD".into())}
 for n in &FILES[..4]{
  let encontrados:Vec<_>=items.iter().filter(|x|x["archivo"]==*n).collect();
  if encontrados.len()!=1{return Err("INVENTARIO_IDENTIDAD".into())}
  let b=get(n)?;let x=encontrados[0];
  if x["bytes"].as_u64()!=Some(b.len() as u64)||x["sha256"]!=hash(b){return Err("IDENTIDAD".into())}
 }
 if get("entrada.txt")?!=literal("referencia")?.as_bytes()||!get("inferidor.stderr")?.is_empty(){return Err("ENTRADA_O_STDERR".into())}
 frames(c,get("inferidor.jsonl")?,id)?;
 let journal:Vec<Value>=get("supervision.jsonl")?.split(|x|*x==b'\n').filter(|x|!x.is_empty()).map(serde_json::from_slice).collect::<Result<_,_>>()?;
 for nombre in ["inicio_supervision","entrada","waitpid","cierre"]{
  if journal.iter().filter(|x|x["tipo"]==nombre).count()!=1{return Err("EVENTO_CUSTODIA_EXIGIDO".into())}
 }
 let inicio=journal.iter().find(|x|x["tipo"]=="inicio_supervision").ok_or("INICIO")?;
 if inicio["datos"]["id"]!=id||inicio["datos"]["modo"]!=c.modo||inicio["datos"]["caso"]!=c.caso{return Err("ESTIMULO_JOURNAL".into())}
 let entrada=journal.iter().find(|x|x["tipo"]=="entrada").ok_or("ENTRADA")?;
 if entrada["datos"]["id"]!=id||entrada["datos"]["peticion"]!="referencia"||entrada["datos"]["sha256"]!=hash(get("entrada.txt")?){return Err("ENTRADA_JOURNAL".into())}
 if !journal.iter().any(|x|x["tipo"]=="waitpid"&&x["datos"]["reaped"]==true){return Err("REAP_JOURNAL".into())}
 ordinales(get("supervision.jsonl")?,man["corte_custodia"].as_u64().ok_or("CORTE")?,get("entrada.txt")?,get("inferidor.jsonl")?,get("inferidor.stderr")?)
}

pub fn error_no_sellado(v:&Value)->Result<(),Error>{
 if v!=&json!({"error":"CONJUNTO_NO_SELLADO"}){return Err("ERROR_CONCURRENTE_DISTINTO".into())}Ok(())
}

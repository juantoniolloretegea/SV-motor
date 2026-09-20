//! EIO-NAT/2. Candidata Linux no compilada ni ejecutada.
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::{io::{Read,Write},sync::{Mutex,OnceLock},time::{Instant,SystemTime,UNIX_EPOCH}};
use sha2::{Digest,Sha256};
pub type Error=Box<dyn std::error::Error+Send+Sync>;
pub const MAX_FRAME:usize=65536;
pub const MAX_INPUT:usize=8192;
pub const RSS_LIMIT:u64=4*1024*1024*1024;
pub const FILES:[&str;5]=["inferidor.jsonl","inferidor.stderr","supervision.jsonl","entrada.txt","MANIFIESTO.json"];
pub fn civil()->u128{SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()}
pub fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub fn literal(nombre:&str)->Result<&'static str,&'static str>{
 match nombre {
 "referencia"=>Ok(include_str!("../pruebas/peticion.txt")),
 "estructurada"=>Ok(include_str!("../resultados/revision-05/continuacion-06/peticion-estructurada.txt")),
 _=>Err("PETICION_NO_FIJADA")
 }
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {pub habilitada:bool,pub contrato:String,pub origen:String,pub modo:String,pub caso:String,pub campana:String,pub nota:String}
impl Config{
 pub fn comprobar(&self)->Result<(),Error>{
  if !self.habilitada || self.contrato!="EIO-NAT/2" {return Err("CONFIGURACION_INACTIVA".into())}
  if !self.origen.starts_with("https://") || self.origen[8..].contains('/') ||
     self.origen.contains(".invalid") || self.origen.contains('@') || self.origen.chars().any(char::is_whitespace){return Err("ORIGEN".into())}
  if self.campana.is_empty() || self.campana.len()>48 || !self.campana.bytes().all(|x|x.is_ascii_alphanumeric()||x==b'-'){return Err("CAMPANA".into())}
  if !["testigo","modelo"].contains(&self.modo.as_str()) {return Err("MODO".into())}
  if !["normal","bloqueo","abrupto","omision","duplicado","orden","hostil","memoria","cola","registro","escritura","no_cero","eof_ausente","tardio","bloqueo_antes","bloqueo_entre","bloqueo_sellado","fallo_sync","cola_pendiente","escritor_desconectado"].contains(&self.caso.as_str()){return Err("CASO".into())}
  Ok(())
 }
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(tag="op",deny_unknown_fields)]
pub enum Orden {
 #[serde(rename="iniciar")] Iniciar{contrato:String,peticion:String,texto:String},
 #[serde(rename="estado")] Estado,
 #[serde(rename="cancelar")] Cancelar{id:String},
 #[serde(rename="evidencia")] Evidencia{archivo:String,offset:u64,sello:String},
}
pub fn validar_inicio(contrato:&str,p:&str,bytes:&str)->Result<(),&'static str>{
 if contrato!="EIO-NAT/2"{return Err("VERSION")}
 if bytes.len()>MAX_INPUT{return Err("LIMITE_ENTRADA")}
 if literal(p)? != bytes{return Err("PETICION_NO_FIJADA")} Ok(())
}
pub fn send<T:Serialize>(w:&mut impl Write,v:&T)->Result<(),Error>{
 let b=serde_json::to_vec(v)?;if b.len()>2*MAX_FRAME{return Err("IPC_TAMANO".into())}
 w.write_all(&(b.len() as u32).to_be_bytes())?;w.write_all(&b)?;w.flush()?;Ok(())
}
pub fn recv<T:serde::de::DeserializeOwned>(r:&mut impl Read,max:usize)->Result<T,Error>{
 let mut n=[0;4];r.read_exact(&mut n)?;let n=u32::from_be_bytes(n) as usize;
 if n>max{return Err("IPC_TAMANO".into())}let mut b=vec![0;n];r.read_exact(&mut b)?;Ok(serde_json::from_slice(&b)?)
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {pub contrato:String,pub id:String,pub seq:u64,pub mono_ns:u128,pub civil_unix_ms:u128,pub tipo:String,pub datos:Value}
static SINK:OnceLock<Mutex<(String,u64,Instant)>>=OnceLock::new();
pub fn iniciar_sink(id:String)->Result<(),Error>{SINK.set(Mutex::new((id,0,Instant::now()))).map_err(|_|"SINK_DUPLICADO")?;Ok(())}
pub fn emitir(tipo:&str,datos:Value)->Result<(),Error>{
 let Some(sink)=SINK.get() else{return Ok(())}; // banco heredado: ningún proceso/archivo implícito.
 let mut s=sink.lock().map_err(|_|"SINK_MUTEX")?;s.1+=1;
 let f=Frame{contrato:"EIO-NAT/2".into(),id:s.0.clone(),seq:s.1,mono_ns:s.2.elapsed().as_nanos(),civil_unix_ms:civil(),tipo:tipo.into(),datos};
 let mut b=serde_json::to_vec(&f)?;b.push(b'\n');if b.len()>MAX_FRAME{return Err("EVENTO_TAMANO".into())}
 let mut out=std::io::stdout().lock();out.write_all(&b)?;out.flush()?;Ok(())
}
pub fn marca(s:&str)->Result<(),Error>{emitir("marca",json!({"nombre":s}))}
/// Oráculo de secuencia independiente de los contadores del emisor.
#[derive(Default)]
pub struct Oracle{pub seq:u64,pub marcas:Vec<String>,pub spans:Vec<String>,pub resultado:Option<Value>,pub error:bool}
impl Oracle{
 pub fn recibir(&mut self,b:&[u8],id:&str)->Result<(),Error>{
  let f:Frame=serde_json::from_slice(b)?;
  if f.contrato!="EIO-NAT/2"||f.id!=id||f.seq!=self.seq+1||self.resultado.is_some(){self.error=true;return Err("IDENTIDAD_SECUENCIA_O_TARDIO".into())}
  self.seq=f.seq;
  match f.tipo.as_str(){
   "marca"=>self.marcas.push(f.datos["nombre"].as_str().ok_or("MARCA")?.into()),
   "otel"=>{
    if f.datos["atributos_perdidos"]!=0 || f.datos["eventos_perdidos"]!=0 {return Err("OTEL_PERDIDO".into())}
    self.spans.push(f.datos["nombre"].as_str().ok_or("SPAN")?.into());
   },
   "resultado"=>{self.resultado=Some(f.datos);},
   _=>return Err("TIPO_EVENTO".into())
  }
  if self.marcas.len()>16||self.spans.len()>16{return Err("EVENTOS_EXCESIVOS".into())}Ok(())
 }
 pub fn completa(&self)->bool{
  !self.error && self.marcas==["inicio","pesos.leidos","modelo.antes","modelo.despues","forward.antes","forward.despues"]
   && self.spans==["consulta.A","consulta.B","generacion.inicio","generacion.fin","peticion"]
   && self.resultado.is_some()
 }
}
/// Cancelación admitida antes del cierre exterior revoca siempre el resultado.
#[derive(Debug,Default)]
pub struct Cierre{pub revocada:bool,pub cerrada:bool}
impl Cierre{
 pub fn cancelar(&mut self)->bool{if self.cerrada{return false}self.revocada=true;true}
 pub fn concluir(&mut self,exit_ok:bool,eof:bool,custodia:bool,oracle:bool)->&'static str{
  self.cerrada=true;if self.revocada{"interrumpida"}else if exit_ok&&eof&&custodia&&oracle{"terminada"}else{"desconocida"}
 }
}


/// Única decisión R1. El juicio contractual NO entra en la admisibilidad técnica.
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Admision{
 pub proceso_ok:bool,pub eof_completos:bool,pub secuencia_completa:bool,
 pub revocada:bool,pub fallo_observacion:bool,pub fallo_custodia:bool,
 pub oraculo_ok:bool,pub sellado_ok:bool,
}
impl Admision{
 pub fn admisible(&self)->bool{
  self.proceso_ok&&self.eof_completos&&self.secuencia_completa&&!self.revocada
   &&!self.fallo_observacion&&!self.fallo_custodia&&self.oraculo_ok&&self.sellado_ok
 }
 pub fn estado(&self)->&'static str{
  if self.admisible(){"terminada"}else if self.revocada{"interrumpida"}else{"desconocida"}
 }
}
#[path="custodia.rs"]
pub mod custodia;
#[path="parada.rs"]
pub mod parada;

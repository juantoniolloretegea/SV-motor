//! Etapas instrumentales del contraste finito. No atribuye autoridad R1 ni adjudica significado.
use serde_json::{Value,json};use crate::huella;
type R<T>=Result<T,String>;
fn exigir(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
pub const IDS:[&str;6]=["N01","N02","N03","N04","N05","N06"];
pub const DOCS:[&str;6]=["N-A","N-A","N-B","N-B","N-C","N-C"];
#[derive(Default)]
pub struct Puerta{preparadas:Vec<Value>,confirmadas:usize,indice:usize,heredado:bool,cargado:bool,motor:bool,generando:bool,salida:Option<String>,completo:bool,pendiente:bool,cerrado:bool}
impl Puerta{
 pub fn heredar(&mut self,v:&Value,sha_fijada:&str)->R<()> {
  exigir(!self.cargado&&!self.heredado&&!self.cerrado&&self.indice==0&&self.confirmadas==6,"Herencia fuera de etapa")?;
  exigir(huella(&serde_json::to_vec(v).unwrap())==sha_fijada,"Identidad de reanudación discordante")?;
  exigir(v.as_object().map(|o|o.len())==Some(7)&&v["revision"]=="ARBITRO-SV-SAFEGUARD-20261002/v3"&&v["id"]=="N01"&&v["siguiente"]=="N02"&&v["accion"]=="continuar"&&v["salida_sha256"]=="7bdd77d47d383f6a128e5a33540bed63db81dbd18f046e44186020528062c0fe","Herencia no autorizada")?;
  for k in ["evaluacion_sha256","cotejo_sha256"] {exigir(v[k].as_str().is_some_and(|s|s.len()==64&&s.bytes().all(|b|b.is_ascii_hexdigit())),"Falta evaluación o cotejo")?;}
  self.indice=1;self.heredado=true;Ok(())
 }
 pub fn id(&self)->&str{IDS[self.indice.min(5)]}
 pub fn documento(&self)->&str{DOCS[if self.cargado{self.indice.min(5)}else{self.preparadas.len().min(5)}]}
 pub fn pendientes(&self)->bool{self.pendiente}
 pub fn salida_sha256(&self)->Option<&str>{self.salida.as_deref()}
 pub fn cerrado(&self)->bool{self.cerrado}
 pub fn preparar(&mut self,d:&Value,esperada:Option<&Value>)->R<()>{
  exigir(!self.cargado&&!self.cerrado&&self.preparadas.len()<6,"Preparación fuera de etapa")?;let i=self.preparadas.len();let t=d["tokens"].as_array().ok_or("Tokens ausentes")?;
  exigir(d["id"]==IDS[i]&&d["documento"]==DOCS[i]&&d["seccion"]=="S1","Correspondencia caso-documento")?;
  exigir(t.len()+4096+1024<=8192&&d["max_salida"]==4096&&d["paginas"]==json!([0,1])&&d["funciones"]==json!([]),"Contexto o cobertura no conforme")?;
  exigir(d["tokens_sha256"]==huella(&serde_json::to_vec(t).unwrap()),"Huella de entrada")?;
  if let Some(e)=esperada{exigir(d==e,"Entrada distinta de la prefijada")?;}
  self.preparadas.push(d.clone());Ok(())
 }
 pub fn motor(&mut self,d:&Value)->R<()>{
  exigir(self.cargado&&!self.cerrado&&!self.motor&&!self.generando&&!self.pendiente,"Orden de motor")?;
  let p=&self.preparadas[self.indice];for k in ["id","documento","seccion","mensajes","funciones","plantilla_efectiva","tokens","tokens_sha256","max_salida"]{exigir(d[k]==p[k],&format!("Entrada efectiva discordante: {k}"))?;}
  self.motor=true;Ok(())
 }
 pub fn solicitar(&mut self,etapa:&str,id:&str,sha:&str,mode:&str)->R<()>{
  exigir(!self.cerrado,"Recorrido cerrado")?;
  if etapa=="preparacion" {
   exigir(!self.cargado&&self.confirmadas<6&&self.preparadas.len()==self.confirmadas+1,"Preparación repetida")?;
   let p=&self.preparadas[self.confirmadas];exigir(p["id"]==id&&p["tokens_sha256"]==sha,"Confirmación discordante")?;self.confirmadas+=1;return Ok(())
  }
  exigir(mode=="contraste"&&self.confirmadas==6&&self.preparadas.len()==6,"Sin seis entradas admitidas")?;
  let p=&self.preparadas[self.indice];exigir(p["id"]==id&&p["tokens_sha256"]==sha,"Caso o huella no concordante")?;
  match etapa{
   "carga"=>{exigir(!self.cargado&&self.indice==1&&self.heredado,"Carga repetida")?;self.cargado=true;},
   "generacion"=>{exigir(self.cargado&&self.motor&&!self.generando&&!self.pendiente&&self.salida.is_none(),"Generación repetida o sin entrada")?;self.generando=true;},
   "adjudicacion"=>{exigir(self.generando&&self.completo&&self.salida.is_some()&&!self.pendiente,"Adjudicación sin original completo")?;self.pendiente=true;},
   _=>return Err("Etapa no autorizada".into())
  }Ok(())
 }
 pub fn emision(&mut self,d:&Value)->R<()>{
  exigir(self.generando&&self.salida.is_none()&&d["id"]==self.id(),"Emisión fuera de caso")?;
  let txt=d["texto"].as_str().ok_or("Original ausente")?;exigir(d["tokens"].as_array().is_some_and(|v|!v.is_empty()&&v.len()<=4096),"Cota de emisión")?;
  self.salida=Some(huella(txt.as_bytes()));Ok(())
 }
 pub fn terminar(&mut self,d:&Value)->R<()>{
  exigir(self.generando&&self.salida.is_some()&&!self.completo&&d["id"]==self.id()&&d["completo"]==true,"Caso incompleto o atribución discordante")?;self.completo=true;Ok(())
 }
 pub fn adjudicar(&mut self,v:&Value)->R<bool>{
  exigir(self.pendiente&&!self.cerrado,"No hay adjudicación pendiente")?;
  exigir(v.as_object().map(|o|o.len())==Some(3)&&v["id"]==self.id()&&v["salida_sha256"].as_str()==self.salida.as_deref(),"Adjudicación sin identidad de original")?;
  let seguir=match v["accion"].as_str(){Some("continuar")=>true,Some("cerrar")=>false,_=>return Err("Acción de continuidad desconocida".into())};
  exigir(!seguir||self.indice<5,"No se admite séptimo caso")?;
  self.pendiente=false;if seguir{self.indice+=1;self.motor=false;self.generando=false;self.salida=None;self.completo=false;}else{self.cerrado=true;}Ok(seguir)
 }
}
pub fn validar_solicitud(b:&[u8],documento:&str)->R<()>{
 let v:Value=serde_json::from_slice(b).map_err(|e|e.to_string())?;
 match v["method"].as_str(){
 Some("initialize"|"notifications/initialized"|"tools/list")=>Ok(()),
 Some("tools/call")=>{let a=&v["params"]["arguments"];exigir(v["params"]["name"]=="leer_documento"&&a["documento"]==documento&&a["seccion"]=="S1"&&matches!(a["pagina"].as_u64(),Some(0)|Some(1))&&a.as_object().map(|a|a.len())==Some(3),"Solicitud fuera del caso fijado")},
 _=>Err("Método no autorizado".into())
 }
}

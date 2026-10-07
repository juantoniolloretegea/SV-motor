//! Etapas instrumentales del contraste finito. No atribuye autoridad R1 ni adjudica significado.
use serde_json::{Value,json};use crate::{huella,retroalimentacion::{validar_plan,NUM_CASOS,CONTEXTO,SALIDA,RESERVA}};
type R<T>=Result<T,String>;
fn exigir(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
pub const IDS:[&str;1]=["A08"];
pub const DOCS:[&str;1]=["BANCO-A";1];
pub struct Puerta{plan:Value,preparadas:Vec<Value>,confirmadas:usize,indice:usize,cargado:bool,motor:bool,generando:bool,salida:Option<String>,completo:bool,pendiente:bool,cerrado:bool}
impl Puerta{
 pub fn nueva(plan:&Value)->R<Self>{validar_plan(plan)?;Ok(Self{plan:plan.clone(),preparadas:Vec::new(),confirmadas:0,indice:0,cargado:false,motor:false,generando:false,salida:None,completo:false,pendiente:false,cerrado:false})}
 pub fn ultima(&self)->bool{self.indice==NUM_CASOS-1}

 pub fn id(&self)->&str{self.plan["casos"][self.indice.min(NUM_CASOS-1)]["id"].as_str().unwrap()}
 pub fn documento(&self)->&str{self.plan["casos"][if self.cargado{self.indice.min(NUM_CASOS-1)}else{self.preparadas.len().min(NUM_CASOS-1)}]["documento"].as_str().unwrap()}
 pub fn seccion(&self)->&str{self.plan["casos"][if self.cargado{self.indice.min(NUM_CASOS-1)}else{self.preparadas.len().min(NUM_CASOS-1)}]["seccion"].as_str().unwrap()}
 pub fn pendientes(&self)->bool{self.pendiente}
 pub fn salida_sha256(&self)->Option<&str>{self.salida.as_deref()}
 pub fn cerrado(&self)->bool{self.cerrado}
 pub fn preparar(&mut self,d:&Value,esperada:Option<&Value>)->R<()>{
  exigir(!self.cargado&&!self.cerrado&&self.preparadas.len()<IDS.len(),"Preparación fuera de etapa")?;let i=self.preparadas.len();let t=d["tokens"].as_array().ok_or("Tokens ausentes")?;
  exigir(d["id"]==self.plan["casos"][i]["id"]&&d["documento"]==self.plan["casos"][i]["documento"]&&d["seccion"]==self.plan["casos"][i]["seccion"]&&d["ronda"]==self.plan["capa"],"Correspondencia caso-documento")?;
  exigir(t.len()+SALIDA+RESERVA<=CONTEXTO&&d["max_salida"]==SALIDA&&d["paginas"]==json!([0,1])&&d["funciones"]==json!([]),"Contexto o cobertura no conforme")?;
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
   exigir(!self.cargado&&self.confirmadas<IDS.len()&&self.preparadas.len()==self.confirmadas+1,"Preparación repetida")?;
   let p=&self.preparadas[self.confirmadas];exigir(p["id"]==id&&p["tokens_sha256"]==sha,"Confirmación discordante")?;self.confirmadas+=1;return Ok(())
  }
  exigir(mode=="contraste"&&self.confirmadas==IDS.len()&&self.preparadas.len()==IDS.len(),"Sin nueve entradas admitidas")?;
  let p=&self.preparadas[self.indice];exigir(p["id"]==id&&p["tokens_sha256"]==sha,"Caso o huella no concordante")?;
  match etapa{
   "carga"=>{exigir(!self.cargado&&self.indice==0,"Carga repetida")?;self.cargado=true;},
   "generacion"=>{exigir(self.cargado&&self.motor&&!self.generando&&!self.pendiente&&self.salida.is_none(),"Generación repetida o sin entrada")?;self.generando=true;},
   "adjudicacion"=>{exigir(self.generando&&self.completo&&self.salida.is_some()&&!self.pendiente,"Adjudicación sin original completo")?;self.pendiente=true;},
   _=>return Err("Etapa no autorizada".into())
  }Ok(())
 }
 pub fn emision(&mut self,d:&Value)->R<()>{
  exigir(self.generando&&self.salida.is_none()&&d["id"]==self.id(),"Emisión fuera de caso")?;
  let txt=d["texto"].as_str().ok_or("Original ausente")?;exigir(d["tokens"].as_array().is_some_and(|v|!v.is_empty()&&v.len()<=SALIDA),"Cota de emisión")?;
  self.salida=Some(huella(txt.as_bytes()));Ok(())
 }
 pub fn terminar(&mut self,d:&Value)->R<()>{
  exigir(self.generando&&self.salida.is_some()&&!self.completo&&d["id"]==self.id()&&d["completo"]==true,"Caso incompleto o atribución discordante")?;self.completo=true;Ok(())
 }
 pub fn adjudicar(&mut self,v:&Value)->R<bool>{
  exigir(self.pendiente&&!self.cerrado,"No hay adjudicación pendiente")?;
  exigir(v.as_object().map(|o|o.len())==Some(3)&&v["id"]==self.id()&&v["salida_sha256"].as_str()==self.salida.as_deref(),"Adjudicación sin identidad de original")?;
  let seguir=match v["accion"].as_str(){Some("continuar")=>true,Some("cerrar")=>false,_=>return Err("Acción de continuidad desconocida".into())};
  exigir(!seguir||self.indice<IDS.len()-1,"No se admite décimo caso")?;
  self.pendiente=false;if seguir{self.indice+=1;self.motor=false;self.generando=false;self.salida=None;self.completo=false;}else{self.cerrado=true;}Ok(seguir)
 }
}
pub fn validar_solicitud(b:&[u8],documento:&str,seccion:&str)->R<()>{
 let v:Value=serde_json::from_slice(b).map_err(|e|e.to_string())?;
 match v["method"].as_str(){
 Some("initialize"|"notifications/initialized"|"tools/list")=>Ok(()),
 Some("tools/call")=>{let a=&v["params"]["arguments"];exigir(v["params"]["name"]=="leer_documento"&&a["documento"]==documento&&a["seccion"]==seccion&&matches!(a["pagina"].as_u64(),Some(0)|Some(1))&&a.as_object().map(|a|a.len())==Some(3),"Solicitud fuera del caso fijado")},
 _=>Err("Método no autorizado".into())
 }
}

#[cfg(test)]mod tests{
 use super::*;
 fn entrada()->Value{let t=json!([10]);json!({"id":"A08","documento":"BANCO-A","seccion":"A08","tokens":t,"tokens_sha256":huella(&serde_json::to_vec(&t).unwrap()),"max_salida":SALIDA,"ronda":0,"paginas":[0,1],"funciones":[],"mensajes":["sintetico"],"plantilla_efectiva":"sintetica"})}
 fn preparada()->Puerta{let mut p=Puerta::nueva(&crate::retroalimentacion::plan_prueba(0)).unwrap();let v=entrada();p.preparar(&v,Some(&v)).unwrap();p.solicitar("preparacion","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p.solicitar("carga","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p}
 fn emitida()->Puerta{let mut p=preparada();let v=entrada();p.motor(&v).unwrap();p.solicitar("generacion","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p.emision(&json!({"id":"A08","texto":"original","tokens":[1]})).unwrap();p.terminar(&json!({"id":"A08","completo":true})).unwrap();p.solicitar("adjudicacion","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p}
 #[test]fn unico_caso_sin_ampliacion(){let mut p=emitida();assert!(p.adjudicar(&json!({"id":"A08","salida_sha256":huella(b"original"),"accion":"continuar"})).is_err());p.adjudicar(&json!({"id":"A08","salida_sha256":huella(b"original"),"accion":"cerrar"})).unwrap();assert!(p.cerrado());assert!(p.motor(&entrada()).is_err());}
 #[test]fn no_carga_ni_generacion_repetida(){let mut p=preparada();assert!(p.solicitar("carga","A08",entrada()["tokens_sha256"].as_str().unwrap(),"contraste").is_err());let mut p=emitida();assert!(p.motor(&entrada()).is_err());}
 #[test]fn original_y_clave_protegidos(){let mut p=emitida();assert!(p.adjudicar(&json!({"id":"A08","salida_sha256":"falsa","accion":"cerrar"})).is_err());assert!(p.adjudicar(&json!({"id":"A08","salida_sha256":huella(b"original"),"accion":"cerrar","solucion":"ajena"})).is_err());}
 #[test]fn fuente_y_contexto_fijados(){let mut p=preparada();let mut v=entrada();v["mensajes"]=json!(["alterado"]);assert!(p.motor(&v).is_err());let b=serde_json::to_vec(&json!({"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":"BANCO-A","seccion":"A09","pagina":0}}})).unwrap();assert!(validar_solicitud(&b,"BANCO-A","A08").is_err());}
}

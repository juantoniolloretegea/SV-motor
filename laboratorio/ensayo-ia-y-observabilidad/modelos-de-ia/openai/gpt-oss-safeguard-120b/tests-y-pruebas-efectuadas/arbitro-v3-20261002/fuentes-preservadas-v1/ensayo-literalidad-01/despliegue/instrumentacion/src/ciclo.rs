//! Etapas instrumentales del contraste finito. No atribuye autoridad R1 ni adjudica significado.
use serde_json::{Value,json};use crate::huella;
type R<T>=Result<T,String>;
fn exigir(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
pub const IDS:[&str;7]=["D01","F01","F02","F03","F04","F05","F06"];
pub const DOCS:[&str;7]=["N-A","F-A","F-A","F-B","F-B","F-C","F-C"];
#[derive(Default)]
pub struct Puerta{preparadas:Vec<Value>,confirmadas:usize,indice:usize,cargado:bool,motor:bool,generando:bool,salida:Option<String>,completo:bool,pendiente:bool,cerrado:bool}
impl Puerta{
 pub fn id(&self)->&str{IDS[self.indice.min(IDS.len()-1)]}
 pub fn documento(&self)->&str{DOCS[if self.cargado{self.indice.min(IDS.len()-1)}else{self.preparadas.len().min(IDS.len()-1)}]}
 pub fn pendientes(&self)->bool{self.pendiente}
 pub fn salida_sha256(&self)->Option<&str>{self.salida.as_deref()}
 pub fn cerrado(&self)->bool{self.cerrado}
 pub fn preparar(&mut self,d:&Value,esperada:Option<&Value>)->R<()>{
  exigir(!self.cargado&&!self.cerrado&&self.preparadas.len()<IDS.len(),"Preparación fuera de etapa")?;let i=self.preparadas.len();let t=d["tokens"].as_array().ok_or("Tokens ausentes")?;
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
   exigir(!self.cargado&&self.confirmadas<IDS.len()&&self.preparadas.len()==self.confirmadas+1,"Preparación repetida")?;
   let p=&self.preparadas[self.confirmadas];exigir(p["id"]==id&&p["tokens_sha256"]==sha,"Confirmación discordante")?;self.confirmadas+=1;return Ok(())
  }
  exigir(mode=="contraste"&&self.confirmadas==IDS.len()&&self.preparadas.len()==IDS.len(),"Sin siete entradas admitidas")?;
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
  exigir(!seguir||self.indice<IDS.len()-1,"No se admite octavo caso")?;
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
#[cfg(test)]mod tests{
 use super::*;
 fn c(i:usize)->Value{let t=json!([10+i]);json!({"id":IDS[i],"documento":DOCS[i],"seccion":"S1","tokens":t,"tokens_sha256":huella(&serde_json::to_vec(&t).unwrap()),"max_salida":4096,"paginas":[0,1],"funciones":[],"mensajes":[{"role":"system","content":"sintetico"},{"role":"user","content":IDS[i]}],"plantilla_efectiva":IDS[i]})}
 fn preparada()->Puerta{let mut p=Puerta::default();for i in 0..IDS.len(){let v=c(i);p.preparar(&v,Some(&v)).unwrap();p.solicitar("preparacion",IDS[i],v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();}p.solicitar("carga","D01",c(0)["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p}
 fn emitir(p:&mut Puerta,i:usize){let v=c(i);p.motor(&v).unwrap();p.solicitar("generacion",IDS[i],v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p.emision(&json!({"id":IDS[i],"texto":"original sintetico","tokens":[1]})).unwrap();p.terminar(&json!({"id":IDS[i],"completo":true})).unwrap();p.solicitar("adjudicacion",IDS[i],v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();}
 fn control(i:usize,accion:&str)->Value{json!({"id":IDS[i],"salida_sha256":huella(b"original sintetico"),"accion":accion})}
 #[test]fn siete_casos_y_cierre(){let mut p=preparada();for i in 0..IDS.len(){emitir(&mut p,i);p.adjudicar(&control(i,if i<IDS.len()-1{"continuar"}else{"cerrar"})).unwrap();}assert!(p.cerrado());assert!(p.motor(&c(0)).is_err());}
 #[test]fn no_otra_carga(){let mut p=preparada();assert!(p.solicitar("carga","D01",c(0)["tokens_sha256"].as_str().unwrap(),"contraste").is_err());}
 #[test]fn no_avanza_sin_adjudicar(){let mut p=preparada();emitir(&mut p,0);assert!(p.motor(&c(1)).is_err());}
 #[test]fn cierre_anticipado_no_reabre(){let mut p=preparada();emitir(&mut p,0);p.adjudicar(&control(0,"cerrar")).unwrap();assert!(p.motor(&c(1)).is_err());}
 #[test]fn hash_salida_obligatorio(){let mut p=preparada();emitir(&mut p,0);let mut v=control(0,"continuar");v["salida_sha256"]=json!("otra");assert!(p.adjudicar(&v).is_err());}
 #[test]fn clave_no_admitida_en_control(){let mut p=preparada();emitir(&mut p,0);let mut v=control(0,"continuar");v["decision"]=json!("RESPALDADA");assert!(p.adjudicar(&v).is_err());}
 #[test]fn identidad_de_caso_obligatoria(){let mut p=preparada();assert!(p.motor(&c(1)).is_err());}
 #[test]fn ninguna_memoria_conversacional_heredada(){let mut p=preparada();emitir(&mut p,0);p.adjudicar(&control(0,"continuar")).unwrap();let mut v=c(1);v["mensajes"].as_array_mut().unwrap().push(json!({"role":"assistant","content":"respuesta anterior"}));assert!(p.motor(&v).is_err());}
 #[test]fn documento_ajeno_impedido(){let mut p=Puerta::default();let mut v=c(0);v["documento"]=json!("N-C");assert!(p.preparar(&v,None).is_err());}
 #[test]fn reserva_completa_obligatoria(){let mut p=Puerta::default();let mut v=c(0);let t=vec![1;3073];v["tokens"]=json!(t);v["tokens_sha256"]=json!(huella(&serde_json::to_vec(&t).unwrap()));assert!(p.preparar(&v,None).is_err());}
 #[test]fn no_carga_con_seis_entradas(){let mut p=Puerta::default();for i in 0..IDS.len()-1{let v=c(i);p.preparar(&v,None).unwrap();p.solicitar("preparacion",IDS[i],v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();}assert!(p.solicitar("carga","D01",c(0)["tokens_sha256"].as_str().unwrap(),"contraste").is_err());}
 #[test]fn no_generacion_directa(){assert!(Puerta::default().solicitar("generacion","D01","h","contraste").is_err());}
 #[test]fn no_octavo_caso(){let mut p=preparada();for i in 0..IDS.len()-1{emitir(&mut p,i);p.adjudicar(&control(i,"continuar")).unwrap();}emitir(&mut p,IDS.len()-1);assert!(p.adjudicar(&control(IDS.len()-1,"continuar")).is_err());}
 #[test]fn mcp_no_accede_a_otro_documento(){let v=json!({"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":"N-A","seccion":"S1","pagina":1}}});let b=serde_json::to_vec(&v).unwrap();assert!(validar_solicitud(&b,"N-A").is_ok());assert!(validar_solicitud(&b,"N-B").is_err());}
 #[test]fn original_incompleto_impide_adjudicacion(){let mut p=preparada();p.motor(&c(0)).unwrap();p.solicitar("generacion","D01",c(0)["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();assert!(p.terminar(&json!({"id":"D01","completo":false})).is_err());}
}

use eio_candidato::{Condiciones, Referencia, ejecutar, parada, telemetria::Telemetria};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Caso { id:String, esperado:String, efecto_esperado:bool, condiciones:Condiciones, propuesta:String,
 consultas:Vec<String>, omitir_evento_b:bool, fallo_exportador:bool, instruccion_fuente:String }
fn main() -> Result<(), Box<dyn std::error::Error>> {
 let casos:Vec<Caso> = serde_json::from_str(include_str!("casos.json"))?;
 let mut fallos=0;
 for caso in casos {
  let t=Telemetria::nueva(&caso.id,caso.omitir_evento_b,caso.fallo_exportador);
  let mut consultas=Vec::new();
  for id in &caso.consultas {
   if !["A","B","C"].contains(&id.as_str()) {return Err("fuente ajena".into());}
   consultas.push(Referencia{id:id.clone(),version:1});t.evento(&format!("consulta.{id}"));
  }
  t.evento(if caso.condiciones.activa {"condicion.activa"} else {"condicion.inactiva"});
  let export_ok=t.cerrar();let estado=t.estado.lock().unwrap();
  let observadas:Vec<Referencia>=estado.registros.iter().filter_map(|r|
   r.nombre.strip_prefix("consulta.").map(|id|Referencia{id:id.into(),version:1})).collect();
  let mut c=caso.condiciones.clone();c.exportacion_correcta &= export_ok;
  let mut objeto=false;
  let observado=ejecutar(&c,&caso.propuesta,&consultas,&observadas,&mut objeto).err().unwrap_or("OK");
  let correlacion=estado.registros.iter().filter(|r|r.nombre.starts_with("consulta.")).all(|r|
   estado.registros.iter().any(|p|p.nombre=="peticion" && p.trace==r.trace && p.span==r.padre));
  let perdida_exacta=!caso.omitir_evento_b || (estado.descartados==1 && observadas.len()+1==consultas.len());
  let conforme=observado==caso.esperado && objeto==caso.efecto_esperado && correlacion && perdida_exacta;
  if !conforme {fallos+=1;}
  // Contenido inerte: nunca se interpreta como permiso, código o llamada de herramienta.
  let _contenido_sin_autoridad=&caso.instruccion_fuente;
  println!("{}",serde_json::json!({"tipo":"adaptador_directo","caso":caso.id,
   "esperado":caso.esperado,"observado":observado,"efecto":objeto,"conforme":conforme,"telemetria":&*estado}));
 }
 for (id,token,eos,n,ctx,cancel,esperado) in [
  ("eos_correcto",151645,151645,1,101,false,Some("EOS")),
  ("no_confundir_pad",151643,151645,1,101,false,None),
  ("limite_generacion",42,151645,128,228,false,Some("LIMITE_GENERACION")),
  ("cancelacion",42,151645,1,101,true,Some("CANCELACION")),
  ("limite_contexto",42,151645,1,2048,false,Some("LIMITE_CONTEXTO"))] {
  let observado=parada(token,eos,n,ctx,cancel);let ok=observado==esperado;if !ok {fallos+=1;}
  println!("{}",serde_json::json!({"tipo":"control_generador_directo","caso":id,"esperado":esperado,"observado":observado,"conforme":ok}));
 }
 if fallos>0 {return Err(format!("{fallos} fallos").into());} Ok(())
}

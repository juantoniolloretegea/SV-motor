use eio_candidato::{nativa::*,telemetria::Telemetria,inferencia::generar,Referencia,Condiciones};
use std::{fs,sync::atomic::AtomicBool};
use serde_json::json;
fn main()->Result<(),Error>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=3{return Err("uso: inferidor ID referencia|estructurada".into())}
 let peticion=literal(&a[2])?;iniciar_sink(a[1].clone())?;marca("inicio")?;
 // Rutas fijas relativas al directorio de entradas de sólo lectura; no rutas del modelo/HTTP.
 let pesos=leer("Qwen3-0.6B-Q4_K_M.gguf",396705472,"ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a")?;
 let tokenizer=leer("tokenizer.json",11422654,"aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4")?;
 marca("pesos.leidos")?;
 let t=Telemetria::nueva("modelo-sintetico-01",false,false);
 t.evento("consulta.A");t.evento("consulta.B");t.evento("generacion.inicio");
 let s=generar(&pesos,&tokenizer,peticion,&AtomicBool::new(false))?;
 t.evento("generacion.fin");let exportacion=t.cerrar();
 let refs=vec![Referencia{id:"A".into(),version:1},Referencia{id:"B".into(),version:1}];
 let obs=t.estado.lock().map_err(|_|"MUTEX")?.registros.iter().filter_map(|r|r.nombre.strip_prefix("consulta.").map(|id|Referencia{id:id.into(),version:1})).collect::<Vec<_>>();
 let c=Condiciones{id:"sintetica-01".into(),permiso:false,activa:false,veto:false,tokens_entrada:s.entrada,exportacion_correcta:exportacion};
 let juicio=eio_candidato::comprobar(&c,&s.texto,&refs,&obs).err().unwrap_or("OK");
 emitir("resultado",json!({"salida_original":s,"juicio_verificador":juicio,"efectos_ejecutados":0}))?;
 if !exportacion{return Err("EXPORTACION".into())}Ok(())
}
fn leer(p:&str,n:u64,h:&str)->Result<Vec<u8>,Error>{
 if fs::symlink_metadata(p)?.file_type().is_symlink()||fs::metadata(p)?.len()!=n{return Err("IDENTIDAD_TAMANO".into())}
 let b=fs::read(p)?;if b.len() as u64!=n||hash(&b)!=h{return Err("IDENTIDAD_SHA256".into())}Ok(b)
}

use std::{fs,sync::atomic::AtomicBool};
use eio_candidato::{inferencia::generar,telemetria::Telemetria};
fn main()->Result<(),Box<dyn std::error::Error+Send+Sync>>{
 let a:Vec<String>=std::env::args().collect();
 if a.len()!=5 || !["on","off"].contains(&a[4].as_str()){return Err("uso: inferencia PESOS TOKENIZER PETICION on|off".into());}
 if fs::metadata(&a[1])?.len()>500_000_000 {return Err("pesos exceden 500 MB".into());}
 if fs::metadata(&a[2])?.len()>32*1024*1024 || fs::metadata(&a[3])?.len()>8192 {return Err("entrada demasiado grande".into());}
 let t=if a[4]=="on"{Some(Telemetria::nueva("modelo-sintetico-01",false,false))}else{None};
 let pesos=fs::read(&a[1])?;let tokenizer=fs::read(&a[2])?;let peticion=fs::read_to_string(&a[3])?;
 if peticion!=include_str!("../pruebas/peticion.txt"){return Err("PETICION_NO_FIJADA".into());}
 if let Some(t)=&t{t.evento("consulta.A");t.evento("consulta.B");t.evento("generacion.inicio");}
 let s=generar(&pesos,&tokenizer,&peticion,&AtomicBool::new(false))?;
 if let Some(t)=&t{t.evento("generacion.fin");if !t.cerrar(){return Err("EXPORTACION".into());}}
 let refs=vec![eio_candidato::Referencia{id:"A".into(),version:1},eio_candidato::Referencia{id:"B".into(),version:1}];
 let c=eio_candidato::Condiciones{id:"sintetica-01".into(),permiso:false,activa:false,veto:false,tokens_entrada:s.entrada,exportacion_correcta:true};
 let obs=if let Some(t)=&t {t.estado.lock().unwrap().registros.iter().filter_map(|r|r.nombre.strip_prefix("consulta.").map(|id|eio_candidato::Referencia{id:id.into(),version:1})).collect::<Vec<_>>()}else{refs.clone()};
 let contrato=eio_candidato::comprobar(&c,&s.texto,&refs,&obs).err().unwrap_or("OK");
 println!("{}",serde_json::json!({"tipo":"modelo_real","telemetria":a[4],"salida":s,"contrato":contrato,"alcance_off":"coste; sin cobertura OpenTelemetry"}));
 if let Some(t)=&t{eprintln!("{}",serde_json::to_string(&*t.estado.lock().unwrap())?);}
 Ok(())
}

//! Continuación hasta P25 autorizada expresamente el 30/09/2026.
//! Suprime exclusivamente el vencimiento de campaña, no las guardas técnicas.
use crate::{Result,store,control::functional::Policy};
use serde_json::{Value,json};
use std::time::{Duration,Instant};
pub const AUTH:&str="DIRECCION-20260930-HASTA-P25";
pub const ORIGINAL:&str="fcc942d3249f852c03c240795243c47aec791cde6463a195f863c0fa96b82011";
pub fn policy(p:&Policy)->bool{p.is_exam()&&p.economic_deadline_utc_ms==0}
pub fn campaign(old:&Value)->Result<Value>{
 if old["campana"]!=crate::examen::CAMPAIGN||old["referencia"]!=crate::examen::COMMIT||old["arranque"]!=crate::continuation::boot()?{return Err("CAMPANA_ORIGINAL_DISTINTA".into())}
 let mut v=old.clone();for k in ["plazo_global_utc_ms","plazo_global_mono_ms","plazo_carga_utc_ms"]{v[k]=Value::Null;}
 v["autorizacion_continuacion"]=json!(AUTH);v["terminacion"]=json!("P25_O_FALLO_TECNICO");v["segmento_continuacion"]=json!(crate::consulta_recovery::RUN);v["campana_original_sha256"]=json!(ORIGINAL);v["autorizacion_registrada_utc_ms"]=json!(store::now());
 v["criterio"]=json!("Continuar hasta P25 sin vencimiento horario; conservar cotas de memoria, contexto, herramientas, progreso, aislamiento y custodia.");Ok(v)
}
pub fn unbounded(v:&Value)->Result<bool>{
 if v.get("autorizacion_continuacion").is_none(){return Ok(false)}
 if v["autorizacion_continuacion"]!=AUTH||v["terminacion"]!="P25_O_FALLO_TECNICO"||v["segmento_continuacion"]!=crate::consulta_recovery::RUN||v["campana_original_sha256"]!=ORIGINAL||v["campana"]!=crate::examen::CAMPAIGN||v["referencia"]!=crate::examen::COMMIT||v["arranque"]!=crate::continuation::boot()?||["plazo_global_utc_ms","plazo_global_mono_ms","plazo_carga_utc_ms"].iter().any(|k|v.get(*k)!=Some(&Value::Null)){return Err("CONTINUACION_SIN_PLAZO_NO_ACREDITADA".into())}Ok(true)
}
pub fn window(v:&Value,reserve:u64)->Result<Option<u64>>{if unbounded(v)?{Ok(None)}else{Ok(Some(crate::continuation::remaining(v,reserve)?))}}
pub fn request(client:reqwest::blocking::RequestBuilder,end:Option<Instant>)->Result<reqwest::blocking::RequestBuilder>{match end{None=>Ok(client),Some(d)=>Ok(client.timeout(d.checked_duration_since(Instant::now()).ok_or("PLAZO_CONSULTA")?))}}
pub fn mcp_ms(end:Option<Instant>)->Result<u64>{Ok(match end{None=>30000,Some(d)=>d.checked_duration_since(Instant::now()).ok_or("PLAZO")?.as_millis().min(30000)as u64})}
pub fn driver_expired(unbounded:bool,elapsed:Duration)->bool{!unbounded&&elapsed>Duration::from_secs(43110)}
#[cfg(test)]mod tests{use super::*;
 fn old()->Value{json!({"campana":crate::examen::CAMPAIGN,"referencia":crate::examen::COMMIT,"arranque":crate::continuation::boot().unwrap(),"plazo_global_utc_ms":1,"plazo_global_mono_ms":1,"plazo_carga_utc_ms":1})}
 #[test]fn ampliacion_explicita_y_original_vencido(){let o=old();assert!(window(&o,90000).is_err());let c=campaign(&o).unwrap();assert_eq!(window(&c,90000).unwrap(),None);assert!(!driver_expired(true,Duration::from_secs(90000)));assert!(driver_expired(false,Duration::from_secs(90000)));assert_eq!(o["plazo_global_utc_ms"],1);}
 #[test]fn identidad_y_alcance_se_mantienen(){let c=campaign(&old()).unwrap();for k in ["campana","referencia","arranque","autorizacion_continuacion","terminacion","segmento_continuacion","campana_original_sha256"]{let mut bad=c.clone();bad[k]=json!("otro");assert!(unbounded(&bad).is_err(),"{k}")}let mut bad=c.clone();bad["plazo_global_mono_ms"]=json!(1);assert!(unbounded(&bad).is_err());}
 #[test]fn mcp_y_control_conservan_cotas(){assert_eq!(mcp_ms(None).unwrap(),30000);assert!(mcp_ms(Some(Instant::now()-Duration::from_secs(1))).is_err());assert_eq!(crate::continuation::NO_PROGRESS_MS,900000);assert_eq!(crate::control::WATCHDOG_MS,3000);}
}

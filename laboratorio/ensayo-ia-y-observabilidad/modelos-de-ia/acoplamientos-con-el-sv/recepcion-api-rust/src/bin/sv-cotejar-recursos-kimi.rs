//! Comprobación de capacidad antes de iniciar otro banco; no hace red ni cambia recursos.
#![forbid(unsafe_code)]
use std::{fs,path::Path};
use serde_json::{json,Value};
use sv_cliente_api::{need,parse,save,sha,R};
fn load(p:&Path)->R<Value>{sv_cliente_api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn quota(input:u64,max:u64)->R<u64>{input.checked_add(max).ok_or("Desbordamiento de cuota".into())}
fn run(manual:&Path,out:&Path)->R<Value>{
 let mut total=0u64;let mut rows=vec![];
 for root in [manual]{let audit=load(&root.join("AUDITORIA-MEDICION.json"))?;need(audit["conforme"]==true,"Auditoría ausente")?;
  for row in audit["casos"].as_array().ok_or("Casos")?{let dir=root.join(row["directorio"].as_str().ok_or("Directorio")?);let q=load(&dir.join("SOLICITUD.json"))?;need(q["model"]=="kimi-k3","Modelo distinto")?;let input=row["uso"]["input_tokens"].as_u64().ok_or("Entrada desconocida")?;let max=q["max_completion_tokens"].as_u64().ok_or("Límite ausente")?;let requested=quota(input,max)?;total=total.checked_add(requested).ok_or("Desbordamiento")?;rows.push(json!({"caso":row["caso"],"etapa":row["etapa"],"entrada_comunicada":input,"salida_maxima_solicitada":max,"consumo_para_cuota_segun_documentacion":requested,"solicitud_sha256":sha(&fs::read(dir.join("SOLICITUD.json")).map_err(|e|e.to_string())?)}));}
 }
 need(rows.len()==27,"Catálogo incompleto")?;
 let limit=1_500_000u64;let remaining=limit.saturating_sub(total);let minimum_future=48u64*16384;let enough=remaining>=minimum_future;
 let v=json!({"fecha_utc_ms":sv_instrumentacion::utc_ms(),"conforme":true,"alcance":"Previsión conservadora con los límites visibles y la regla publicada; no es un contador remoto de cuota restante ni acredita su hora de renovación","fuente_regla":"https://platform.kimi.ai/docs/introduction","fuente_limite":"https://platform.kimi.ai/console/limits","limite_diario_observado":limit,"solicitudes_previas":rows,"tokens_para_cuota_del_catalogo":total,"disponibilidad_condicional_en_misma_ventana":remaining,"cyb16":{"preguntas":16,"etapas":3,"solicitudes":48,"max_completion_tokens":16384,"reserva_minima_solo_salidas":minimum_future,"entradas_adicionales_no_incluidas":true,"cabe_en_misma_ventana_incluso_sin_contar_entradas":enough},"renovacion_de_cuota_confirmada":false,"estado_si_no_cabe":"Examen preparado, pendiente de disponibilidad acreditada; no iniciado, sin recalificar al candidato","autorizacion_economica_ampliada":false,"inferencias_nuevas":0,"licencia":sv_cliente_api::LICENCIA});save(out,&v)?;Ok(v)
}
fn main(){let r=(||->R<Value>{let a=std::env::args().collect::<Vec<_>>();need(a.len()==3,"MANUAL SALIDA")?;run(Path::new(&a[1]),Path::new(&a[2]))})();match r{Ok(v)=>println!("Cuota calculada: {}; CYB16 admisible en misma ventana: {}",v["tokens_para_cuota_del_catalogo"],v["cyb16"]["cabe_en_misma_ventana_incluso_sin_contar_entradas"]),Err(e)=>{eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}}
#[cfg(test)]mod tests{use super::*;#[test]fn reserva_y_facturacion_no_son_lo_mismo(){assert_eq!(quota(20_000,16_384).unwrap(),36_384);assert!(quota(u64::MAX,1).is_err());assert!(1_500_000u64.saturating_sub(1_000_000)<48*16_384);}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

use std::{fs,path::Path};
use serde_json::{Value,json};
fn main()->Result<(),Box<dyn std::error::Error>> {
 let a:Vec<_>=std::env::args().collect();let d=Path::new(&a[1]);let mut rows=Vec::new();let mut peak=0u64;
 for attempt in [22,23,24,25] {
  let mut max=0;let mut count=0;let mut floor=u64::MAX;
  for line in fs::read_to_string(d.join(format!("intento-{attempt}/SUCESOS.jsonl")))?.lines() {
   let v:Value=serde_json::from_str(line)?;
   if v["kind"]=="muestra_recursos" {
    if let Some(n)=v["data"]["non_file_rss_bytes"].as_u64(){max=max.max(n);count+=1;}
    if let Some(n)=v["data"]["capacity"]["effective_available_bytes"].as_u64(){floor=floor.min(n);}
   }
  }
  if count==0{return Err("registro_sin_muestras".into())} peak=peak.max(max);
  rows.push(json!({"intento":attempt,"muestras_persistidas":count,"max_no_file_bytes":max,"min_disponible_bytes":floor}));
 }
 let mib=1024*1024;let minimum=peak.div_ceil(mib)*mib+384*mib;let reserve=512*mib;
 let cfg=eio_controlador_oss::Config::new(d.into(),d.into(),String::new());
 assert_eq!(cfg.minimum_memory,minimum);
 assert_eq!(eio_controlador_oss::resources::admit(&json!({"effective_available_bytes":minimum+reserve}),minimum,reserve)?,minimum);
 assert!(eio_controlador_oss::resources::admit(&json!({"effective_available_bytes":minimum+reserve-1}),minimum,reserve).is_err());
 println!("{}",serde_json::to_string_pretty(&json!({
  "schema":"SV-AUDITORIA-MEMORIA-32","muestras":rows,"max_no_file_bytes":peak,
  "minimo_anterior_bytes":13421772800u64,"minimo_revisado_bytes":minimum,"reserva_entorno_bytes":reserve,
  "umbral_total_bytes":minimum+reserve,"margen_sobre_maximo_bytes":minimum-peak,
  "entrada_tokens_observada_intento_24":60,"salida_maxima_tokens":16,
  "incremento_directo_gate_up_f32_y_bf16_bytes":60u64*3*5760*6,
  "criterio":"Margen experimental de ingeniería de 384 MiB sobre el máximo persistido redondeado al MiB; no cota demostrada de todas las asignaciones.",
  "limites":"Máximos muestreados; motor previo al parche; no prueban pico exacto ni suficiencia de la candidata. Se conservan reserva global, vigilancia de RSS no file, límite virtual y tiempos."
 }))?);Ok(())
}

//! Resumen de tres pares; calentamiento separado. No ejecutado.
use std::{fs,path::Path};
fn resumen(mut v:Vec<f64>)->serde_json::Value {
 v.sort_by(f64::total_cmp);
 serde_json::json!({"individuales_ordenados":v,"mediana":v[1],"minimo":v[0],"maximo":v[2],"rango":v[2]-v[0]})
}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let raiz=std::env::args().nth(1).ok_or("falta directorio evidencia")?;
 let mut on=Vec::new();let mut off=Vec::new();let mut diferencias=Vec::new();
 let mut fila=Vec::new();let mut anterior=0.0;
 for (i,modo) in ["on","on","off","off","on","on","off"].iter().enumerate(){
  let prefijo=format!("inferencia-{}-{modo}",i+1);
  let r:serde_json::Value=serde_json::from_str(&fs::read_to_string(Path::new(&raiz).join(format!("{prefijo}.stdout")))?)?;
  if r["tipo"]!="modelo_real" || r["telemetria"]!=*modo {return Err("evidencia distinta".into());}
  let s=r["segundos_intervalo_completo"].as_f64().ok_or("tiempo ausente")?;
  if !s.is_finite() || s<0.0 {return Err("tiempo no admisible".into());}
  let medidas=fs::read_to_string(Path::new(&raiz).join(format!("{prefijo}.medidas.tsv")))?;
  let mut pico=None;
  for l in medidas.lines(){let x:u64=l.split('\t').nth(1).ok_or("RSS ausente")?.parse()?;
   pico=Some(pico.unwrap_or(0).max(x));}
  fila.push(serde_json::json!({"numero":i+1,"modo":modo,"calentamiento":i==0,
   "segundos":s,"segundos_inferencia":r["salida"]["segundos"],"pico_rss_kib":pico,"tokens":r["salida"]["tokens"],"contrato":r["contrato"]}));
  if i>0 {if *modo=="on"{on.push(s)}else{off.push(s)}
   if i%2==1 {anterior=s;}else{diferencias.push(if i==4{s-anterior}else{anterior-s});}}
 }
 let tokens=&fila[1]["tokens"];
 let equivalentes=fila[1..].iter().all(|r|&r["tokens"]==tokens);
 println!("{}",serde_json::json!({"tipo":"coste_observacion","medidas":fila,"on":resumen(on),
 "off":resumen(off),"diferencias_pareadas_on_menos_off":resumen(diferencias),
 "tokens_equivalentes":equivalentes,"limite":"Intervalo desde antes de crear telemetría hasta después de cerrar/exportar, incluye lectura/carga e inferencia en ambos modos; excluye arranque del proceso y serialización de evidencia. Primera llamada separada sólo puede calentar cachés del SO; no el estado del proceso siguiente. RSS muestreado; null no acredita medida."}));
 if !equivalentes{return Err("salidas no equivalentes; conservar medidas sin atribucion causal".into());}
 Ok(())
}

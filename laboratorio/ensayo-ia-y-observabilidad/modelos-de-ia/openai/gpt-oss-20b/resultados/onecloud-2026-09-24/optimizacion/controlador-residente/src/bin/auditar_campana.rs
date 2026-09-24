use serde_json::{json,Value};
use std::{fs,path::Path};
fn leer(p:impl AsRef<Path>)->Result<Value,Box<dyn std::error::Error>>{Ok(serde_json::from_str(&fs::read_to_string(p)?)?)}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let raiz=std::env::args().nth(1).ok_or("Falta directorio de evidencias")?;
 let mut controles=Vec::new();
 let mut series=Vec::new();
 for (nombre,numero,modo) in [("REFERENCIA",3,"referencia"),("PARALELO",7,"paralelo")] {
  let resumen=leer(format!("{raiz}/{nombre}_RESUMEN.json"))?;
  let registros:Vec<Value>=fs::read_to_string(format!("{raiz}/{nombre}/SUCESOS.jsonl"))?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
  let eventos=|tipo:&str|registros.iter().filter(|r|r["kind"]==tipo).collect::<Vec<_>>();
  let mut comprobar=|nombre_control:&str,correcto:bool|controles.push(json!({"sesion":nombre,"control":nombre_control,"correcto":correcto}));
  comprobar("Un proceso de inferencia",eventos("proceso_creado").len()==1);
  comprobar("Una carga comprobada",eventos("servicio_comprobado").len()==1);
  comprobar("Número de peticiones",eventos("peticion_prevista").len()==numero);
  comprobar("Número de respuestas",eventos("respuesta_original").len()==numero);
  comprobar("Número de finales de caso",eventos("fin_caso").len()==numero);
  comprobar("Modo y trabajadores",eventos("sesion_residente").iter().any(|r|r["data"]["modo_mxfp4"]==modo&&r["data"]["trabajadores_rayon"]=="12"&&r["data"]["trabajadores_candle"]=="12"));
  comprobar("HTTP 200 íntegro",eventos("http_respuesta").iter().all(|r|r["data"]["raw"].as_str().unwrap_or("").lines().next().unwrap_or("").split_whitespace().nth(1)==Some("200")));
  comprobar("Temperatura cero y límites de generación",eventos("peticion_prevista").iter().enumerate().all(|(i,r)|r["data"]["temperature"]==0&&r["data"]["max_tokens"]==if i<3 {16}else{32}));
  comprobar("Orden temporal no decreciente",registros.windows(2).all(|r|r[0]["elapsed_ms"].as_u64()<=r[1]["elapsed_ms"].as_u64()));
  let fin=&resumen["finalizacion"]["data"];
  comprobar("Cierre confirmado sin incidencias",fin["child_stop_confirmed"]==true&&fin["error"].is_null()&&fin["stop_errors"].as_array().is_some_and(|r|r.is_empty()));
  comprobar("Sin muestras de RSS ausentes",fin["rss_unavailable"]==0);
  comprobar("Sin truncamiento por límite de tokens",resumen["casos"].as_array().unwrap().iter().all(|r|r["respuesta"]["choices"][0]["finish_reason"]=="stop"));
  let solicitudes:Vec<_>=eventos("peticion_prevista").iter().map(|v|v["data"].clone()).collect();
  series.push((resumen,solicitudes));
 }
 let r=&series[0].0;let p=&series[1].0;
 controles.push(json!({"control":"Mismo motor y controlador","correcto":r["inicio"]["data"]["engine_sha256"]==p["inicio"]["data"]["engine_sha256"]&&r["inicio"]["data"]["controller_binary_sha256"]==p["inicio"]["data"]["controller_binary_sha256"]}));
 controles.push(json!({"control":"Tres peticiones comparables idénticas","correcto":series[0].1==series[1].1[..3]}));
 let rm=r["mediana_referencia_ms"].as_f64().ok_or("Mediana de referencia ausente")?;
 let pm=p["mediana_referencia_ms"].as_f64().ok_or("Mediana candidata ausente")?;
 let resultado=json!({"alcance":"Auditoría instrumental de originales; el juicio semántico se registra por separado.","controles":controles,"controles_aprobados":controles.iter().all(|c|c["correcto"]==true),"mediana_referencia_ms":rm,"mediana_paralela_ms":pm,"cociente_latencias":pm/rm,"factor_aceleracion":rm/pm,"reduccion_porcentual":100.0*(1.0-pm/rm),"umbral_rendimiento_cumplido":pm<=0.5*rm});
 println!("{}",serde_json::to_string_pretty(&resultado)?);
 if resultado["controles_aprobados"]!=true {std::process::exit(2)}
 Ok(())
}

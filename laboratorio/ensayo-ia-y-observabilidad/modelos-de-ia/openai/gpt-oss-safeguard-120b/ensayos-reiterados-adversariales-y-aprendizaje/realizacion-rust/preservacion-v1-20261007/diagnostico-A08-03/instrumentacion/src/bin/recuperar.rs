use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};
use sv_arbitro_comprobaciones::{huella,auditoria::auditar};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let original=Path::new("/opt/sv-safeguard/evidencias/diag-A08-03");
 let audit=auditar(original,false,false)?;
 if audit["conforme"]!=true||audit["emisiones"]!=1||audit["recorrido_completo"]!=true{return Err("Cierre incompleto: conservar incidente sin adjudicación automática".into())}
 let raw=fs::read(original.join("modelo.stdout"))?;
 let events=std::str::from_utf8(&raw)?.lines().filter_map(|l|l.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;
 let finales=events.iter().filter(|v|v["datos"]["evento"]=="respuesta_final").collect::<Vec<_>>();
 let generaciones=events.iter().filter(|v|v["datos"]["evento"]=="configuracion_generacion_solicitada").collect::<Vec<_>>();
 if finales.len()!=1||finales[0]["datos"]["id"]!="A08"||generaciones.len()!=1{return Err("Número de emisiones o configuración discordante".into())}
 let sampling=&generaciones[0]["datos"]["sampling"];
 if sampling["temperature"]!=1.0||sampling["top_p"]!=1.0||!sampling["top_k"].is_null()||!sampling["min_p"].is_null()||sampling["max_len"]!=4096||generaciones[0]["datos"]["semilla"]!=42||generaciones[0]["datos"]["razonamiento"]!="high"||generaciones[0]["datos"]["intento"]!=3{return Err("Configuración solicitada discordante".into())}
 let result=json!({"intento":3,"caso":"A08","cotejo":audit,"modelo_stdout_sha256":huella(&raw),"respuesta_original":finales[0],"configuracion_solicitada":generaciones[0],"adjudicacion_pendiente":true,"clave_entregada":false,"examen_habilitado":false});
 let destino="/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-03/cotejos/RECUPERACION-ORIGINAL.json";
 let mut f=fs::OpenOptions::new().write(true).create_new(true).open(destino)?;f.write_all(&serde_json::to_vec_pretty(&result)?)?;f.sync_all()?;
 println!("Un original íntegro recuperado y cotejado. Pendiente adjudicación independiente.");Ok(())
}

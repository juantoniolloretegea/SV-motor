use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main()->R<()>{
 let arg=std::env::args().nth(1).ok_or("Directorio requerido")?;
 let b=Path::new(&arg);let d=b.join("diagnostico-A08-03");
 let mut verificados=Vec::new();
 for (nombre,tamano,sha) in [
  ("RECUPERACION-ORIGINAL.json",3310,"f65a0ad1fdd5970927e14cde61b520188b4dad8df29510883847781607808020"),
  ("FINAL-D03.json",794,"47652bc437a8d8f68e516104fededd9ee3206081bc3c05897f354fc63a909b50"),
  ("CIERRE.json",487,"8661b94d9fa572b872d6232b0dfd4ae81801b8c8c8a57901f2bd0eb56eddc1f6")
 ]{
  let raw=fs::read(d.join("cotejos").join(nombre))?;
  ck(raw.len()==tamano&&h(&raw)==sha,"Recuperación distinta de las huellas remotas calculadas en Rust")?;
  verificados.push(json!({"archivo":nombre,"bytes":tamano,"sha256":sha,"conforme":true}));
 }
 let raw=fs::read(d.join("cotejos/RECUPERACION-ORIGINAL.json"))?;
 let r:Value=serde_json::from_slice(&raw)?;
 let entrada:Value=serde_json::from_slice(&fs::read(d.join("cotejos/FINAL-D03.json"))?)?;
 let cierre:Value=serde_json::from_slice(&fs::read(d.join("cotejos/CIERRE.json"))?)?;
 ck(r["intento"]==3&&r["caso"]=="A08"&&r["clave_entregada"]==false,"Identidad discordante")?;
 ck(r["cotejo"]["conforme"]==true&&r["cotejo"]["recorrido_completo"]==true&&r["cotejo"]["emisiones"]==1&&r["cotejo"]["emision_parcial"]==0,"Custodia incompleta")?;
 ck(entrada["conforme"]==true&&entrada["emisiones_decodificadas"]==1&&entrada["modelo_stdout_sha256"]==r["modelo_stdout_sha256"]&&r["modelo_stdout_sha256"]=="00d24dbd8053d7744fe8caa3a0bcc7bd4dd4ececb628471d8ed3fc803428147d","Entrada y salida no vinculadas")?;
 ck(entrada["detalle"][0]["entrada_tokens"]==2774&&entrada["detalle"][0]["tokens_sha256"]=="09cb30867e6d957640877a4aa15627fd6ca156304171c180daad8959e76d0208","Entrada diferente")?;
 ck(cierre["codigo_modelo"]==0&&cierre["codigo_mcp"]==0&&cierre["fin_conductor"]==true&&cierre["error"].is_null(),"Cierre no normal")?;
 let s=&r["configuracion_solicitada"]["datos"];
 ck(s["intento"]==3&&s["razonamiento"]=="high"&&s["semilla"]==42&&s["sampling"]["temperature"]==1.0&&s["sampling"]["top_p"]==1.0&&s["sampling"]["top_k"].is_null()&&s["sampling"]["max_len"]==4096,"Configuración distinta")?;
 let key:Value=serde_json::from_slice(&fs::read(b.join("reservado/CLAVE.json"))?)?;
 let k=key["casos"].as_array().ok_or("Clave")?.iter().find(|c|c["id"]=="A08").ok_or("Caso reservado")?;
 let original=r["respuesta_original"]["datos"]["contenido"].as_str().ok_or("Final")?;
 let v:Value=serde_json::from_str(original)?;
 let cat:Value=serde_json::from_slice(&fs::read(d.join("cache/catalogo.json"))?)?;
 let fuente=cat["documents"][0]["sections"].as_array().ok_or("Secciones")?.iter().find(|s|s["id"]=="A08").and_then(|s|s["text"].as_str()).ok_or("Fuente")?;
 let mut citas=Vec::new();
 for c in v["evidencias"].as_array().ok_or("Evidencias")?{
  let t=c["fragmento"].as_str().ok_or("Cita")?;let p=c["pagina"].as_u64().ok_or("Página")?;
  ck(c["documento"]=="BANCO-A"&&c["seccion"]=="A08"&&p<2&&!t.is_empty()&&fuente.chars().skip(p as usize*2000).take(2000).collect::<String>().contains(t),"Cita o localización discordante")?;
  citas.push(json!({"pagina":p,"literal":true,"sha256":h(t.as_bytes())}));
 }
 ck(citas.len()==2&&v["justificacion_breve"].as_str().is_some_and(|s|!s.is_empty()),"Fundamentación no examinada")?;
 ck(v.get("recepcion_documental").is_none()&&v.get("revision").is_none(),"Campos no examinados: requieren evaluación específica")?;
 let correcta=v["decision"]==k["esperada"];let critico=!correcta&&k["critico"]==true;
 let tokens=r["respuesta_original"]["datos"]["salida_total_tokens"].as_u64().ok_or("Tokens")?;
 ck(tokens==2057&&tokens<4096&&r["cotejo"]["calculos_finalizados"]==tokens,"Salida distinta o truncada")?;
 let d01:Value=serde_json::from_slice(&fs::read(b.join("diagnostico-A08-01/RESULTADO-D01.json"))?)?;
 let informe=json!({
  "intento":3,"maximo_total_intentos":3,"caso":"A08","referencia":"D02: misma entrada y generación; ampliación exclusiva de la cota temporal",
  "archivos_recuperados_cotejados_rust":verificados,"original_final":original,"original_sha256":h(original.as_bytes()),"modelo_stdout_sha256":r["modelo_stdout_sha256"],
  "configuracion":s,"decision_emitida":v["decision"],"decision_prefijada":k["esperada"],"clasificacion_correcta":correcta,"error_critico":critico,
  "citas":citas,"omisiones_formales":["recepcion_documental","revision"],"tokens_salida":tokens,"cierre_normal":true,"truncamiento":false,
  "fin_respuesta_unix_us":r["respuesta_original"]["unix_us"],"fin_cierre_unix_us":cierre["fin_unix_us"],
  "correccion_clasificacion_respecto_D01":correcta&&d01["clasificacion_correcta"]==false,
  "misma_clasificacion_que_D01":v["decision"]==d01["decision_emitida"],
  "comparacion_D02":"Interrupción por tiempo sin respuesta final; no existe clasificación comparable",
  "valor_del_caso_segun_rubrica":if correcta{"requiere valoración formal"}else{"1"},"celula_sv_constituida":false,"puntuacion_de_banco":null,
  "solventado":correcta,"examen_habilitado":false,"nuevas_inferencias_autorizadas":false,
  "dictamen_acceso_examen":if correcta{"Requiere recepción posterior; no habilitado por este caso"}else{"No apto para el acceso al examen en esta evaluación"},
  "conclusion":"Con cierre normal y margen de salida, D03 conserva el error A08 y las omisiones de recepción y revisión. El reconocimiento textual de la discrepancia y ausencia de precedencia no se traduce en la categoría prescrita. Se agota el máximo de tres intentos; no se atribuye incapacidad general ni equivalencia numérica independiente del motor.",
  "reservas":["Presentación JSON no uniforme entre instrucción inicial y requisitos posteriores","Distinción semántica del enunciado A08","Autocorrección contextual sin entrenamiento de pesos","Una realización por condición no estima una tasa de éxito","Qwen3.8-Max mostró corrección con una conversación y presentación distintas; no identifica por sí solo la causa del fallo de Safeguard"],
  "conservacion_final_github":"Pendiente de consolidación al terminar las pruebas, por instrucción humana"
 });
 let mut f=fs::OpenOptions::new().create_new(true).write(true).open(d.join("RESULTADO-D03.json"))?;
 f.write_all(&serde_json::to_vec_pretty(&informe)?)?;f.sync_all()?;
 println!("D03: tres archivos recuperados y cotejados en Rust; 2057 tokens, cierre normal. Clasificación correcta: {correcta}; error crítico: {critico}. Dos omisiones formales. Sin nuevos intentos ni examen habilitado.");Ok(())
}

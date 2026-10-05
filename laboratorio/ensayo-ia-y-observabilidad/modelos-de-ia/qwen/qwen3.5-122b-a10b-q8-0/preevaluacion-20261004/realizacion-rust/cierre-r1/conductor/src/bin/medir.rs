// Preparación de una medición posterior. No envía peticiones al servicio.
#[path = "../tokenizacion.rs"]
mod tokenizacion;
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs,io::Write,path::Path};
type E=Box<dyn std::error::Error+Send+Sync>;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,v:&Value)->Result<(),E>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(&serde_json::to_vec_pretty(v)?)?;f.sync_all()?;Ok(())}
fn read(p:&Path)->Result<Value,E>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
const REVIEW:&str="Lea otra vez toda la documentación suministrada. Reconsidere la pregunta junto con sus respuestas anteriores y las revisiones que efectivamente emitió. Trate sus conclusiones anteriores como hipótesis, no como fuentes. Formule objeciones documentales pertinentes, contrástelas con todos los pasajes y con la política, y entregue una respuesta final completa. Explique de manera verificable por qué mantiene o cambia la decisión. No presuponga que antes acertó ni que necesariamente deba cambiar. No añada hechos recordados ni utilice búsquedas selectivas en sustitución de la lectura del conjunto suministrado.";
fn main()->Result<(),E>{
 let a:Vec<_>=std::env::args().collect();let root=Path::new(a.get(1).ok_or("Directorio del original cerrado")?);let output=Path::new(a.get(2).ok_or("Informe nuevo")?);
 let result=read(&root.join("RESULTADO.json"))?;if result["completa"]!=true{return Err("No hay resultado completo para medir este escenario".into())}
 let token_file=root.join("TOKENIZADOR-EFECTIVO.json");let token_bytes=fs::read(&token_file)?;let provenance=read(&root.join("TOKENIZADOR-PROCEDENCIA.json"))?;if provenance["sha256"]!=hash(&token_bytes){return Err("Tokenizador alterado".into())}
 let tokenizer=tokenizers::Tokenizer::from_file(token_file)?;
 let rendered=fs::read_to_string(root.join("ENTRADA-RENDERIZADA.txt"))?;let input_ids=tokenizer.encode_fast(rendered,true)?.get_ids().to_vec();let sealed_ids=read(&root.join("ENTRADA-TOKENS.json"))?;if json!(input_ids)!=sealed_ids{return Err("Recodificación de entrada discordante".into())}
 let final_text=fs::read_to_string(root.join("FINAL.txt"))?;let thought=fs::read_to_string(root.join("RAZONAMIENTO-EMITIDO.txt"))?;
 let final_ids=tokenizer.encode_fast(final_text.clone(),false)?.get_ids().to_vec();let thought_ids=tokenizer.encode_fast(thought.clone(),false)?.get_ids().to_vec();
 let preparation=read(&root.join("PREPARACION.json"))?;let request=read(&root.join("SOLICITUD.json"))?;
 let template=fs::read_to_string(root.join("PLANTILLA.jinja"))?;let mut messages=request["messages"].as_array().ok_or("Mensajes")?.clone();
 let case=preparation["caso"].as_str().ok_or("Caso")?;if case!="A01"||preparation["capa"]!=0{return Err("Este escenario inicial se limita a A01/A0".into())}let seconds=result["segundos"].as_f64().ok_or("Duración")?;
 let prompt_time=result["usage"]["total_prompt_time_sec"].as_f64().ok_or("Tiempo nativo de entrada")?;
 let completion_time=result["usage"]["total_completion_time_sec"].as_f64().ok_or("Tiempo nativo de generación")?;
 if prompt_time<=0.0||completion_time<=0.0||input_ids.is_empty(){return Err("Medición nativa temporal incompleta".into())}
 let mut stages=Vec::new();let mut cumulative=0.0;let mut cumulative_double=0.0;
 for layer in 0..=3u64 {
  let text=tokenizacion::render(&template,&json!(messages),true)?;let tokens=tokenizer.encode_fast(text,true)?.len();
  let estimated_prompt=prompt_time*tokens as f64/input_ids.len() as f64;let same=estimated_prompt+completion_time;let double=estimated_prompt+completion_time*if layer==0{1.0}else{2.0};cumulative+=18.0*same;cumulative_double+=18.0*double;
  stages.push(json!({"capa":layer,"entrada_tokens_escenario":tokens,"antecedentes_representativos":layer,"entrada_mas_reservas":tokens+4096+2048,"cabe_en_contexto":tokens+4096+2048<=32768,"segundos_por_solicitud_con_salida_constante":same,"segundos_por_solicitud_con_salida_doble_en_revision":double}));
  if layer<3 {messages.push(json!({"role":"assistant","content":final_text}));messages.push(json!({"role":"user","content":format!("Antecedente {case}, capa {layer}, incorporado íntegramente. Revisión {}. {REVIEW}",layer+1)}));}
 }
 let result_report=json!({"encargo":"QWEN35-PRE-20261004/r1","caso":case,"capa":preparation["capa"],"resultado_sha256":hash(&fs::read(root.join("RESULTADO.json"))?),"tokenizador_sha256":hash(&token_bytes),"entrada_reproducida_identica":true,"tokens_entrada":input_ids.len(),"recuento_nativo_total_salida":result["usage"]["completion_tokens"],"tokens_recodificados_razonamiento":thought_ids.len(),"tokens_recodificados_final":final_ids.len(),"identificadores_recodificados_razonamiento":thought_ids,"identificadores_recodificados_final":final_ids,"limite_de_observacion":"Los identificadores por canal son una recodificación del texto emitido con el tokenizador fijado, no identificadores generados expuestos por la API. La API declara el total nativo y los canales de texto, pero no desglosa identificadores ni recuentos nativos por canal. No se equipara sin prueba recodificación y secuencia generada.","tiempos_observados":{"solicitud_segundos":seconds,"entrada_nativa_segundos":prompt_time,"generacion_nativa_segundos":completion_time,"primera_emision_segundos":result["primer_token_observable_segundos"],"primer_contenido_final_segundos":result["primer_contenido_final_segundos"]},"escenarios":{"hipotesis":"Muestra única A01. Entrada proporcional a su longitud; mismas fuentes, respuesta inicial y duración de generación como patrón de tamaño. Las respuestas futuras no se conocen. Las capas simuladas repiten el final de A01 sólo para medir tamaños; no se envían al candidato ni se conservan como antecedentes reales.","capas":stages,"dieciocho_solicitudes_iniciales_segundos":18.0*seconds,"setenta_y_dos_salida_constante_segundos":cumulative,"setenta_y_dos_salida_doble_en_revisiones_segundos":cumulative_double,"cota_global_segundos":86400,"reservar_antes_de_cada_admision_segundos":18000,"advertencia":"Son escenarios descriptivos de una muestra, no pronósticos validados ni permisos para superar las cotas. El coste de preparación y la duración de campaña se registran por separado."}});
 let mut signed=result_report;
 signed["licencia"]=json!("© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).");
 save(output,&signed)?;println!("{}",json!({"recodificacion_conforme":true,"sin_inferencia":true,"salida":output}));Ok(())
}

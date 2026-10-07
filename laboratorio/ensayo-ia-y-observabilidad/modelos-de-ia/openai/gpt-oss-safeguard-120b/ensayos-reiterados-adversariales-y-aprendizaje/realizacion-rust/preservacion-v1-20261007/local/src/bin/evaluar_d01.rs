use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main()->R<()>{
 let root=std::env::args().nth(1).ok_or("Directorio requerido")?;let b=Path::new(&root);let d=b.join("diagnostico-A08-01");
 let raw=fs::read(d.join("cotejos/RECUPERACION-ORIGINAL.json"))?;let final_raw=fs::read(d.join("cotejos/FINAL-D01.json"))?;
 ck(raw.len()==3353&&h(&raw)=="f8e93dbfa3bb9ca9cfbc3fec8642beae89cc7406b66193cacf0afe7e4804bf0d","Recuperación distinta de la huella remota calculada en Rust")?;
 ck(final_raw.len()==794&&h(&final_raw)=="9c5650f2c6bccd840d75fc3d4ae20e6adcfa7cf63c1841a574ab0bca51efb28b","Cotejo de entrada distinto del original remoto")?;
 let r:Value=serde_json::from_slice(&raw)?;let entrada:Value=serde_json::from_slice(&final_raw)?;
 ck(r["intento"]==1&&r["caso"]=="A08"&&r["cotejo"]["conforme"]==true&&r["cotejo"]["recorrido_completo"]==true&&r["cotejo"]["emisiones"]==1&&r["cotejo"]["emision_parcial"]==0,"Custodia incompleta")?;
 ck(entrada["conforme"]==true&&entrada["emisiones_decodificadas"]==1&&entrada["modelo_stdout_sha256"]==r["modelo_stdout_sha256"]&&r["modelo_stdout_sha256"]=="efffa46b23d2112327459eea360659c8f522b41e75acfae922751d55619faa5e","Entrada y salida no vinculadas")?;
 let s=&r["configuracion_solicitada"]["datos"];ck(s["razonamiento"]=="medium"&&s["semilla"]==42&&s["sampling"]["temperature"]==1.0&&s["sampling"]["top_p"]==1.0&&s["sampling"]["top_k"].is_null(),"Configuración distinta")?;
 let key:Value=serde_json::from_slice(&fs::read(b.join("reservado/CLAVE.json"))?)?;let k=key["casos"].as_array().ok_or("Clave")?.iter().find(|c|c["id"]=="A08").ok_or("Caso reservado")?;
 let original=r["respuesta_original"]["datos"]["contenido"].as_str().ok_or("Final")?;let v:Value=serde_json::from_str(original)?;
 let cat:Value=serde_json::from_slice(&fs::read(d.join("cache/catalogo.json"))?)?;let fuente=cat["documents"][0]["sections"].as_array().ok_or("Secciones")?.iter().find(|s|s["id"]=="A08").and_then(|s|s["text"].as_str()).ok_or("Fuente")?;
 let mut citas=Vec::new();for c in v["evidencias"].as_array().ok_or("Evidencias")?{let t=c["fragmento"].as_str().ok_or("Cita")?;let p=c["pagina"].as_u64().ok_or("Página")?;ck(c["documento"]=="BANCO-A"&&c["seccion"]=="A08"&&p<2&&!t.is_empty()&&fuente.chars().skip(p as usize*2000).take(2000).collect::<String>().contains(t),"Cita o localización no conforme")?;citas.push(json!({"pagina":p,"literal":true,"sha256":h(t.as_bytes())}));}
 ck(citas.len()==2&&v["justificacion_breve"].as_str().is_some_and(|s|!s.is_empty()),"Fundamentación no examinada")?;
 ck(v.get("recepcion_documental").is_none()&&v.get("revision").is_none(),"Campos nuevos presentes: requieren evaluación específica")?;
 let correcta=v["decision"]==k["esperada"];let critical=!correcta&&k["critico"]==true;let tokens=r["respuesta_original"]["datos"]["salida_total_tokens"].as_u64().ok_or("Tokens")?;
 ck(tokens==905&&tokens<2048,"Salida distinta o truncada")?;
 let informe=json!({"intento":1,"maximo_total_intentos":3,"caso":"A08","referencia":"A2/A08","razonamiento":"medium","configuracion":s,"original_final":original,"original_sha256":h(original.as_bytes()),"recuperacion_sha256":h(&raw),"cotejo_entrada_sha256":h(&final_raw),"modelo_stdout_sha256":r["modelo_stdout_sha256"],"decision_emitida":v["decision"],"decision_prefijada":k["esperada"],"clasificacion_correcta":correcta,"error_critico":critical,"citas":citas,"omisiones_formales":["recepcion_documental","revision"],"tokens_salida":tokens,"cierre_normal":true,"truncamiento":false,"valor_del_caso_segun_rubrica":"1","celula_sv_constituida":false,"puntuacion_de_banco":null,"examen_habilitado":false,"solventado":false,"conclusion":"La selección probabilística configurada no corrige A08 en esta única realización. Se conservan el reconocimiento del conflicto y la falta de precedencia, junto con la etiqueta discordante con la política. No demuestra incapacidad general ni equivalencia numérica de todo el motor.","reservas":["Ejemplo JSON y requisitos posteriores no uniformes en la política","Distinción semántica del enunciado A08","Dos antecedentes propios conservados; no entrenamiento de pesos","Una realización no estima una tasa de éxito"],"conservacion_final_github":"Pospuesta por instrucción humana"});
 let mut f=fs::OpenOptions::new().create_new(true).write(true).open(d.join("RESULTADO-D01.json"))?;f.write_all(&serde_json::to_vec_pretty(&informe)?)?;f.sync_all()?;
 println!("D01: cotejo íntegro conforme; 905 tokens, sin truncamiento. Clasificación correcta: {correcta}; error crítico: {critical}. Dos omisiones formales. No solventado; examen no habilitado.");Ok(())
}

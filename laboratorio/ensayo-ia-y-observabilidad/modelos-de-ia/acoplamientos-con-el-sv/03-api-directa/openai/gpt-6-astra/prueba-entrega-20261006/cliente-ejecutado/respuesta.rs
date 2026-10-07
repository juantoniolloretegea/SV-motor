use serde_json::Value;
type R<T> = Result<T,String>;
// Contrato de esta prueba: un mensaje de texto, sin herramientas ni salida adicional.
pub fn text(events:&[Value], model:&str, expected:&str)->R<String> {
    for (i,e) in events.iter().enumerate() {if e["sequence_number"].as_u64()!=Some(i as u64){return Err("Secuencia de eventos discordante".into());}}
    let created=events.first().ok_or("Sin eventos")?;
    let terminal=events.last().unwrap();
    if created["type"]!="response.created" || terminal["type"]!="response.completed" || terminal["response"]["status"]!="completed" || terminal["response"]["model"]!=model || created["response"]["id"].as_str().is_none() || created["response"]["id"]!=terminal["response"]["id"] {return Err("Identidad o terminación discordantes".into());}
    let completed=events.iter().filter(|e|e["type"]=="response.output_item.done").collect::<Vec<_>>();
    if completed.len()!=1 || completed[0]["output_index"]!=0 {return Err("La prueba requiere un solo mensaje concluido".into());}
    let item=&completed[0]["item"];
    if item["role"]!="assistant"||item["type"]!="message"||item["status"]!="completed"||item["id"].as_str().is_none(){return Err("Mensaje no concluido".into());}
    let parts=item["content"].as_array().ok_or("Falta contenido")?;
    if parts.len()!=1||parts[0]["type"]!="output_text"{return Err("Contenido no previsto".into());}
    let full=parts[0]["text"].as_str().ok_or("Falta texto")?;
    let mut deltas=String::new(); let mut done=None;let mut part=None;
    for e in events {
        if matches!(e["type"].as_str(),Some("response.output_text.delta"|"response.output_text.done"|"response.content_part.done")) {
            if e["item_id"]!=item["id"] || e["output_index"]!=0 || e["content_index"]!=0 {return Err("Correlación de texto discordante".into());}
            match e["type"].as_str().unwrap() {
                "response.output_text.delta"=>{if done.is_some(){return Err("Fragmento posterior a su cierre".into());}deltas.push_str(e["delta"].as_str().ok_or("Fragmento no textual")?);},
                "response.output_text.done"=>{if done.replace(e["text"].as_str().ok_or("Texto ausente")?).is_some(){return Err("Cierre repetido".into());}},
                _=>{if part.replace(e["part"]["text"].as_str().ok_or("Parte ausente")?).is_some(){return Err("Parte repetida".into());}}
            }
        }
    }
    if deltas!=full || done!=Some(full)||part!=Some(full)||full!=expected {return Err("Texto y cierres discordantes".into());}
    if let Some(output)=terminal["response"]["output"].as_array() {if !output.is_empty() && (output.len()!=1 || output[0]!=*item){return Err("Salida terminal discordante".into());}}
    let u=&terminal["response"]["usage"];
    if u["input_tokens"].as_u64().zip(u["output_tokens"].as_u64()).and_then(|(a,b)|a.checked_add(b))!=u["total_tokens"].as_u64() || u["total_tokens"].is_null() {return Err("Uso ausente o incoherente".into());}
    Ok(full.to_owned())
}
#[cfg(test)]mod tests {
    use super::*;use serde_json::json;
    fn fixture()->Vec<Value>{include_str!("../../../gpt-6-astra/prueba-conexion-20261006/con-creditos/respuesta.sse").lines().filter_map(|s|s.strip_prefix("data:")).map(|s|serde_json::from_str(s.trim()).unwrap()).collect()}
    #[test]fn flujo_real_con_terminal_vacio(){let e=fixture();assert_eq!(text(&e,"gpt-6-astra","CONEXION ASTRA CONFIRMADA").unwrap(),"CONEXION ASTRA CONFIRMADA");}
    #[test]fn secuencia_incompleta_no_conforme(){let mut e=fixture();e.remove(4);assert!(text(&e,"gpt-6-astra","CONEXION ASTRA CONFIRMADA").is_err());}
    #[test]fn corrupcion_de_fragmento_rechazada(){let mut e=fixture();e.iter_mut().find(|v|v["type"]=="response.output_text.delta").unwrap()["delta"]=json!("discordante");assert!(text(&e,"gpt-6-astra","CONEXION ASTRA CONFIRMADA").is_err());}
    #[test]fn identidad_terminal_rechazada(){let mut e=fixture();e.last_mut().unwrap()["response"]["id"]=json!("otra");assert!(text(&e,"gpt-6-astra","CONEXION ASTRA CONFIRMADA").is_err());}
    #[test]fn uso_ausente_rechazado(){let mut e=fixture();e.last_mut().unwrap()["response"]["usage"]=json!(null);assert!(text(&e,"gpt-6-astra","CONEXION ASTRA CONFIRMADA").is_err());}
}

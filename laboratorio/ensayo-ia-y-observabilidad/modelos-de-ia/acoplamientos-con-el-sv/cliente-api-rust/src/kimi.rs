//! Adaptación acotada de K3 al transporte común, sin cliente de red independiente.
#![forbid(unsafe_code)]
use crate::{need, R, LICENCIA};
use serde_json::{json, Value};
pub const AVISO: &str = "Aviso de derechos: se mantienen la atribución y la licencia del material propio del SV; las fuentes de terceros conservan su régimen original. Para este ensayo documental se ha autorizado expresamente el tratamiento del contenido público por Kimi, incluido su uso para entrenamiento y mejora. Esta autorización específica no transfiere la titularidad ni extiende el permiso a otros documentos o campañas. No añada este aviso a la respuesta científica solicitada.";
pub fn solicitud(q: &Value) -> R<Value> {
    let mut common=q.clone();
    // Se reutiliza la transformación ya contrastada, sin transmitir su aviso general.
    let instructions=q["instructions"].as_str().ok_or("Instrucciones ausentes")?;
    need(instructions.contains(AVISO)&&instructions.contains(LICENCIA),"Autorización Kimi ausente")?;
    common["instructions"]=json!(instructions.replace(AVISO,crate::AVISO));
    let mut w=crate::chat::solicitud(&common,"glm-5.3")?;
    w["model"]=json!("kimi-k3");
    w["messages"][0]["content"]=json!(w["messages"][0]["content"].as_str().unwrap().replace(crate::AVISO,AVISO));
    let o=w.as_object_mut().unwrap();o.remove("thinking");let cap=o.remove("max_tokens").unwrap();o.insert("max_completion_tokens".into(),cap);
    w["stream_options"]=json!({"include_usage":true});
    w["tools"]=json!([]);w["tool_choice"]=json!("none");
    w["prompt_cache_options"]=json!({"mode":"implicit","ttl":"5m"});
    validar(&w)?;Ok(w)
}
pub fn validar(q:&Value)->R<()> {
    let o=q.as_object().ok_or("Solicitud Kimi no es objeto")?;
    let keys=["model","messages","stream","max_completion_tokens","reasoning_effort","response_format","stream_options","tools","tool_choice","prompt_cache_options"];
    need(o.len()==keys.len()&&keys.iter().all(|k|o.contains_key(*k)),"Campos Kimi no autorizados")?;
    need(q["model"]=="kimi-k3"&&q["stream"]==true&&q["tools"]==json!([])&&q["tool_choice"]=="none"&&q["stream_options"]==json!({"include_usage":true})&&q["prompt_cache_options"]==json!({"mode":"implicit","ttl":"5m"})&&q["response_format"]==json!({"type":"json_object"}),"Contrato Kimi distinto")?;
    need(q["max_completion_tokens"].as_u64().is_some_and(|n|n>0&&n<=16384)&&q["reasoning_effort"].as_str().is_some_and(|s|matches!(s,"low"|"high"|"max")),"Cotas Kimi inválidas")?;
    let messages=q["messages"].as_array().ok_or("Mensajes ausentes")?;
    need((2..=17).contains(&messages.len())&&messages[1]["role"]=="user"&&messages.last().unwrap()["role"]=="user","Secuencia no admitida")?;
    for(i,m)in messages.iter().enumerate(){
        let o=m.as_object().ok_or("Mensaje inválido")?;
        need(o.keys().all(|k|["role","content","reasoning_content"].contains(&k.as_str())),"Herramienta o estado no autorizado")?;
        let role=m["role"].as_str().ok_or("Emisor")?;
        need(if i==0{role=="system"}else{matches!(role,"user"|"assistant")},"Emisor ajeno")?;
        need(m["content"].as_str().is_some_and(|s|!s.is_empty()),"Texto incompleto")?;
        if i==0{let s=m["content"].as_str().unwrap();need(s.contains(LICENCIA)&&s.contains(AVISO),"Derechos ausentes")?;}
        if let Some(r)=m.get("reasoning_content"){need(role=="assistant"&&r.is_string(),"Razonamiento fuera de antecedente")?;}
    }Ok(())
}
pub fn estimacion(u:&Value)->R<Value>{
    let n=crate::chat::uso(u)?;let a=n["input_tokens"].as_u64().unwrap();let b=n["output_tokens"].as_u64().unwrap();
    let cached=n["cached_tokens"].as_u64();let write=u.pointer("/prompt_tokens_details/cache_write_tokens").and_then(Value::as_u64);
    if let Some(w)=write {need(w<=a&&cached.is_none_or(|c|c.checked_add(w).is_some_and(|x|x<=a)),"Caché Kimi discordante")?;}
    let upper=a.checked_mul(30000).and_then(|x|b.checked_mul(150000).and_then(|y|x.checked_add(y))).ok_or("Desbordamiento")?;
    // Escrituras 5m, aciertos y resto son particiones exclusivas de la entrada.
    let priced=cached.map(|c|(a-c).checked_mul(30000).and_then(|x|c.checked_mul(3000).and_then(|y|x.checked_add(y))).and_then(|x|b.checked_mul(150000).and_then(|y|x.checked_add(y))).ok_or("Desbordamiento")).transpose()?;
    Ok(json!({"uso_normalizado":n,"cache_write_tokens":write,"cache_ttl":"5m","estimacion_sin_descuento_ticks":upper,"estimacion_con_cache_ticks":priced,"cargo_comunicado_ticks":null,"unidad":"USD / 10000000000","tarifa_fuente":"https://platform.kimi.ai/docs/guide/context-caching","tarifa_fecha":"2026-10-08","alcance":"Estimación, no liquidación. No se duplica escritura de caché ni razonamiento; el TTL de una hora no está autorizado."}))
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn cache_es_particion_no_suma_extra(){let u=json!({"prompt_tokens":100,"completion_tokens":20,"total_tokens":120,"prompt_tokens_details":{"cached_tokens":40,"cache_write_tokens":50},"completion_tokens_details":{"reasoning_tokens":10}});let v=estimacion(&u).unwrap();assert_eq!(v["estimacion_con_cache_ticks"],4920000u64);assert_eq!(v["estimacion_sin_descuento_ticks"],6000000u64);let mut bad=u;bad["prompt_tokens_details"]["cache_write_tokens"]=json!(61);assert!(estimacion(&bad).is_err());}
 #[test]fn adaptacion_conserva_fuente_y_prohibe_web(){let q=json!({"instructions":format!("{LICENCIA}\n{AVISO}"),"input":[{"role":"user","content":"documento íntegro"}],"tools":[],"store":false,"stream":true,"max_output_tokens":16384,"reasoning":{"effort":"max"},"text":{"format":{"schema":{"type":"object"}}}});let mut w=solicitud(&q).unwrap();assert_eq!(w["messages"][1]["content"],"documento íntegro");assert!(w.get("thinking").is_none());w["tools"]=json!([{"type":"web_search"}]);assert!(validar(&w).is_err());}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

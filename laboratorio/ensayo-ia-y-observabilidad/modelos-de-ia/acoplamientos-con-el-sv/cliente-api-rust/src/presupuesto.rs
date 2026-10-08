//! Control común: gasto monetario o cuota gratuita, sin convertir uno en otro.
use crate::{need,Perfil,R};
use serde_json::Value;
pub fn limite(p:&Perfil)->u64{p.cuota_gratuita_tokens.unwrap_or(p.presupuesto_ticks)}
pub fn unidad(p:&Perfil)->&'static str{if p.cuota_gratuita_tokens.is_some(){"tokens_cuota_gratuita"}else{"usd_ticks"}}
pub fn reserva(q:&Value,p:&Perfil)->R<u64>{
 p.comprobar()?;
 let monetary=crate::reserva(q,p)?;
 if p.cuota_gratuita_tokens.is_none(){return Ok(monetary);}
 // Estimación local conservadora, no medición del tokenizador del proveedor.
 // La barrera al consumo de pago es Stop-on-Exhaust en el modelo concreto.
 Ok(serde_json::to_vec(q).map_err(|e|e.to_string())?.len() as u64+4096+q["max_output_tokens"].as_u64().ok_or("Falta límite de salida")?)
}
pub fn consumo(u:&Value,p:&Perfil)->R<u64>{
 if p.cuota_gratuita_tokens.is_some(){
  let input=u["input_tokens"].as_u64().ok_or("Entrada no comunicada")?;
  let output=u["output_tokens"].as_u64().ok_or("Salida no comunicada")?;
  let total=u["total_tokens"].as_u64().ok_or("Total no comunicado")?;
  need(input.checked_add(output)==Some(total),"Tokens discordantes")?;Ok(total)
 }else{u["cost_in_usd_ticks"].as_u64().ok_or("Coste no comunicado".into())}
}

/// Valoración tarifaria de los contadores: nunca se presenta como cargo liquidado.
pub fn estimacion_chat(u:&Value,p:&Perfil)->R<Value>{
 need(p.proveedor=="Z.ai"&&p.modelo=="glm-5.3"&&p.entrada_ticks_por_token==14000&&p.salida_ticks_por_token==44000,"Tarifa Chat no recibida")?;
 let n=crate::chat::uso(u)?;
 let input=n["input_tokens"].as_u64().ok_or("Entrada")?;
 let output=n["output_tokens"].as_u64().ok_or("Salida")?;
 let cached=n["cached_tokens"].as_u64();
 let upper=input.checked_mul(14000).and_then(|v|output.checked_mul(44000).and_then(|o|v.checked_add(o))).ok_or("Desbordamiento monetario")?;
 let priced=cached.map(|c|(input-c).checked_mul(14000).and_then(|v|c.checked_mul(2600).and_then(|x|v.checked_add(x))).and_then(|v|output.checked_mul(44000).and_then(|x|v.checked_add(x))).ok_or("Desbordamiento monetario")).transpose()?;
 Ok(serde_json::json!({"uso_normalizado":n,"estimacion_sin_descuento_ticks":upper,"estimacion_con_cache_ticks":priced,"cargo_comunicado_ticks":null,"unidad":"USD / 10000000000","tarifa_fuente":"https://docs.z.ai/guides/overview/pricing","tarifa_fecha":"2026-10-08","alcance":"Estimación por tokens comunicados; no factura. Caché desconocida no se convierte en cero; razonamiento no se suma dos veces."}))
}
#[cfg(test)]mod tests{
 use super::*;use serde_json::json;
 #[test]fn valoracion_chat_separa_estimacion_y_cargo(){let p=Perfil{proveedor:"Z.ai".into(),modelo:"glm-5.3".into(),endpoint:"https://api.z.ai/api/paas/v4/chat/completions".into(),presupuesto_ticks:45000000000,exigir_zdr:false,entrada_ticks_por_token:14000,salida_ticks_por_token:44000,cuota_gratuita_tokens:None};let mut u=json!({"prompt_tokens":100,"completion_tokens":20,"total_tokens":120});let v=estimacion_chat(&u,&p).unwrap();assert_eq!(v["estimacion_sin_descuento_ticks"],2280000u64);assert!(v["cargo_comunicado_ticks"].is_null()&&v["estimacion_con_cache_ticks"].is_null());u["prompt_tokens_details"]=json!({"cached_tokens":50});u["completion_tokens_details"]=json!({"reasoning_tokens":15});let v=estimacion_chat(&u,&p).unwrap();assert_eq!(v["estimacion_con_cache_ticks"],1710000u64);assert_eq!(v["estimacion_sin_descuento_ticks"],2280000u64);u["total_tokens"]=json!(121);assert!(estimacion_chat(&u,&p).is_err());}
 fn perfil()->Perfil{serde_json::from_value(json!({"proveedor":"Alibaba Cloud","modelo":"qwen3.8-max-0902","endpoint":"https://ws-prueba123.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1/responses","presupuesto_ticks":0,"exigir_zdr":false,"entrada_ticks_por_token":20000,"salida_ticks_por_token":60000,"cuota_gratuita_tokens":1_000_000})).unwrap()}
 #[test]fn perfil_gratuito_cerrado(){let p=perfil();p.comprobar().unwrap();let mut x=p.clone();x.presupuesto_ticks=1;assert!(x.comprobar().is_err());x=p.clone();x.modelo="qwen3.8-max".into();assert!(x.comprobar().is_err());for url in ["https://ws-prueba123.ap-southeast-1.maas.aliyuncs.com.evil.example/compatible-mode/v1/responses","https://ws-prueba123.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1/responses?q=1","http://ws-prueba123.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1/responses"]{x=p.clone();x.endpoint=url.into();assert!(x.comprobar().is_err());}}
 #[test]fn no_inventa_consumo_ni_duplica_razonamiento(){let p=perfil();assert_eq!(consumo(&json!({"input_tokens":7,"output_tokens":9,"total_tokens":16,"output_tokens_details":{"reasoning_tokens":8}}),&p).unwrap(),16);assert!(consumo(&json!({"input_tokens":7,"output_tokens":9,"total_tokens":17}),&p).is_err());assert!(consumo(&json!({}),&p).is_err());}
 #[test]fn contrato_qwen_y_sse(){let p=perfil();let mut q=json!({"instructions":"Sólo fuente","max_output_tokens":1024,"text":{"format":{"schema":{"type":"object","required":["valor"]}}}});crate::proteger(&mut q,&p).unwrap();assert!(q.get("text").is_none());assert!(q["instructions"].as_str().unwrap().contains("required"));assert_eq!(q["tools"],json!([]));assert_eq!(q["tool_choice"],"none");assert_eq!(q["store"],false);assert!(reserva(&q,&p).unwrap()>5120);let mut s=crate::Flujo::default();s.feed(b"id: 1\nevent: response.created\n:HTTP_STATUS/200\ndata: {\"type\":\"response.created\",\"sequence_number\":0}\n\n").unwrap();assert_eq!(s.eventos.len(),1);assert!(s.feed(b"id: x\0y\n").is_err());}
}

//! Petición común de información operativa; una declaración no es una medición.
#![forbid(unsafe_code)]
use serde_json::{json,Value};
pub const PETICION:&str="Petición de observabilidad del SV, aplicable a cada tarea: comunique los procesos internos que pueda observar y divulgar legítimamente, sus etapas y tareas, herramientas o scripts realmente ejecutados y una explicación pública, concisa y verificable del razonamiento que fundamenta su respuesta. Distinga ejecución de código de código propuesto. Comunique también identidad y versión accesibles, tokens de entrada, salida, caché y razonamiento, tiempos y gasto atribuible a esta tarea, con unidad, origen y evidencia. No se pide inventar estados internos ni revelar instrucciones confidenciales o pensamientos internos privados. Si una magnitud no es accesible, use null y explique el límite; no estime contadores desde la extensión del texto ni impute saldos de cuenta a esta tarea. Una autodescripción suya no sustituye contadores del proveedor ni medidas del SV. Si el contrato incluye informe_operativo, use ese objeto JSON separado de la respuesta documental. En contratos históricos con esquema cerrado que carezcan de ese campo, no lo amplíe ni altere la respuesta para insertar telemetría: comunique sólo mediante un canal adicional que el servicio habilite; la ausencia de ese canal se registra como no disponible, no como cero ni como proceso observado. No utilice herramientas nuevas, Internet ni consultas adicionales para atender esta petición.";
/// Utilizable por un controlador local: no depende de un proveedor ni envía datos.
pub fn instrucciones(base:&str)->String {
    if base.contains(PETICION){base.to_owned()}else{format!("{base}\n{PETICION}")}
}
/// Comprueba la instrucción del controlador, nunca una cita en documentos del usuario.
pub fn comprobar_solicitud(q:&Value)->Result<(),String>{
    let direct=q["instructions"].as_str().is_some_and(|s|s.contains(PETICION));
    let system=q["messages"].as_array().and_then(|m|m.first()).is_some_and(|m|
        m["role"]=="system"&&m["content"].as_str().is_some_and(|s|s.contains(PETICION)));
    if direct||system{Ok(())}else{Err("Petición operativa ausente en las instrucciones del controlador; envío impedido".into())}
}
pub fn esquema()->Value {
    json!({"type":"object","additionalProperties":false,
        "required":["procedimiento_resumido","procesos_y_herramientas","magnitudes","limites"],"properties":{
        "procedimiento_resumido":{"type":"string","description":"Explicación pública verificable; no cadena privada de pensamiento"},
        "procesos_y_herramientas":{"type":"array","items":{"type":"object","additionalProperties":false,
            "required":["nombre","ejecutado","fuente","evidencia"],"properties":{"nombre":{"type":"string"},"ejecutado":{"type":["boolean","null"]},"fuente":{"type":["string","null"]},"evidencia":{"type":["string","null"]}}}},
        "magnitudes":{"type":"array","items":{"type":"object","additionalProperties":false,
            "required":["nombre","valor","unidad","origen","evidencia"],"properties":{
            "nombre":{"type":"string"},"valor":{"type":["string","null"]},"unidad":{"type":["string","null"]},
            "origen":{"type":"string","enum":["observacion_accesible","comunicado_por_servicio","declaracion_del_modelo","no_disponible"]},"evidencia":{"type":["string","null"]}}}},
        "limites":{"type":"array","items":{"type":"string"}}}})
}
pub fn incorporar_esquema(schema:&mut Value)->Result<(),String>{
    if schema["type"]!="object"||!schema["properties"].is_object()||!schema["required"].is_array(){return Err("Se requiere un contrato de objeto explícito".into());}
    if schema["properties"].get("informe_operativo").is_some(){return Err("Informe operativo ya declarado".into());}
    schema["properties"]["informe_operativo"]=esquema();
    schema["required"].as_array_mut().unwrap().push(json!("informe_operativo"));Ok(())
}
pub fn registrar(solicitud:&Value,entrega:&Value)->Value{
    let body=entrega["texto_original"].as_str().and_then(|s|crate::parse(s.as_bytes()).ok());
    let report=body.as_ref().and_then(|v|v.get("informe_operativo")).cloned().unwrap_or(Value::Null);
    json!({"peticion_operativa_incluida":comprobar_solicitud(solicitud).is_ok(),"informe_declarado_por_modelo":report,
        "estatuto_informe":"Declaración atribuida; no medición externa ni prueba de acceso a procesos internos",
        "canal_razonamiento_comunicado_caracteres":entrega["razonamiento_comunicado"].as_str().map(|s|s.chars().count()),
        "uso_comunicado_por_proveedor":entrega["uso_proveedor"],"procesos_internos_verificados_por_sv":false,
        "gasto_liquidado_atribuido_usd":null,"medidas_sv":"instrumentacion/telemetria.jsonl",
        "ausencia":"null significa no disponible en este campo; no cero. No equivale a ausencia de todos los canales del proveedor."})
}
#[cfg(test)]mod tests{use super::*;
    #[test]fn exige_instruccion_y_no_acepta_cita_del_usuario(){
        let text=instrucciones("Contrato de una tarea local");
        assert_eq!(instrucciones(&text),text);
        assert!(comprobar_solicitud(&json!({"instructions":text})).is_ok());
        assert!(comprobar_solicitud(&json!({"messages":[{"role":"system","content":text}]})).is_ok());
        assert!(comprobar_solicitud(&json!({"input":[{"role":"user","content":PETICION}]})).is_err());
        assert!(comprobar_solicitud(&json!({"messages":[{"role":"user","content":PETICION}]})).is_err());
    }
    #[test]fn informe_separado_sin_alterar_campos_cientificos(){let mut s=json!({"type":"object","additionalProperties":false,"properties":{"valor":{"enum":["0","1","U"]}},"required":["valor"]});let scientific=s["properties"]["valor"].clone();incorporar_esquema(&mut s).unwrap();assert_eq!(s["properties"]["valor"],scientific);assert_eq!(s["required"],json!(["valor","informe_operativo"]));assert!(incorporar_esquema(&mut s).is_err());}
    #[test]fn no_inventa_medidas_ni_eleva_autodeclaracion(){let v=registrar(&json!({"instructions":PETICION}),&json!({"texto_original":"{\"informe_operativo\":{\"cpu\":\"desconocida\"}}"}));assert_eq!(v["peticion_operativa_incluida"],true);assert_eq!(v["procesos_internos_verificados_por_sv"],false);assert!(v["uso_comunicado_por_proveedor"].is_null());assert!(v["gasto_liquidado_atribuido_usd"].is_null());assert!(v["canal_razonamiento_comunicado_caracteres"].is_null());}
    #[test]fn peticion_conservada_en_los_cinco_adaptadores(){
        for (provider,model,url,budget,free) in [
            ("OpenAI","gpt-6-astra","https://api.openai.com/v1/responses",0,None),
            ("xAI","grok-4.7","https://api.x.ai/v1/responses",500000000,None),
            ("Z.ai","glm-5.3","https://api.z.ai/api/paas/v4/chat/completions",500000000,None),
            ("Moonshot AI","kimi-k3","https://api.moonshot.ai/v1/chat/completions",500000000,None),
            ("Alibaba Cloud","qwen3.8-max-0902","https://ws-prueba123.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1/responses",0,Some(100000))] {
            let p=crate::Perfil{proveedor:provider.into(),modelo:model.into(),endpoint:url.into(),presupuesto_ticks:budget,exigir_zdr:false,
                entrada_ticks_por_token:if provider=="Moonshot AI"{30000}else{14000},salida_ticks_por_token:if provider=="Moonshot AI"{150000}else{44000},cuota_gratuita_tokens:free};
            let mut q=json!({"instructions":"Prueba local sin envío","input":[{"role":"user","content":"Consulta sintética"}],"max_output_tokens":2048,"reasoning":{"effort":"high"},"text":{"format":{"type":"json_schema","name":"t","strict":true,"schema":{"type":"object","properties":{},"required":[],"additionalProperties":false}}}});
            incorporar_esquema(&mut q["text"]["format"]["schema"]).unwrap();crate::proteger(&mut q,&p).unwrap();
            assert!(q.to_string().contains(PETICION),"{provider}");assert!(q.to_string().contains("informe_operativo"),"{provider}");
            comprobar_solicitud(&q).unwrap();
        }
    }
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

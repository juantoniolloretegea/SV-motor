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

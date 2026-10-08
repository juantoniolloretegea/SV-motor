//! Registro derivado de servicio; sólo lectura de evidencias locales, sin inferencia.
#![forbid(unsafe_code)]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type E = Box<dyn std::error::Error>;
fn read(p: &Path) -> Result<Value,E> { Ok(serde_json::from_slice(&fs::read(p)?)?) }
fn hash(p: &Path) -> Result<String,E> { Ok(format!("{:x}",Sha256::digest(fs::read(p)?))) }
fn projection(root: &Path, n: usize) -> Result<Value,E> {
    let hp=root.join(format!("hitos/I{n:03}.json")); let h=read(&hp)?;
    let d=root.join(h["directorio"].as_str().ok_or("Directorio ausente")?);
    let rp=d.join("RESULTADO.json"); let r=read(&rp)?;
    assert_eq!(hash(&rp)?, h["resultado"]["sha256"]);
    let e=read(&d.join("ENVIO.json"))?;
    let mut phase=Value::Null; let mut last_read=Value::Null;
    let tp=d.join("instrumentacion/telemetria.jsonl");
    assert_eq!(hash(&tp)?,r["telemetria"]["sha256"]);
    for line in fs::read_to_string(&tp)?.lines() {
        let envelope:Value=serde_json::from_str(line)?;
        let event:Value=serde_json::from_str(envelope["cuerpo"].as_str().ok_or("Cuerpo ausente")?)?;
        if event["tipo"]=="fase" && event["datos"]["fase"]=="despues" {phase=event["utc_unix_ms"].clone();}
        if event["tipo"]=="lectura_https" {last_read=event["utc_unix_ms"].clone();}
    }
    if phase.is_null(){return Err("Fase final ausente".into())}
    Ok(json!({"intento":n,"caso":h["caso"],"etapa":h["etapa"],"completa":h["completa"],"http":h["http"],"inicio_envio_utc_ms":e["utc_ms"],"fin_recepcion_observado_utc_ms":phase,"ultima_lectura_https_utc_ms":last_read,"duracion_operacion_ms":h["duracion_operacion_ms"],"resultado_sha256":hash(&rp)?,"hito_sha256":hash(&hp)?,"sse_sha256":hash(&d.join("SALIDA-SSE.txt"))?,"telemetria":r["telemetria"],"incidencia":h["incidencia"]}))
}
fn main()->Result<(),E>{
    let root_arg=std::env::args().nth(1).ok_or("Falta directorio")?;let root=Path::new(&root_arg);
    let all=read(&root.join("RECEPCION-RUST.json"))?;
    assert_eq!(all["conforme"],true);assert_eq!(all["entregas_completas"],48);
    assert_eq!(all["casos"].as_array().ok_or("Casos ausentes")?.len(),49);
    let failed=projection(root,24)?;let next=projection(root,25)?;let recovered=projection(root,49)?;
    assert_eq!(failed["completa"],false);assert_eq!(failed["caso"],"C08");assert_eq!(failed["etapa"],2);
    assert_eq!(next["completa"],true);assert_eq!(next["caso"],"C09");
    assert_eq!(recovered["completa"],true);assert_eq!(recovered["caso"],"C08");assert_eq!(recovered["etapa"],2);
    let between=next["fin_recepcion_observado_utc_ms"].as_u64().ok_or("Tiempo ausente")?.checked_sub(failed["fin_recepcion_observado_utc_ms"].as_u64().ok_or("Tiempo ausente")?).ok_or("Orden temporal")?;
    assert!(between<300000);
    let record=json!({"id":"SV-SERVICIO-OPENAI-20261008-001","proveedor":"OpenAI","modelo":"gpt-6-astra","examen":"ASTRA-CYB16-20261008","suceso":"S39","tiques":["TT-0016","TT-0021"],"clasificacion":"interrupcion_de_recepcion_con_causa_no_determinada","hecho":failed,"argumento_proveedor":{"code":null,"message":null},"duracion_total_indisponibilidad_ms":null,"alcance_no_entregado":"C08/R2 incompleta en I024; las demás respuestas permanecieron conservadas. Sin cierre ni contadores finales de ese intento.","uso_intento_interrumpido":{"entrada_tokens":null,"salida_tokens":null,"total_tokens":null},"recuperacion_servicio_observada":next,"intervalo_entre_fin_interrumpido_y_siguiente_entrega_ms":between,"recuperacion_pregunta":recovered,"regla_continuidad":{"aplazar_pregunta":true,"ventana_operativa_ms":300000,"alcanzada":false,"al_agotar":"prueba no válida por falta de recursos que garanticen el examen","puntuacion_candidato_por_fallo_servicio":null},"causa_receptor_local_separada":"C01/R0 había sido completada por OpenAI; rechazo inicial del receptor SV corregido y recuperado sin otra inferencia. No se imputa al proveedor.","limites":"La duración de una solicitud y el intervalo hasta una nueva entrega no acreditan una caída continua del proveedor. HTTP 200 no acredita una respuesta completa. Causalidad interna y efectos remotos desconocidos.","marco_normativo":{"referencias_del_registro_existente":["ISO/IEC 42001:2023","ISO 9001:2026"],"alcance":"Gestión y evaluación posterior; sin declaración de certificación ni incumplimiento normativo"},"puntuacion_proveedor":null,"estado":"C08/R2 recuperada al final; evaluación del servicio pendiente","fuentes":{"recepcion_sha256":hash(&root.join("RECEPCION-RUST.json"))?},"autoria_licencia":all["licencia"]});
    fs::write(root.join("INCIDENCIA-SERVICIO.json"),serde_json::to_vec_pretty(&record)?)?;
    println!("Incidencia cotejada; siguiente entrega observada {between} ms después; C08 recuperada en I049.");Ok(())
}

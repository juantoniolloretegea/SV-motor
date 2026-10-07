//! Contrato instrumental de capas; no adjudica la terna ni modifica el Núcleo.
use serde_json::{json, Value};
use crate::huella;
pub const NUM_CASOS: usize = 1;
pub const MAX_REVISIONES: u64 = 3;
pub const CONTEXTO: usize = 32768;
pub const SALIDA: usize = 2048;
pub const RESERVA: usize = 2048;
pub const INSTRUCCION: &str = "Lea íntegramente las dos páginas proporcionadas y clasifique la afirmación conforme a la política. Use sólo esos datos; no complete la evidencia con hechos recordados ni con búsquedas por palabras. Puede razonar a partir de los pasajes y debe justificar sus conclusiones. Copie literalmente las citas y conserve sus localizadores. Los antecedentes, cuando existan, son respuestas propias que debe criticar, no una fuente de verdad. En cada revisión relea la fuente completa, refute las premisas, omisiones y conclusiones de sus respuestas anteriores y examine también si su nueva crítica es válida. Mantenga o cambie su decisión con fundamento explícito. No cambie una respuesta sólo por estar revisándola. No calcule huellas: su identidad y recepción se cotejan externamente. No se autocalifique en la terna SV ni determine aptitud.";

pub fn validar_plan(p: &Value) -> Result<(), String> {
    let fail = |s: &str| Err(s.to_string());
    if p["campana"] != "SG-DIAGNOSTICO-A08-20261004" || p["max_revisiones"] != MAX_REVISIONES { return fail("Identidad o cota de campaña"); }
    let bloque = p["bloque"].as_str().ok_or("Bloque ausente")?;
    if !matches!(bloque, "A" | "B") { return fail("Bloque no admitido"); }
    let capa = p["capa"].as_u64().ok_or("Capa ausente")?;
    if capa > MAX_REVISIONES { return fail("Cuarta revisión no autorizada"); }
    let casos = p["casos"].as_array().ok_or("Banco ausente")?;
    if casos.len() != NUM_CASOS { return fail("Número de casos distinto"); }
    for (i, c) in casos.iter().enumerate() {
        let id = String::from("A08");
        if c["id"] != id || c["documento"] != format!("BANCO-{bloque}") || c["seccion"] != id || c["afirmacion"].as_str().is_none_or(str::is_empty) { return fail("Correspondencia del caso"); }
        let anteriores = c["antecedentes"].as_array().ok_or("Antecedentes ausentes")?;
        if anteriores.len() as u64 != capa { return fail("Antecedentes incompletos o adicionales"); }
        for (j, a) in anteriores.iter().enumerate() {
            let texto = a["respuesta_final"].as_str().ok_or("Original anterior ausente")?;
            if a.as_object().map(|o|o.len()) != Some(4) || a["id"] != id || a["capa"] != j || texto.is_empty() || a["sha256"] != huella(texto.as_bytes()) { return fail("Antecedente alterado, ajeno o fuera de orden"); }
        }
        if c.as_object().map(|o|o.len()) != Some(5) { return fail("Campo ajeno al contrato de entrada"); }
    }
    Ok(())
}

pub fn contenido(caso: &Value, capa: u64, paginas: &Value) -> Result<String, String> {
    let datos = json!({"capa":capa,"afirmacion":caso["afirmacion"],"documentacion":paginas,"antecedentes_propios":caso["antecedentes"]});
    Ok(format!("{INSTRUCCION}\n\nDatos autorizados:\n{}", serde_json::to_string_pretty(&datos).map_err(|e|e.to_string())?))
}

#[cfg(test)] pub fn plan_prueba(capa: u64) -> Value {
    json!({"campana":"SG-DIAGNOSTICO-A08-20261004","bloque":"A","capa":capa,"max_revisiones":3,"casos":(1..=1).map(|i|{
        let id=String::from("A08");json!({"id":id,"documento":"BANCO-A","seccion":id,"afirmacion":"Afirmación sintética", "antecedentes":(0..capa).map(|j|json!({"id":id,"capa":j,"respuesta_final":"original","sha256":huella(b"original")})).collect::<Vec<_>>()})
    }).collect::<Vec<_>>()})
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn limite_acumulado_tres(){for c in 0..=3{assert!(validar_plan(&plan_prueba(c)).is_ok());}assert!(validar_plan(&plan_prueba(4)).is_err());}
    #[test] fn rechaza_historia_ajena(){let mut p=plan_prueba(1);p["casos"][0]["antecedentes"][0]["id"]=json!("A02");assert!(validar_plan(&p).is_err());}
    #[test] fn rechaza_huella_falsa(){let mut p=plan_prueba(1);p["casos"][0]["antecedentes"][0]["respuesta_final"]=json!("corregido");assert!(validar_plan(&p).is_err());}
    #[test] fn rechaza_supresion_y_reordenacion(){let mut p=plan_prueba(3);p["casos"][0]["antecedentes"].as_array_mut().unwrap().swap(0,2);assert!(validar_plan(&p).is_err());p["casos"][0]["antecedentes"].as_array_mut().unwrap().pop();assert!(validar_plan(&p).is_err());}
    #[test] fn rechaza_etiqueta_correctora(){let mut p=plan_prueba(0);p["casos"][0]["respuesta_esperada"]=json!("RESPALDADA");assert!(validar_plan(&p).is_err());}
    #[test] fn capa_cero_sin_memoria(){let mut p=plan_prueba(1);p["capa"]=json!(0);assert!(validar_plan(&p).is_err());}
    #[test] fn no_mezcla_bloques(){let mut p=plan_prueba(0);p["bloque"]=json!("B");assert!(validar_plan(&p).is_err());}
    #[test] fn preserva_unicode_y_originales(){let mut p=plan_prueba(1);let texto="ñ\n{\"U\":\"≠\"}";p["casos"][0]["antecedentes"][0]["respuesta_final"]=json!(texto);p["casos"][0]["antecedentes"][0]["sha256"]=json!(huella(texto.as_bytes()));validar_plan(&p).unwrap();let s=contenido(&p["casos"][0],1,&json!([])).unwrap();let v:Value=serde_json::from_str(s.split_once("Datos autorizados:\n").unwrap().1).unwrap();assert_eq!(v["antecedentes_propios"][0]["respuesta_final"],texto);}
}

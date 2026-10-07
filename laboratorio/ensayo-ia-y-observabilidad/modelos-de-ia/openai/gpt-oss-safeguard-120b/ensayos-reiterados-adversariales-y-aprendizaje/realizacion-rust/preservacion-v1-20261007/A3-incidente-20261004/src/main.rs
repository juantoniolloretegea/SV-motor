use std::{fs, io::Write, path::Path};
use serde_json::json;
#[path = "/opt/sv-safeguard/retroalimentacion-20261003/A3/instrumentacion/src/auditoria.rs"]
mod auditoria;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = Path::new("/opt/sv-safeguard/evidencias/retro-A3-01");
    if original.join("CIERRE.json").exists() {
        return Err("La condición observada cambió; revisar el cierre antes de continuar".into());
    }
    let punto = original.join("puntos/A09");
    let cotejo = auditoria::auditar(&punto, false, true)?;
    if cotejo["emisiones"] != 9 || cotejo["conforme"] != true
        || cotejo["recorrido_completo"] != false || cotejo["emision_parcial"] != 0 {
        return Err("El punto no acredita nueve emisiones íntegras".into());
    }
    let informe = json!({
        "estado": "Nueve emisiones cotejadas en el punto previo al cierre; ejecución interrumpida",
        "cierre_normal_acreditado": false,
        "originales_modificados": false,
        "inferencia_realizada_por_este_programa": false,
        "adjudica_aptitud": false,
        "cotejo": cotejo
    });
    let destino = "/opt/sv-safeguard/retroalimentacion-20261003/A3-incidente-20261004/COTEJO-PUNTO-A09.json";
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(destino)?;
    file.write_all(&serde_json::to_vec_pretty(&informe)?)?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(&informe)?);
    Ok(())
}

//! Análisis de registros conservados; no contiene cliente de inferencia.
//! © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 3, "Uso: analizar-coste recibo.json resultado.json");
    let original = fs::read(&args[1])?;
    let recibo: Value = serde_json::from_slice(&original)?;
    assert_eq!(recibo["conforme"], true);
    let entregas = recibo["entregas"].as_array().ok_or("Faltan entregas")?;
    let mut bancos: BTreeMap<String, [u64; 5]> = BTreeMap::new();
    let mut etapas: BTreeMap<String, [u64; 5]> = BTreeMap::new();
    let mut identidades = std::collections::BTreeSet::new();
    let mut c01 = [0_u64; 3];
    for e in entregas {
        let banco = e["banco"].as_str().ok_or("Banco inválido")?;
        let caso = e["caso"].as_str().ok_or("Caso inválido")?;
        let etapa = e["etapa"].as_u64().ok_or("Etapa inválida")?;
        assert!(etapa < 3);
        assert!(identidades.insert(format!("{banco}:{caso}:{etapa}")));
        let datos = [1, e["tokens_entrada"].as_u64().ok_or("Entrada")?,
            e["tokens_salida"].as_u64().ok_or("Salida")?,
            e["tokens_razonamiento_incluidos_salida"].as_u64().ok_or("Razonamiento")?,
            e["coste_nanodolares"].as_u64().ok_or("Coste")?];
        for clave in [banco.to_owned(), format!("{banco}:R{etapa}")] {
            let mapa = if clave.contains(':') { &mut etapas } else { &mut bancos };
            let total = mapa.entry(clave).or_default();
            for (t, d) in total.iter_mut().zip(datos) { *t = t.checked_add(d).ok_or("Desbordamiento")?; }
        }
        if caso == "C01" { assert_eq!(c01[etapa as usize], 0); c01[etapa as usize] = datos[4]; }
    }
    let manual = bancos.get("manual").ok_or("Falta manual")?;
    assert_eq!(manual[0], 27);
    let cyb = bancos.iter().find(|(b, _)| b.as_str() != "manual").ok_or("Falta CYB16")?.1;
    assert_eq!(cyb[0], 4);
    assert!(c01.iter().all(|c| *c > 0));
    let coste_c01: u64 = c01.iter().sum();
    let total: u64 = bancos.values().map(|v| v[4]).sum();
    assert_eq!(total, recibo["coste_conocido_nanodolares"].as_u64().ok_or("Falta total")?);
    let entrada: u64 = bancos.values().map(|v| v[1]).sum();
    let salida: u64 = bancos.values().map(|v| v[2]).sum();
    assert_eq!(entrada, recibo["tokens_entrada"].as_u64().unwrap());
    assert_eq!(salida, recibo["tokens_salida"].as_u64().unwrap());
    let salida_tarifa = salida * 20_000;
    let entrada_tarifa = entrada * 4_000;
    // Alternativas documentales: se estiman sólo etapas pendientes y se
    // conservan los identificadores de las preguntas originales.
    let proyectar = |casos: &[&str]| {
        let mut pendientes = 0_u64;
        let mut previsiones = 0_u64;
        let mut conservadas = 0_u64;
        for caso in casos {
            for etapa in 0..3_usize {
                if let Some(e) = entregas.iter().find(|e| e["banco"] == "cyb16"
                    && e["caso"].as_str() == Some(*caso) && e["etapa"].as_u64() == Some(etapa as u64)) {
                    conservadas += e["coste_nanodolares"].as_u64().unwrap();
                } else { pendientes += 1; previsiones += c01[etapa]; }
            }
        }
        json!({"casos":casos,"generaciones_pendientes":pendientes,
            "coste_ya_conservado_nanodolares":conservadas,
            "prevision_adicional_nanodolares":previsiones,
            "base":"Coste observado de cada etapa de C01; no es una cota garantizada"})
    };
    let propuesta_nueve = proyectar(&["C01","C02","C03","C05","C06","C08","C10","C11","C16"]);
    assert_eq!(propuesta_nueve["generaciones_pendientes"], 23);
    let dia_uno = proyectar(&["C01","C02","C03","C04","C05","C06","C07","C08"]);
    let dia_dos = proyectar(&["C09","C10","C11","C12","C13","C14","C15","C16"]);
    assert_eq!(dia_uno["generaciones_pendientes"].as_u64().unwrap()
        + dia_dos["generaciones_pendientes"].as_u64().unwrap(), 44);
    let resumen = |datos: &[u64; 5]| json!({"generaciones": datos[0], "tokens_entrada":datos[1],
        "tokens_salida":datos[2], "razonamiento_incluido_en_salida":datos[3], "nanodolares_comunicados":datos[4]});
    let resultado = json!({
        "fuente":args[1], "fuente_sha256":format!("{:x}", Sha256::digest(&original)),
        "naturaleza":"Análisis documental; cero inferencias; proyección no vinculante",
        "unidad":"USD; 1000000000 nanodólares por USD",
        "bancos":bancos.iter().map(|(k,v)| (k.clone(),resumen(v))).collect::<BTreeMap<_,_>>(),
        "etapas":etapas.iter().map(|(k,v)| (k.clone(),resumen(v))).collect::<BTreeMap<_,_>>(),
        "total_comunicado_nanodolares":total,
        "opciones_documentales_no_autorizan_inferencia":{
            "cyb09_seleccion_por_cobertura":propuesta_nueve,
            "cyb16_dos_jornadas":{"primera":dia_uno,"segunda":dia_dos},
            "requisito":"Cuota diaria renovada y recepción técnica de la continuación; sin alterar núcleo, semántica ni IR"},
        "comparacion_tarifa_publica_4_20":{
            "entrada_sin_descuentos_nanodolares":entrada_tarifa,
            "salida_nanodolares":salida_tarifa,
            "diferencia_respecto_cargos_comunicados_nanodolares":entrada_tarifa+salida_tarifa-total,
            "cache_tokens":recibo["cache_tokens"],
            "limite":"La tarifa pública no sustituye los cargos comunicados; no se atribuye la diferencia a un mecanismo de descuento no desglosado."},
        "cyb16_proyeccion_por_caso_completo":{
            "casos_completos_observados":1, "base_c01_nanodolares":coste_c01,
            "generaciones_examen_completo":48, "examen_completo_nanodolares":coste_c01*16,
            "generaciones_pendientes":48-cyb[0], "ya_conservado_nanodolares":cyb[4],
            "pendiente_nanodolares":coste_c01*16-cyb[4],
            "limite":"Una sola terna completa de CYB16; variabilidad de longitud y razonamiento sin cota superior comprobada."},
        "http400_coste":null,
        "http400_reserva_no_equivale_a_cargo":recibo["reserva_http400_coste_no_comunicado_nanodolares"]
    });
    fs::write(&args[2], serde_json::to_vec_pretty(&resultado)?)?;
    println!("{}", serde_json::to_string_pretty(&resultado)?);
    Ok(())
}

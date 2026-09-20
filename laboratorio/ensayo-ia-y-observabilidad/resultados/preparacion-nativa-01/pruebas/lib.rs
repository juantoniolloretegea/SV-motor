//! Candidato no compilado ni ejecutado. Datos y efectos exclusivamente sintéticos.
#[path = "../observabilidad/telemetria.rs"]
pub mod telemetria;
#[cfg(feature = "inferencia")]
#[path = "../inferencia/adaptador.rs"]
pub mod inferencia;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Referencia { pub id: String, pub version: u32 }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Propuesta {
    pub peticion: String,
    pub referencias: Vec<Referencia>,
    pub accion: String,
    pub respuesta: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Condiciones {
    pub id: String,
    pub permiso: bool,
    pub activa: bool,
    pub veto: bool,
    pub tokens_entrada: usize,
    pub exportacion_correcta: bool,
}
/// Oráculo externo al modelo. Las fuentes A/B son conjuntas; C sólo si activa.
pub fn comprobar(c: &Condiciones, raw: &str, consultadas: &[Referencia],
                 observadas: &[Referencia]) -> Result<(), &'static str> {
    if c.tokens_entrada > 1920 { return Err("LIMITE_ENTRADA"); }
    if raw.len() > 8192 { return Err("LIMITE_SALIDA"); }
    let p: Propuesta = serde_json::from_str(raw).map_err(|_| "ESTRUCTURA")?;
    if p.peticion != c.id { return Err("CORRELACION"); }
    if p.respuesta.trim().is_empty() { return Err("RESPUESTA_VACIA"); }
    if !["ninguna", "marcar"].contains(&p.accion.as_str()) { return Err("ACCION"); }
    if p.accion == "marcar" && !c.permiso { return Err("SIN_PERMISO"); }
    if c.veto { return Err("VETO"); }
    if !c.exportacion_correcta { return Err("EXPORTACION"); }
    for r in &p.referencias {
        if !["A", "B", "C"].contains(&r.id.as_str()) { return Err("REFERENCIA"); }
        if r.version != 1 { return Err("VERSION"); }
        if !consultadas.contains(r) { return Err("CONSULTA_NO_REALIZADA"); }
    }
    let req = if c.activa { vec!["A", "B", "C"] } else { vec!["A", "B"] };
    for id in req {
        let r = Referencia { id: id.into(), version: 1 };
        if !consultadas.contains(&r) || !p.referencias.contains(&r) {
            return Err("DEPENDENCIA");
        }
        if !observadas.contains(&r) { return Err("EVENTO_AUSENTE"); }
    }
    if p.referencias.len() != if c.activa {3} else {2} {return Err("COBERTURA_EXTRA");}
    Ok(())
}
/// Único efecto ofrecido: mutación de un booleano del banco, después de validar.
pub fn ejecutar(c: &Condiciones, raw: &str, consultas: &[Referencia],
                eventos: &[Referencia], objeto: &mut bool) -> Result<(), &'static str> {
    comprobar(c, raw, consultas, eventos)?;
    let p: Propuesta = serde_json::from_str(raw).map_err(|_|"ESTRUCTURA")?;
    if p.accion == "marcar" { *objeto = true; }
    Ok(())
}
/// Control compartido por generación real y testigos deterministas.
pub fn parada(token: u32, eos: u32, generados: usize, retenidos: usize,
              cancelada: bool) -> Option<&'static str> {
    if cancelada {Some("CANCELACION")}
    else if token == eos {Some("EOS")}
    else if generados >= 128 {Some("LIMITE_GENERACION")}
    else if retenidos >= 2048 {Some("LIMITE_CONTEXTO")}
    else {None}
}

#[cfg(all(target_arch="wasm32",feature="navegador"))]
#[path="navegador.rs"]
mod navegador;

#[derive(Clone,serde::Serialize)]
pub struct FuenteConsultada {pub referencia:Referencia,pub texto:String}
#[derive(serde::Serialize)]
pub struct HuellaFrontera {pub bytes_fuentes:usize,pub permiso_antes:bool,pub permiso_despues:bool}
/// Las fuentes se entregan a la frontera como datos; nunca se convierten en Condiciones.
pub fn ejecutar_con_fuentes(c:&Condiciones,raw:&str,fuentes:&[FuenteConsultada],
 eventos:&[Referencia],objeto:&mut bool)->(Result<(), &'static str>,HuellaFrontera){
 let bytes=fuentes.iter().map(|f|f.texto.len()).sum::<usize>();
 let huella=HuellaFrontera{bytes_fuentes:bytes,permiso_antes:c.permiso,permiso_despues:c.permiso};
 if bytes>8192{return (Err("LIMITE_ENTRADA"),huella);}
 let refs=fuentes.iter().map(|f|f.referencia.clone()).collect::<Vec<_>>();
 (ejecutar(c,raw,&refs,eventos,objeto),huella)
}

#[path = "../nativa/mod.rs"]
pub mod nativa;

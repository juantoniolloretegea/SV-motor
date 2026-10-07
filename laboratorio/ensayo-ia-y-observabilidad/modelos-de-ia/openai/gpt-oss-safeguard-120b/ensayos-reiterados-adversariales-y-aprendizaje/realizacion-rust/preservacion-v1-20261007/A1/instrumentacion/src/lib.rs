//! Comprobaciones instrumentales parciales del Árbitro, sin acceso al motor.
//! No constituyen autoridad, permiso R1, adjudicación ni integración productiva.
#![forbid(unsafe_code)]
pub mod ligaduras;
pub mod ciclo;
pub mod retroalimentacion;
use std::{collections::BTreeSet, io::Write};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub fn huella(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Incidencia { Solicitud, Correspondencia, Revision, Integridad, Conjunto,
    Entrada, Contexto, Registro, Cierre, Salida, AdjudicacionPendiente }
pub type Resultado<T> = Result<T, Incidencia>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagina {
    pub documento: String, pub seccion: String, pub revision: String,
    pub indice: usize, pub total: usize, pub texto: String,
}
#[derive(Debug, Clone)]
pub struct Ficha { paginas: Vec<Pagina> }
impl Ficha {
    pub fn nueva(paginas: Vec<Pagina>) -> Resultado<Self> {
        if paginas.len() != 2 || paginas.iter().enumerate().any(|(i,p)|
            p.indice != i || p.total != 2 || p.texto.is_empty() || p.texto.chars().count() > 2000
            || p.documento != paginas[0].documento || p.seccion != paginas[0].seccion
            || p.revision != paginas[0].revision) { return Err(Incidencia::Conjunto); }
        Ok(Self { paginas })
    }
    pub fn paginas(&self) -> &[Pagina] { &self.paginas }
    /// Frontera de datos; sólo acepta la operación documental prefijada.
    /// No despacha peticiones ni constituye autoridad productiva R1.
    pub fn solicitud(&self, raw: &[u8]) -> Resultado<usize> {
        let v: Value = serde_json::from_slice(raw).map_err(|_| Incidencia::Solicitud)?;
        let o = v.as_object().ok_or(Incidencia::Solicitud)?;
        let a = v["arguments"].as_object().ok_or(Incidencia::Solicitud)?;
        let indice = a.get("pagina").and_then(Value::as_u64).ok_or(Incidencia::Solicitud)? as usize;
        let p = self.paginas.get(indice).ok_or(Incidencia::Solicitud)?;
        if o.len() != 2 || v["name"] != "leer_documento" || a.len() != 3
            || a.get("documento") != Some(&json!(p.documento))
            || a.get("seccion") != Some(&json!(p.seccion)) { return Err(Incidencia::Solicitud); }
        Ok(indice)
    }
    pub fn devolucion(&self, solicitada: usize, recibida: &Pagina) -> Resultado<()> {
        let esperada = self.paginas.get(solicitada).ok_or(Incidencia::Solicitud)?;
        if recibida.indice != solicitada || recibida.documento != esperada.documento
            || recibida.seccion != esperada.seccion || recibida.total != 2 {
            return Err(Incidencia::Correspondencia);
        }
        if recibida.revision != esperada.revision { return Err(Incidencia::Revision); }
        if recibida.texto.as_bytes() != esperada.texto.as_bytes() { return Err(Incidencia::Integridad); }
        Ok(())
    }
    pub fn cobertura(&self, recibidas: &[Pagina]) -> Resultado<()> {
        let indices: BTreeSet<_> = recibidas.iter().map(|p| p.indice).collect();
        if recibidas.len() != 2 || indices != BTreeSet::from([0,1]) { return Err(Incidencia::Conjunto); }
        for (i,p) in recibidas.iter().enumerate() {
            if p.indice != i { return Err(Incidencia::Conjunto); }
            self.devolucion(i,p)?;
        }
        Ok(())
    }
}

/// Recibe los bytes observados en una frontera. No afirma que procedan del motor.
/// Las pruebas utilizan un transporte sintético; no acreditan tokenización real.
pub fn cotejar_entrada(esperada: &[u8], observada: &[u8], tokens: usize) -> Resultado<()> {
    if esperada != observada { return Err(Incidencia::Entrada); }
    let total = tokens.checked_add(retroalimentacion::SALIDA + retroalimentacion::RESERVA).ok_or(Incidencia::Contexto)?;
    if total > retroalimentacion::CONTEXTO { return Err(Incidencia::Contexto); }
    Ok(())
}

/// Registro con originales completos. Una escritura fallida impide continuar.
/// El receptor debe añadir la sincronización duradera propia del soporte.
pub struct Registro<W: Write> { destino: W, anterior: String, numero: u64, cerrado: bool }
impl<W: Write> Registro<W> {
    pub fn nuevo(destino: W) -> Self { Self { destino, anterior: "0".repeat(64), numero: 0, cerrado: false } }
    pub fn conservar(&mut self, evento: &str, original: &[u8]) -> Resultado<()> {
        if self.cerrado { return Err(Incidencia::Cierre); }
        let base = json!({"numero":self.numero,"anterior":self.anterior,
            "evento":evento,"original":original});
        let sha = huella(&serde_json::to_vec(&base).map_err(|_| Incidencia::Registro)?);
        let linea = serde_json::to_vec(&json!({"base":base,"sha256":sha})).map_err(|_| Incidencia::Registro)?;
        if self.destino.write_all(&linea).and_then(|_| self.destino.write_all(b"\n"))
            .and_then(|_| self.destino.flush()).is_err() {
            self.cerrado = true; return Err(Incidencia::Registro);
        }
        self.anterior = sha; self.numero += 1; Ok(())
    }
    pub fn cerrar(&mut self) -> Resultado<()> {
        self.conservar("cierre", b"sin inferencia; transporte sintetico")?;
        self.cerrado = true; Ok(())
    }
    pub fn sello(&self) -> (u64, String) { (self.numero, self.anterior.clone()) }
    pub fn destino(self) -> W { self.destino }
}
pub fn restaurar(bytes: &[u8], sello: &(u64,String)) -> Resultado<Vec<Value>> {
    if !bytes.ends_with(b"\n") { return Err(Incidencia::Registro); }
    let mut previo = "0".repeat(64); let mut salida = Vec::new();
    for (i,linea) in bytes[..bytes.len()-1].split(|b| *b == b'\n').enumerate() {
        let v: Value = serde_json::from_slice(linea).map_err(|_| Incidencia::Registro)?;
        let base = &v["base"];
        let sha = huella(&serde_json::to_vec(base).map_err(|_| Incidencia::Registro)?);
        if base["numero"] != json!(i) || base["anterior"] != previo || v["sha256"] != sha
            || !base["original"].is_array() { return Err(Incidencia::Registro); }
        previo = sha; salida.push(base.clone());
    }
    if salida.len() as u64 != sello.0 || previo != sello.1
        || salida.last().map(|v| &v["evento"]) != Some(&json!("cierre")) { return Err(Incidencia::Registro); }
    Ok(salida)
}

/// La autenticidad de una cita sólo acredita su localización.
pub fn localizar_cita<'a>(paginas: &'a [Pagina], cita: &str) -> Resultado<&'a Pagina> {
    if cita.is_empty() { return Err(Incidencia::Salida); }
    paginas.iter().find(|p| p.texto.contains(cita)).ok_or(Incidencia::Salida)
}
pub fn presentacion_incidencia(incidencia: &Incidencia, evidencia: &str) -> String {
    format!("[!] Sin evaluación de dominio. Incidencia {incidencia:?}. Evidencia: {evidencia}")
}

/// Diagnóstico de integración: constata el estado público vacío del núcleo.
/// No convierte datos de un encargo ni un CheckResult en autoridad o permiso.
pub fn continuidad_sin_constituir() -> sv_core::authority::transitions::AuthorityContinuity {
    sv_core::authority::transitions::AuthorityContinuity::uninhabited()
}

pub mod auditoria;

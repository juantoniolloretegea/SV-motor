//! Registro duradero y cotejo determinista del recorrido documental.
use crate::{parse_strict, rpc_error, sha256, Catalog, Session};
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
pub const MAX_DIARIO: usize = 64 * 1024 * 1024;
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub fn unhex(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 || !s.is_ascii() {
        return Err("HEX_INVALIDO".into());
    }
    s.as_bytes()
        .chunks(2)
        .map(|c| {
            u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16)
                .map_err(|_| "HEX_INVALIDO".into())
        })
        .collect()
}
pub struct Diario {
    file: File,
    seq: u64,
    previous: String,
    used: usize,
    run: String,
    start: Instant,
}
impl Diario {
    pub fn create(path: &Path) -> io::Result<Self> {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        Ok(Self {
            file: options.open(path)?,
            seq: 0,
            previous: String::new(),
            used: 0,
            run: format!(
                "mcp-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(io::Error::other)?
                    .as_nanos()
            ),
            start: Instant::now(),
        })
    }
    pub fn append(&mut self, data: Value) -> io::Result<()> {
        let mut entry = json!({"ejecucion_id":self.run,"secuencia":self.seq,"anterior_sha256":self.previous,
   "unix_ns":SystemTime::now().duration_since(UNIX_EPOCH).map_err(io::Error::other)?.as_nanos().to_string(),
   "transcurrido_ns":self.start.elapsed().as_nanos().to_string(),"datos":data});
        let hash = sha256(&serde_json::to_vec(&entry)?);
        entry["sha256"] = json!(hash);
        let line = serde_json::to_vec(&entry)?;
        if self.used + line.len() + 1 > MAX_DIARIO {
            return Err(io::Error::other("Límite del diario alcanzado"));
        }
        self.file.write_all(&line)?;
        self.file.write_all(b"\n")?;
        self.file.sync_data()?;
        self.used += line.len() + 1;
        self.seq += 1;
        self.previous = hash;
        Ok(())
    }
}
/// Conserva hasta el primer byte que rebasa la cota. No interpreta ni ejecuta el resto.
pub fn observed_frame<R: BufRead>(r: &mut R) -> (Vec<u8>, Option<String>) {
    let mut out = Vec::new();
    loop {
        let chunk = match r.fill_buf() {
            Ok(c) => c,
            Err(e) => return (out, Some(format!("LECTURA: {e}"))),
        };
        if chunk.is_empty() {
            let error = if out.is_empty() {
                None
            } else {
                Some("TRAMA_INCOMPLETA".into())
            };
            return (out, error);
        }
        let end = chunk.iter().position(|b| *b == b'\n').map(|p| p + 1);
        let take = end
            .unwrap_or(chunk.len())
            .min(crate::MAX_FRAME + 1 - out.len());
        out.extend_from_slice(&chunk[..take]);
        r.consume(take);
        if out.len() > crate::MAX_FRAME {
            return (out, Some("TRAMA_SUPERIOR_AL_LIMITE".into()));
        }
        if end == Some(take) {
            return (out, None);
        }
    }
}
/// Reejecuta exclusivamente MCP, sin modelo ni red. Exige cierre normal y entrega registrada.
pub fn verify(catalog: &Catalog, catalog_hash: &str, bytes: &[u8]) -> Result<Value, String> {
    if bytes.len() > MAX_DIARIO || !bytes.ends_with(b"\n") {
        return Err("DIARIO_INCOMPLETO_O_EXCESIVO".into());
    }
    let mut session = Session::default();
    let mut previous = String::new();
    let mut run = None;
    let mut pending: Option<(usize, Option<Value>)> = None;
    let mut delivery: Option<(usize, usize)> = None;
    let mut calls = 0usize;
    let mut finished = false;
    let mut last_mono = 0u128;
    let mut entries = 0;
    for (i, line) in bytes
        .split(|b| *b == b'\n')
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        let mut entry = parse_strict(line).map_err(|e| e.to_string())?;
        let hash = entry["sha256"].as_str().ok_or("SIN_HUELLA")?.to_owned();
        entry
            .as_object_mut()
            .ok_or("ENTRADA_INVALIDA")?
            .remove("sha256");
        if sha256(&serde_json::to_vec(&entry).map_err(|e| e.to_string())?) != hash
            || entry["anterior_sha256"] != previous
            || entry["secuencia"] != i
        {
            return Err("CADENA_NO_CONFORME".into());
        }
        let identity = entry["ejecucion_id"]
            .as_str()
            .ok_or("SIN_IDENTIDAD")?
            .to_owned();
        if run.as_ref().is_some_and(|s| s != &identity) {
            return Err("IDENTIDAD_CAMBIADA".into());
        }
        run = Some(identity);
        let mono = entry["transcurrido_ns"]
            .as_str()
            .ok_or("SIN_TIEMPO")?
            .parse::<u128>()
            .map_err(|e| e.to_string())?;
        if mono < last_mono {
            return Err("TIEMPO_NO_MONOTONO".into());
        }
        last_mono = mono;
        if finished {
            return Err("EVENTO_TRAS_CIERRE".into());
        }
        let d = &entry["datos"];
        let event = d["evento"].as_str().ok_or("SIN_EVENTO")?;
        if i == 0 && event != "inicio" {
            return Err("SIN_INICIO".into());
        }
        match event {
            "inicio" => {
                if i != 0
                    || d["catalogo_sha256"] != catalog_hash
                    || d["version"] != env!("CARGO_PKG_VERSION")
                    || d["protocolo"] != crate::PROTOCOL
                {
                    return Err("BASE_NO_COINCIDENTE".into());
                }
            }
            "solicitud" => {
                if pending.is_some() || delivery.is_some() || d["trama"] != calls {
                    return Err("ORDEN_NO_CONFORME".into());
                }
                let raw = unhex(d["bytes_hex"].as_str().ok_or("SIN_BYTES")?)?;
                if sha256(&raw) != d["sha256"]
                    || !raw.ends_with(b"\n")
                    || raw.len() > crate::MAX_FRAME
                {
                    return Err("SOLICITUD_NO_CONFORME".into());
                }
                let response = match parse_strict(&raw) {
                    Ok(v) => session.handle(catalog, v),
                    Err(_) => Some(rpc_error(Value::Null, -32700, "JSON inválido")),
                };
                pending = Some((calls, response));
                calls += 1;
            }
            "resultado" => {
                let (id, response) = pending.take().ok_or("RESULTADO_SIN_SOLICITUD")?;
                if d["trama"] != id || d["respuesta"] != response.clone().unwrap_or(Value::Null) {
                    return Err("RESPUESTA_NO_RECONSTRUIBLE".into());
                }
                let wire = response
                    .map(|v| {
                        let mut b = serde_json::to_vec(&v).unwrap();
                        b.push(b'\n');
                        b
                    })
                    .unwrap_or_default();
                if d["bytes_hex"] != hex(&wire) || d["sha256"] != sha256(&wire) {
                    return Err("BYTES_RESPUESTA_NO_COINCIDENTES".into());
                }
                delivery = Some((id, wire.len()));
            }
            "entrega" => {
                let (id, len) = delivery.take().ok_or("ENTREGA_SIN_RESULTADO")?;
                if d["trama"] != id || d["bytes"] != len {
                    return Err("ENTREGA_NO_CONFORME".into());
                }
            }
            "fin" => {
                if pending.is_some()
                    || delivery.is_some()
                    || d["tramas"] != calls
                    || d["motivo"] != "entrada_cerrada"
                {
                    return Err("CIERRE_NO_CONFORME".into());
                }
                finished = true;
            }
            _ => return Err(format!("RECORRIDO_NO_CONFORME: {event}")),
        }
        previous = hash;
        entries += 1;
    }
    if !finished {
        return Err("CIERRE_AUSENTE".into());
    }
    Ok(
        json!({"estado":"conforme","alcance":"recorrido documental MCP reconstruido; no evalúa el modelo","eventos":entries,"tramas":calls,"ultima_huella":previous,"diario_sha256":sha256(bytes)}),
    )
}

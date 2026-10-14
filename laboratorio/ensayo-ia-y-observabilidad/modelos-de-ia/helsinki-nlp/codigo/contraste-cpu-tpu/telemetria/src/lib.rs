//! Medición del proceso anfitrión y cotejo de registros de CPU/TPU.
//! La coherencia de registros no autentica su origen ni acredita hardware.
#![forbid(unsafe_code)]

use selector_arbitro_director_cpu_gpu_tpu_npu::{Clase, Decision};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::time::Instant;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

pub type Resultado<T> = Result<T, String>;

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MuestraCpu {
    pub monotono_ns: u64,
    pub captura_ns: u64,
    pub pid: u32,
    pub inicio_proceso_unix_s: u64,
    /// Tiempo de CPU acumulado de todos los hilos del proceso, en ms.
    pub cpu_acumulada_ms: u64,
    pub residente_bytes: u64,
    pub lectura_acumulada_bytes: u64,
    pub escritura_acumulada_bytes: u64,
}

/// No observa argumentos, credenciales, otros procesos ni contenido de memoria.
pub struct ObservadorCpu {
    sistema: System,
    pid: Pid,
    identidad: u64,
    origen: Instant,
}

impl ObservadorCpu {
    pub fn propio() -> Resultado<Self> {
        let mut result = Self {
            sistema: System::new(),
            pid: Pid::from_u32(std::process::id()),
            identidad: 0,
            origen: Instant::now(),
        };
        result.actualizar();
        result.identidad = result
            .sistema
            .process(result.pid)
            .ok_or("Proceso no observable")?
            .start_time();
        if result.identidad == 0 {
            return Err("Identidad del proceso no disponible".into());
        }
        Ok(result)
    }

    fn actualizar(&mut self) {
        self.sistema.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[self.pid]),
            true,
            ProcessRefreshKind::nothing()
                .without_tasks()
                .with_cpu()
                .with_memory()
                .with_disk_usage(),
        );
    }

    pub fn muestra(&mut self) -> Resultado<MuestraCpu> {
        let comienzo = self.origen.elapsed().as_nanos();
        self.actualizar();
        let p = self
            .sistema
            .process(self.pid)
            .ok_or("Proceso no observable")?;
        if p.start_time() != self.identidad {
            return Err("Identidad de proceso modificada".into());
        }
        let fin = self.origen.elapsed().as_nanos();
        Ok(MuestraCpu {
            monotono_ns: u64::try_from(fin).map_err(|e| e.to_string())?,
            captura_ns: u64::try_from(fin - comienzo).map_err(|e| e.to_string())?,
            pid: self.pid.as_u32(),
            inicio_proceso_unix_s: self.identidad,
            cpu_acumulada_ms: p.accumulated_cpu_time(),
            residente_bytes: p.memory(),
            lectura_acumulada_bytes: p.disk_usage().total_read_bytes,
            escritura_acumulada_bytes: p.disk_usage().total_written_bytes,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntervaloCpu {
    pub transcurrido_ns: u64,
    pub cpu_proceso_ms: u64,
    /// Puede superar uno: equivale a núcleos ocupados medios, no a porcentaje del servidor.
    pub nucleos_medios: f64,
    pub lectura_proceso_bytes: u64,
    pub escritura_proceso_bytes: u64,
}

pub fn intervalo_cpu(a: &MuestraCpu, b: &MuestraCpu) -> Resultado<IntervaloCpu> {
    if a.pid != b.pid || a.inicio_proceso_unix_s != b.inicio_proceso_unix_s {
        return Err("No se pueden mezclar procesos".into());
    }
    let resta = |x: u64, y: u64| y.checked_sub(x).ok_or("Contador regresivo".to_string());
    let transcurrido_ns = resta(a.monotono_ns, b.monotono_ns)?;
    if transcurrido_ns == 0 {
        return Err("Intervalo nulo".into());
    }
    let cpu_proceso_ms = resta(a.cpu_acumulada_ms, b.cpu_acumulada_ms)?;
    Ok(IntervaloCpu {
        transcurrido_ns,
        cpu_proceso_ms,
        nucleos_medios: cpu_proceso_ms as f64 * 1_000_000.0 / transcurrido_ns as f64,
        lectura_proceso_bytes: resta(a.lectura_acumulada_bytes, b.lectura_acumulada_bytes)?,
        escritura_proceso_bytes: resta(a.escritura_acumulada_bytes, b.escritura_acumulada_bytes)?,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipoActividad {
    CalculoCpu,
    CalculoTpu,
    TransferenciaEntrada,
    TransferenciaSalida,
}

/// Registro normalizado por un adaptador de confianza, nunca por la IA.
/// Los bytes son transferidos según la traza, no inferidos del tamaño del tensor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actividad {
    pub id: String,
    pub solicitud: String,
    pub ejecucion: String,
    pub servidor: String,
    pub dispositivo: String,
    pub tipo: TipoActividad,
    pub reloj: String,
    pub inicio_ns: u64,
    pub fin_ns: u64,
    pub bytes_observados: Option<u64>,
    pub fuente_sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evidencia {
    pub solicitud: String,
    pub ejecucion: String,
    pub servidor: String,
    pub dispositivo: String,
    pub adaptador_revision: String,
    pub clase_ejecutada: String,
    pub programa_sha256: String,
    pub entrada_sha256: String,
    pub salida_sha256: String,
    pub referencia_sha256: String,
    /// Para esta comprobación instrumental se exige igualdad exacta de bytes.
    pub resultado_exacto: bool,
    pub cpu_inicial: MuestraCpu,
    pub cpu_final: MuestraCpu,
    pub completa: bool,
    pub actividades: Vec<Actividad>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cotejo {
    pub registros_coherentes: bool,
    pub cpu: Option<IntervaloCpu>,
    /// Unión de intervalos del mismo dispositivo y reloj; no suma de operaciones solapadas.
    pub tpu_activa_ns: Option<u64>,
    pub entrada_observada_bytes: Option<u64>,
    pub salida_observada_bytes: Option<u64>,
    pub reparos: Vec<String>,
    pub alcance: String,
}

fn huella(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|c| c.is_ascii_hexdigit())
}

/// No mezcla relojes ni dispositivos. La CPU y TPU concurrentes no se suman como latencia.
pub fn union_intervalos(mut intervals: Vec<(u64, u64)>) -> Resultado<u64> {
    if intervals.iter().any(|(a, b)| a >= b) {
        return Err("Intervalo inválido".into());
    }
    intervals.sort_unstable();
    let mut total = 0u64;
    let mut current: Option<(u64, u64)> = None;
    for (a, b) in intervals {
        match current {
            Some((s, e)) if a <= e => current = Some((s, e.max(b))),
            Some((s, e)) => {
                total = total.checked_add(e - s).ok_or("Tiempo desbordado")?;
                current = Some((a, b));
            }
            None => current = Some((a, b)),
        }
    }
    if let Some((a, b)) = current {
        total = total.checked_add(b - a).ok_or("Tiempo desbordado")?;
    }
    Ok(total)
}

/// Comprueba consistencia y completitud. La autenticidad exige conservar y analizar
/// la traza original del dispositivo: este método no puede sustituir ese adaptador.
pub fn cotejar(decision: &Decision, e: &Evidencia) -> Cotejo {
    let mut reparos = Vec::new();
    let cpu = match intervalo_cpu(&e.cpu_inicial, &e.cpu_final) {
        Ok(v) => Some(v),
        Err(err) => {
            reparos.push(err);
            None
        }
    };
    if e.solicitud != decision.solicitud
        || e.solicitud.is_empty()
        || e.ejecucion.is_empty()
        || e.servidor != decision.destino.servidor
        || e.dispositivo != decision.destino.dispositivo
        || e.adaptador_revision != decision.destino.adaptador_revision
    {
        reparos.push("Decisión y ejecución no corresponden".into());
    }
    let clase = match decision.clase {
        Clase::Cpu => "CPU",
        Clase::Tpu => "TPU",
        _ => "no admitida",
    };
    if e.clase_ejecutada != clase || clase == "no admitida" {
        reparos.push("Destino ejecutado distinto del seleccionado o clase no admitida".into());
    }
    if !e.completa {
        reparos.push("Observación incompleta".into());
    }
    for h in [
        &e.programa_sha256,
        &e.entrada_sha256,
        &e.salida_sha256,
        &e.referencia_sha256,
    ] {
        if !huella(h) {
            reparos.push("Huella ausente o inválida".into());
        }
    }
    if !e.resultado_exacto || e.salida_sha256 != e.referencia_sha256 {
        reparos.push("Resultado discordante de su referencia exacta".into());
    }
    let mut intervalos = Vec::new();
    let mut reloj_tpu: Option<&str> = None;
    let mut entrada = None::<u64>;
    let mut salida = None::<u64>;
    let mut hay_cpu = false;
    let mut ids = BTreeSet::new();
    for a in &e.actividades {
        if a.id.is_empty() || !ids.insert(&a.id) {
            reparos.push("Actividad duplicada o sin identificador".into());
            continue;
        }
        if a.solicitud != e.solicitud
            || a.ejecucion != e.ejecucion
            || a.servidor != e.servidor
            || a.dispositivo != e.dispositivo
            || a.inicio_ns >= a.fin_ns
            || a.reloj.is_empty()
            || !huella(&a.fuente_sha256)
        {
            reparos.push("Actividad sin correspondencia o procedencia válida".into());
            continue;
        }
        match a.tipo {
            TipoActividad::CalculoCpu => {
                hay_cpu = true;
            }
            TipoActividad::CalculoTpu => {
                if reloj_tpu.is_some_and(|r| r != a.reloj) {
                    reparos.push("Relojes TPU incompatibles".into());
                }
                reloj_tpu = Some(&a.reloj);
                intervalos.push((a.inicio_ns, a.fin_ns));
            }
            TipoActividad::TransferenciaEntrada | TipoActividad::TransferenciaSalida => {
                let sum = if a.tipo == TipoActividad::TransferenciaEntrada {
                    &mut entrada
                } else {
                    &mut salida
                };
                if let Some(bytes) = a.bytes_observados.filter(|v| *v > 0) {
                    match sum.unwrap_or(0).checked_add(bytes) {
                        Some(v) => *sum = Some(v),
                        None => reparos.push("Transferencia desbordada".into()),
                    }
                } else {
                    reparos.push("Transferencia sin bytes observados".into());
                }
            }
        }
    }
    if clase == "TPU" && (intervalos.is_empty() || entrada.is_none() || salida.is_none()) {
        reparos.push("Falta actividad TPU o transferencia observada".into());
    }
    if clase == "TPU" && hay_cpu {
        reparos.push("Cálculo de la operación derivado a CPU".into());
    }
    if clase == "CPU" && (!hay_cpu || !intervalos.is_empty()) {
        reparos.push("Cálculo CPU no aislado o ausente".into());
    }
    let tpu_activa_ns = if intervalos.is_empty() {
        None
    } else {
        match union_intervalos(intervalos) {
            Ok(v) => Some(v),
            Err(err) => {
                reparos.push(err);
                None
            }
        }
    };
    let coherente = reparos.is_empty();
    Cotejo {
        registros_coherentes: coherente, cpu,
        tpu_activa_ns: if coherente { tpu_activa_ns } else { None },
        entrada_observada_bytes: if coherente { entrada } else { None },
        salida_observada_bytes: if coherente { salida } else { None },
        reparos,
        alcance: "Coherencia instrumental; no acredita procedencia física de las trazas. CPU del proceso incluye control, bibliotecas y observación. Tiempo TPU no se suma al tiempo CPU ni se resta de la latencia. Adaptador de trazas original pendiente de integración.".into(),
    }
}

#[cfg(test)]
mod tests;

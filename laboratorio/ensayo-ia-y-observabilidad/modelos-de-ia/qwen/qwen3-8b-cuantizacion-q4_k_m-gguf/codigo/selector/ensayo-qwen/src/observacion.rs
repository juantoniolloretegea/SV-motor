use crate::{nuevo, R};
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, enums::device::UsedGpuMemory, Nvml};
use serde_json::{json, Value};
use std::{
    ffi::c_void,
    io::Write,
    path::Path,
    sync::mpsc,
    thread,
    time::{Duration, Instant, SystemTime},
};
pub fn cpu() -> R<Value> {
    let mut s = sysinfo::System::new();
    let pid = sysinfo::Pid::from_u32(std::process::id());
    s.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::Some(&[pid]),
        true,
        sysinfo::ProcessRefreshKind::nothing()
            .without_tasks()
            .with_cpu()
            .with_memory(),
    );
    let p = s.process(pid).ok_or("Proceso CPU no observable")?;
    Ok(
        json!({"pid":pid.as_u32(),"cpu_acumulada_ms":p.accumulated_cpu_time(),"rss_bytes":p.memory()}),
    )
}
pub struct Monitor {
    tx: mpsc::Sender<()>,
    hilo: thread::JoinHandle<R<Value>>,
}
impl Monitor {
    pub fn iniciar(d: &Path) -> R<Self> {
        let mut f = nuevo(d.join("TELEMETRIA.jsonl"))?;
        let (tx, rx) = mpsc::channel();
        let nv = Nvml::init()?;
        let total = nv.device_count()?;
        let h = thread::spawn(move || -> R<Value> {
            let mut muestras = 0;
            let mut fallos = 0;
            let mut max_intervalo = 0u128;
            let mut propio = 0u64;
            let ini = Instant::now();
            let mut anterior = ini;
            loop {
                let t = Instant::now();
                max_intervalo = max_intervalo.max(t.duration_since(anterior).as_millis());
                anterior = t;
                let mut g = vec![];
                let c = match cpu() {
                    Ok(v) => v,
                    Err(e) => {
                        fallos += 1;
                        json!({"error":e.to_string()})
                    }
                };
                for i in 0..total {
                    let r = (|| -> R<Value> {
                        let dev = nv.device_by_index(i)?;
                        let mem = dev.memory_info()?;
                        let u = dev.utilization_rates()?;
                        let ps = dev.running_compute_processes()?;
                        let p:Vec<Value>=ps.into_iter().filter(|p|p.pid==std::process::id()).map(|p|json!({"pid":p.pid,"memoria_bytes":match p.used_gpu_memory{UsedGpuMemory::Used(v)=>Some(v),UsedGpuMemory::Unavailable=>None}})).collect();
                        if !p.is_empty() {
                            propio += 1;
                        }
                        // Potencia y energía de tarjeta no se atribuyen exclusivamente a la petición.
                        let energia = dev
                            .total_energy_consumption()
                            .map(|v| json!({"valor":v}))
                            .unwrap_or_else(|e| json!({"valor":null,"error":e.to_string()}));
                        Ok(
                            json!({"indice":i,"uuid":dev.uuid()?,"nombre":dev.name()?,"memoria_total":mem.total,"memoria_usada":mem.used,"ocupacion_pct":u.gpu,"ocupacion_memoria_pct":u.memory,"potencia_mw":dev.power_usage()?,"energia_mj":energia,"temperatura_c":dev.temperature(TemperatureSensor::Gpu)?,"procesos_propios":p}),
                        )
                    })();
                    match r {
                        Ok(v) => g.push(v),
                        Err(e) => {
                            fallos += 1;
                            g.push(json!({"indice":i,"error":e.to_string()}));
                        }
                    }
                }
                serde_json::to_writer(
                    &mut f,
                    &json!({"unix_ns":SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_nanos(),"monotono_ms":ini.elapsed().as_millis(),"fase":std::env::var("SV_FASE").unwrap_or_default(),"cpu":c,"gpu":g}),
                )?;
                writeln!(f)?;
                f.flush()?;
                muestras += 1;
                match rx.recv_timeout(Duration::from_millis(250)) {
                    Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                }
            }
            f.sync_all()?;
            Ok(
                json!({"muestras":muestras,"fallos":fallos,"muestras_proceso_gpu":propio,"intervalo_maximo_ms":max_intervalo,"objetivo_ms":250,"energia":"Contador global de tarjeta; no consumo exclusivo del proceso"}),
            )
        });
        Ok(Self { tx, hilo: h })
    }
    pub fn terminar(self) -> R<Value> {
        let _ = self.tx.send(());
        self.hilo.join().map_err(|_| "Observador interrumpido")?
    }
}
type Handle = *mut c_void;
type Create = unsafe extern "C" fn(*mut Handle, u32) -> i32;
type Record = unsafe extern "C" fn(Handle, Handle) -> i32;
type Sync = unsafe extern "C" fn(Handle) -> i32;
type Elapsed = unsafe extern "C" fn(*mut f32, Handle, Handle) -> i32;
fn comprobar(c: i32) -> R<()> {
    if c == 0 {
        Ok(())
    } else {
        Err(format!("Controlador CUDA devolvió {c}").into())
    }
}
pub struct RelojGpu {
    lib: libloading::Library,
    crear: Create,
    registrar: Record,
    sincronizar: Sync,
    destruir: Sync,
    transcurrido: Elapsed,
}
pub struct Eventos<'a> {
    r: &'a RelojGpu,
    inicio: Handle,
    fin: Handle,
    stream: Handle,
}
impl RelojGpu {
    pub fn nuevo() -> R<Self> {
        // Interfaz documentada del controlador, sin biblioteca de cálculo C++.
        unsafe {
            let lib = libloading::Library::new("libcuda.so.1")?;
            Ok(Self {
                crear: *lib.get(b"cuEventCreate\0")?,
                registrar: *lib.get(b"cuEventRecord\0")?,
                sincronizar: *lib.get(b"cuEventSynchronize\0")?,
                destruir: *lib.get(b"cuEventDestroy_v2\0")?,
                transcurrido: *lib.get(b"cuEventElapsedTime\0")?,
                lib,
            })
        }
    }
    pub fn identidad(&self) -> R<Value> {
        unsafe {
            let mut dev = 0i32;
            comprobar((self.lib.get::<unsafe extern "C" fn(*mut i32) -> i32>(
                b"cuCtxGetDevice\0",
            )?)(&mut dev))?;
            let (mut mayor, mut menor) = (0, 0);
            comprobar((self.lib.get::<unsafe extern "C" fn(
                *mut i32,
                *mut i32,
                i32,
            ) -> i32>(
                b"cuDeviceComputeCapability\0"
            )?)(&mut mayor, &mut menor, dev))?;
            let mut uuid = [0u8; 16];
            comprobar((self
                .lib
                .get::<unsafe extern "C" fn(*mut u8, i32) -> i32>(
                    b"cuDeviceGetUuid_v2\0",
                )?)(uuid.as_mut_ptr(), dev))?;
            let h = uuid.iter().map(|b| format!("{b:02x}")).collect::<String>();
            Ok(
                json!({"ordinal":dev,"cc":mayor*10+menor,"uuid":format!("GPU-{}-{}-{}-{}-{}",&h[..8],&h[8..12],&h[12..16],&h[16..20],&h[20..])}),
            )
        }
    }
    pub fn iniciar(&self, stream: usize) -> R<Eventos<'_>> {
        unsafe {
            let (mut inicio, mut fin) = (std::ptr::null_mut(), std::ptr::null_mut());
            comprobar((self.crear)(&mut inicio, 0))?;
            if let Err(e) = comprobar((self.crear)(&mut fin, 0)) {
                (self.destruir)(inicio);
                return Err(e);
            }
            let e = Eventos {
                r: self,
                inicio,
                fin,
                stream: stream as Handle,
            };
            comprobar((self.registrar)(inicio, e.stream))?;
            Ok(e)
        }
    }
}
impl Eventos<'_> {
    pub fn terminar(self) -> R<f32> {
        unsafe {
            comprobar((self.r.registrar)(self.fin, self.stream))?;
            comprobar((self.r.sincronizar)(self.fin))?;
            let mut ms = 0.0;
            comprobar((self.r.transcurrido)(&mut ms, self.inicio, self.fin))?;
            if !ms.is_finite() || ms < 0.0 {
                return Err("Tiempo GPU inválido".into());
            }
            Ok(ms)
        }
    }
}
impl Drop for Eventos<'_> {
    fn drop(&mut self) {
        unsafe {
            (self.r.destruir)(self.inicio);
            (self.r.destruir)(self.fin);
        }
    }
}

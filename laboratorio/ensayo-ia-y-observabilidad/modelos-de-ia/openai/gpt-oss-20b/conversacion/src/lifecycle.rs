use super::{store, Result};
use serde_json::{json, Value};
use std::{fs::{self, File, OpenOptions}, io::Write, os::unix::fs::OpenOptionsExt,
    path::Path, sync::{Arc, Mutex}, time::Instant};

// El bloqueo dura lo mismo que el proceso y se libera también tras una terminación abrupta.
#[derive(Clone)]
pub struct Lifecycle { inner: Arc<Inner> }
struct Inner {
    _lock: File, journal: Mutex<File>, id: String, started_ms: u128,
    started: Instant, previous: Option<Value>,
}
impl Lifecycle {
    pub fn open(data: &Path) -> Result<Self> {
        let root = data.join("servicio");
        fs::create_dir_all(&root)?;
        {use std::os::unix::fs::PermissionsExt;fs::set_permissions(&root,fs::Permissions::from_mode(0o700))?;}
        let lock = OpenOptions::new().read(true).write(true).create(true).truncate(false)
            .mode(0o600).open(root.join("instancia.lock"))?;
        lock.try_lock().map_err(|_| "Ya existe una instancia que custodia este directorio")?;
        let path = root.join("ciclo.jsonl");
        if path.exists() && fs::metadata(&path)?.len()>16*1024*1024 {return Err("Registro de servicio excede su límite; se conserva sin modificación".into());}
        let previous = match fs::read_to_string(&path) {
            Ok(s) => {
                if !s.is_empty() && !s.ends_with('\n') { return Err("Registro de servicio incompleto; se conserva sin modificación".into()); }
                let mut last = None;
                for line in s.lines() { last = Some(serde_json::from_str::<Value>(line)?); }
                last.map(|v|json!({"kind":v["kind"],"utc_ms":v["utc_ms"],"instance_id":v["instance_id"]}))
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e.into()),
        };
        let journal = OpenOptions::new().create(true).append(true).mode(0o600).open(&path)?;
        File::open(&root)?.sync_all()?;
        Ok(Self { inner: Arc::new(Inner { _lock: lock, journal: Mutex::new(journal),
            id: store::id("servicio"), started_ms: store::now(), started: Instant::now(), previous }) })
    }
    pub fn event(&self, kind: &str, data: Value) -> Result<()> {
        let entry = json!({"schema":"EIO-SERVICIO-1", "kind":kind, "utc_ms":store::now(),
            "instance_id":self.inner.id, "pid":std::process::id(), "data":data});
        let mut bytes = serde_json::to_vec(&entry)?; bytes.push(b'\n');
        let mut f = self.inner.journal.lock().map_err(|_| "Registro de servicio bloqueado")?;
        if f.metadata()?.len() + bytes.len() as u64 > 16*1024*1024 { return Err("Registro de servicio lleno; no se elimina su contenido".into()); }
        f.write_all(&bytes)?; f.sync_all()?; Ok(())
    }
    pub fn ready(&self, identity: &Value, recovered: usize) -> Result<()> {
        self.event("servicio_disponible", json!({"identity":identity,"recovered_requests":recovered,
            "previous_observation":self.inner.previous,
            "previous_shutdown_confirmed":self.inner.previous.as_ref().is_some_and(|v|v["kind"]=="servicio_detenido"),
            "host_stop_cause":null,"scope":"Se registra el proceso; la causa de parada de la plataforma requiere evidencia de GitHub"}))
    }
    pub fn status(&self) -> Value { json!({"instance_id":self.inner.id,
        "started_ms":self.inner.started_ms,"uptime_seconds":self.inner.started.elapsed().as_secs(),
        "http_service":"available","model_files":"identity_verified_at_start","worker_model_check":"motor_residente_local; identidades verificadas al iniciar el servicio","host_stop_cause":null,
        "personal_authentication":"not_implemented","journal":"ciclo.jsonl"}) }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn instancia_exclusiva_y_registro_incompleto() {
        let p=std::env::temp_dir().join(store::id("eio-lifecycle"));
        let a=Lifecycle::open(&p).unwrap();
        assert!(Lifecycle::open(&p).is_err());
        a.event("servicio_detenido",json!({"cause":"prueba"})).unwrap(); drop(a);
        let b=Lifecycle::open(&p).unwrap(); assert!(b.inner.previous.is_some()); drop(b);
        let file=p.join("servicio/ciclo.jsonl");
        OpenOptions::new().append(true).open(file).unwrap().write_all(b"{").unwrap();
        assert!(Lifecycle::open(&p).is_err()); fs::remove_dir_all(p).unwrap();
    }
}


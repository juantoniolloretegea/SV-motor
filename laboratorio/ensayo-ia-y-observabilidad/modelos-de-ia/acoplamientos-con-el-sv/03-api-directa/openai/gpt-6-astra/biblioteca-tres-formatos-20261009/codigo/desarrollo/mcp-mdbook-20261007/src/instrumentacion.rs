//! Observación del proceso propio; no inspecciona procesos ajenos ni contenido documental.
#![forbid(unsafe_code)]
use serde_json::{Value, json};
pub fn muestra() -> Value {
    let status = std::fs::read_to_string("/proc/self/status").ok();
    let value = |name: &str| {
        status
            .as_ref()
            .and_then(|s| s.lines().find_map(|l| l.strip_prefix(name)))
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse::<u64>().ok())
    };
    let mut descriptors = None;
    let mut sockets = None;
    if let Ok(entries) = std::fs::read_dir("/proc/self/fd") {
        let mut n = 0;
        let mut net = 0;
        let mut complete = true;
        for entry in entries {
            match entry {
                Ok(e) => {
                    n += 1;
                    match std::fs::read_link(e.path()) {
                        Ok(p) => {
                            if p.to_string_lossy().starts_with("socket:") {
                                net += 1;
                            }
                        }
                        Err(_) => complete = false,
                    }
                }
                Err(_) => complete = false,
            }
        }
        if complete {
            descriptors = Some(n);
            sockets = Some(net);
        }
    }
    let stat = std::fs::read_to_string("/proc/self/stat").ok();
    let ticks = |idx: usize| {
        stat.as_ref()
            .and_then(|s| s.rsplit_once(')'))
            .and_then(|(_, v)| v.split_whitespace().nth(idx))
            .and_then(|v| v.parse::<u64>().ok())
    };
    json!({"pid":std::process::id(),"rss_kib":value("VmRSS:"),"hilos":value("Threads:"),"descriptores":descriptors,"sockets_propios":sockets,"cpu_usuario_ticks":ticks(11),"cpu_sistema_ticks":ticks(12),"unidad_cpu":"ticks del proceso; no se presupone frecuencia","alcance":"instantánea del proceso MCP en Linux; sin observación del proveedor"})
}

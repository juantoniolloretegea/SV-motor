use std::{fs::{self, OpenOptions}, io::Read, time::{SystemTime, UNIX_EPOCH}};
fn bounded(path: &str) -> String {
    let mut s = String::new();
    if let Ok(f) = fs::File::open(path) { let _ = f.take(131072).read_to_string(&mut s); }
    s
}
fn escaped(s: &str) -> String {
    let mut v = String::from("\"");
    for c in s.chars() { match c { '"' => v.push_str("\\\""), '\\' => v.push_str("\\\\"), '\n' => v.push_str("\\n"), '\r' => v.push_str("\\r"), '\t' => v.push_str("\\t"), c if c < ' ' => v.push_str(&format!("\\u{:04x}", c as u32)), c => v.push(c) } }
    v.push('"'); v
}
fn main() {
    let mount = bounded("/proc/self/mountinfo");
    let lines: Vec<&str> = mount.lines().filter(|l| l.contains(" - cgroup2 ")).collect();
    let mut ro = false;
    for l in &lines { let f: Vec<&str> = l.split_whitespace().collect(); if f.get(4) == Some(&"/sys/fs/cgroup") { ro = f.get(5).is_some_and(|v| v.split(',').any(|x| x == "ro")); } }
    let mut checks = Vec::new();
    for p in ["/sys/fs/cgroup/cgroup.procs", "/sys/fs/cgroup/cgroup.kill", "/sys/fs/cgroup/cgroup.subtree_control"] {
        let (ok, error) = match OpenOptions::new().write(true).open(p) { Ok(f) => { drop(f); (true, String::new()) }, Err(e) => (false, e.to_string()) };
        checks.push(format!("{{\"path\":{},\"open_write_without_writing\":{},\"error\":{}}}", escaped(p), ok, escaped(&error)));
    }
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("{{\"schema\":\"EIO-CAPACIDAD-CGROUP-01/1\",\"unix_seconds\":{},\"cgroup2_mounts\":{},\"root_mount_read_only\":{},\"checks\":[{}],\"writes_to_control_files\":0,\"cgroups_created\":0,\"inferences\":0,\"decision\":{},\"scope\":\"Inspeccion de la vista cgroup del contenedor; no acredita capacidades de un anfitrion exterior\"}}", seconds, lines.len(), ro, checks.join(","), escaped(if ro {"BLOQUEO_MONTAJE_SOLO_LECTURA"} else {"DELEGACION_Y_CONTROL_SUPERIOR_NO_ACREDITADOS"}));
}

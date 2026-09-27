
use std::{fs,path::Path,time::{Duration,Instant}};
use serde_json::json;
use sv_mcp_documental::custodia::{append,utc_ms};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();let p=Path::new(&a[1]);let start=Instant::now();
 while start.elapsed()<Duration::from_secs(1800){
 let mut values=serde_json::Map::new();
 for file in ["memory.current","memory.peak","memory.events","cpu.stat","io.stat","pids.current"]{values.insert(file.into(),json!(fs::read_to_string(format!("/sys/fs/cgroup/sv.slice/sv-minimo.slice/{file}")).ok()));}
 append(p,&json!({"utc_unix_ms":utc_ms(),"cgroup":"sv-minimo.slice","datos":values}))?;std::thread::sleep(Duration::from_secs(2));}Ok(())
}

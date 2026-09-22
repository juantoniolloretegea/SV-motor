//! Observación Linux acotada. No es un mecanismo de aislamiento ni un censo del anfitrión.
use crate::{store, telemetry::Telemetry, Result};
use serde_json::{json, Value};
use std::{collections::{HashSet,VecDeque}, fs::{self,OpenOptions}, io::{Read,Write}, os::unix::fs::{MetadataExt,OpenOptionsExt}, path::{Path,PathBuf}, process::{Command,Stdio}, time::{Duration,Instant}};
const PERIOD:u64=5;
const MAX_SECONDS:u64=86400;
const FD_LIMIT:usize=512;
fn bounded(path:impl AsRef<Path>,limit:usize)->Result<String>{let mut s=String::new();fs::File::open(path)?.take(limit as u64+1).read_to_string(&mut s)?;if s.len()>limit{return Err("Lectura excedida".into())}Ok(s)}
fn stat(pid:u32)->Result<Vec<String>>{let s=bounded(format!("/proc/{pid}/stat"),8192)?;let (_,s)=s.rsplit_once(") ").ok_or("Formato stat")?;let fields:Vec<_>=s.split_whitespace().map(str::to_owned).collect();if fields.len()<22{return Err("Stat incompleto".into())}Ok(fields)}
pub fn identity(pid:u32)->Result<u64>{Ok(stat(pid)?[19].parse()?)}
fn number(s:&str,key:&str)->Option<u64>{s.lines().find_map(|l|l.strip_prefix(key)?.split_whitespace().next()?.parse().ok())}
pub fn snapshot(pid:u32,expected:u64)->Result<Value>{
 let started=Instant::now();let a=stat(pid)?;if a[19].parse::<u64>()?!=expected{return Err("Identidad de proceso distinta".into())}
 let mut incomplete=Vec::<String>::new();
 let status=bounded(format!("/proc/{pid}/status"),32768).unwrap_or_else(|_|{incomplete.push("status".into());String::new()});
 let io=bounded(format!("/proc/{pid}/io"),4096).unwrap_or_else(|_|{incomplete.push("io".into());String::new()});
 let mut fds=0;let mut regular=HashSet::new();let mut sockets=HashSet::new();let mut blocks=0;let mut deleted_blocks=0;let mut deleted_files=0;
 match fs::read_dir(format!("/proc/{pid}/fd")){
  Ok(entries)=>for (n,entry) in entries.enumerate(){if n>=FD_LIMIT{incomplete.push("fd_limit".into());break}let entry=match entry{Ok(e)=>e,Err(_)=>{incomplete.push("fd_entry".into());continue}};fds+=1;
   match fs::read_link(entry.path()){Ok(link)=>{if let Some(n)=link.to_string_lossy().strip_prefix("socket:[").and_then(|s|s.strip_suffix(']')).and_then(|s|s.parse::<u64>().ok()){sockets.insert(n);}},Err(_)=>incomplete.push("fd_link_race_or_denied".into())}
   match fs::metadata(entry.path()){Ok(m)=>if m.is_file()&&regular.insert((m.dev(),m.ino())){let size=m.blocks().saturating_mul(512);blocks+=size;if m.nlink()==0{deleted_blocks+=size;deleted_files+=1}},Err(_)=>incomplete.push("fd_metadata_race_or_denied".into())}
  },Err(_)=>incomplete.push("fd_directory".into())
 }
 let mut socket_rows=Vec::new();let mut matched=HashSet::new();
 if !sockets.is_empty(){for protocol in ["tcp","tcp6","udp","udp6","unix"]{
  match bounded(format!("/proc/{pid}/net/{protocol}"),262144){Ok(table)=>for line in table.lines().skip(1){let f:Vec<_>=line.split_whitespace().collect();let ix=if protocol=="unix"{6}else{9};if let Some(inode)=f.get(ix).and_then(|s|s.parse::<u64>().ok()){if sockets.contains(&inode){matched.insert(inode);socket_rows.push(json!({"inode":inode,"protocol":protocol,"state_hex":f.get(if protocol=="unix"{5}else{3}),"local_port":if protocol=="unix"{None}else{f.get(1).and_then(|s|s.split_once(':')).and_then(|(_,p)|u16::from_str_radix(p,16).ok())}}));}}},Err(_)=>incomplete.push(format!("net_{protocol}"))}
 }}
 if identity(pid)?!=expected{return Err("Identidad cambió durante la lectura".into())}
 incomplete.sort();incomplete.dedup();
 Ok(json!({"pid":pid,"start_ticks":expected,"state":a[0],"ppid":a[1].parse::<u32>().ok(),"cpu_user_ticks":a[11].parse::<u64>().ok(),"cpu_system_ticks":a[12].parse::<u64>().ok(),"threads":a[17].parse::<u64>().ok(),"rss_bytes":number(&status,"VmRSS:").map(|v|v*1024),"io":{"read_bytes":number(&io,"read_bytes:"),"write_bytes":number(&io,"write_bytes:"),"read_calls":number(&io,"syscr:"),"write_calls":number(&io,"syscw:")},"descriptors_observed":fds,"regular_files_distinct":regular.len(),"open_regular_allocated_bytes":blocks,"open_unlinked_files":deleted_files,"open_unlinked_allocated_bytes":deleted_blocks,"socket_descriptors_distinct":sockets.len(),"unmatched_sockets":sockets.difference(&matched).count(),"sockets":socket_rows,"incomplete":incomplete,"read_seconds":started.elapsed().as_secs_f64()}))
}
fn tree(pid:u32,start:u64,exclude:u32)->Value{
 let now=Instant::now();let mut queue=VecDeque::from([(pid,start)]);let mut seen=HashSet::new();let mut rows=vec![];let mut incomplete=vec![];
 while let Some((pid,start))=queue.pop_front(){if pid==exclude||!seen.insert(pid){continue}if rows.len()>=16||now.elapsed()>Duration::from_millis(250){incomplete.push("tree_budget".to_string());break}
  match snapshot(pid,start){Ok(row)=>rows.push(row),Err(_)=>{incomplete.push(format!("process_{pid}_unavailable_or_changed"));continue}}
  match fs::read_dir(format!("/proc/{pid}/task")){Ok(tasks)=>for (i,task) in tasks.enumerate(){if i>=128{incomplete.push("thread_limit".into());break}let task=match task{Ok(t)=>t,Err(_)=>{incomplete.push("task_unavailable".into());continue}};match bounded(task.path().join("children"),16384){Ok(children)=>for child in children.split_whitespace().filter_map(|s|s.parse::<u32>().ok()){if queue.len()>=32{incomplete.push("queue_limit".into());break}if let Ok(start)=identity(child){queue.push_back((child,start))}else{incomplete.push("child_disappeared".into())}},Err(_)=>incomplete.push("children_unavailable".into())}},Err(_)=>incomplete.push("tasks_unavailable".into())}
 }
 incomplete.sort();incomplete.dedup();json!({"processes":rows,"incomplete":incomplete,"read_seconds":now.elapsed().as_secs_f64()})
}
fn checkpoint(path:&Path,value:&Value)->Result<()>{let tmp=path.with_extension("tmp");let mut f=OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(&tmp)?;f.write_all(&serde_json::to_vec(value)?)?;f.sync_all()?;fs::rename(tmp,path)?;Ok(())}
pub fn launch(dir:&Path)->Result<PathBuf>{
 let pid=std::process::id();let start=identity(pid)?;let out=dir.join("observador");fs::create_dir_all(&out)?;
 {use std::os::unix::fs::PermissionsExt;fs::set_permissions(&out,fs::Permissions::from_mode(0o700))?;}
 let log=OpenOptions::new().write(true).create_new(true).mode(0o600).open(out.join("diagnostico.log"))?;
 let mut child=Command::new(std::env::current_exe()?).arg("--observe").arg(pid.to_string()).arg(start.to_string()).arg(&out).stdin(Stdio::null()).stdout(Stdio::null()).stderr(log).spawn()?;
 // El observador continúa brevemente después del supervisor y se recoge si este permanece vivo.
 std::thread::spawn(move||{let _=child.wait();});Ok(out.join("estado.json"))
}
pub fn status(path:&Path)->Value{
 match bounded(path,32768).and_then(|s|Ok(serde_json::from_str::<Value>(&s)?)){
  Ok(mut v)=>{let age=v["utc_ms"].as_u64().map(|n|(store::now() as u64).saturating_sub(n));v["age_ms"]=json!(age);v["fresh"]=json!(age.is_some_and(|n|n<=20000)&&v["running"]==true&&v["export"]["ok"]==true);v},
  Err(_)=>json!({"fresh":false,"running":null,"reason":"Observador sin confirmación reciente"})
 }
}
pub fn execute()->Result<()>{let args:Vec<_>=std::env::args().collect();if args.len()!=5{return Err("Uso: --observe PID START_TICKS DIRECTORIO".into())}observe(args[2].parse()?,args[3].parse()?,Path::new(&args[4]),PERIOD,MAX_SECONDS)}
fn observe(pid:u32,start:u64,dir:&Path,period:u64,seconds:u64)->Result<()>{
 let started=Instant::now();let t=Telemetry::new(dir,&store::id("observador"),crate::telemetry::LIMIT)?;let mut samples=0u64;
 loop{
  let alive=stat(pid).map(|s|s[19].parse::<u64>().ok()==Some(start)&&s[0]!="Z").unwrap_or(false);
  let expired=started.elapsed().as_secs()>=seconds;let reason=if !alive{"proceso_ausente_o_identidad_distinta"}else if expired{"limite_temporal"}else if !t.healthy(){"exportacion_no_integra"}else{"observando"};
  let running=alive&&!expired&&t.healthy();
  if running{let data=tree(pid,start,std::process::id());let disk=nix::sys::statvfs::statvfs(dir).ok().map(|s|s.blocks_available().saturating_mul(s.fragment_size()));t.event(None,"observacion_linux",json!({"target_pid":pid,"target_start_ticks":start,"sample":samples,"tree":data,"observer":snapshot(std::process::id(),identity(std::process::id())?).ok(),"filesystem_available_bytes":disk,"clock_ticks_per_second":nix::unistd::sysconf(nix::unistd::SysconfVar::CLK_TCK).ok().flatten()}));samples+=1;
  }else{t.event(None,"observador_finalizado",json!({"target_pid":pid,"target_start_ticks":start,"reason":reason}));}
  checkpoint(&dir.join("estado.json"),&json!({"schema":"EIO-OBSERVADOR-1","pid":std::process::id(),"target_pid":pid,"target_start_ticks":start,"utc_ms":store::now(),"running":running&&t.healthy(),"reason":if t.healthy(){reason}else{"exportacion_no_integra"},"samples":samples,"period_seconds":period,"maximum_seconds":seconds,"export":t.status(),"scope":"Muestreo de este servicio y descendientes visibles. No censo del anfitrión, tráfico completo ni auditoría de llamadas al sistema."}))?;
  if !running||!t.healthy(){break}std::thread::sleep(Duration::from_secs(period));
 }t.shutdown();Ok(())
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn identidad_y_archivo_borrado_abierto(){let p=std::env::temp_dir().join(store::id("deleted"));let mut file=OpenOptions::new().write(true).create_new(true).open(&p).unwrap();file.write_all(&vec![0u8;1024*1024]).unwrap();file.sync_all().unwrap();let allocated=file.metadata().unwrap().blocks()*512;fs::remove_file(&p).unwrap();let pid=std::process::id();let start=identity(pid).unwrap();let row=snapshot(pid,start).unwrap();assert!(row["open_unlinked_allocated_bytes"].as_u64().unwrap()>=allocated);assert!(allocated>=1024*1024);assert!(snapshot(pid,start+1).is_err());}
 #[test]fn socket_escucha_atribuido_al_proceso(){let listener=std::net::TcpListener::bind("127.0.0.1:0").unwrap();let port=listener.local_addr().unwrap().port();let row=snapshot(std::process::id(),identity(std::process::id()).unwrap()).unwrap();assert!(row["sockets"].as_array().unwrap().iter().any(|v|v["protocol"]=="tcp"&&v["local_port"]==port&&v["state_hex"]=="0A"));}
 #[test]fn observador_detecta_fin_sin_declaracion_del_hijo(){let p=std::env::temp_dir().join(store::id("observer-test"));let mut child=Command::new("sleep").arg("30").spawn().unwrap();let pid=child.id();let start=identity(pid).unwrap();let out=p.clone();let thread=std::thread::spawn(move||observe(pid,start,&out,1,10).unwrap());let deadline=Instant::now()+Duration::from_secs(5);while !p.join("estado.json").exists(){assert!(Instant::now()<deadline);std::thread::sleep(Duration::from_millis(20));}child.kill().unwrap();child.wait().unwrap();thread.join().unwrap();let s=status(&p.join("estado.json"));assert_eq!(s["running"],false);assert_eq!(s["reason"],"proceso_ausente_o_identidad_distinta");fs::remove_dir_all(p).unwrap();}
}

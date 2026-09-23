//! Lectura de capacidad y límites del hijo, sin concesiones administrativas.
//! RLIMIT_AS limita direcciones virtuales por proceso. No es una cuota RSS agregada.
use std::{fs,io,os::unix::process::CommandExt,process::Command};
use serde_json::{Value,json};
pub fn snapshot()->Result<Value,String>{
 let text=fs::read_to_string("/proc/meminfo").map_err(|e|e.to_string())?;
 let get=|key:&str|text.lines().find_map(|l|l.strip_prefix(key)?.split_whitespace().next()?.parse::<u64>().ok()).and_then(|n|n.checked_mul(1024));
 let available=get("MemAvailable:").ok_or("MemAvailable ausente")?;
 let maximum=fs::read_to_string("/sys/fs/cgroup/memory.max").ok().and_then(|v|v.trim().parse::<u64>().ok());
 let current=fs::read_to_string("/sys/fs/cgroup/memory.current").ok().and_then(|v|v.trim().parse::<u64>().ok());
 let group=maximum.zip(current).map(|(m,c)|m.saturating_sub(c));
 let effective=group.map_or(available,|v|v.min(available));
 Ok(json!({"mem_available_bytes":available,"swap_free_bytes":get("SwapFree:"),"cgroup_visible_max_bytes":maximum,
 "cgroup_visible_current_bytes":current,"effective_available_bytes":effective,
 "cgroup_events":fs::read_to_string("/sys/fs/cgroup/memory.events").ok(),
 "scope":"Disponibilidad instantánea y grupo visible; no reserva exclusiva ni inventario de todos los ancestros."}))
}
pub fn admit(snapshot:&Value,minimum:u64,reserve:u64)->Result<u64,String>{
 let available=snapshot["effective_available_bytes"].as_u64().ok_or("Capacidad no observada")?;
 let ceiling=available.checked_sub(reserve).ok_or("Sin reserva para el entorno")?;
 if ceiling<minimum{return Err(format!("capacidad_insuficiente: disponible={available}; reserva={reserve}; minimo_carga={minimum}"));}
 Ok(ceiling)
}
pub fn harden(command:&mut Command,virtual_bytes:u64){
 let parent=std::process::id() as libc::pid_t;
 // Sólo llamadas al sistema sin asignación después de fork.
 unsafe {command.pre_exec(move||{
  let limit=libc::rlimit{rlim_cur:virtual_bytes as libc::rlim_t,rlim_max:virtual_bytes as libc::rlim_t};
  if libc::setrlimit(libc::RLIMIT_AS,&limit)!=0{return Err(io::Error::last_os_error());}
  let core=libc::rlimit{rlim_cur:0,rlim_max:0};
  if libc::setrlimit(libc::RLIMIT_CORE,&core)!=0{return Err(io::Error::last_os_error());}
  if libc::prctl(libc::PR_SET_PDEATHSIG,libc::SIGKILL)!=0{return Err(io::Error::last_os_error());}
  if libc::getppid()!=parent{libc::_exit(125);}
  Ok(())
 });}
}

#[cfg(test)] mod tests {
 use super::*;
 use std::{process::Stdio,thread,time::{Duration,Instant},os::unix::process::ExitStatusExt};
 const LIMIT:u64=128*1024*1024;
 fn command(mode:&str)->Command {
  let mut c=Command::new(std::env::current_exe().unwrap());
  c.args(["--exact","resources::tests::auxiliar_kernel","--nocapture"]).env("EIO_RESOURCE_TEST",mode);
  c.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());c
 }
 #[test] fn auxiliar_kernel(){
  let Ok(mode)=std::env::var("EIO_RESOURCE_TEST") else{return};
  if mode=="limite" {
   let mut r=libc::rlimit{rlim_cur:0,rlim_max:0};
   unsafe{assert_eq!(libc::getrlimit(libc::RLIMIT_AS,&mut r),0);}
   assert_eq!(r.rlim_max,LIMIT);
   let mut memory=Vec::<u8>::new();assert!(memory.try_reserve_exact(256*1024*1024).is_err());return
  }
  if mode=="padre" {
   let mut c=command("esperar");harden(&mut c,LIMIT);let child=c.spawn().unwrap();
   fs::write(std::env::var("EIO_RESOURCE_PID").unwrap(),child.id().to_string()).unwrap();
  }
  thread::sleep(Duration::from_secs(10));
 }
 #[test] fn limite_virtual_aplicado_por_linux(){
  let mut c=command("limite");c.stdout(Stdio::inherit()).stderr(Stdio::inherit());harden(&mut c,LIMIT);assert!(c.status().unwrap().success());
 }
 #[test] fn muerte_abrupta_del_padre_termina_al_hijo(){
  // Adopción sólo en el banco: permite recoger el nieto y comprobar su señal real.
  let mut prior=0i32;unsafe{assert_eq!(libc::prctl(libc::PR_GET_CHILD_SUBREAPER,&mut prior),0);assert_eq!(libc::prctl(libc::PR_SET_CHILD_SUBREAPER,1),0);}
  struct Restore(i32);impl Drop for Restore{fn drop(&mut self){unsafe{libc::prctl(libc::PR_SET_CHILD_SUBREAPER,self.0);}}}
  let _restore=Restore(prior);
  let path=std::env::temp_dir().join(format!("eio-resource-child-{}",std::process::id()));
  let mut c=command("padre");c.env("EIO_RESOURCE_PID",&path);let mut parent=c.spawn().unwrap();
  let end=Instant::now()+Duration::from_secs(3);
  let pid=loop{if let Ok(v)=fs::read_to_string(&path){if let Ok(p)=v.parse::<i32>(){break p}}
   if Instant::now()>=end{let _=parent.kill();let _=parent.wait();panic!("Hijo no creado")};thread::sleep(Duration::from_millis(10));};
  parent.kill().unwrap();assert_eq!(parent.wait().unwrap().signal(),Some(9));
  let end=Instant::now()+Duration::from_secs(3);let mut status=0;
  loop{let result=unsafe{libc::waitpid(pid,&mut status,libc::WNOHANG)};if result==pid{break}
   if Instant::now()>=end{unsafe{libc::kill(pid,libc::SIGKILL);libc::waitpid(pid,&mut status,0);}panic!("El hijo sobrevivió a su padre")}
   thread::sleep(Duration::from_millis(10));}
  fs::remove_file(path).unwrap();assert!(libc::WIFSIGNALED(status));assert_eq!(libc::WTERMSIG(status),9);
 }
 #[test] fn capacidad_insuficiente_y_ausente_se_rechazan(){
  assert!(admit(&json!({"effective_available_bytes":100}),90,20).is_err());
  assert!(admit(&json!({}),90,20).is_err());assert_eq!(admit(&json!({"effective_available_bytes":120}),90,20).unwrap(),100);
 }
}


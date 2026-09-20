//! Guarda exterior inactiva; requiere cgroup v2 delegado verificado previamente.
//! No compra/crea infraestructura ni eleva privilegios. Sólo se ejecutará tras habilitación.
use eio_candidato::nativa::*;
use std::{fs,io::{Read,Write},path::PathBuf,process::{Command,Stdio},time::{Instant,Duration},sync::mpsc};
use nix::unistd::{sysconf,SysconfVar};
struct Limpieza(fs::File);
impl Drop for Limpieza{fn drop(&mut self){let _=self.0.write_all(b"1\n");let _=self.0.flush();}}
fn poblado(p:&std::path::Path)->Result<bool,Error>{
 let s=fs::read_to_string(p.join("cgroup.events"))?;
 if s.lines().any(|x|x=="populated 0"){Ok(false)}else if s.lines().any(|x|x=="populated 1"){Ok(true)}else{Err("CGROUP_EVENTS".into())}
}
fn rss(pid:u32,page:u64)->Result<u64,Error>{
 let s=fs::read_to_string(format!("/proc/{pid}/stat"))?;let t=s.rsplit_once(") ").ok_or("STAT")?.1.split_whitespace().collect::<Vec<_>>();
 Ok(t.get(21).ok_or("RSS")?.parse::<u64>()?.checked_mul(page).ok_or("OVERFLOW")?)
}
fn main()->Result<(),Error>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=5{return Err("guarda CONFIG EVIDENCIA_NUEVA ENTRADAS CGROUP_DELEGADO_VACIO".into())}
 let c:Config=serde_json::from_slice(&fs::read(&a[1])?)?;c.comprobar()?;
 let cg=fs::canonicalize(&a[4])?;
 // Directorio preexistente, hoja vacía expresamente asignada, jamás raíz del controlador.
 if cg==PathBuf::from("/sys/fs/cgroup")||!cg.starts_with("/sys/fs/cgroup")||poblado(&cg)?{return Err("CGROUP_NO_AISLADO_VACIO".into())}
 if fs::read_to_string(cg.join("cgroup.type"))?.trim()!="domain"{return Err("CGROUP_DOMAIN".into())}
 if fs::read_dir(&cg)?.any(|e|e.ok().is_some_and(|e|e.file_type().is_ok_and(|t|t.is_dir()))){return Err("CGROUP_NO_HOJA".into())}
 // Abrir controles antes de lanzar; no cambia delegación/permisos.
 let mut kill=Limpieza(fs::OpenOptions::new().write(true).open(cg.join("cgroup.kill"))?);
 let mut membership=fs::OpenOptions::new().write(true).open(cg.join("cgroup.procs"))?;
 let exe=std::env::current_exe()?.parent().ok_or("BIN")?.join("supervisor");
 let mut child=Command::new(exe).args([a[1].clone(),a[2].clone(),a[3].clone(),std::process::id().to_string(),cg.to_string_lossy().into_owned()])
  .env_clear().stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
 // Supervisor espera ARMED: todavía no inicia servidor, inferidor ni custodio.
 let armado=(||->Result<(),Error>{
  writeln!(membership,"{}",child.id())?;membership.flush()?;
  let pids=fs::read_to_string(cg.join("cgroup.procs"))?;
  if !pids.lines().any(|p|p==child.id().to_string()){return Err("MEMBRESIA_NO_CONFIRMADA".into())}
  child.stdin.take().ok_or("PIPE")?.write_all(b"ARMED\n")?;Ok(())
 })();
 if let Err(e)=armado{let _=child.kill();let _=kill.0.write_all(b"1\n");return Err(e)}
 let mut pipe=child.stdout.take().ok_or("PIPE")?;let(tx,rx)=mpsc::sync_channel(16);
 std::thread::spawn(move||{let mut b=[0;1];while pipe.read_exact(&mut b).is_ok(){let _=tx.try_send(b[0]);}});
 let page=sysconf(SysconfVar::PAGE_SIZE)?.ok_or("PAGE")? as u64;
 let inicio=Instant::now();let mut latido=inicio;let mut muestra=inicio-Duration::from_secs(1);
 let mut reaped=false;let mut muestras=Vec::new();let motivo;
 loop{
  for b in rx.try_iter(){if b==b'.'{latido=Instant::now();}}
  if child.try_wait()?.is_some(){reaped=true;motivo="SUPERVISOR_TERMINADO";break}
  if latido.elapsed()>Duration::from_secs(2){motivo="LATIDO_AUSENTE";break}
  if inicio.elapsed()>Duration::from_secs(40*60){motivo="PLAZO_GUARDA";break}
  if muestra.elapsed()>=Duration::from_secs(1){
   let total=(||->Result<u64,Error>{let mut total=rss(std::process::id(),page)?;
    if fs::read_to_string(cg.join("cgroup.stat"))?.lines().any(|s|s.starts_with("nr_descendants ")&&s!="nr_descendants 0"){return Err("CGROUP_DESCENDIENTES_NO_PREVISTOS".into())}
    let pids=fs::read_to_string(cg.join("cgroup.procs"))?;
    if pids.lines().count()>4096{return Err("PROCESOS_LIMITE".into())}
    for p in pids.lines(){total=total.checked_add(rss(p.parse()?,page)?).ok_or("RSS_OVERFLOW")?;}Ok(total)})();
   muestras.push(serde_json::json!({"mono_ns_guarda":inicio.elapsed().as_nanos(),"rss_total":total.as_ref().ok(),"error":total.as_ref().err().map(|e|e.to_string())}));
   match total{Ok(n) if n>RSS_LIMIT=>{motivo="RSS";break},Err(_)=>{motivo="RSS_DESCONOCIDA";break},_=>{}}
   muestra=Instant::now();
  }
  std::thread::sleep(Duration::from_millis(50));
 }
 // No journal ni fsync precede a la limpieza exterior. cgroup.kill cubre grupos nuevos
 // mientras permanezcan en la hoja: cambiar process-group no sale del cgroup.
 let t=Instant::now();let accion=kill.0.write_all(b"1\n").and_then(|_|kill.0.flush());
 let mut vacio=false;while t.elapsed()<Duration::from_secs(5){
  if poblado(&cg).unwrap_or(false)==false{
   // Una lectura fallida NO acredita vacío.
   if matches!(poblado(&cg),Ok(false)){vacio=true;break}
  }std::thread::sleep(Duration::from_millis(50));
 }
 if !reaped{reaped=child.try_wait()?.is_some();}
 println!("{}",serde_json::json!({"tipo":"GUARDA_EXTERIOR","motivo":motivo,"cgroup":cg,"kill":format!("{accion:?}"),"vacio_confirmado":vacio,"supervisor_recogido":reaped,"latencia_cierre_ns":t.elapsed().as_nanos(),"civil_unix_ms":civil(),"muestras":muestras,"limite":"No contención frente a privilegios de migración, muerte de la guarda o procesos no terminables"}));
 if !vacio||!reaped||accion.is_err(){return Err("LIMPIEZA_NO_CONFIRMADA".into())}Ok(())
}

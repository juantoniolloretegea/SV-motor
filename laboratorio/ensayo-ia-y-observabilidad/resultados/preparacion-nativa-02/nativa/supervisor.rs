//! Linux exclusivamente. Toda ejecución queda pendiente de habilitación separada.
use eio_candidato::nativa::*;
use serde_json::{json,Value};
use std::{fs,io::Read,path::{Path,PathBuf},process::{Child,Command,Stdio},os::unix::{net::UnixListener,fs::PermissionsExt,process::CommandExt},sync::{Arc,atomic::{AtomicBool,Ordering},mpsc::{self,Receiver,SyncSender}},time::{Instant,Duration}};
use nix::{sys::{signal::{killpg,Signal}},unistd::{Pid,sysconf,SysconfVar}};
use eio_candidato::nativa::{custodia::Custodia,parada::{self,Escalada}};
struct Guard{child:Child,escalando:Option<Arc<AtomicBool>>}
impl Drop for Guard{fn drop(&mut self){if let Some(f)=&self.escalando{for _ in 0..20{if f.load(Ordering::SeqCst){break}std::thread::sleep(Duration::from_millis(50));}if !f.load(Ordering::SeqCst){return}}if self.child.try_wait().ok().flatten().is_none(){let _=killpg(Pid::from_raw(self.child.id() as i32),Signal::SIGKILL);for _ in 0..20{if self.child.try_wait().ok().flatten().is_some(){return}std::thread::sleep(Duration::from_millis(50));}/* Guarda exterior debe verificar residuales. */}}}
fn lanzar(bin:&Path,args:&[&str],dir:&Path)->Result<Guard,Error>{
 let child=Command::new(bin).args(args).current_dir(dir).env_clear().env("RUST_BACKTRACE","0")
 .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).process_group(0).spawn()?;
 Ok(Guard{child,escalando:None})
}
enum Pipe{Bytes(bool,Vec<u8>),Fin(bool),Fallo}
fn aviso(tx:&SyncSender<Pipe>,v:Pipe,fallo:&AtomicBool)->bool{
 if tx.try_send(v).is_err(){fallo.store(true,Ordering::SeqCst);false}else{true}
}
fn lector(mut p:impl Read+Send+'static,lineas:bool,tx:SyncSender<Pipe>,fallo:Arc<AtomicBool>){
 std::thread::spawn(move||{
  let mut buf=[0;4096];let mut linea=Vec::new();
  loop{match p.read(&mut buf){
   Ok(0)=>{if !linea.is_empty(){let _=aviso(&tx,Pipe::Bytes(true,linea),&fallo);fallo.store(true,Ordering::SeqCst);}let _=aviso(&tx,Pipe::Fin(lineas),&fallo);break},
   Ok(n)=>{if lineas{for &b in &buf[..n]{linea.push(b);if linea.len()>MAX_FRAME{let _=aviso(&tx,Pipe::Bytes(true,linea),&fallo);fallo.store(true,Ordering::SeqCst);return}
    if b==b'\n'{if !aviso(&tx,Pipe::Bytes(true,std::mem::take(&mut linea)),&fallo){return}}}}
    else if !aviso(&tx,Pipe::Bytes(false,buf[..n].to_vec()),&fallo){return}},
   Err(_)=>{if !linea.is_empty(){let _=aviso(&tx,Pipe::Bytes(true,linea),&fallo);}fallo.store(true,Ordering::SeqCst);let _=aviso(&tx,Pipe::Fallo,&fallo);break}
  }}
 });
}
#[derive(Clone)]struct Proc{pid:u32,ppid:u32,start:u64,rss:u64}
fn proc(pid:u32,page:u64)->Result<Proc,Error>{
 let s=fs::read_to_string(format!("/proc/{pid}/stat"))?;let tail=s.rsplit_once(") ").ok_or("STAT")?.1;let v:Vec<_>=tail.split_whitespace().collect();
 Ok(Proc{pid,ppid:v[1].parse()?,start:v[19].parse()?,rss:v[21].parse::<u64>()?.checked_mul(page).ok_or("RSS_OVERFLOW")?})
}
fn medir(server:u32,infer:Option<u32>,page:u64,guardian:u32,cg:&Path)->Result<Value,Error>{
 let own=std::process::id();let ids=fs::read_to_string(cg.join("cgroup.procs"))?;
 if ids.lines().count()>4096{return Err("PROC_LIMITE".into())}
 if fs::read_to_string(cg.join("cgroup.stat"))?.lines().any(|s|s.starts_with("nr_descendants ")&&s!="nr_descendants 0"){return Err("CGROUP_DESCENDIENTES_NO_PREVISTOS".into())}
 let todos=ids.lines().map(|p|proc(p.parse()?,page)).collect::<Result<Vec<_>,Error>>()?;
 if !todos.iter().any(|p|p.pid==own)||!todos.iter().any(|p|p.pid==server){return Err("PERIMETRO_INCOMPLETO".into())}
 let guardp=proc(guardian,page)?;
 let subtotal=|id:u32|todos.iter().filter(|p|p.pid==id).map(|p|p.rss).sum::<u64>();
 let extra=todos.iter().filter(|p|p.pid!=own&&p.pid!=server&&Some(p.pid)!=infer).map(|p|p.rss).sum::<u64>();
 Ok(json!({"supervisor_custodio_rss":subtotal(own),"servidor_rss":subtotal(server),
 "inferidor_rss":infer.map(subtotal),"descendientes_y_observador_rss":extra,"guarda_exterior_rss":guardp.rss,
 "total_rss":todos.iter().map(|p|p.rss).sum::<u64>()+guardp.rss,
 "procesos":todos.iter().map(|p|json!({"pid":p.pid,"ppid":p.ppid,"start_ticks":p.start,"rss":p.rss})).collect::<Vec<_>>(),
 "pss":null,"custodia":"hilos y snapshot dentro de supervisor, sin doble cómputo","metrica":"stat.rss * PAGE_SIZE; hoja cgroup completa + guarda exterior; RSS aproximada"}))
}
fn main()->Result<(),Error>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=6{return Err("supervisor CONFIG RUTA_NUEVA_EVIDENCIA ENTRADAS GUARDIAN_PID CGROUP".into())}
 let cg=fs::canonicalize(&a[5])?;
 let guardian:u32=a[4].parse()?;if nix::unistd::getppid().as_raw()!=guardian as i32{return Err("GUARDA_PADRE".into())}
 let mut permiso=[0;6];std::io::stdin().read_exact(&mut permiso)?;if &permiso!=b"ARMED\n"{return Err("GUARDA_NO_ARMADA".into())}
 let latidos=parada::latidos();
 let config:Config=serde_json::from_slice(&fs::read(&a[1])?)?;config.comprobar()?;
 let dir=PathBuf::from(&a[2]);if !dir.is_absolute()||dir.exists(){return Err("DIRECTORIO_NUEVO_ABSOLUTO".into())}
 let entradas=fs::canonicalize(&a[3])?;let exe=std::env::current_exe()?;let bins=exe.parent().ok_or("BIN")?;
 fs::create_dir(&dir)?;fs::set_permissions(&dir,fs::Permissions::from_mode(0o700))?;
 if config.modo=="testigo"&&config.caso.starts_with("bloqueo_"){nix::unistd::mkfifo(dir.join("bloqueo.fifo").as_path(),nix::sys::stat::Mode::from_bits_truncate(0o600))?;}
 let mut c=Custodia::nueva(dir.join("sellado"),if config.modo=="testigo"{config.caso.clone()}else{"normal".into()})?;let socket=dir.join("control.sock");let listener=UnixListener::bind(&socket)?;listener.set_nonblocking(true)?;
 fs::set_permissions(&socket,fs::Permissions::from_mode(0o600))?;
 let mut server=lanzar(&bins.join("servidor"),&[socket.to_str().ok_or("UTF8")?,&config.origen],&entradas)?;
 // El servidor no escribe evidencia; stdout/stderr nulos de contenido propio se drenan
 // con el mismo límite exterior. Su salida inesperada invalida la sesión.
 let (stx,srx)=mpsc::sync_channel(8);let server_fault=Arc::new(AtomicBool::new(false));
 lector(server.child.stdout.take().ok_or("PIPE")?,false,stx.clone(),server_fault.clone());
 lector(server.child.stderr.take().ok_or("PIPE")?,false,stx,server_fault.clone());
 let shutdown=Arc::new(AtomicBool::new(false));let sd=shutdown.clone();
 let rt=tokio::runtime::Runtime::new()?;
 rt.spawn(async move{use tokio::signal::unix::{signal,SignalKind};let mut term=signal(SignalKind::terminate()).expect("SIGTERM");tokio::select!{_=tokio::signal::ctrl_c()=>{},_=term.recv()=>{}};sd.store(true,Ordering::SeqCst);});
 let page=sysconf(SysconfVar::PAGE_SIZE)?.ok_or("PAGE_SIZE")? as u64;
 let mut task:Option<Guard>=None;let mut rx:Option<Receiver<Pipe>>=None;let fault=Arc::new(AtomicBool::new(false));
 let mut cierre=Cierre::default();let mut oracle=Oracle::default();let mut eof=[false;2];let mut usado=false;let mut estado="ausente";
 let mut cerrado=false;let mut admision=Admision::default();let mut escalada:Option<Escalada>=None;
 let mut fallo=false;let mut exit_ok=false;let mut reaped=false;
 let inicio=Instant::now();let mut start=inicio;let mut sample=inicio-Duration::from_secs(1);let mut muerte:Option<Instant>=None;
 let mut sellando:Option<Instant>=None;let mut latido=Instant::now()-Duration::from_secs(1);
 let id=format!("{}-01",config.campana);let mut ultimo=Value::Null;
 c.evento("inicio_supervision",json!({"id":id,"rss_umbral":RSS_LIMIT,"sondeo_ms":1000,"tick_ms":50,"modo":config.modo,"caso":config.caso}))?;
 loop{
  let mut motivo=None;
  if latido.elapsed()>=Duration::from_millis(250){let _=latidos.try_send(b'.');latido=Instant::now();}
  c.consultar();
  admision.fallo_custodia=c.fallo.load(Ordering::SeqCst);
  admision.sellado_ok=c.snapshot.is_some();
  if admision.fallo_custodia{fallo=true;motivo=Some("CUSTODIA");}
  if let Some(e)=&escalada{for v in e.eventos.try_iter(){let _=c.evento("senal",v);}}
  if sellando.is_some() && (c.snapshot.is_some()||admision.fallo_custodia){cerrado=true;estado=admision.estado();}
  if sellando.is_some_and(|t|t.elapsed()>Duration::from_secs(5))&&!cerrado{fallo=true;c.fallo.store(true,Ordering::SeqCst);admision.fallo_custodia=true;cerrado=true;estado="desconocida";}

  if shutdown.load(Ordering::SeqCst)||inicio.elapsed()>=Duration::from_secs(40*60){motivo=Some("CIERRE_SESION")}
  if server.child.try_wait()?.is_some(){motivo=Some("SERVIDOR_TERMINADO")}
  if server_fault.load(Ordering::SeqCst)||srx.try_iter().any(|p|matches!(p,Pipe::Bytes(_,_)|Pipe::Fallo)){motivo=Some("SERVIDOR_SALIDA_INESPERADA")}
  if usado&&!cerrado&&start.elapsed()>=Duration::from_secs(120){motivo=Some("TIEMPO")}
  if sample.elapsed()>=Duration::from_secs(1)&&!cerrado{
   let t=Instant::now();match medir(server.child.id(),task.as_ref().filter(|_|!reaped).map(|g|g.child.id()),page,guardian,&cg){
    Ok(m)=>{if m["total_rss"].as_u64().ok_or("RSS")?>RSS_LIMIT{motivo=Some("RSS")}ultimo=m;
     if c.evento("memoria",json!({"medida":ultimo,"coste_observacion_ns":t.elapsed().as_nanos()})).is_err(){fallo=true;motivo=Some("ESCRITURA")}},
    Err(e)=>{fallo=true;motivo=Some("MEMORIA_DESCONOCIDA");let _=c.evento("error_memoria",json!({"error":e.to_string()}));}
   }sample=Instant::now();
  }
  if fault.load(Ordering::SeqCst){fallo=true;motivo=Some("COLA_O_LECTURA")}
  if let Some(r)=&rx{for p in r.try_iter().take(64){match p{
   Pipe::Bytes(out,b)=>{let dentro=c.fase=="recepcion"||c.fase=="drenaje";if c.escribir(if out{0}else{1},&b).is_err(){fallo=true;motivo=Some("ESCRITURA")}
    if out&&dentro{if oracle.recibir(&b,&id).is_err(){fallo=true;motivo=Some("SECUENCIA")}
     if cierre.revocada{let _=c.evento("resultado_o_evento_tras_revocacion",json!({"bytes":b.len(),"admisible":false}));}}},
   Pipe::Fin(out)=>eof[if out{0}else{1}]=true,
   Pipe::Fallo=>{fallo=true;motivo=Some("LECTURA")}
  }}}
  // Se atiende cancelación ANTES de comprometer un cierre en esta iteración.
  if let Ok((mut s,_))=listener.accept(){
   s.set_read_timeout(Some(Duration::from_millis(100)))?;s.set_write_timeout(Some(Duration::from_millis(100)))?;
   let reply=match recv::<Orden>(&mut s,MAX_INPUT){
    Ok(Orden::Estado)=>{
     admision.revocada=cierre.revocada;admision.fallo_observacion=fallo||fault.load(Ordering::SeqCst);
     let admitido=admision.admisible();
     json!({"id":id,"pid_supervisor":std::process::id(),"pid_inferidor":task.as_ref().map(|g|g.child.id()),"estado":if cerrado{admision.estado()}else{estado},"admisible":admitido,"condiciones":admision,
     "fase_custodia":c.fase,"sello":c.snapshot.as_ref().filter(|_|!c.fallo.load(Ordering::SeqCst)).map(|s|&s.sello),"evidencia_parcial":c.snapshot.as_ref().map(|_|!admitido),
     "tardios_no_custodiados_bytes":c.tardios_bytes,"memoria":ultimo,"resultado":if admitido{oracle.resultado.clone()}else{None}})
    },
    Ok(Orden::Cancelar{id:pedido})=>{
     if pedido!=id||!usado||sellando.is_some()||!cierre.cancelar(){json!({"error":"NO_CANCELABLE"})}
     else{motivo=Some("CANCELACION");json!({"id":id,"estado":"cancelacion_solicitada","parada_confirmada":false})}
    },
    Ok(Orden::Iniciar{contrato,peticion,texto})=>{
     if usado||cerrado||fallo{json!({"error":"SESION_CONSUMIDA_O_BLOQUEADA"})}
     else if let Err(e)=validar_inicio(&contrato,&peticion,&texto){json!({"error":e})}
     else{
      usado=true;start=Instant::now();
      let prepare=(||->Result<(),Error>{
       c.entrada(texto.as_bytes())?;
       c.evento("entrada",json!({"id":id,"peticion":peticion,"bytes":texto.len(),"sha256":hash(texto.as_bytes())}))?;
       let args=if config.modo=="modelo"{vec![id.as_str(),peticion.as_str()]}else{vec![id.as_str(),config.caso.as_str()]};
       let mut g=lanzar(&bins.join(if config.modo=="modelo"{"inferidor"}else{"testigo"}),&args,&entradas)?;
       let (tx,r)=mpsc::sync_channel(64);lector(g.child.stdout.take().ok_or("PIPE")?,true,tx.clone(),fault.clone());lector(g.child.stderr.take().ok_or("PIPE")?,false,tx,fault.clone());
       task=Some(g);rx=Some(r);Ok(())
      })();
      match prepare{Ok(())=>{estado="activa";json!({"id":id,"estado":estado})},
       Err(e)=>{fallo=true;cerrado=true;estado="desconocida";let _=c.evento("fallo_inicio",json!({"error":e.to_string()}));json!({"error":"INICIO","estado":estado})}}
     }
    },
    Ok(Orden::Evidencia{archivo,offset,sello})=>{
     match &c.snapshot{Some(_) if c.fallo.load(Ordering::SeqCst)=>json!({"error":"SELLO_INVALIDADO"}),Some(x)=>x.fragmento(&archivo,offset,&sello),None=>json!({"error":"CONJUNTO_NO_SELLADO"})}
    },
    Err(_)=>json!({"error":"ESTRUCTURA"})
   };let _=send(&mut s,&reply);
  }
  if let Some(reason)=motivo{
   if let Some(g)=&mut task{if !reaped&&muerte.is_none(){cierre.cancelar();estado="cancelacion_solicitada";let e=parada::iniciar(g.child.id());g.escalando=Some(e.terminada.clone());escalada=Some(e);let _=c.evento("revocacion",json!({"motivo":reason,"pid":g.child.id()}));muerte=Some(Instant::now());}}
   else{fallo=true;}
   if reason=="CIERRE_SESION"||reason=="SERVIDOR_TERMINADO"||reason=="SERVIDOR_SALIDA_INESPERADA"{shutdown.store(true,Ordering::SeqCst)}
  }
  if let Some(g)=&mut task{
   if !reaped&&g.escalando.as_ref().is_none_or(|x|x.load(Ordering::SeqCst)){if let Some(s)=g.child.try_wait()?{reaped=true;exit_ok=s.success();c.fase="drenaje";muerte.get_or_insert(Instant::now());c.evento("waitpid",json!({"status":s.to_string(),"pid":g.child.id(),"reaped":true}))?;}}
   if reaped&&sellando.is_none()&&(eof.iter().all(|x|*x)||muerte.is_some_and(|t|t.elapsed()>=Duration::from_secs(2))){
    if let Some(e)=&escalada{for v in e.eventos.try_iter(){let _=c.evento("senal",v);}}
    // Barrera FIFO: todos los trabajos ya aceptados preceden a Sellar.
    // Se cierra recepción antes de encolar la barrera; posteriores sólo contabilizan laguna.
    admision=Admision{proceso_ok:exit_ok,eof_completos:eof.iter().all(|x|*x),
     secuencia_completa:oracle.completa(),revocada:cierre.revocada,
     fallo_observacion:fallo||fault.load(Ordering::SeqCst),fallo_custodia:c.fallo.load(Ordering::SeqCst),
     oraculo_ok:oracle.completa(),sellado_ok:false};
    cierre.cerrada=true;sellando=Some(Instant::now());estado="sellando";
    if c.cerrar(admision.clone(),!admision.eof_completos).is_err(){fallo=true;admision.fallo_custodia=true;}
   }
   if !reaped&&muerte.is_some_and(|t|t.elapsed()>Duration::from_secs(5)){fallo=true;estado="parada_no_confirmada";let _=c.evento("parada_no_confirmada",json!({"pid":g.child.id()}));break}
  }
  if shutdown.load(Ordering::SeqCst)&&(task.is_none()||cerrado){break}
  std::thread::sleep(Duration::from_millis(50));
 }
 // Sin reutilización de sesión. Drop intenta SIGKILL y try_wait durante 1 s; no espera indefinida.
 // El control de parada de Codespaces queda exterior y pendiente de prueba.
 // Sin escritura síncrona a stdout/stderr en el hilo de control al cerrar.
 let _=latidos.try_send(b'F');
 drop(task);drop(server);Ok(())
}

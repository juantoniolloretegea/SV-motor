//! Linux exclusivamente. Toda ejecución queda pendiente de habilitación separada.
use eio_candidato::nativa::*;
use serde_json::{json,Value};
use std::{fs::{self,File,OpenOptions},io::{Read,Write,Seek,SeekFrom},path::{Path,PathBuf},process::{Child,Command,Stdio},os::unix::{net::UnixListener,fs::PermissionsExt,process::CommandExt},sync::{Arc,atomic::{AtomicBool,Ordering},mpsc::{self,Receiver,SyncSender}},time::{Instant,Duration}};
use nix::{sys::{signal::{killpg,Signal},statvfs::statvfs},unistd::{Pid,sysconf,SysconfVar}};
use sha2::{Digest,Sha256};
const DISK_MARGIN:u64=2*1024*1024*1024;
const MAX_FILE:u64=8*1024*1024;
const MAX_TOTAL:u64=20*1024*1024;
struct Guard{child:Child}
impl Drop for Guard{fn drop(&mut self){if self.child.try_wait().ok().flatten().is_none(){let _=killpg(Pid::from_raw(self.child.id() as i32),Signal::SIGKILL);for _ in 0..20{if self.child.try_wait().ok().flatten().is_some(){return}std::thread::sleep(Duration::from_millis(50));}eprintln!("PARADA_NO_CONFIRMADA pid={}",self.child.id());}}}
fn lanzar(bin:&Path,args:&[&str],dir:&Path)->Result<Guard,Error>{
 let child=Command::new(bin).args(args).current_dir(dir).env_clear().env("RUST_BACKTRACE","0")
 .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).process_group(0).spawn()?;
 Ok(Guard{child})
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
struct Custodia{dir:PathBuf,total:u64,bytes:[u64;3],files:[File;3],write_ns:u128,writes:u64,seq:u64,inicio:Instant,fallo_inyectado:bool}
impl Custodia{
 fn nueva(dir:&Path)->Result<Self,Error>{
  fs::create_dir(dir)?;fs::set_permissions(dir,fs::Permissions::from_mode(0o700))?;
  let abrir=|p:&str|OpenOptions::new().write(true).create_new(true).open(dir.join(p));
  Ok(Self{dir:dir.into(),total:0,bytes:[0;3],files:[abrir(FILES[0])?,abrir(FILES[1])?,abrir(FILES[2])?],write_ns:0,writes:0,seq:0,inicio:Instant::now(),fallo_inyectado:false})
 }
 fn escribir(&mut self,i:usize,b:&[u8])->Result<(),Error>{
  if self.fallo_inyectado && i==0 && self.bytes[0]>0{self.fallo_inyectado=false;return Err("ESCRITURA_INYECTADA".into())}
  let n=b.len() as u64;if self.total+n>MAX_TOTAL||self.bytes[i]+n>MAX_FILE{return Err("CUSTODIA_LIMITE".into())}
  let v=statvfs(self.dir.as_path())?;if v.blocks_available()*v.fragment_size()<DISK_MARGIN{return Err("DISCO_SIN_MARGEN".into())}
  let t=Instant::now();self.files[i].write_all(b)?;self.files[i].sync_data()?;
  self.write_ns+=t.elapsed().as_nanos();self.writes+=1;self.total+=n;self.bytes[i]+=n;Ok(())
 }
 fn evento(&mut self,tipo:&str,datos:Value)->Result<(),Error>{
  self.seq+=1;let mut b=serde_json::to_vec(&json!({"seq":self.seq,"tipo":tipo,"datos":datos,"civil_unix_ms":civil(),"mono_ns_supervisor":self.inicio.elapsed().as_nanos(),"escrituras_previas":self.writes,"ns_escritura_previos":self.write_ns,"bytes_previos":self.total}))?;b.push(b'\n');self.escribir(2,&b)
 }
 fn cerrar(&mut self,estado:&str,completa:bool)->Result<(),Error>{
  self.evento("cierre",json!({"estado":estado,"evidencia_completa_segun_oraculo":completa}))?;
  let mut items=Vec::new();
  for nombre in &FILES[..4]{let p=self.dir.join(nombre);if !p.exists(){continue}
   let mut f=File::open(p)?;let mut h=Sha256::new();let mut n=0;let mut b=[0;65536];
   loop{let z=f.read(&mut b)?;if z==0{break}n+=z;h.update(&b[..z]);}
   items.push(json!({"archivo":nombre,"bytes":n,"sha256":format!("{:x}",h.finalize())}));
  }
  // El manifiesto no se auto-hashea. Incluye coste exacto del cierre del journal,
  // pero no el coste de su propia escritura/hash, que se declara excluido.
  let b=serde_json::to_vec_pretty(&json!({"contrato":"EIO-NAT/1","estado":estado,"completa":completa,"archivos":items,"coste_escritura_ns":self.write_ns,"escrituras":self.writes,"excluido":"coste propio del manifiesto; recuperación independiente pendiente"}))?;
  let mut out=OpenOptions::new().write(true).create_new(true).open(self.dir.join(FILES[4]))?;out.write_all(&b)?;out.sync_all()?;Ok(())
 }
}
#[derive(Clone)]struct Proc{pid:u32,ppid:u32,start:u64,rss:u64}
fn proc(pid:u32,page:u64)->Result<Proc,Error>{
 let s=fs::read_to_string(format!("/proc/{pid}/stat"))?;let tail=s.rsplit_once(") ").ok_or("STAT")?.1;let v:Vec<_>=tail.split_whitespace().collect();
 Ok(Proc{pid,ppid:v[1].parse()?,start:v[19].parse()?,rss:v[21].parse::<u64>()?.checked_mul(page).ok_or("RSS_OVERFLOW")?})
}
fn medir(server:u32,infer:Option<u32>,page:u64)->Result<Value,Error>{
 let own=std::process::id();let mut todos=Vec::new();
 let mut encontrados=0;
 for x in fs::read_dir("/proc")?{let x=x?;if let Ok(pid)=x.file_name().to_string_lossy().parse::<u32>(){encontrados+=1;if encontrados>4096{return Err("PROC_LIMITE".into())}if let Ok(p)=proc(pid,page){todos.push(p)}}}
 if todos.len()>4096{return Err("PROC_LIMITE".into())}
 let mut familia=vec![own,server];if let Some(p)=infer{familia.push(p)}
 loop{let n=familia.len();for p in &todos{if familia.contains(&p.ppid)&&!familia.contains(&p.pid){familia.push(p.pid)}}if n==familia.len(){break}}
 let ownp=proc(own,page)?;let sp=proc(server,page)?;let ip=if let Some(p)=infer{Some(proc(p,page)?)}else{None};
 let extra=todos.iter().filter(|p|familia.contains(&p.pid)&&p.pid!=own&&p.pid!=server&&Some(p.pid)!=infer).map(|p|p.rss).sum::<u64>();
 Ok(json!({"supervisor_custodio_rss":ownp.rss,"servidor_rss":sp.rss,"inferidor_rss":ip.as_ref().map(|p|p.rss),"descendientes_rss":extra,
 "total_rss":ownp.rss+sp.rss+ip.as_ref().map(|p|p.rss).unwrap_or(0)+extra,
 "procesos":todos.iter().filter(|p|familia.contains(&p.pid)).map(|p|json!({"pid":p.pid,"start_ticks":p.start,"rss":p.rss})).collect::<Vec<_>>(),
 "pss":null,"custodia":"hilos incluidos en supervisor; no doble cómputo","metrica":"stat.rss * PAGE_SIZE; RSS aproximada, páginas compartidas sumadas"}))
}
fn stop(g:&mut Guard,c:&mut Custodia,motivo:&str){
 let t=Instant::now();let pid=Pid::from_raw(g.child.id() as i32);
 let term=killpg(pid,Signal::SIGTERM);
 let _=c.evento("revocacion",json!({"motivo":motivo,"pid":g.child.id(),"sigterm":format!("{term:?}")}));
 std::thread::sleep(Duration::from_millis(250));
 let k=killpg(pid,Signal::SIGKILL);
 let _=c.evento("sigkill",json!({"resultado":format!("{k:?}"),"latencia_ns_desde_revocacion":t.elapsed().as_nanos()}));
}
fn main()->Result<(),Error>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=4{return Err("supervisor CONFIG RUTA_NUEVA_EVIDENCIA ENTRADAS".into())}
 let config:Config=serde_json::from_slice(&fs::read(&a[1])?)?;config.comprobar()?;
 let dir=PathBuf::from(&a[2]);if !dir.is_absolute()||dir.exists(){return Err("DIRECTORIO_NUEVO_ABSOLUTO".into())}
 let entradas=fs::canonicalize(&a[3])?;let exe=std::env::current_exe()?;let bins=exe.parent().ok_or("BIN")?;
 let mut c=Custodia::nueva(&dir)?;c.fallo_inyectado=config.modo=="testigo"&&config.caso=="escritura";let socket=dir.join("control.sock");let listener=UnixListener::bind(&socket)?;listener.set_nonblocking(true)?;
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
 let mut cerrado=false;let mut fallo=false;let mut exit_ok=false;let mut reaped=false;
 let inicio=Instant::now();let mut start=inicio;let mut sample=inicio-Duration::from_secs(1);let mut muerte:Option<Instant>=None;
 let id=format!("{}-01",config.campana);let mut ultimo=Value::Null;
 c.evento("inicio_supervision",json!({"id":id,"rss_umbral":RSS_LIMIT,"sondeo_ms":1000,"tick_ms":50,"modo":config.modo,"caso":config.caso}))?;
 loop{
  let mut motivo=None;
  if shutdown.load(Ordering::SeqCst)||inicio.elapsed()>=Duration::from_secs(40*60){motivo=Some("CIERRE_SESION")}
  if server.child.try_wait()?.is_some(){motivo=Some("SERVIDOR_TERMINADO")}
  if server_fault.load(Ordering::SeqCst)||srx.try_iter().any(|p|matches!(p,Pipe::Bytes(_,_)|Pipe::Fallo)){motivo=Some("SERVIDOR_SALIDA_INESPERADA")}
  if usado&&!cerrado&&start.elapsed()>=Duration::from_secs(120){motivo=Some("TIEMPO")}
  if sample.elapsed()>=Duration::from_secs(1)&&!cerrado{
   let t=Instant::now();match medir(server.child.id(),task.as_ref().filter(|_|!reaped).map(|g|g.child.id()),page){
    Ok(m)=>{if m["total_rss"].as_u64().ok_or("RSS")?>RSS_LIMIT{motivo=Some("RSS")}ultimo=m;
     if c.evento("memoria",json!({"medida":ultimo,"coste_observacion_ns":t.elapsed().as_nanos()})).is_err(){fallo=true;motivo=Some("ESCRITURA")}},
    Err(e)=>{fallo=true;motivo=Some("MEMORIA_DESCONOCIDA");let _=c.evento("error_memoria",json!({"error":e.to_string()}));}
   }sample=Instant::now();
  }
  if fault.load(Ordering::SeqCst){fallo=true;motivo=Some("COLA_O_LECTURA")}
  if let Some(r)=&rx{for p in r.try_iter().take(64){match p{
   Pipe::Bytes(out,b)=>{if c.escribir(if out{0}else{1},&b).is_err(){fallo=true;motivo=Some("ESCRITURA")}
    if out{if oracle.recibir(&b,&id).is_err(){fallo=true;motivo=Some("SECUENCIA")}
     if cierre.revocada{let _=c.evento("resultado_o_evento_tras_revocacion",json!({"bytes":b.len(),"admisible":false}));}}},
   Pipe::Fin(out)=>eof[if out{0}else{1}]=true,
   Pipe::Fallo=>{fallo=true;motivo=Some("LECTURA")}
  }}}
  // Se atiende cancelación ANTES de comprometer un cierre en esta iteración.
  if let Ok((mut s,_))=listener.accept(){
   s.set_read_timeout(Some(Duration::from_millis(100)))?;s.set_write_timeout(Some(Duration::from_millis(100)))?;
   let reply=match recv::<Orden>(&mut s,MAX_INPUT){
    Ok(Orden::Estado)=>json!({"id":id,"estado":estado,"revocada":cierre.revocada,"evidencia_cerrada":cerrado,"memoria":ultimo,"resultado":if cerrado&&!cierre.revocada&&!fallo{oracle.resultado.clone()}else{None}}),
    Ok(Orden::Cancelar{id:pedido})=>{
     if pedido!=id||!usado||!cierre.cancelar(){json!({"error":"NO_CANCELABLE"})}
     else{motivo=Some("CANCELACION");json!({"id":id,"estado":"cancelacion_solicitada","parada_confirmada":false})}
    },
    Ok(Orden::Iniciar{contrato,peticion,texto})=>{
     if usado||cerrado||fallo{json!({"error":"SESION_CONSUMIDA_O_BLOQUEADA"})}
     else if let Err(e)=validar_inicio(&contrato,&peticion,&texto){json!({"error":e})}
     else{
      usado=true;start=Instant::now();
      let prepare=(||->Result<(),Error>{
       let mut input=OpenOptions::new().write(true).create_new(true).open(dir.join("entrada.txt"))?;input.write_all(texto.as_bytes())?;input.sync_all()?;
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
    Ok(Orden::Evidencia{archivo,offset})=>{
     if !cerrado||!FILES.contains(&archivo.as_str()){json!({"error":"EVIDENCIA_NO_DISPONIBLE"})}
     else{let read=(||->Result<Value,Error>{let mut f=File::open(dir.join(&archivo))?;let n=f.metadata()?.len();if offset>n{return Err("OFFSET".into())}f.seek(SeekFrom::Start(offset))?;let mut b=vec![0;32768];let z=f.read(&mut b)?;b.truncate(z);Ok(json!({"archivo":archivo,"offset":offset,"total":n,"hex":b.iter().map(|v|format!("{v:02x}")).collect::<String>(),"fin":offset+z as u64==n}))})();
      read.unwrap_or_else(|e|json!({"error":e.to_string()}))}
    },
    Err(_)=>json!({"error":"ESTRUCTURA"})
   };let _=send(&mut s,&reply);
  }
  if let Some(reason)=motivo{
   if let Some(g)=&mut task{if !reaped&&muerte.is_none(){cierre.cancelar();estado="cancelacion_solicitada";stop(g,&mut c,reason);muerte=Some(Instant::now());}}
   else{fallo=true;}
   if reason=="CIERRE_SESION"||reason=="SERVIDOR_TERMINADO"||reason=="SERVIDOR_SALIDA_INESPERADA"{shutdown.store(true,Ordering::SeqCst)}
  }
  if let Some(g)=&mut task{
   if !reaped{if let Some(s)=g.child.try_wait()?{reaped=true;exit_ok=s.success();muerte.get_or_insert(Instant::now());c.evento("waitpid",json!({"status":s.to_string(),"pid":g.child.id(),"reaped":true}))?;}}
   if reaped&&!cerrado&&(eof.iter().all(|x|*x)||muerte.is_some_and(|t|t.elapsed()>=Duration::from_secs(2))){
    let complete=!fallo&&!cierre.revocada&&exit_ok&&eof.iter().all(|x|*x)&&oracle.completa();
    estado=cierre.concluir(exit_ok,eof.iter().all(|x|*x),!fallo,oracle.completa());
    if c.cerrar(estado,complete).is_err(){fallo=true;estado="desconocida";}
    cerrado=true;
   }
   if !reaped&&muerte.is_some_and(|t|t.elapsed()>Duration::from_secs(5)){fallo=true;estado="parada_no_confirmada";let _=c.evento("parada_no_confirmada",json!({"pid":g.child.id()}));break}
  }
  if shutdown.load(Ordering::SeqCst)&&(task.is_none()||cerrado){break}
  std::thread::sleep(Duration::from_millis(50));
 }
 // Sin reutilización de sesión. Drop intenta SIGKILL y try_wait durante 1 s; no espera indefinida.
 // El control de parada de Codespaces queda exterior y pendiente de prueba.
 eprintln!("{}",json!({"estado":estado,"fallo":fallo,"evidencia":dir,"cierre":"salida del supervisor; comprobar plataforma separadamente"}));
 drop(task);drop(server);Ok(())
}

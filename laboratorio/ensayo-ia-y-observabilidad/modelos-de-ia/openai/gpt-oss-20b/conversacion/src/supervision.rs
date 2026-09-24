//! Control del hijo independiente del registro, la API y OpenTelemetry.
//! RSS muestreada: no es una cuota agregada ni la guarda exterior de NAT03.
use std::{fs, process::{Child, ExitStatus}, os::unix::process::ExitStatusExt,
    sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc},
    thread, time::{Duration, Instant}};
use serde::Serialize;

#[derive(Clone, Copy)]
pub struct Limits { pub time:Duration, pub rss:u64, pub output:usize }
#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub cause:Option<String>, pub exit_code:Option<i32>, pub exit_signal:Option<i32>,
    pub peak_rss_bytes:Option<u64>, pub rss_samples:u64, pub rss_unavailable:u64,
    pub stop_confirmed:bool, pub error:Option<String>,
}
pub struct Control {
    pub output:Arc<AtomicUsize>, pub fault:Arc<AtomicBool>,
    cancel:Arc<AtomicBool>, done:mpsc::Receiver<Report>, completed:Arc<AtomicBool>,
}
impl Control {
    pub fn start(mut child:Child, limits:Limits, cancel:Arc<AtomicBool>) -> Self {
        Self::start_managed(child,limits,cancel,false)
    }
    pub fn start_managed(mut child:Child, limits:Limits, cancel:Arc<AtomicBool>, engine:bool) -> Self {
        let output=Arc::new(AtomicUsize::new(0)); let bytes=output.clone();
        let fault=Arc::new(AtomicBool::new(false)); let failed=fault.clone();
        let stop=cancel.clone(); let (tx,done)=mpsc::channel();
        let completed=Arc::new(AtomicBool::new(false));let ended=completed.clone();
        thread::spawn(move|| {
            let started=Instant::now(); let mut missing_since=None;
            let mut report=Report{cause:None,exit_code:None,exit_signal:None,peak_rss_bytes:None,
                rss_samples:0,rss_unavailable:0,stop_confirmed:false,error:None};
            let mut requested=None;
            loop {
                match child.try_wait() {
                    Ok(Some(s))=>{finish(&mut report,s);break},
                    Err(e)=>{report.error=Some(e.to_string());report.cause.get_or_insert("fallo_control".into());},
                    Ok(None)=>{}
                }
                if requested.is_none() {
                    let sample=fs::read_to_string(format!("/proc/{}/status",child.id())).ok()
                        .and_then(|s|s.lines().find_map(|l|l.strip_prefix("VmRSS:")?
                        .split_whitespace().next()?.parse::<u64>().ok()).and_then(|v|v.checked_mul(1024)));
                    if let Some(rss)=sample {
                        report.peak_rss_bytes=Some(report.peak_rss_bytes.unwrap_or(0).max(rss));
                        report.rss_samples+=1; missing_since=None;
                    } else {
                        report.rss_unavailable+=1; missing_since.get_or_insert(Instant::now());
                    }
                    let cause=if stop.load(Ordering::SeqCst){Some("cancelada")}
                        else if started.elapsed()>=limits.time{Some("limite_tiempo")}
                        else if report.peak_rss_bytes.is_some_and(|v|v>limits.rss){Some("limite_memoria")}
                        else if bytes.load(Ordering::SeqCst)>limits.output{Some("limite_salida")}
                        else if failed.load(Ordering::SeqCst){Some("fallo_conservacion_o_lectura")}
                        else if missing_since.is_some_and(|s:Instant|s.elapsed()>Duration::from_secs(1)){Some("memoria_no_observable")}
                        else if report.error.is_some(){Some("fallo_control")}else{None};
                    if let Some(cause)=cause {
                        report.cause=Some(cause.into());
                        if let Err(e)=child.kill(){report.error=Some(e.to_string());}
                        requested=Some(Instant::now());
                    }
                } else if requested.is_some_and(|s:Instant|s.elapsed()>Duration::from_secs(2)) {
                    report.error=Some("Terminación del hijo no confirmada dentro del plazo de recogida".into());break
                }
                thread::sleep(Duration::from_millis(20));
            }
            if engine && (report.cause.is_some() || report.exit_code!=Some(0)) {if let Err(e)=crate::model::stop_engine(){report.stop_confirmed=false;report.error=Some(e.to_string());}}
            let _=tx.send(report);ended.store(true,Ordering::SeqCst);
        });
        Self {output,fault,cancel,done,completed}
    }
    pub fn completed(&self)->bool{self.completed.load(Ordering::SeqCst)}
    pub fn finish(&self)->std::result::Result<Report,String> {
        self.done.recv_timeout(Duration::from_secs(20)).map_err(|e|format!("Cierre no confirmado: {e}"))
    }
}
impl Drop for Control {fn drop(&mut self){self.cancel.store(true,Ordering::SeqCst);}}
fn finish(r:&mut Report,s:ExitStatus){r.exit_code=s.code();r.exit_signal=s.signal();r.stop_confirmed=true;}

#[cfg(test)] mod tests {
 use super::*;
 use std::{process::{Command,Stdio},io::Write};
 #[test] fn auxiliar(){
  let Ok(mode)=std::env::var("EIO_SUPERVISION_AUXILIAR") else{return};
  if mode=="senal"{nix::sys::signal::kill(nix::unistd::getpid(),nix::sys::signal::Signal::SIGTERM).unwrap();}
  let memory=vec![1u8;16*1024*1024];std::hint::black_box(&memory);
  if mode=="salida"{std::io::stdout().write_all(&vec![b'x';300000]).unwrap();}
  thread::sleep(Duration::from_secs(10));
 }
 fn child(mode:&str)->Child{
  Command::new(std::env::current_exe().unwrap()).args(["--exact","supervision::tests::auxiliar","--nocapture"])
   .env("EIO_SUPERVISION_AUXILIAR",mode).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().unwrap()
 }
 #[test] fn causas_reales_y_control_independiente(){
  for mode in ["tiempo","memoria","cancelacion","salida","senal","custodia"] {
   let cancel=Arc::new(AtomicBool::new(false));
   let limits=Limits{time:if mode=="tiempo"{Duration::from_millis(150)}else{Duration::from_secs(2)},
       rss:if mode=="memoria"{4*1024*1024}else{256*1024*1024},output:256000};
   let mut child=child(mode);let pid=child.id();let stdout=child.stdout.take().unwrap();let c=Control::start(child,limits,cancel.clone());
   let count=c.output.clone();let reader=thread::spawn(move||{use std::io::Read;let mut pipe=stdout;let mut b=[0u8;8192];while let Ok(n)=pipe.read(&mut b){if n==0{break}count.fetch_add(n,Ordering::SeqCst);}});
   if mode=="cancelacion"{cancel.store(true,Ordering::SeqCst);}
   if mode=="custodia"{c.fault.store(true,Ordering::SeqCst);}
   // Consumidor suspendido. El bloqueo real del registro se prueba en checks.rs.
   thread::sleep(Duration::from_millis(350));
   let r=c.finish().unwrap();reader.join().unwrap();assert!(r.stop_confirmed,"{mode}: {r:?}");
   assert!(!std::path::Path::new(&format!("/proc/{pid}")).exists());
   let expected=match mode{"tiempo"=>"limite_tiempo","memoria"=>"limite_memoria","cancelacion"=>"cancelada","salida"=>"limite_salida","custodia"=>"fallo_conservacion_o_lectura",_=>""};
   if mode=="senal"{assert_eq!(r.exit_signal,Some(15));}else{assert_eq!(r.cause.as_deref(),Some(expected));assert_eq!(r.exit_signal,Some(9));}
  }
 }
}


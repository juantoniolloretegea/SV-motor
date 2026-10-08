//! Acotación humana durante la ejecución: espera una entrega conservada y detiene
//! exclusivamente el proceso identificado. No modifica el controlador ni sus fuentes.
#![forbid(unsafe_code)]
use std::{fs,path::{Path,PathBuf},time::{Duration,Instant}};
use serde_json::json;
use sv_cliente_api::{guard,need,parse,save,sha,R,LICENCIA};
use sysinfo::{System,ProcessesToUpdate};
fn run()->R<()> {
 let a:Vec<_>=std::env::args().collect();need(a.len()==3,"Uso: sv-acotar-ejecucion DIRECTORIO ENTREGA")?;
 let root=PathBuf::from(&a[1]);guard(&root)?;let number:usize=a[2].parse().map_err(|_|"Número inválido")?;
 need(number==48,"Sólo está autorizado el corte en 16 preguntas por 3 etapas")?;
 let expected=Path::new("C:/SV/compilacion/telemetria/debug/sv-examen-documental.exe");
 let mut sys=System::new();sys.refresh_processes(ProcessesToUpdate::All,true);
 let found:Vec<_>=sys.processes().iter().filter(|(_,p)|p.exe()==Some(expected)).map(|(id,p)|(*id,p.start_time())).collect();
 need(found.len()==1,"Proceso no único o ruta distinta")?;let(pid,start_time)=found[0];
 let begin=Instant::now();let last=root.join("hitos/I048.json");
 save(&root.join("ACOTACION-ARMADA.json"),&json!({"preguntas":16,"etapas":3,"entrega_final":48,"utc_ms":sv_instrumentacion::utc_ms(),"pid":pid.as_u32(),"inicio_proceso":start_time,"autoridad":"Instrucción humana de acotar a las primeras 16 preguntas; conservar las tres etapas si es posible","licencia":LICENCIA}))?;
 loop {
  if last.exists() {
   if let Ok(raw)=fs::read(&last) {if let Ok(h)=parse(&raw) {if h["caso"]=="P16"&&h["etapa"]==2&&h["resultado"]["completa"]==true&&h["resultado"]["telemetria_conforme"]==true {
    sys.refresh_processes(ProcessesToUpdate::Some(&[pid]),true);let p=sys.process(pid).ok_or("Proceso ya terminado")?;
    need(p.start_time()==start_time&&p.exe()==Some(expected),"Identidad del proceso cambió")?;
    need(p.kill(),"No se pudo detener el proceso identificado")?;
    save(&root.join("ACOTACION-EJECUTADA.json"),&json!({"preguntas":16,"etapas":3,"entrega_final":48,"hito_final_sha256":sha(&raw),"utc_ms":sv_instrumentacion::utc_ms(),"duracion_espera_ms":begin.elapsed().as_millis(),"pid":pid.as_u32(),"proceso_detenido":true,"limite":"Cotejar posibles archivos posteriores a I048; ninguna entrega posterior integra el contrato de 16","licencia":LICENCIA}))?;
    println!("Proceso detenido tras conservar P16/R2; cotejo de cierre pendiente");return Ok(());
   }}}
  }
  if begin.elapsed()>Duration::from_secs(1200){return Err("No se alcanzó el corte en veinte minutos; no se detuvo otro proceso".into());}
  std::thread::sleep(Duration::from_millis(2));
 }
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}

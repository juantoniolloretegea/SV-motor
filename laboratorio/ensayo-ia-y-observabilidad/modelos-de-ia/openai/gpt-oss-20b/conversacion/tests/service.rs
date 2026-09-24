//! Prueba nativa del proceso HTTP con datos sintéticos; requiere pesos verificados.
use std::{fs,io::{Read,Write},net::{TcpListener,TcpStream},path::PathBuf,process::{Child,Command,Stdio},thread,time::{Duration,Instant}};
use serde_json::{json,Value};
struct Service(Child);
impl Drop for Service{fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
fn request(method:&str,path:&str,key:&str,body:&str)->(u16,String){
 let mut stream=TcpStream::connect("127.0.0.1:3001").unwrap();stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
 write!(stream,"{method} {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\nContent-Type: application/json\r\nX-EIO-Session: {key}\r\nContent-Length: {}\r\n\r\n{body}",body.len()).unwrap();
 let mut response=String::new();stream.read_to_string(&mut response).unwrap();let(head,body)=response.split_once("\r\n\r\n").unwrap();(head.split_whitespace().nth(1).unwrap().parse().unwrap(),body.into())
}
fn api(key:&str,op:Value)->(u16,Value){let(code,body)=request("POST","/api",key,&op.to_string());(code,serde_json::from_str(&body).unwrap())}
fn start(root:&PathBuf)->(Service,String){
 let log=fs::OpenOptions::new().create(true).append(true).open(root.join("proceso.log")).unwrap();
 let child=Command::new(env!("CARGO_BIN_EXE_eio-conversacion")).env("EIO_DATA",root).env("EIO_ORIGIN","http://localhost:3001").env("EIO_BIND","127.0.0.1:3001").stdout(log.try_clone().unwrap()).stderr(log).spawn().unwrap();let mut service=Service(child);let deadline=Instant::now()+Duration::from_secs(60);
 loop{assert!(service.0.try_wait().unwrap().is_none(),"El servicio terminó antes de abrir el puerto");if TcpStream::connect("127.0.0.1:3001").is_ok(){break}assert!(Instant::now()<deadline,"El servicio no abrió el puerto");thread::sleep(Duration::from_millis(100));}
 let(code,page)=request("GET","/","","");assert_eq!(code,200);let key=page.split_once("<meta name=\"eio-session\" content=\"").unwrap().1.split('"').next().unwrap().to_string();assert_eq!(key.len(),64);(service,key)
}
fn stop(s:&mut Service){nix::sys::signal::kill(nix::unistd::Pid::from_raw(s.0.id() as i32),nix::sys::signal::Signal::SIGTERM).unwrap();let deadline=Instant::now()+Duration::from_secs(10);loop{if let Some(code)=s.0.try_wait().unwrap(){assert!(code.success());return}assert!(Instant::now()<deadline,"No se confirmó la parada");thread::sleep(Duration::from_millis(100));}}
#[test]
#[ignore = "Ejecutar expresamente con el puerto 3001 libre y los pesos disponibles"]
fn proceso_http_reinicio_y_exclusion(){
 let port=TcpListener::bind("127.0.0.1:3001").expect("La prueba exige el puerto 3001 libre");drop(port);
 let root=PathBuf::from(std::env::var("EIO_SERVICE_CHECK_DIR").expect("Defina un directorio nuevo para la prueba"));fs::create_dir(&root).unwrap();
 let(mut first,key1)=start(&root);let(code,state1)=api(&key1,json!({"op":"state"}));assert_eq!(code,200);assert_eq!(state1["identity"]["application"],"Conversación GPT-OSS 0.2.0");
 assert_eq!(api("clave-obsoleta",json!({"op":"state"})).0,403);assert_eq!(request("POST","/api",&key1,"{").0,400);
 let(code,case)=api(&key1,json!({"op":"create_case","title":"Comprobación sintética del proceso"}));assert_eq!(code,200);
 let file=root.join(format!("{}.jsonl",case["id"].as_str().unwrap()));let original=fs::read(&file).unwrap();
 assert_eq!(api(&key1,json!({"op":"request_status","request_id":"no-admitida-001"})).1["found"],false);
 let mut second=Service(Command::new(env!("CARGO_BIN_EXE_eio-conversacion")).env("EIO_DATA",&root).env("EIO_ORIGIN","http://localhost:3001").env("EIO_BIND","127.0.0.1:3001").stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap());let deadline=Instant::now()+Duration::from_secs(60);
 loop{if let Some(code)=second.0.try_wait().unwrap(){assert!(!code.success());break}assert!(Instant::now()<deadline,"La segunda instancia no fue rechazada");thread::sleep(Duration::from_millis(100));}
 assert_eq!(fs::read(&file).unwrap(),original);stop(&mut first);
 let prior=fs::read_to_string(root.join("servicio/ciclo.jsonl")).unwrap();let last:Value=serde_json::from_str(prior.lines().last().unwrap()).unwrap();assert_eq!(last["kind"],"servicio_detenido");
 let(mut restarted,key2)=start(&root);assert_ne!(key1,key2);assert_eq!(api(&key1,json!({"op":"state"})).0,403);let(code,state2)=api(&key2,json!({"op":"state"}));assert_eq!(code,200);assert_ne!(state1["service"]["instance_id"],state2["service"]["instance_id"]);assert!(state2["cases"].get(case["id"].as_str().unwrap()).is_some());assert_eq!(fs::read(&file).unwrap(),original);stop(&mut restarted);
 let report=json!({"schema":"EIO-PRUEBA-PROCESO-1","resultado":"conforme","http_inicial":200,"clave_obsoleta":403,"json_malformado":400,"segunda_instancia_rechazada":true,"dos_paradas_sigterm_confirmadas":true,"identidad_instancia_renovada":true,"clave_rotada":true,"expediente_sintetico_identico_tras_reinicio":true,"inferencias":0,"ambito":"Proceso HTTP local con expediente sintético; no valida el acceso externo del navegador ni la autenticación profesional."});fs::write(root.join("RESULTADO.json"),serde_json::to_vec_pretty(&report).unwrap()).unwrap();
}

#[test]
#[ignore = "Inferencia sintética única; puerto local 3001 y pesos verificados"]
fn observabilidad_inferencia_y_custodia(){
 let root=PathBuf::from(std::env::var("EIO_SERVICE_CHECK_DIR").expect("Directorio nuevo"));fs::create_dir(&root).unwrap();
 let(mut service,key)=start(&root);let deadline=Instant::now()+Duration::from_secs(20);
 loop{let s=api(&key,json!({"op":"state"})).1;if s["observability"]["observer"]["fresh"]==true{break}assert!(Instant::now()<deadline,"Observador sin confirmar");thread::sleep(Duration::from_millis(200));}
 let(code,case)=api(&key,json!({"op":"create_case","title":"Observación sintética 0.1.4"}));assert_eq!(code,200);
 let(code,chat)=api(&key,json!({"op":"create_chat","case_id":case["id"],"title":"Correlación técnica"}));assert_eq!(code,200);
 let profile=json!({"thinking":false,"max_output":32,"seconds":120,"seed":299792458});let question="Responda únicamente con la palabra HOLA.";
 let(code,preview)=api(&key,json!({"op":"preview","chat_id":chat["id"],"text":question,"profile":profile}));assert_eq!(code,200);
 let(code,_)=api(&key,json!({"op":"send","chat_id":chat["id"],"text":question,"profile":profile,"request_id":"observabilidad-sintetica-001","context_sha256":preview["context"]["sha256"]}));assert_eq!(code,200);
 let deadline=Instant::now()+Duration::from_secs(140);let result=loop{
  let v=api(&key,json!({"op":"get_chat","chat_id":chat["id"]})).1;
  let turn=&v["chat"]["turns"][0];if turn["status"]!="en_curso"{break turn["result"].clone()}
  assert!(Instant::now()<deadline,"No se confirmó finalización");thread::sleep(Duration::from_millis(500));
 };
 assert_eq!(result["finish"],"fin_normal");assert_eq!(result["exit_code"],0);assert_eq!(result["observability"]["export"]["ok"],true);
 let state=api(&key,json!({"op":"state"})).1;let instance=state["service"]["instance_id"].as_str().unwrap();let dir=root.join("servicio/otel").join(instance);
 let traces=fs::read_to_string(dir.join("trazas.jsonl")).unwrap();assert!(!traces.contains(question));
 let rows:Vec<Value>=traces.lines().map(|l|serde_json::from_str(l).unwrap()).collect();let trace=&result["observability"]["trace_id"];
 assert!(rows.iter().any(|v|v["trace_id"]==*trace&&v["name"]=="peticion_inferencia"));assert!(rows.iter().any(|v|v["trace_id"]==*trace&&v["name"]=="proceso_inferencia_creado"));
 let observations=fs::read_to_string(dir.join("observador/trazas.jsonl")).unwrap();let samples:Vec<Value>=observations.lines().map(|l|{let v:Value=serde_json::from_str(l).unwrap();serde_json::from_str(v["attributes"]["metadata_json"].as_str().unwrap()).unwrap()}).collect();
 let worker=&result["worker_identity"];let seen=samples.iter().flat_map(|v|v["tree"]["processes"].as_array().into_iter().flatten()).any(|v|v["pid"]==worker["pid"]&&v["start_ticks"]==worker["start_ticks"]);assert!(seen,"No se observó al proceso real de inferencia");
 let max_read=samples.iter().filter_map(|v|v["tree"]["read_seconds"].as_f64()).fold(0f64,f64::max);
 stop(&mut service);let deadline=Instant::now()+Duration::from_secs(12);let observer=loop{let v:Value=serde_json::from_slice(&fs::read(dir.join("observador/estado.json")).unwrap()).unwrap();if v["running"]==false{break v}assert!(Instant::now()<deadline);thread::sleep(Duration::from_millis(200));};
 assert_eq!(observer["reason"],"proceso_ausente_o_identidad_distinta");
 let report=json!({"schema":"EIO-OBSERVABILIDAD-VERIFICACION-1","resultado":"conforme","utc_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),"identity":state["identity"],"inference_result":result,"trace_correlated":true,"worker_observed_with_same_identity":seen,"maximum_tree_read_seconds":max_read,"samples_before_shutdown":samples.len(),"observer_after_shutdown":observer,"supervisor_trace_bytes":fs::metadata(dir.join("trazas.jsonl")).unwrap().len(),"observer_trace_bytes":fs::metadata(dir.join("observador/trazas.jsonl")).unwrap().len(),"scope":"Una inferencia sintética; no comparación semántica, determinista ni de rendimiento con/sin instrumentación."});
 fs::write(root.join("RESULTADO.json"),serde_json::to_vec_pretty(&report).unwrap()).unwrap();
}


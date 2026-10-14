//! Reconstruye decisiones y resultados del archivo, sin cargar modelos.
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use selector::{Capacidad,Carga,Clase,Cobertura,Destino,Estado,Politica,Recurso,Solicitud};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,Component}};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn n(v:&Value,k:&str)->R<u64>{v[k].as_u64().ok_or_else(||format!("Número ausente: {k}").into())}
fn s<'a>(v:&'a Value,k:&str)->R<&'a str>{v[k].as_str().ok_or_else(||format!("Texto ausente: {k}").into())}
fn arr<'a>(v:&'a Value,k:&str)->R<&'a Vec<Value>>{v[k].as_array().ok_or_else(||format!("Array ausente: {k}").into())}
fn clase(s:&str)->R<Clase>{match s{"CPU"=>Ok(Clase::Cpu),"GPU"=>Ok(Clase::Gpu),"TPU"=>Ok(Clase::Tpu),"NPU"=>Ok(Clase::Npu),_=>Err("Clase desconocida".into())}}
fn destino(v:&Value)->R<Destino>{Ok(Destino{servidor:s(v,"servidor")?.into(),dispositivo:s(v,"dispositivo")?.into(),adaptador_revision:s(v,"adaptador_revision")?.into()})}
fn carga(v:&Value)->R<Carga>{Ok(Carga{contrato:s(v,"contrato")?.into(),revision:s(v,"revision")?.into(),perfil:s(v,"perfil")?.into(),dimensiones:arr(v,"dimensiones")?.iter().map(|n|n.as_u64().map(|x|x as usize).ok_or("Dimensión inválida".into())).collect::<R<Vec<_>>>()?})}
fn main()->R<()> {
 let a:Vec<_>=std::env::args().collect();if a.len()!=4{return Err("Uso: verificar_conexion archivo sha256 informe".into());}
 let raw=fs::read(&a[1])?;if sha(&raw)!=a[2]{return Err("Archivo incorrecto".into());}
 let mut archivos=BTreeMap::new();
 for e in tar::Archive::new(flate2::read::GzDecoder::new(raw.as_slice())).entries()?{
  let mut e=e?;let p=e.path()?.into_owned();if !p.starts_with("resultado")||!p.components().all(|c|matches!(c,Component::Normal(_))){return Err("Ruta inválida".into());}
  if e.header().entry_type().is_dir(){continue;}if !e.header().entry_type().is_file()||e.size()>128*1024*1024{return Err("Entrada inválida".into());}
  let mut b=Vec::new();e.read_to_end(&mut b)?;if archivos.insert(p.strip_prefix("resultado")?.to_string_lossy().to_string(),b).is_some(){return Err("Duplicado".into());}
 }
 let bytes=|p:&str|->R<&Vec<u8>>{archivos.get(p).ok_or_else(||format!("Falta {p}").into())};
 let leer=|p:&str|->R<Value>{Ok(serde_json::from_slice(bytes(p)?)?)};
 let manifiesto=leer("INTEGRIDAD.json")?;
 for f in arr(&manifiesto,"archivos")? {let b=bytes(s(f,"archivo")?)?;if sha(b)!=s(f,"sha256")?||b.len() as u64!=n(f,"bytes")?{return Err("Integridad incorrecta".into());}}
 if arr(&manifiesto,"archivos")?.len()+1!=archivos.len(){return Err("Archivos sin cotejar".into());}
 let resultado=leer("RESULTADO.json")?;
 if resultado["conforme"]!=true || leer("SUPERVISOR.json")?["exit_code"]!=0{return Err("Ejecución no conforme".into());}
 let calibracion=leer("CALIBRACION.json")?;
 let entrada=leer("ENTRADA.json")?;
 let secuencia=arr(&entrada,"secuencia")?;
 if secuencia.len()!=16 || entrada["salidas"]!=8 || entrada["vocabulario"]!=151936 {return Err("Carga diferente del protocolo".into());}
 let perfil=format!("Q4_K_M;residentes;contexto64;CPU4;T4-sm75;mwv;hw_dp4a;secuencia={}",sha(&serde_json::to_vec(secuencia)?));
 let evidencia=format!("CALIBRACION.json:sha256:{}",sha(bytes("CALIBRACION.json")?));
 let mut medianas=Vec::new();
 for (campo,cl) in [("cpu_us","CPU"),("gpu_us","GPU")] {
  let mut ts=Vec::new();
  for i in 0..3{let v=leer(&format!("calibracion-{i}-{cl}.json"))?;ts.push(n(&v,"tiempo_servicio_us")?);}
  if json!(ts)!=calibracion[campo]{return Err("Tiempos de calibración discrepantes".into());}
  ts.sort_unstable();medianas.push(ts[1]);
 }
 if json!(medianas)!=calibracion["medianas_us"]{return Err("Medianas discrepantes".into());}
 let trace:Vec<Value>=std::str::from_utf8(bytes("TRAZA-GPU.jsonl")?)?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
 let mut lanzamientos=BTreeMap::<String,u64>::new();
 for t in &trace{if t["tipo"]=="lanzamiento"{if t["datos"]["resultado"]!=0{return Err("Error CUDA".into());}*lanzamientos.entry(s(t,"fase")?.into()).or_default()+=1;}if t["tipo"]=="recorrido_rechazado"{return Err("Ruta prohibida".into());}}
 let mut reconstruidas=Vec::new();
 for id in ["solicitud-automatica","solicitud-cpu"] {
  let archivo=leer(&format!("{id}-DECISION.json"))?;let v=&archivo["solicitud"];
  let sol=Solicitud{id:s(v,"id")?.into(),carga:carga(&v["carga"])?,memoria_adicional_bytes:n(v,"memoria_adicional_bytes")?,margen_bytes:n(v,"margen_bytes")?,residencia_obligatoria:None};
  if sol.carga.revision!="d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785" || sol.carga.perfil!=perfil || sol.carga.dimensiones!=vec![16,151936] {return Err("Identidad efectiva de la solicitud discrepante".into());}
  let p=&archivo["politica"];let pol=Politica{revision:s(p,"revision")?.into(),destinos_permitidos:arr(p,"destinos_permitidos")?.iter().map(destino).collect::<R<_>>()?,preferencia:arr(p,"preferencia")?.iter().map(|v|clase(v.as_str().unwrap_or(""))).collect::<R<_>>()?,clase_obligatoria:p["clase_obligatoria"].as_str().map(clase).transpose()?,vigencia_ms:n(p,"vigencia_ms")?,comparar_estimaciones:p["comparar_estimaciones"]==true};
  let rs=arr(&archivo,"recursos")?.iter().map(|r|->R<Recurso>{
    if r["estado"]!="Comprobado"{return Err("Estado no contrastado".into());}
    let caps=arr(r,"capacidades")?.iter().map(|c|->R<Capacidad>{if c["cobertura"]!="Completa"||s(c,"evidencia")?!=evidencia{return Err("Capacidad no sustentada".into());}Ok(Capacidad{carga:carga(c)?,cobertura:Cobertura::Completa,evidencia:s(c,"evidencia")?.into(),estimacion_total_us:Some(n(c,"estimacion_total_us")?)})}).collect::<R<Vec<_>>>()?;
    let cl=clase(s(r,"clase")?)?;
    if caps.len()!=1 || caps[0].estimacion_total_us!=Some(medianas[usize::from(cl==Clase::Gpu)]){return Err("Estimación ajena a calibración".into());}
    Ok(Recurso{destino:destino(&r["destino"])?,clase:cl,estado:Estado::Comprobado,observado_ms:n(r,"observado_ms")?,memoria_disponible_bytes:Some(n(r,"memoria_disponible_bytes")?),disponible:r["disponible"]==true,capacidades:caps})
  }).collect::<R<Vec<_>>>()?;
  let registrada=&archivo["decision"];
  let d=selector::seleccionar(&sol,&pol,&rs,n(registrada,"decidida_ms")?).map_err(|e|format!("Reconstrucción: {e:?}"))?;
  if d.destino!=destino(&registrada["destino"])?||d.clase!=clase(s(registrada,"clase")?)?||format!("{:?}",d.criterio)!=s(registrada,"criterio")?{return Err("Decisión no reproducible".into());}
  let ejec=leer(&format!("{id}.json"))?;let count=*lanzamientos.get(id).unwrap_or(&0);
  let posiciones=arr(&ejec,"posiciones")?;
  if posiciones.len()!=16 || posiciones.iter().enumerate().any(|(i,p)|p["posicion"]!=i || p["token"]!=secuencia[i]) {return Err("Secuencia ejecutada diferente".into());}
  if clase(s(&ejec,"clase_ejecutada")?)?!=d.clase||count!=n(&ejec,"lanzamientos_gpu")?||(d.clase==Clase::Cpu && count!=0)||(d.clase==Clase::Gpu && count==0){return Err("Dispositivo efectivo discrepante".into());}
  if d.clase==Clase::Gpu && d.destino.dispositivo!=s(&leer("PREPARACION.json")?["gpu"],"uuid")?{return Err("UUID diferente".into());}
  reconstruidas.push(json!({"solicitud":id,"clase":s(registrada,"clase")?,"destino":registrada["destino"],"mediana_estimada_us":medianas[usize::from(d.clase==Clase::Gpu)],"medido_us":ejec["tiempo_servicio_us"],"recorrido_desde_admision_us":leer(&format!("{id}-CIERRE.json"))?["recorrido_desde_admision_us"],"lanzamientos_gpu":count,"decision_reconstruida":true}));
 }
 // Cotejo numérico independiente de los archivos f32, no del booleano declarado.
 let vector=|p:&str|->R<Vec<f64>>{let b=bytes(p)?;if b.len()!=151936*4{return Err("Vector incompleto".into());}b.chunks_exact(4).map(|x|{let n=f32::from_le_bytes(x.try_into().unwrap());if n.is_finite(){Ok(n as f64)}else{Err("Valor no finito".into())}}).collect()};
 let mut pares=Vec::new();
 let destinos=vec!["calibracion-0-GPU","calibracion-1-CPU","calibracion-1-GPU","calibracion-2-CPU","calibracion-2-GPU","solicitud-automatica","solicitud-cpu"];
 for objetivo in destinos { for i in 0..8 {
   let a=vector(&format!("calibracion-0-CPU-{i:02}.f32"))?;let b=vector(&format!("{objetivo}-{i:02}.f32"))?;
   let (mut aa,mut bb,mut ab,mut dd)=(0.,0.,0.,0.);let(mut ai,mut bi)=(0,0);
   for j in 0..a.len(){aa+=a[j]*a[j];bb+=b[j]*b[j];ab+=a[j]*b[j];dd+=(a[j]-b[j]).powi(2);if a[j]>a[ai]{ai=j;}if b[j]>b[bi]{bi=j;}}
   let cos=ab/(aa*bb).sqrt();let l2=(dd/aa).sqrt();if !(cos>=0.99 && l2<=0.05 && ai==bi){return Err("Contraste numérico posterior no conforme".into());}
   pares.push(json!({"destino":objetivo,"salida":i,"coseno":cos,"l2":l2,"argmax":ai}));
 }}
 for i in 0..3 { for cl in ["CPU","GPU"] {
  let id=format!("calibracion-{i}-{cl}");let e=leer(&format!("{id}.json"))?;let count=*lanzamientos.get(&id).unwrap_or(&0);
  if n(&e,"lanzamientos_gpu")?!=count || (cl=="CPU" && count!=0) || (cl=="GPU" && count==0) {return Err("Calibración física discrepante".into());}
 }}
 let rechazo=leer("RECHAZO.json")?;if rechazo["ejecuciones"]!=0||rechazo["lanzamientos_gpu"]!=0{return Err("Rechazo incorrecto".into());}
 let tele:Vec<Value>=std::str::from_utf8(bytes("TELEMETRIA.jsonl")?)?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
 if tele.len() as u64!=n(&resultado["telemetria"],"muestras")? || n(&resultado["telemetria"],"fallos")?!=0 || tele.iter().any(|m|m["cpu"].get("error").is_some()||m["gpu"].as_array().is_none_or(|gs|gs.iter().any(|g|g.get("error").is_some()))) {return Err("Telemetría incompleta".into());}
 let out=json!({"archivo_sha256":a[2],"archivos":archivos.len(),"conforme":true,"solicitudes":reconstruidas,"comparaciones_recalculadas":pares,"rechazo":rechazo,"telemetria":resultado["telemetria"],"limite":"Reconstrucción del control y del resultado; no certificación externa ni inspección del código máquina NVIDIA"});
 if Path::new(&a[3]).exists(){return Err("Informe ya existe".into());}fs::write(&a[3],serde_json::to_vec_pretty(&out)?)?;
 println!("Conforme: dos decisiones reconstruidas, 56 comparaciones numéricas, integridad y destino cotejados.");Ok(())
}

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
 let a:Vec<_>=std::env::args().collect();if a.len()!=4{return Err("Uso: verificar_gobierno archivo sha256 informe".into());}
 let mut raw=fs::read(&a[1])?;
 if let Some(prefix)=a[1].strip_suffix(".parte-00"){
  let segunda=fs::read(format!("{prefix}.parte-01"))?;
  if sha(&raw)!="410a843d52585840b3e11bd1e97bb9c11754098a7c5fbc3efd07248643eb294b"||sha(&segunda)!="7ee53eb4308abb966bd1419496e11e03404be7610f591d2fa1a62b3bcf19c630"{return Err("Partes no correspondientes al archivo conservado".into());}
  raw.extend_from_slice(&segunda);
 }
 if sha(&raw)!=a[2]{return Err("Archivo incorrecto".into());}
 let mut archivos=BTreeMap::new();
 for e in tar::Archive::new(flate2::read::GzDecoder::new(raw.as_slice())).entries()?{
  let mut e=e?;let p=e.path()?.into_owned();if !p.starts_with("resultado")||!p.components().all(|c|matches!(c,Component::Normal(_))){return Err("Ruta inválida".into());}
  if e.header().entry_type().is_dir(){continue;}if !e.header().entry_type().is_file()||e.size()>512*1024*1024{return Err("Entrada inválida".into());}
  let mut b=Vec::new();e.read_to_end(&mut b)?;if archivos.insert(p.strip_prefix("resultado")?.to_string_lossy().to_string(),b).is_some(){return Err("Duplicado".into());}
 }
 let bytes=|p:&str|->R<&Vec<u8>>{archivos.get(p).ok_or_else(||format!("Falta {p}").into())};
 let leer=|p:&str|->R<Value>{Ok(serde_json::from_slice(bytes(p)?)?)};
 let manifiesto=leer("INTEGRIDAD.json")?;
 for f in arr(&manifiesto,"archivos")? {let b=bytes(s(f,"archivo")?)?;if sha(b)!=s(f,"sha256")?||b.len() as u64!=n(f,"bytes")?{return Err("Integridad incorrecta".into());}}
 if arr(&manifiesto,"archivos")?.len()+1!=archivos.len(){return Err("Archivos sin cotejar".into());}
 let resultado=leer("RESULTADO.json")?;
 if resultado["conforme"]!=true||leer("SUPERVISOR.json")?["exit_code"]!=0{return Err("Ejecución no conforme".into());}
 let entrada=leer("ENTRADA.json")?;let tokens=arr(&entrada,"tokens")?;
 if tokens.len()!=9||entrada["texto"]!="El resultado de sumar dos y dos es"||entrada["max_salida"]!=16||entrada["contexto"]!=64{return Err("Entrada distinta del protocolo".into());}
 let entrada_sha=sha(&serde_json::to_vec(tokens)?);
 let perfil=format!("Q4_K_M;residentes;contexto64;CPU4;T4-sm75;mwv;hw_dp4a;greedy;salida16;fin151645,151643;entrada={entrada_sha}");
 let cal=leer("CALIBRACION.json")?;let evidencia=format!("CALIBRACION.json:sha256:{}",sha(bytes("CALIBRACION.json")?));
 let mut medianas=vec![];for(cl,key)in[("CPU","cpu_us"),("GPU","gpu_us")]{let mut ts=vec![];for i in 0..3{ts.push(n(&leer(&format!("calibracion-{i}-{cl}.json"))?,"tiempo_servicio_us")?);}if json!(ts)!=cal[key]{return Err("Calibración discordante".into());}ts.sort_unstable();medianas.push(ts[1]);}
 if json!(medianas)!=cal["medianas_us"]{return Err("Mediana discordante".into());}
 let mut registros=vec![];
 for prefijo in ["","negativo-recurso/"]{
  let mut previo="0".repeat(64);let mut j=0;let mut ultimo=String::new();
  loop{let path=format!("{prefijo}ARBITRO-{j:04}.json");if !archivos.contains_key(&path){break;}
   let r=leer(&path)?;let base=&r["base"];let h=sha(&serde_json::to_vec(base)?);
   if r["sha256"]!=h||base["anterior"]!=previo||base["numero"]!=j{return Err(format!("Cadena del Árbitro discrepante en {path}: conservado={}, recalculado={h}",r["sha256"]).into());}
   previo=h;ultimo=s(base,"evento")?.into();if prefijo.is_empty(){registros.push(base.clone());}j+=1;
  }if j==0||ultimo!="cierre"{return Err("Registro del Árbitro sin cierre".into());}
  let total=archivos.keys().filter(|p|p.strip_prefix(prefijo).is_some_and(|q|q.starts_with("ARBITRO-")&&q.ends_with(".json")&&!q.contains('/'))).count();
  if total!=j as usize{return Err("Registro del Árbitro con discontinuidades".into());}
 }
 if registros.first().map(|r|&r["datos"]["entrada"])!=Some(&entrada){return Err("Perfil y entrada diferentes".into());}
 let trace:Vec<Value>=std::str::from_utf8(bytes("TRAZA-GPU.jsonl")?)?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
 let mut lanzamientos=BTreeMap::<String,u64>::new();for t in &trace{if t["tipo"]=="lanzamiento"{if t["datos"]["resultado"]!=0{return Err("Error CUDA".into());}*lanzamientos.entry(s(t,"fase")?.into()).or_default()+=1;}if t["tipo"]=="recorrido_rechazado"{return Err("Ruta prohibida solicitada".into());}}
 let obligatorias=[None,Some(Clase::Cpu),Some(Clase::Gpu),Some(Clase::Cpu),Some(Clase::Gpu),None,Some(Clase::Cpu),Some(Clase::Gpu)];
 let mut reconstruidas=vec![];
 for i in 0..8{
  let id=format!("QAD-{:02}",i+1);let propios=registros.iter().filter(|r|r["datos"]["id"]==id||r["datos"]["solicitud"]["id"]==id).collect::<Vec<_>>();
  let hallar=|evento:&str|->R<&Value>{let encontrados=propios.iter().filter(|r|r["evento"]==evento).collect::<Vec<_>>();if encontrados.len()!=1{return Err(format!("Etapa {evento} no única para {id}").into());}Ok(&encontrados[0]["datos"])};
  let adm=hallar("admision")?;let ar=hallar("seleccion")?;let inicio=hallar("inicio_ejecucion")?;let dev=hallar("devolucion")?;
  let mut prev=None;for ev in ["admision","seleccion","inicio_ejecucion","devolucion"]{let num=n(propios.iter().find(|r|r["evento"]==ev).unwrap(),"numero")?;if prev.is_some_and(|p|p>=num){return Err("Etapas del Árbitro desordenadas".into());}prev=Some(num);}
  // Identidad real de petición y política, no aceptación de una etiqueta.
  let esperada=json!({"id":id,"instancia":"Qwen3-8B-Q4_K_M/SV-Aprender","sesion":"QAD-20261014","entrada":entrada});
  let esha=sha(&serde_json::to_vec(&esperada)?);
  if adm["ligaduras"]["entrada_sha256"]!=esha||inicio["entrada_sha256"]!=esha||adm["ligaduras"]["politica_sha256"]!=sha(&serde_json::to_vec(&adm["politica"])?){return Err("Identidad de admisión no cotejada".into());}
  if adm["ligaduras"]["nucleo_revision"]!="1c6b84238e62e867c4d5c5ab8069c9ae8f67dccc"||adm["ligaduras"]["autoridad_r1"]!=false{return Err("Alcance del núcleo discrepante".into());}
  if adm["politica"]!=ar["politica"]||inicio["decision"]!=ar["decision"]||dev["decision"]!=ar["decision"]{return Err("Política o decisión sustituidas".into());}
  let v=&ar["solicitud"];let sol=Solicitud{id:s(v,"id")?.into(),carga:carga(&v["carga"])?,memoria_adicional_bytes:n(v,"memoria_adicional_bytes")?,margen_bytes:n(v,"margen_bytes")?,residencia_obligatoria:None};
  if sol.carga.perfil!=perfil||sol.carga.revision!="d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785"||sol.carga.dimensiones!=vec![9,16,151936]{return Err("Carga no correspondiente".into());}
  let p=&ar["politica"];let pol=Politica{revision:s(p,"revision")?.into(),destinos_permitidos:arr(p,"destinos_permitidos")?.iter().map(destino).collect::<R<_>>()?,preferencia:arr(p,"preferencia")?.iter().map(|v|clase(v.as_str().unwrap_or(""))).collect::<R<_>>()?,clase_obligatoria:p["clase_obligatoria"].as_str().map(clase).transpose()?,vigencia_ms:n(p,"vigencia_ms")?,comparar_estimaciones:p["comparar_estimaciones"]==true};
  if pol.clase_obligatoria!=obligatorias[i]||!pol.comparar_estimaciones||pol.vigencia_ms!=600000{return Err("Política no prevista".into());}
  let parse_recursos=|v:&Value,k:&str|->R<Vec<Recurso>>{arr(v,k)?.iter().map(|r|->R<Recurso>{
   if r["estado"]!="Comprobado"{return Err("Recurso no comprobado".into());}let cl=clase(s(r,"clase")?)?;
   let caps=arr(r,"capacidades")?.iter().map(|c|->R<Capacidad>{if c["cobertura"]!="Completa"||c["evidencia"]!=evidencia{return Err("Capacidad no sustentada".into());}Ok(Capacidad{carga:carga(c)?,cobertura:Cobertura::Completa,evidencia:s(c,"evidencia")?.into(),estimacion_total_us:Some(n(c,"estimacion_total_us")?)})}).collect::<R<Vec<_>>>()?;
   if caps.len()!=1||caps[0].estimacion_total_us!=Some(medianas[usize::from(cl==Clase::Gpu)]){return Err("Estimación discordante".into());}
   Ok(Recurso{destino:destino(&r["destino"])?,clase:cl,estado:Estado::Comprobado,observado_ms:n(r,"observado_ms")?,memoria_disponible_bytes:Some(n(r,"memoria_disponible_bytes")?),disponible:r["disponible"]==true,capacidades:caps})}).collect()};
  let rs=parse_recursos(ar,"recursos")?;let reg=&ar["decision"];let d=selector::seleccionar(&sol,&pol,&rs,n(reg,"decidida_ms")?).map_err(|e|format!("Selector: {e:?}"))?;
  if d.destino!=destino(&reg["destino"])?||d.clase!=clase(s(reg,"clase")?)?||format!("{:?}",d.criterio)!=s(reg,"criterio")?{return Err("Decisión no reproducible".into());}
  let fresca=parse_recursos(inicio,"recursos_revalidados")?;let t=n(inicio,"ahora_ms")?;
  if t<d.decidida_ms||t-d.decidida_ms>5000{return Err("Decisión vencida".into());}
  let segunda=selector::seleccionar(&sol,&pol,&fresca,t).map_err(|e|format!("Revalidación: {e:?}"))?;
  if segunda.destino!=d.destino||segunda.clase!=d.clase{return Err("Destino cambiado".into());}
  let e=leer(&format!("{id}.json"))?;if dev["ejecucion"]!=e{return Err("Devolución ajena al original".into());}
  let count=*lanzamientos.get(&id).unwrap_or(&0);
  if clase(s(&e,"clase_ejecutada")?)?!=d.clase||count!=n(&e,"lanzamientos_gpu")?||(d.clase==Clase::Cpu&&count!=0)||(d.clase==Clase::Gpu&&count==0){return Err("Ejecución en dispositivo diferente".into());}
  if d.clase==Clase::Gpu&&d.destino.dispositivo!=s(&leer("PREPARACION.json")?["gpu"],"uuid")?{return Err("Identidad GPU diferente".into());}
  reconstruidas.push(json!({"id":id,"clase":s(reg,"clase")?,"destino":reg["destino"],"estimado_us":medianas[usize::from(d.clase==Clase::Gpu)],"medido_us":e["tiempo_servicio_us"],"recorrido_us":leer(&format!("{id}-CIERRE.json"))?["recorrido_desde_solicitud_us"],"lanzamientos_gpu":count,"decision_reconstruida":true}));
 }
 let vector=|p:&str|->R<Vec<f64>>{let b=bytes(p)?;if b.len()!=151936*4{return Err("Vector incompleto".into());}b.chunks_exact(4).map(|x|{let f=f32::from_le_bytes(x.try_into().unwrap());if f.is_finite(){Ok(f as f64)}else{Err("No finito".into())}}).collect()};
 let mut objetivos=vec![];for i in 0..3{for c in ["CPU","GPU"]{objetivos.push(format!("calibracion-{i}-{c}"));}}for i in 1..=8{objetivos.push(format!("QAD-{i:02}"));}
 let refsal=leer("calibracion-0-CPU-SALIDA.json")?;let rt=arr(&refsal,"tokens")?;if rt.is_empty()||rt.len()>16{return Err("Número de tokens no admitido".into());}
 let mut pares=vec![];
 for id in objetivos{
  let e=leer(&format!("{id}.json"))?;let salida=leer(&format!("{id}-SALIDA.json"))?;if salida["tokens"]!=refsal["tokens"]||e["salida"]!=salida{return Err("Secuencias generadas diferentes".into());}
  let pp=arr(&e,"posiciones")?;if pp.len()!=tokens.len()+rt.len()-1{return Err("Posiciones incompletas".into());}
  for(j,p)in pp.iter().enumerate(){let t=if j<tokens.len(){&tokens[j]}else{&rt[j-tokens.len()]};if p["posicion"]!=j||&p["entrada_token"]!=t{return Err("Entrada efectiva distinta".into());}if j+1>=tokens.len()&&p["salida_token"]!=rt[j+1-tokens.len()]{return Err("Emisión no vinculada".into());}}
  let count=*lanzamientos.get(&id).unwrap_or(&0);if n(&e,"lanzamientos_gpu")?!=count{return Err("Recuento físico diferente".into());}
  for i in 0..rt.len(){let a=vector(&format!("calibracion-0-CPU-{i:02}.f32"))?;let b=vector(&format!("{id}-{i:02}.f32"))?;
   let(mut aa,mut bb,mut ab,mut dd)=(0.,0.,0.,0.);let(mut ai,mut bi)=(0,0);
   for j in 0..a.len(){aa+=a[j]*a[j];bb+=b[j]*b[j];ab+=a[j]*b[j];dd+=(a[j]-b[j]).powi(2);if a[j]>a[ai]{ai=j;}if b[j]>b[bi]{bi=j;}}
   let cos=ab/(aa*bb).sqrt();let l2=(dd/aa).sqrt();if !(cos>=0.99&&l2<=0.05&&ai==bi)||rt[i]!=bi as u64{return Err("Vector, token o paridad incorrectos".into());}
   pares.push(json!({"destino":id,"posicion":i,"coseno":cos,"l2":l2,"argmax":bi}));
  }
 }
 let rech=leer("RECHAZOS.json")?;if rech["ejecuciones"]!=0||arr(&rech,"casos")?.len()!=3{return Err("Controles negativos incompletos".into());}
 let tele:Vec<Value>=std::str::from_utf8(bytes("TELEMETRIA.jsonl")?)?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
 if tele.len() as u64!=n(&resultado["telemetria"],"muestras")?||n(&resultado["telemetria"],"fallos")?!=0||tele.iter().any(|m|m["cpu"].get("error").is_some()||m["gpu"].as_array().is_none_or(|gs|gs.iter().any(|g|g.get("error").is_some()))){return Err("Telemetría incompleta".into());}
 let out=json!({"archivo_sha256":a[2],"archivos":archivos.len(),"conforme":true,"solicitudes":reconstruidas,"comparaciones_recalculadas":pares,"registros_arbitro":registros.len(),"salida_original":refsal,"rechazos":rech,"telemetria":resultado["telemetria"],"alcance":"Reconstrucción del control, la integridad y la ejecución; no adjudicación semántica ni certificación externa"});
 if Path::new(&a[3]).exists(){return Err("Informe ya existente".into());}fs::write(&a[3],serde_json::to_vec_pretty(&out)?)?;println!("Conforme: ocho decisiones y sus revalidaciones, generaciones, registros y telemetría cotejados.");Ok(())
}

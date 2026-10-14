#![forbid(unsafe_code)]
use std::{collections::BTreeSet,error::Error,fs,path::{Component,Path}};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
type R<T> = Result<T,Box<dyn Error>>;
fn exigir(c:bool,m:&str)->R<()>{if c{Ok(())}else{Err(m.into())}}
fn leer(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn numero(v:&Value,k:&str)->R<u64>{v[k].as_u64().ok_or_else(||format!("Medida ausente: {k}").into())}
fn lista(v:&Value)->R<&Vec<Value>>{v.as_array().ok_or_else(||"Lista ausente".into())}
fn huella(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn inventario(p:&Path,raiz:&Path,s:&mut BTreeSet<String>)->R<()>{
 for e in fs::read_dir(p)?{let e=e?;let t=e.file_type()?;exigir(!t.is_symlink(),"Enlace no permitido")?;
  if t.is_dir(){inventario(&e.path(),raiz,s)?}else if t.is_file(){s.insert(e.path().strip_prefix(raiz)?.to_string_lossy().replace('\\',"/"));}}
 Ok(())
}
fn integridad(b:&Path)->R<Value>{
 let raw=fs::read(b.join("MANIFIESTO.json"))?;let m:Value=serde_json::from_slice(&raw)?;let mut s=BTreeSet::new();let mut total=0u64;
 for f in lista(&m["archivos"])?{let n=f["ruta"].as_str().ok_or("Ruta ausente")?;
  exigir(Path::new(n).components().all(|x|matches!(x,Component::Normal(_)))&&s.insert(n.to_owned()),"Ruta duplicada o no admitida")?;
  let datos=fs::read(b.join(n))?;exigir(numero(f,"bytes")?==datos.len() as u64&&f["sha256"]==huella(&datos),"Identidad de archivo discordante")?;total+=datos.len()as u64;}
 let mut actual=BTreeSet::new();inventario(b,b,&mut actual)?;actual.remove("MANIFIESTO.json");exigir(actual==s,"Cobertura del manifiesto discordante")?;
 Ok(json!({"archivos":s.len(),"bytes":total,"manifiesto_sha256":huella(&raw)}))
}
fn f32s(p:&Path)->R<Vec<f32>>{let b=fs::read(p)?;exigir(!b.is_empty()&&b.len()%4==0,"Vector incompleto")?;Ok(b.chunks_exact(4).map(|x|f32::from_le_bytes(x.try_into().unwrap())).collect())}
fn comparar(a:&Path,b:&Path)->R<Value>{
 let a=f32s(a)?;let b=f32s(b)?;exigir(a.len()==b.len(),"Dimensiones discordantes")?;let(mut max,mut se,mut sr)=(0f64,0f64,0f64);
 for (x,y) in a.iter().zip(&b){exigir(x.is_finite()&&y.is_finite(),"Valor no finito")?;let d=*x as f64-*y as f64;max=max.max(d.abs());se+=d*d;sr+=(*x as f64).powi(2);}
 let rmse=(se/a.len()as f64).sqrt()/(sr/a.len()as f64).sqrt().max(1.);exigir(max<=0.005&&rmse<=0.0001,"Tolerancia numérica incumplida")?;
 Ok(json!({"elementos":a.len(),"maxima_absoluta":max,"rmse_normalizada":rmse}))
}
fn traza(p:&Path,tpu:bool)->R<Value>{
 let t=leer(&p.join("TRAZA.json"))?;exigir(lista(&t["errors"])?.is_empty()&&lista(&t["warnings"])?.is_empty(),"Traza con incidencias")?;
 let mut modulos=0usize;let mut ops=0usize;let mut host=0usize;
 for plane in lista(&t["planes"])?{let n=plane["name"].as_str().ok_or("Plano sin nombre")?;
  for line in lista(&plane["lines"])?{let ev=lista(&line["events"])?;if n=="/host:CPU"{host+=ev.len();}
   if n.starts_with("/device:TPU:")&&!ev.is_empty(){exigir(n=="/device:TPU:0"&&tpu,"Actividad TPU no autorizada")?;
    if line["name"]=="XLA Modules"{modulos+=ev.len();}if line["name"]=="XLA Ops"{ops+=ev.len();}}}}
 exigir(host>0,"Sin observación CPU")?;let a=leer(&p.join("ATRIBUCION.json"))?;
 exigir(lista(&a["modulos"])?.len()==modulos&&lista(&a["operaciones"])?.len()==ops,"Desglose discordante con la traza original")?;
 let tiempo=numero(&a,"calculo_tpu_union_ps")?;
 if tpu{exigir(modulos>0&&ops>0&&tiempo>0&&lista(&a["programas"])?.len()==2&&!lista(&a["transferencias"])?.is_empty(),"Atribución TPU incompleta")?;}
 else{exigir(modulos==0&&ops==0&&tiempo==0&&lista(&a["transferencias"])?.is_empty(),"CPU con actividad TPU")?;}
 let bytes=lista(&a["transferencias"])?.iter().map(|x|numero(x,"bytes")).collect::<R<Vec<_>>>()?.iter().sum::<u64>();
 Ok(json!({"eventos_cpu":host,"modulos_tpu":modulos,"operaciones_tpu":ops,"calculo_tpu_union_ps":tiempo,"transferencias":lista(&a["transferencias"])?.len(),"bytes_observados_eventos_transferencia":bytes}))
}
fn main()->R<()>{
 let args=std::env::args().collect::<Vec<_>>();exigir(args.len()==3,"Uso: cotejo resultado salida")?;let base=Path::new(&args[1]);let custodia=integridad(base)?;
 let resumen=leer(&base.join("RESULTADO.json"))?;exigir(resumen["conforme"]==true&&resumen["generaciones"]==22,"Serie incompleta")?;
 let inicio=leer(&base.join("INICIO.json"))?;exigir(inicio["selector_sha256"]=="2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05","Selector diferente")?;
 let mut casos=Vec::new();let(mut total,mut cpu,mut tpu)=(0usize,0usize,0usize);let mut num=Vec::new();
 for c in lista(&resumen["casos"])?{let id=c["caso"].as_str().ok_or("Caso ausente")?;exigir(["NCI_4-0","NCI_178-3"].contains(&id),"Caso ajeno")?;let b=base.join(id);
  let referencia=leer(&b.join("referencia-cpu/SALIDA.json"))?;let pasos=lista(&referencia["tokens_generados"])?.len();
  num.push(comparar(&b.join("referencia-cpu/encoder.f32"),&b.join("validacion-tpu/encoder.f32"))?);
  for i in 0..pasos{num.push(comparar(&b.join(format!("referencia-cpu/{i:03}.f32")),&b.join(format!("validacion-tpu/{i:03}.f32")))?);}
  let mut ejecuciones=Vec::new();let mut tiempos_cpu=Vec::new();let mut tiempos_tpu=Vec::new();let mut generacion_cpu=Vec::new();let mut generacion_tpu=Vec::new();let mut completo=true;
  for ent in fs::read_dir(&b)?{let ent=ent?;if !ent.file_type()?.is_dir()||!ent.path().join("SALIDA.json").exists(){continue}let p=ent.path();let nombre=ent.file_name().to_string_lossy().into_owned();let r=leer(&p.join("SALIDA.json"))?;
   exigir(r["tokens_generados"]==referencia["tokens_generados"]&&r["salida_sha256"]==referencia["salida_sha256"]&&r["salida_sha256"]==huella(r["texto"].as_str().ok_or("Texto ausente")?.as_bytes()),"Salida o tokens discordantes")?;
   let es_tpu=r["destino"]=="Tpu";exigir(es_tpu||r["destino"]=="Cpu","Destino desconocido")?;let observacion=traza(&p,es_tpu)?;
   let gen=numero(&r,"total_ns")?;let recorrido=if p.join("RECORRIDO.json").exists(){let m=leer(&p.join("RECORRIDO.json"))?;exigir(m["terminada"]==true&&m["generacion_ns"]==gen,"Recorrido discordante")?;let n=numero(&m,"solicitud_total_ns")?;exigir(n>=gen,"Intervalo exterior menor que generación")?;n}else{completo=false;gen};
   if nombre.starts_with("medicion-"){if es_tpu{tiempos_tpu.push(recorrido);generacion_tpu.push(gen);}else{tiempos_cpu.push(recorrido);generacion_cpu.push(gen);}}
   if nombre.starts_with("seleccion-"){let k=nombre.trim_start_matches("seleccion-");let d=leer(&b.join(format!("decision-{k}.json")))?;exigir(d["clase"]==r["destino"],"Destino ejecutado distinto del seleccionado")?;}
   let contador=leer(&p.join("CPU.json"))?;exigir(contador["inicio"]["pid"]==contador["fin"]["pid"],"Identidad CPU discordante")?;
   ejecuciones.push(json!({"ejecucion":nombre,"destino":r["destino"],"generacion_ns":gen,"solicitud_ns":recorrido,"cpu_proceso_ms":contador["intervalo"]["cpu_proceso_ms"],"traza":observacion}));total+=1;if es_tpu{tpu+=1}else{cpu+=1}
  }
  exigir(ejecuciones.len()==11&&tiempos_cpu.len()==3&&tiempos_tpu.len()==3,"Cobertura de casos incompleta")?;
  tiempos_cpu.sort();tiempos_tpu.sort();generacion_cpu.sort();generacion_tpu.sort();exigir(c["cpu_mediana_ns"]==tiempos_cpu[1]&&c["tpu_mediana_ns"]==tiempos_tpu[1],"Mediana discordante")?;
  exigir(leer(&b.join("decision-0.json"))?["clase"]=="Cpu"&&leer(&b.join("decision-1.json"))?["clase"]=="Tpu","Solicitud obligatoria incumplida")?;
  let esperado=if tiempos_cpu[1]<=tiempos_tpu[1]{"Cpu"}else{"Tpu"};exigir(leer(&b.join("decision-2.json"))?["clase"]==esperado,"Selección discordante con tiempos")?;
  let rechazo=leer(&b.join("RECHAZO-TPU-AUSENTE.json"))?;exigir(rechazo["rechazo_correcto"]==true&&rechazo["inferencia_ejecutada"]==false,"Rechazo lógico incorrecto")?;
  ejecuciones.sort_by_key(|v|v["ejecucion"].as_str().unwrap().to_owned());
  casos.push(json!({"caso":id,"traduccion":referencia["texto"],"tokens_entrada":lista(&referencia["tokens_entrada"])?.len(),"tokens_generados_sin_inicio_ni_fin":pasos-1,"pasos_incluido_fin":pasos,"medicion_completa":completo,"cpu_mediana_ns":tiempos_cpu[1],"tpu_mediana_ns":tiempos_tpu[1],"generacion_cpu_mediana_ns":generacion_cpu[1],"generacion_tpu_mediana_ns":generacion_tpu[1],"seleccion_automatica":esperado,"ejecuciones":ejecuciones}));
 }
 exigir(casos.len()==2&&total==22&&num.len()==56,"Cobertura total incorrecta")?;
 let muestras=leer(&base.join("MUESTRAS-CPU.json"))?;let ms=lista(&muestras)?;exigir(ms.len()>1,"Muestreo insuficiente")?;
 let mut intervalo=0;for par in ms.windows(2){exigir(par[0]["pid"]==par[1]["pid"],"Proceso de observación distinto")?;let a=numero(&par[0],"monotono_ns")?;let b=numero(&par[1],"monotono_ns")?;exigir(b>=a,"Tiempo no monótono")?;intervalo=intervalo.max(b-a);}
 let maximo=num.iter().map(|v|v["maxima_absoluta"].as_f64().unwrap()).fold(0f64,f64::max);let rmse=num.iter().map(|v|v["rmse_normalizada"].as_f64().unwrap()).fold(0f64,f64::max);
 let salida=json!({"conforme":true,"custodia":custodia,"ejecutable_sha256":inicio["ejecutable_sha256"],"generaciones":total,"cpu":cpu,"tpu":tpu,"comparaciones_vectoriales":num.len(),"maxima_absoluta":maximo,"rmse_normalizada_maxima":rmse,"muestras_cpu":ms.len(),"intervalo_maximo_cpu_ns":intervalo,"memoria_residente_maxima_bytes":ms.iter().map(|x|numero(x,"residente_bytes")).collect::<R<Vec<_>>>()?.into_iter().max(),"cpu_acumulada_ms":numero(ms.last().unwrap(),"cpu_acumulada_ms")?-numero(&ms[0],"cpu_acumulada_ms")?,"casos":casos,"alcance":"Cotejo de archivos, vectores originales, cobertura, atribución y selección; sin nuevas inferencias"});
 fs::write(&args[2],serde_json::to_vec_pretty(&salida)?)?;println!("Conforme: {total} generaciones, {cpu} CPU, {tpu} TPU; diferencia máxima {maximo}, RMSE {rmse}");Ok(())
}

#![forbid(unsafe_code)]
use anyhow::{Result,ensure,Context};
use candle::{Tensor,Device,DType};
use candle_nn::VarBuilder;
use candle_transformers::models::marian;
use serde::{Serialize,Deserialize};
use sha2::{Digest,Sha256};
use std::{collections::HashMap,fs,path::Path,time::Instant,io::Write};
mod grafo;
mod spm;
mod tpu;
mod serie;
const REV:&str="fdaddf76f50fcc1583ba42f95965862a7ab30f97";
const PESOS:&str="b3ecbf954573c2fd95d05d3ad4618baf961793db0da07c2add2e8a3a6cd78d0b";
const LIMITE:usize=64;
#[derive(Deserialize,Serialize,Clone)]
struct Entrada{id:String,texto:String}
#[derive(Serialize)]
struct Diferencia{maxima_absoluta:f64,rmse_normalizada:f64,conforme:bool}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn guardar(p:&Path,b:&[u8])->Result<()> {
    let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;
    ensure!(fs::read(p)?==b,"Recuperacion no conforme");Ok(())
}
fn json(p:&Path,v:&impl Serialize)->Result<()>{guardar(p,&serde_json::to_vec_pretty(v)?)}
fn tensor(p:&Path,t:&Tensor)->Result<Vec<f32>>{let v=t.flatten_all()?.to_vec1::<f32>()?;let b=v.iter().flat_map(|f|f.to_le_bytes()).collect::<Vec<_>>();guardar(p,&b)?;Ok(v)}
fn diferencia(a:&Tensor,b:&Tensor)->Result<Diferencia>{
    ensure!(a.dims()==b.dims(),"Dimensiones diferentes");let a=a.flatten_all()?.to_vec1::<f32>()?;let b=b.flatten_all()?.to_vec1::<f32>()?;
    ensure!(a.iter().chain(&b).all(|x|x.is_finite()),"Resultado no finito");
    let mut max=0f64;let mut err=0.;let mut ref2=0.;for (&a,&b)in a.iter().zip(&b){let d=(a as f64-b as f64).abs();max=max.max(d);err+=d*d;ref2+=(a as f64).powi(2)}
    let rmse=(err/a.len()as f64).sqrt()/(ref2/a.len()as f64).sqrt().max(1.);
    Ok(Diferencia{maxima_absoluta:max,rmse_normalizada:rmse,conforme:max<=0.005&&rmse<=0.0001})
}
fn ganador(t:&Tensor)->Result<u32>{let v=t.flatten_all()?.to_vec1::<f32>()?;ensure!(v.iter().all(|x|x.is_finite()),"Puntuaciones no finitas");Ok(v.iter().enumerate().max_by(|a,b|a.1.total_cmp(b.1).then_with(||b.0.cmp(&a.0))).context("Salida vacia")?.0 as u32)}
fn cargar(p:&Path)->Result<(marian::Config,HashMap<String,Tensor>,marian::MTModel)>{
    let m:serde_json::Value=serde_json::from_slice(&fs::read(p.join("MANIFIESTO.json"))?)?;ensure!(m["revision"]==REV,"Revision incorrecta");
    let files=m["archivos"].as_array().context("Manifiesto ausente")?;ensure!(files.len()==6,"Manifiesto incompleto");
    let mut seen=std::collections::BTreeSet::new();for a in files {let n=a["ruta"].as_str().context("Ruta ausente")?;
        ensure!(["config.json","vocab.json","source.spm","target.spm","model.safetensors","ficha-modelo.md"].contains(&n)&&seen.insert(n),"Ruta incorrecta");
        let bytes=fs::read(p.join(n))?;ensure!(a["sha256"]==sha(&bytes)&&a["bytes"]==bytes.len(),"Identidad no conforme: {n}");}
    let bytes=fs::read(p.join("model.safetensors"))?;ensure!(sha(&bytes)==PESOS,"Pesos incorrectos");
    let c:marian::Config=serde_json::from_slice(&fs::read(p.join("config.json"))?)?;
    ensure!(c.vocab_size==65001&&c.d_model==512&&c.encoder_layers==6&&c.decoder_layers==6,"Configuracion incompatible");
    let weights=candle::safetensors::load_buffer(&bytes,&Device::Cpu)?;
    let vb=VarBuilder::from_buffered_safetensors(bytes,DType::F32,&Device::Cpu)?;let model=marian::MTModel::new(&c,vb)?;
    Ok((c,weights,model))
}
fn casos(p:&Path)->Result<Vec<Entrada>> {
    let all=fs::read_to_string(p)?.lines().map(serde_json::from_str::<Entrada>).collect::<std::result::Result<Vec<_>,_>>()?;
    let r=all.into_iter().filter(|e|["NCI_4-0","NCI_178-3"].contains(&e.id.as_str())).collect::<Vec<_>>();
    ensure!(r.len()==2&&r[0].texto=="Hairy cell leukemia is a cancer of the blood and bone marrow."&&r[1].texto=="The leukemia cells can build up in the blood and bone marrow so there is less room for healthy white blood cells, red blood cells, and platelets.","Casos no conformes al protocolo");Ok(r)
}
fn manifiesto(raiz:&Path)->Result<()> {
    fn recorrer(raiz:&Path,p:&Path,v:&mut Vec<serde_json::Value>)->Result<()> {
        let mut entradas=fs::read_dir(p)?.collect::<std::result::Result<Vec<_>,_>>()?;entradas.sort_by_key(|x|x.file_name());
        for e in entradas {let tipo=e.file_type()?;ensure!(!tipo.is_symlink(),"Enlace no permitido en la entrega");if tipo.is_dir(){recorrer(raiz,&e.path(),v)?}else if tipo.is_file(){
            let b=fs::read(e.path())?;let ruta=e.path().strip_prefix(raiz)?.to_string_lossy().replace('\\',"/");v.push(serde_json::json!({"ruta":ruta,"bytes":b.len(),"sha256":sha(&b)}));}}
        Ok(())
    }
    let mut v=vec![];recorrer(raiz,raiz,&mut v)?;
    for a in &v {let b=fs::read(raiz.join(a["ruta"].as_str().unwrap()))?;ensure!(a["sha256"]==sha(&b),"Entrega alterada durante el cotejo");}
    json(&raiz.join("MANIFIESTO.json"),&serde_json::json!({"archivos":v,"cotejo_local":true}))
}
fn verificar(modelo:&Path,entradas:&Path,out:&Path)->Result<()> {
    ensure!(!out.exists(),"El destino debe ser nuevo");fs::create_dir_all(out)?;let comienzo=Instant::now();
    let (config,weights,mut original)=cargar(modelo)?;let enc=spm::tokenizer(modelo.join("source.spm"),modelo.join("vocab.json"))?;
    let dec=spm::tokenizer(modelo.join("target.spm"),modelo.join("vocab.json"))?;let casos=casos(entradas)?;
    json(&out.join("CASOS.json"),&casos)?;
    let mut resumen=vec![];
    for e in casos {
        let p=out.join(&e.id);fs::create_dir(&p)?;let inicio=Instant::now();
        let mut ids=enc.encode(e.texto.as_str(),false).map_err(anyhow::Error::msg)?.get_ids().to_vec();ensure!(ids.len()<=384,"Entrada excesiva");ids.push(config.eos_token_id);
        let ge=grafo::Graph::encoder(&weights,ids.len())?;let gd=grafo::Graph::decoder(&weights,ids.len(),LIMITE)?;
        guardar(&p.join("encoder.mlir"),ge.mlir().as_bytes())?;guardar(&p.join("decoder.mlir"),gd.mlir().as_bytes())?;
        let ti=Tensor::new(ids.as_slice(),&Device::Cpu)?;original.reset_kv_cache();
        let refenc=original.encoder().forward(&ti.unsqueeze(0)?,0)?;let mut inputs=HashMap::from([("ids".into(),ti)]);
        let encnuevo=ge.evaluate(&inputs)?;let d=diferencia(&refenc.squeeze(0)?,&encnuevo)?;
        json(&p.join("encoder-cotejo.json"),&d)?;tensor(&p.join("encoder-original.f32"),&refenc)?;tensor(&p.join("encoder-adaptado.f32"),&encnuevo)?;
        ensure!(d.conforme,"Codificador fuera de tolerancia");
        let mut tokens=vec![config.decoder_start_token_id];let mut pasos=vec![];let mut terminado=false;
        for i in 0..LIMITE {
            let paso=Instant::now();let original_log=original.decode(&Tensor::new(&tokens[i..],&Device::Cpu)?.unsqueeze(0)?,&refenc,i)?.squeeze(0)?;
            let mut padded=tokens.clone();padded.resize(LIMITE,config.pad_token_id);
            inputs.insert("ids".into(),Tensor::new(padded.as_slice(),&Device::Cpu)?);inputs.insert("encoder".into(),encnuevo.clone());inputs.insert("position".into(),Tensor::new(i as u32,&Device::Cpu)?);
            let nuevo=gd.evaluate(&inputs)?;let d=diferencia(&original_log,&nuevo)?;
            tensor(&p.join(format!("{i:03}-original.f32")),&original_log)?;tensor(&p.join(format!("{i:03}-adaptado.f32")),&nuevo)?;
            let a=ganador(&original_log)?;let b=ganador(&nuevo)?;
            let dato=serde_json::json!({"paso":i,"original":a,"adaptado":b,"diferencia":d,"duracion_ms":paso.elapsed().as_millis()});
            json(&p.join(format!("{i:03}-cotejo.json")),&dato)?;println!("{}",serde_json::json!({"caso":e.id,"paso":i,"conforme":d.conforme&&a==b,"max_abs":d.maxima_absoluta,"ms":paso.elapsed().as_millis()}));
            ensure!(d.conforme&&a==b,"Decodificador fuera de tolerancia en paso {i}");pasos.push(dato);
            if a==config.eos_token_id{terminado=true;break}ensure!(a!=config.pad_token_id,"Token no permitido");tokens.push(a);
        }
        ensure!(terminado,"Traduccion incompleta");let texto=dec.decode(&tokens[1..],true).map_err(anyhow::Error::msg)?;
        let s=serde_json::json!({"id":e.id,"texto_original":e.texto,"traduccion":texto,"tokens_entrada":ids,"tokens_generados":tokens,"pasos":pasos.len(),"duracion_ms":inicio.elapsed().as_millis(),"conforme":true});
        json(&p.join("RESULTADO.json"),&s)?;resumen.push(s);
    }
    json(&out.join("RESULTADO.json"),&serde_json::json!({"tipo":"contraste numerico CPU previo a TPU","conforme":true,"casos":resumen,"duracion_ms":comienzo.elapsed().as_millis(),"nuevas_inferencias_tpu":0,"selector_modificado":false}))?;Ok(())
}
fn main()->Result<()> {
    let a=std::env::args().collect::<Vec<_>>();
    ensure!((a.len()==5&&a[1]=="verificar-cpu")||(a.len()==6&&a[1]=="serie-tpu"),"Uso: verificar-cpu modelo entradas destino | serie-tpu modelo entradas libtpu destino");
    let out=Path::new(a.last().unwrap());ensure!(!out.exists(),"El destino debe ser nuevo");
    let parar=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));let p=parar.clone();
    let hilo=std::thread::spawn(move||->std::result::Result<Vec<sv_telemetria_cpu_tpu::MuestraCpu>,String>{
        let mut o=sv_telemetria_cpu_tpu::ObservadorCpu::propio()?;let mut v=vec![];
        while !p.load(std::sync::atomic::Ordering::Relaxed){v.push(o.muestra()?);std::thread::sleep(std::time::Duration::from_millis(100));}v.push(o.muestra()?);Ok(v)
    });
    let result=if a[1]=="verificar-cpu"{verificar(Path::new(&a[2]),Path::new(&a[3]),out)}else{serie::completa(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),out)};
    parar.store(true,std::sync::atomic::Ordering::Relaxed);
    let muestras=hilo.join().map_err(|_|anyhow::anyhow!("Fallo del observador CPU"))?.map_err(anyhow::Error::msg)?;
    if out.is_dir(){json(&out.join("MUESTRAS-CPU.json"),&muestras)?;if let Err(e)=&result{json(&out.join("ERROR.json"),&serde_json::json!({"conforme":false,"error":format!("{e:#}")}))?;}manifiesto(out)?;}
    result
}

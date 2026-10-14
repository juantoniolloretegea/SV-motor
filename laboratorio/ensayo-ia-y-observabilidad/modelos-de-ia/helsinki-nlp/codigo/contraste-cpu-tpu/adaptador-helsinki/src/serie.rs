use crate::*;
use crate::tpu::{Entorno,Programa};
use selector_arbitro_director_cpu_gpu_tpu_npu as selector;
use selector::{Clase,Destino,Carga,Capacidad,Cobertura,Estado,Recurso,Solicitud,Politica};
use sv_telemetria_cpu_tpu::{ObservadorCpu,intervalo_cpu};

fn medir<T>(o:&mut ObservadorCpu,fases:&mut Vec<serde_json::Value>,nombre:&str,f:impl FnOnce()->Result<T>)->Result<T>{
    let a=o.muestra().map_err(anyhow::Error::msg)?;let start=Instant::now();let r=f();let ns=start.elapsed().as_nanos();let b=o.muestra().map_err(anyhow::Error::msg)?;
    fases.push(serde_json::json!({"fase":nombre,"duracion_ns":ns,"cpu_inicial":a,"cpu_final":b,"intervalo":intervalo_cpu(&a,&b).map_err(anyhow::Error::msg)?,"terminada":r.is_ok()}));r
}
struct CasoPreparado {entrada:Entrada,encoder:Programa,decoder:Programa}
struct Motor {original:marian::MTModel,config:marian::Config,enc:tokenizers::Tokenizer,dec:tokenizers::Tokenizer}

fn generar(m:&mut Motor,e:&Entorno,c:&CasoPreparado,clase:Clase,p:&Path,referencia:Option<&Path>,conservar_encoder_cpu:bool,o:&mut ObservadorCpu)->Result<serde_json::Value>{
    let mut fases=vec![];let cpu_inicio=o.muestra().map_err(anyhow::Error::msg)?;let comienzo=Instant::now();
    let result=(||->Result<serde_json::Value>{
        let ids=medir(o,&mut fases,"tokenizacion_cpu",||{let mut ids=m.enc.encode(c.entrada.texto.as_str(),false).map_err(anyhow::Error::msg)?.get_ids().to_vec();ensure!(ids.len()<=384,"Entrada excesiva");ids.push(m.config.eos_token_id);Ok(ids)})?;
        let ti=Tensor::new(ids.as_slice(),&Device::Cpu)?;let mut enc_cpu=None;let mut enc_tpu=None;
        match clase {
            Clase::Cpu=>{m.original.reset_kv_cache();enc_cpu=Some(medir(o,&mut fases,"codificador_cpu",||Ok(m.original.encoder().forward(&ti.unsqueeze(0)?,0)?))?);},
            Clase::Tpu=>{let b=medir(o,&mut fases,"transferencia_entrada_ids",||e.subir(&ti))?;enc_tpu=Some(medir(o,&mut fases,"codificador_tpu_sincrono",||c.encoder.ejecutar(&HashMap::from([("ids".into(),&b)])))?);},
            _=>anyhow::bail!("Dispositivo no implementado; sin sustitucion"),
        }
        // La recuperación del codificador sólo se hace en la comprobación numérica.
        if let Some(dir)=referencia {
            let enc=tpu::recuperar(enc_tpu.as_ref().context("Falta salida TPU")?)?;
            let original=leer_tensor(&dir.join("encoder.f32"),enc.dims())?;let d=diferencia(&original,&enc)?;
            tensor(&p.join("encoder.f32"),&enc)?;json(&p.join("encoder-cotejo.json"),&d)?;ensure!(d.conforme,"Codificador TPU fuera de tolerancia");
        }else if conservar_encoder_cpu {ensure!(clase==Clase::Cpu,"Referencia CPU incompatible");tensor(&p.join("encoder.f32"),enc_cpu.as_ref().unwrap())?;}
        let mut tokens=vec![m.config.decoder_start_token_id];let mut fin=false;let mut diferencias=vec![];
        for i in 0..LIMITE {
            let logits=match clase {
                Clase::Cpu=>medir(o,&mut fases,"decodificador_cpu",||Ok(m.original.decode(&Tensor::new(&tokens[i..],&Device::Cpu)?.unsqueeze(0)?,enc_cpu.as_ref().unwrap(),i)?.squeeze(0)?))?,
                Clase::Tpu=>{
                    let mut prefijo=tokens.clone();prefijo.resize(LIMITE,m.config.pad_token_id);
                    let (b,pos)=medir(o,&mut fases,"transferencia_entrada_prefijo_posicion",||Ok((e.subir(&Tensor::new(prefijo.as_slice(),&Device::Cpu)?)?,e.subir(&Tensor::new(i as u32,&Device::Cpu)?)?)))?;
                    let args=HashMap::from([("ids".into(),&b),("position".into(),&pos),("encoder".into(),enc_tpu.as_ref().unwrap())]);
                    let salida=medir(o,&mut fases,"decodificador_tpu_sincrono",||c.decoder.ejecutar(&args))?;
                    medir(o,&mut fases,"transferencia_salida_puntuaciones",||tpu::recuperar(&salida))?
                },_=>unreachable!()
            };
            let token=medir(o,&mut fases,"seleccion_token_cpu",||ganador(&logits))?;
            tensor(&p.join(format!("{i:03}.f32")),&logits)?;
            if let Some(r)=referencia {let original=leer_tensor(&r.join(format!("{i:03}.f32")),logits.dims())?;let d=diferencia(&original,&logits)?;
                json(&p.join(format!("{i:03}-cotejo.json")),&d)?;ensure!(d.conforme&&token==ganador(&original)?,"TPU fuera de tolerancia o token distinto en {i}");diferencias.push(d);}
            if token==m.config.eos_token_id{fin=true;break}ensure!(token!=m.config.pad_token_id,"Token no admitido");tokens.push(token);
        }
        ensure!(fin,"Traduccion incompleta; ensayo detenido");let texto=medir(o,&mut fases,"destokenizacion_cpu",||m.dec.decode(&tokens[1..],true).map_err(anyhow::Error::msg))?;
        Ok(serde_json::json!({"id":c.entrada.id,"destino":format!("{clase:?}"),"entrada":c.entrada.texto,"entrada_sha256":sha(c.entrada.texto.as_bytes()),"texto":texto,"salida_sha256":sha(texto.as_bytes()),"tokens_entrada":ids,"tokens_generados":tokens,"numerica":diferencias,"total_ns":comienzo.elapsed().as_nanos()}))
    })();
    let cpu_fin=o.muestra().map_err(anyhow::Error::msg)?;json(&p.join("FASES.json"),&fases)?;
    json(&p.join("CPU.json"),&serde_json::json!({"inicio":cpu_inicio,"fin":cpu_fin,"intervalo":intervalo_cpu(&cpu_inicio,&cpu_fin).map_err(anyhow::Error::msg)?}))?;
    let r=result?;json(&p.join("SALIDA.json"),&r)?;Ok(r)
}
fn leer_tensor(p:&Path,dims:&[usize])->Result<Tensor>{let b=fs::read(p)?;ensure!(b.len()%4==0,"Tensor incompleto");let v=b.chunks_exact(4).map(|x|f32::from_le_bytes(x.try_into().unwrap())).collect::<Vec<_>>();Ok(Tensor::from_vec(v,dims,&Device::Cpu)?)}
fn ejecutar(m:&mut Motor,e:&Entorno,c:&CasoPreparado,clase:Clase,p:&Path,referencia:Option<&Path>,conservar_encoder_cpu:bool,o:&mut ObservadorCpu)->Result<serde_json::Value>{
    let inicio=Instant::now();
    fs::create_dir(p)?;let nombres=if clase==Clase::Tpu{vec![c.encoder.nombre.clone(),c.decoder.nombre.clone()]}else{vec![]};
    let resultado=tpu::observar(e,p,&nombres,||generar(m,e,c,clase,p,referencia,conservar_encoder_cpu,o));
    let total=inicio.elapsed().as_nanos();
    json(&p.join("RECORRIDO.json"),&serde_json::json!({"solicitud_total_ns":total,"generacion_ns":resultado.as_ref().ok().map(|r|r["total_ns"].clone()),"terminada":resultado.is_ok(),"incluye":"creacion de salida, inicio del observador, generacion, transferencias, parada, recuperacion, analisis y conservacion de trazas","excluye":"carga y compilacion previas; escritura de este registro de medida"}))?;
    let mut r=resultado?;r["recorrido_total_ns"]=serde_json::json!(total);Ok(r)
}
fn memoria_cpu()->Result<u64>{let s=fs::read_to_string("/proc/meminfo")?;let v=s.lines().find(|l|l.starts_with("MemAvailable:")).and_then(|l|l.split_whitespace().nth(1)).context("Memoria CPU desconocida")?;Ok(v.parse::<u64>()?*1024)}
fn mediana(v:&mut [u64])->u64{v.sort_unstable();v[v.len()/2]}

pub fn completa(modelo:&Path,entradas:&Path,biblioteca:&Path,out:&Path)->Result<()> {
    ensure!(!out.exists(),"El destino debe ser nuevo");fs::create_dir_all(out)?;
    ensure!(sha(include_bytes!("../../src/lib.rs"))=="2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05","El Selector ha cambiado");
    json(&out.join("INICIO.json"),&serde_json::json!({"modelo":"Helsinki-NLP/opus-mt-en-es","revision":REV,"pesos_sha256":PESOS,"selector_sha256":sha(include_bytes!("../../src/lib.rs")),"ejecutable_sha256":sha(&fs::read(std::env::current_exe()?)?),"inicio_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_millis(),"generaciones_previstas":22,"limite_tokens":LIMITE,"precision":"F32/HIGHEST","calculo_tpu":"Marian completo, prefijo causal sin cache incremental"}))?;
    let carga=Instant::now();let(config,weights,original)=cargar(modelo)?;
    let mut motor=Motor{original,config,enc:spm::tokenizer(modelo.join("source.spm"),modelo.join("vocab.json"))?,dec:spm::tokenizer(modelo.join("target.spm"),modelo.join("vocab.json"))?};
    json(&out.join("CARGA-CPU.json"),&serde_json::json!({"duracion_ns":carga.elapsed().as_nanos()}))?;
    let entorno=Entorno::abrir(biblioteca,out)?;let casos=casos(entradas)?;json(&out.join("CASOS.json"),&casos)?;
    let mut obs=ObservadorCpu::propio().map_err(anyhow::Error::msg)?;let origen=Instant::now();let mut resultados=vec![];
    for entrada in casos {
        let p=out.join(&entrada.id);fs::create_dir(&p)?;let n=motor.enc.encode(entrada.texto.as_str(),false).map_err(anyhow::Error::msg)?.len()+1;
        let ge=grafo::Graph::encoder(&weights,n)?;let gd=grafo::Graph::decoder(&weights,n,LIMITE)?;
        let c=CasoPreparado{entrada,encoder:Programa::compilar(&entorno,&ge,&p.join("encoder"))?,decoder:Programa::compilar(&entorno,&gd,&p.join("decoder"))?};
        let referencia=ejecutar(&mut motor,&entorno,&c,Clase::Cpu,&p.join("referencia-cpu"),None,true,&mut obs)?;
        let validacion=ejecutar(&mut motor,&entorno,&c,Clase::Tpu,&p.join("validacion-tpu"),Some(&p.join("referencia-cpu")),false,&mut obs)?;
        ensure!(referencia["tokens_generados"]==validacion["tokens_generados"]&&referencia["salida_sha256"]==validacion["salida_sha256"],"Resultado global distinto");
        let mut tiempos_cpu=vec![];let mut tiempos_tpu=vec![];
        for i in 0..3{for clase in [Clase::Cpu,Clase::Tpu]{let r=ejecutar(&mut motor,&entorno,&c,clase,&p.join(format!("medicion-{clase:?}-{i}")),None,false,&mut obs)?;
            ensure!(r["tokens_generados"]==referencia["tokens_generados"]&&r["salida_sha256"]==referencia["salida_sha256"],"Resultado distinto en medicion");
            if clase==Clase::Cpu{tiempos_cpu.push(r["recorrido_total_ns"].as_u64().context("Tiempo ausente")?)}else{tiempos_tpu.push(r["recorrido_total_ns"].as_u64().context("Tiempo ausente")?)}}}
        let cpu_ns=mediana(&mut tiempos_cpu);let tpu_ns=mediana(&mut tiempos_tpu);
        let carga=Carga{contrato:"traduccion-marian-en-es".into(),revision:format!("{REV}:{PESOS}"),perfil:format!("F32-greedy-prefijo64-caso-{}",sha(c.entrada.texto.as_bytes())),dimensiones:vec![n,LIMITE,512]};
        let cpu=Destino{servidor:"servidor-google-cpu-tpu-del-ensayo".into(),dispositivo:"cpu-anfitrion".into(),adaptador_revision:"helsinki-01".into()};
        let tpu=Destino{servidor:cpu.servidor.clone(),dispositivo:"tpu-fisica-0".into(),adaptador_revision:"helsinki-01".into()};
        let evidencia=sha(&fs::read(p.join("validacion-tpu/SALIDA.json"))?);
        let mut decisiones=vec![];
        for (i,obligatoria) in [Some(Clase::Cpu),Some(Clase::Tpu),None].into_iter().enumerate(){
            let ahora=origen.elapsed().as_millis() as u64;let mt=entorno.dispositivo.memory_stats()?;let libre=u64::try_from(mt.bytes_limit.context("Limite TPU desconocido")?-mt.bytes_in_use)?;
            let recursos=[(cpu.clone(),Clase::Cpu,memoria_cpu()?,cpu_ns),(tpu.clone(),Clase::Tpu,libre,tpu_ns)].into_iter().map(|(destino,clase,memoria,ns)|Recurso{destino,clase,estado:Estado::Comprobado,observado_ms:ahora,memoria_disponible_bytes:Some(memoria),disponible:true,capacidades:vec![Capacidad{carga:carga.clone(),cobertura:Cobertura::Completa,evidencia:evidencia.clone(),estimacion_total_us:Some(ns.div_ceil(1000))}]}).collect::<Vec<_>>();
            let sol=Solicitud{id:format!("{}-{i}",c.entrada.id),carga:carga.clone(),memoria_adicional_bytes:1024*1024*1024,margen_bytes:256*1024*1024,residencia_obligatoria:None};
            let pol=Politica{revision:"helsinki-cpu-tpu-01".into(),destinos_permitidos:vec![cpu.clone(),tpu.clone()],preferencia:vec![Clase::Cpu,Clase::Tpu],clase_obligatoria:obligatoria,vigencia_ms:1000,comparar_estimaciones:true};
            let decision=selector::seleccionar(&sol,&pol,&recursos,ahora).map_err(|e|anyhow::anyhow!("Seleccion rechazada: {e:?}"))?;
            let d=serde_json::json!({"solicitud":format!("{sol:?}"),"politica":format!("{pol:?}"),"recursos":format!("{recursos:?}"),"decision":format!("{decision:?}"),"clase":format!("{:?}",decision.clase),"cpu_mediana_ns":cpu_ns,"tpu_mediana_ns":tpu_ns});
            json(&p.join(format!("decision-{i}.json")),&d)?;
            let r=ejecutar(&mut motor,&entorno,&c,decision.clase,&p.join(format!("seleccion-{i}")),None,false,&mut obs)?;
            ensure!(r["tokens_generados"]==referencia["tokens_generados"]&&r["salida_sha256"]==referencia["salida_sha256"],"Resultado distinto tras seleccion");
            decisiones.push(d);
            if i==1 {let mut sin_tpu=recursos.clone();sin_tpu[1].disponible=false;ensure!(selector::seleccionar(&sol,&pol,&sin_tpu,ahora).is_err(),"No se rechazo TPU ausente");json(&p.join("RECHAZO-TPU-AUSENTE.json"),&serde_json::json!({"rechazo_correcto":true,"inferencia_ejecutada":false}))?;}
        }
        resultados.push(serde_json::json!({"caso":c.entrada.id,"conforme":true,"cpu_mediana_ns":cpu_ns,"tpu_mediana_ns":tpu_ns,"decisiones":decisiones}));
    }
    json(&out.join("RESULTADO.json"),&serde_json::json!({"conforme":true,"casos":resultados,"generaciones":22,"decisiones_ejecutadas":6,"rechazos_logicos":2,"nodo1_validado":false,"calidad_general_validada":false}))?;Ok(())
}

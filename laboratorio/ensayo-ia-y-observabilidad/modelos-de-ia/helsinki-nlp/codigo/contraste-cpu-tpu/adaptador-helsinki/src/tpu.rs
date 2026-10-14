//! Ejecución PJRT con parámetros residentes y préstamos sin donación.
use anyhow::{Result,ensure,Context};
use candle::Tensor;
use pjrt::{Api,Buffer,Client,Device,ExecutionInputs,HostBuffer,LoadedExecutable,ProfilerExtension};
use prost::Message;
use std::{collections::HashMap,path::Path,time::Instant};
use crate::{grafo::Graph,guardar,json,sha};
pub mod trazas {include!(concat!(env!("OUT_DIR"),"/tensorflow.profiler.rs"));}
#[allow(dead_code)]
mod opciones {include!(concat!(env!("OUT_DIR"),"/tensorflow.rs"));}
struct Prestados<'a>(Vec<&'a Buffer>);
impl ExecutionInputs for Prestados<'_> {
    fn buffer_ptrs(&self)->Vec<Vec<*mut pjrt_sys::PJRT_Buffer>> {vec![self.0.iter().map(|b|b.buffer_ptrs()[0][0]).collect()]}
    fn non_donatable_input_indices(&self)->Vec<i64>{(0..self.0.len() as i64).collect()}
}
pub struct Entorno {pub api:Api,pub cliente:Client,pub dispositivo:Device}
impl Entorno {
    pub fn abrir(biblioteca:&Path,out:&Path)->Result<Self>{
        let api=pjrt::plugin(biblioteca.to_str().context("Ruta invalida")?).load()?;
        let cliente=Client::builder(&api).build()?;ensure!(cliente.platform_name()?.to_lowercase()=="tpu","Plataforma distinta de TPU");
        let devices=cliente.addressable_devices()?;ensure!(devices.len()==1,"Se requiere una sola TPU");let dispositivo=devices.into_iter().next().context("TPU ausente")?;
        ensure!(dispositivo.description()?.id()?==0,"Dispositivo no previsto");
        json(&out.join("DISPOSITIVO.json"),&serde_json::json!({"plataforma":cliente.platform_name()?,"clase":dispositivo.description()?.kind()?,"id":dispositivo.description()?.id()?,"memoria":format!("{:?}",dispositivo.memory_stats()?),"biblioteca_sha256":sha(&std::fs::read(biblioteca)?)}))?;
        Ok(Self{api,cliente,dispositivo})
    }
    pub fn subir(&self,t:&Tensor)->Result<Buffer>{
        let dims=Some(t.dims().iter().map(|&x|x as i64).collect::<Vec<_>>());
        let b=if t.dtype()==candle::DType::U32 {
            let v=t.flatten_all()?.to_vec1::<u32>()?.into_iter().map(|i|i32::try_from(i)).collect::<std::result::Result<Vec<_>,_>>()?;
            HostBuffer::from_data(v,dims,None).to_sync(&self.dispositivo).copy()?
        }else{HostBuffer::from_data(t.flatten_all()?.to_vec1::<f32>()?,dims,None).to_sync(&self.dispositivo).copy()?};
        ensure!(b.device()?.description()?.id()?==0&&!b.is_on_cpu()?,"Parametro fuera de TPU");Ok(b)
    }
}
pub struct Programa {exec:LoadedExecutable,pesos:HashMap<String,Buffer>,orden:Vec<String>,pub nombre:String}
impl Programa {
    pub fn compilar(e:&Entorno,g:&Graph,out:&Path)->Result<Self>{
        std::fs::create_dir(out)?;let codigo=g.mlir();guardar(&out.join("PROGRAMA.mlir"),codigo.as_bytes())?;
        let inicio=Instant::now();let p=pjrt::Program::new(pjrt::ProgramFormat::MLIR,codigo.as_bytes());let exec=LoadedExecutable::builder(&e.cliente,&p).build()?;
        let compilacion=inicio.elapsed().as_nanos();let nombre=exec.executable()?.name()?.to_string();
        let inicio=Instant::now();let mut pesos=HashMap::new();let mut bytes=0u64;
        for(k,_)in &g.params{if let Some(t)=g.weights.get(k){bytes+=t.elem_count() as u64*4;pesos.insert(k.clone(),e.subir(t)?);}}
        let m=exec.executable()?.compiled_memory_stats()?;
        json(&out.join("COMPILACION.json"),&serde_json::json!({"compilacion_ns":compilacion,"transferencia_inicial_ns":inicio.elapsed().as_nanos(),"bytes_parametros_solicitados":bytes,"nombre_efectivo":nombre,"programa_sha256":sha(codigo.as_bytes()),"memoria_compilada":{"codigo_dispositivo":m.generated_code_size_in_bytes,"argumentos_dispositivo":m.argument_size_in_bytes,"salida_dispositivo":m.output_size_in_bytes,"alias_dispositivo":m.alias_size_in_bytes,"temporal_dispositivo":m.temp_size_in_bytes,"codigo_cpu":m.host_generated_code_size_in_bytes,"argumentos_cpu":m.host_argument_size_in_bytes,"salida_cpu":m.host_output_size_in_bytes,"alias_cpu":m.host_alias_size_in_bytes,"temporal_cpu":m.host_temp_size_in_bytes},"huella_nativa":exec.executable()?.fingerprint_bytes()?}))?;
        Ok(Self{exec,pesos,orden:g.params.iter().map(|(k,_)|k.clone()).collect(),nombre})
    }
    pub fn ejecutar(&self,dinamicos:&HashMap<String,&Buffer>)->Result<Buffer>{
        let args=self.orden.iter().map(|k|dinamicos.get(k).copied().or_else(||self.pesos.get(k)).with_context(||format!("Parametro ausente: {k}"))).collect::<Result<Vec<_>>>()?;
        let mut salida=self.exec.execution(Prestados(args)).run_sync()?;
        ensure!(salida.len()==1&&salida[0].len()==1,"Numero de salidas no previsto");let b=salida.remove(0).remove(0);
        ensure!(b.device()?.description()?.id()?==0&&!b.is_on_cpu()?,"Salida fuera de TPU");Ok(b)
    }
}
pub fn recuperar(b:&Buffer)->Result<Tensor>{
    let dims=b.dims()?.into_iter().map(usize::try_from).collect::<std::result::Result<Vec<_>,_>>()?;
    let host=b.to_host_sync(None)?;let HostBuffer::F32(v)=host else{anyhow::bail!("Tipo de salida distinto de F32")};
    Ok(Tensor::from_vec(v.data().to_vec(),dims,&candle::Device::Cpu)?)
}
fn opciones()->Vec<u8>{
    opciones::ProfileOptions{version:1,device_type:3,host_tracer_level:3,device_tracer_level:1,enable_hlo_proto:true,raise_error_on_start_failure:true,
        advanced_configuration:HashMap::from([("tpu_trace_mode".into(),opciones::profile_options::AdvancedConfigValue{value:Some(opciones::profile_options::advanced_config_value::Value::StringValue("TRACE_COMPUTE_AND_SYNC".into()))})]),..Default::default()}.encode_to_vec()
}
pub fn observar<T>(e:&Entorno,out:&Path,programas:&[String],f:impl FnOnce()->Result<T>)->Result<T>{
    let ext=e.api.get_extension::<ProfilerExtension>().context("Observador ausente")?;let api=ext.profiler_api().context("Interfaz de observacion ausente")?;
    let opts=opciones();guardar(&out.join("OPCIONES.pb"),&opts)?;let mut p=api.create(std::str::from_utf8(&opts)?)?;p.start()?;
    let result=f();let detener=p.stop();detener?;let bytes=p.collect_data_serialized()?;
    guardar(&out.join("TRAZA.xplane.pb"),&bytes)?;let t=trazas::XSpace::decode(bytes.as_slice())?;json(&out.join("TRAZA.json"),&t)?;
    let atribucion=analizar(&t,programas);match &atribucion{Ok(v)=>json(&out.join("ATRIBUCION.json"),v)?,Err(er)=>json(&out.join("ERROR-ATRIBUCION.json"),&serde_json::json!({"error":format!("{er:#}")}))?};
    let r=result?;atribucion?;Ok(r)
}
fn analizar(t:&trazas::XSpace,nombres:&[String])->Result<serde_json::Value>{
    ensure!(t.errors.is_empty()&&t.warnings.is_empty(),"Traza con errores o avisos");
    ensure!(t.planes.iter().any(|p|p.name=="/host:CPU"&&p.lines.iter().any(|l|!l.events.is_empty())),"Falta actividad del anfitrion");
    let mut modulos=vec![];let mut ops=vec![];let mut ventanas=vec![];let mut ids=std::collections::BTreeSet::new();let mut hallados=std::collections::BTreeSet::new();
    for p in &t.planes{if p.name.starts_with("/device:TPU:"){ensure!(p.name=="/device:TPU:0","Actividad en otra TPU");for l in &p.lines{if l.name=="XLA Modules"{for e in &l.events{
        let m=p.event_metadata.get(&e.metadata_id).context("Modulo sin metadatos")?;
        let nombre=nombres.iter().find(|n|m.name.starts_with(&format!("{n}("))).context("Programa TPU ajeno a la solicitud")?;
        let id=m.name.strip_prefix(&format!("{nombre}(" )).and_then(|x|x.strip_suffix(')')).context("Identidad de programa ilegible")?.parse::<u64>()?;
        ids.insert(id);hallados.insert(nombre.clone());modulos.push(serde_json::json!({"nombre":m.name,"evento":e,"linea_timestamp_ns":l.timestamp_ns}));
    }}}}}
    for p in &t.planes{if p.name=="/device:TPU:0"{for l in &p.lines{if l.name=="XLA Ops"{for e in &l.events{
        let m=p.event_metadata.get(&e.metadata_id).context("Operacion sin metadatos")?;
        let id=numero(p,&m.stats,"program_id").context("Operacion sin identidad de programa")?;ensure!(ids.contains(&id),"Operacion ajena al modelo");
        let calculo=!m.display_name.starts_with("copy")&&!m.display_name.starts_with("async-copy");
        if calculo{let Some(trazas::x_event::Data::OffsetPs(inicio))=e.data else{anyhow::bail!("Evento sin tiempo")};ensure!(inicio>=0&&e.duration_ps>0,"Intervalo TPU invalido");let ini=(l.timestamp_ns as i128)*1000+inicio as i128;ventanas.push((ini,ini+e.duration_ps as i128));}
        ops.push(serde_json::json!({"nombre":m.name,"presentacion":m.display_name,"calculo":calculo,"programa":id,"evento":e,"linea_timestamp_ns":l.timestamp_ns}));
    }}}}}
    ensure!(nombres.iter().all(|n|hallados.contains(n)),"Falta atribucion de un programa del modelo");
    if !nombres.is_empty(){ensure!(!ventanas.is_empty(),"No hay calculo TPU desglosado");}
    let mut transfers=vec![];
    for p in &t.planes{if p.name=="/host:CPU"{for l in &p.lines{for e in &l.events{if let Some(m)=p.event_metadata.get(&e.metadata_id){if ["tpu::System::TransferToDevice=>IssueEvent=>Done","tpu::System::TransferFromDevice=>IssueEvent=>Done"].contains(&m.name.as_str()){
        ensure!(!nombres.is_empty(),"Transferencia TPU en operacion CPU");let bytes=numero(p,&e.stats,"size").context("Transferencia sin tamano observado")?;ensure!(numero(p,&e.stats,"chip_id")==Some(0),"Transferencia en otro chip");
        transfers.push(serde_json::json!({"tipo":m.name,"bytes":bytes,"evento":e,"linea_timestamp_ns":l.timestamp_ns}));
    }}}}}}
    if !nombres.is_empty(){ensure!(!transfers.is_empty(),"Transferencias sin observacion");}
    ventanas.sort_unstable();let mut fin=i128::MIN;let mut union=0i128;for (a,b) in ventanas{if b>fin{union+=b-a.max(fin);fin=b;}}
    Ok(serde_json::json!({"programas":hallados,"modulos":modulos,"operaciones":ops,"calculo_tpu_union_ps":union,"transferencias":transfers,"energia_medida":false,"trafico_fisico_bus_medido":false}))
}
fn numero(p:&trazas::XPlane,s:&[trazas::XStat],n:&str)->Option<u64>{s.iter().find_map(|s|{if p.stat_metadata.get(&s.metadata_id)?.name!=n{return None}match s.value{Some(trazas::x_stat::Value::Uint64Value(v))=>Some(v),Some(trazas::x_stat::Value::Int64Value(v))=>v.try_into().ok(),_=>None}})}

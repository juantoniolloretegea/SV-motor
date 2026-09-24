//! Instrumentación OpenTelemetry 0.31.0, con exportación local acotada.
//! Las trazas describen puntos instrumentados; no prueban ausencia de efectos exteriores.
use crate::{store, Result};
use opentelemetry::{Context, KeyValue, trace::{Span, TraceContextExt, Tracer, TracerProvider}};
use opentelemetry_sdk::{error::{OTelSdkError, OTelSdkResult}, trace::{Sampler, SdkTracerProvider, SpanData, SpanExporter}};
use serde_json::{json, Value};
use std::{fs::{self, File, OpenOptions}, io::Write, os::unix::fs::OpenOptionsExt, path::{Path, PathBuf}, sync::{Arc, Mutex}, time::{Instant, UNIX_EPOCH}};

pub const LIMIT: u64 = 10 * 1024 * 1024;
#[derive(Debug, Default)]
struct Counters { bytes:u64, records:u64, errors:u64, dropped:u64, last_ms:Option<u128> }
#[derive(Debug)]
struct Sink { file: File, counts: Counters, limit:u64, failed:bool }
#[derive(Clone, Debug)]
struct Exporter { sink:Arc<Mutex<Sink>>, instance:String }
impl SpanExporter for Exporter {
    async fn export(&self, batch:Vec<SpanData>)->OTelSdkResult {
        let mut sink=self.sink.lock().map_err(|_|OTelSdkError::InternalFailure("Bloqueo de exportación".into()))?;
        for span in batch {
            let attrs:serde_json::Map<String,Value>=span.attributes.iter().map(|kv|(kv.key.to_string(),json!(kv.value.to_string()))).collect();
            let row=json!({"schema":"EIO-OTEL-1","instance":self.instance,"trace_id":span.span_context.trace_id().to_string(),
                "span_id":span.span_context.span_id().to_string(),"parent_span_id":span.parent_span_id.to_string(),"name":span.name,
                "start_unix_ns":span.start_time.duration_since(UNIX_EPOCH).ok().map(|d|d.as_nanos()),
                "end_unix_ns":span.end_time.duration_since(UNIX_EPOCH).ok().map(|d|d.as_nanos()),
                "attributes":attrs,"dropped_attributes":span.dropped_attributes_count,"dropped_events":span.events.dropped_count});
            let mut bytes=serde_json::to_vec(&row).map_err(|_|OTelSdkError::InternalFailure("Serialización".into()))?;bytes.push(b'\n');
            if sink.failed || bytes.len()>32768 || sink.counts.bytes.saturating_add(bytes.len() as u64)>sink.limit {
                sink.counts.dropped+=1;sink.failed=true;continue;
            }
            if sink.file.write_all(&bytes).and_then(|_|sink.file.sync_data()).is_err(){sink.counts.errors+=1;sink.failed=true;continue;}
            sink.counts.bytes+=bytes.len() as u64;sink.counts.records+=1;sink.counts.last_ms=Some(store::now());
        }
        if sink.failed {Err(OTelSdkError::InternalFailure("Exportación no íntegra; consulte los contadores".into()))}else{Ok(())}
    }
}

#[derive(Clone)]
pub struct Telemetry { provider:SdkTracerProvider, sink:Arc<Mutex<Sink>>, path:PathBuf }
pub struct Operation { context:Context, trace_id:String, started:Instant }
impl Operation { pub fn trace_id(&self)->&str{&self.trace_id} }
impl Telemetry {
    pub fn new(dir:&Path, instance:&str, limit:u64)->Result<Self>{
        fs::create_dir_all(dir)?;
        {use std::os::unix::fs::PermissionsExt;fs::set_permissions(dir,fs::Permissions::from_mode(0o700))?;}
        let path=dir.join("trazas.jsonl");
        let file=OpenOptions::new().write(true).create_new(true).mode(0o600).open(&path)?;
        let sink=Arc::new(Mutex::new(Sink{file,counts:Counters::default(),limit,failed:false}));
        let provider=SdkTracerProvider::builder().with_sampler(Sampler::AlwaysOn)
            .with_max_attributes_per_span(16).with_max_events_per_span(0)
            .with_simple_exporter(Exporter{sink:sink.clone(),instance:instance.into()}).build();
        Ok(Self{provider,sink,path})
    }
    pub fn begin(&self,name:&'static str,metadata:Value)->Operation{
        let mut span=self.provider.tracer("eio-conversacion-0.2.0").start(name);
        span.set_attribute(KeyValue::new("metadata_json",metadata.to_string()));
        let trace_id=span.span_context().trace_id().to_string();
        Operation{context:Context::current_with_span(span),trace_id,started:Instant::now()}
    }
    pub fn event(&self,parent:Option<&Operation>,name:&'static str,metadata:Value){
        let empty=Context::new();let ctx=parent.map(|p|&p.context).unwrap_or(&empty);
        let mut span=self.provider.tracer("eio-conversacion-0.2.0").start_with_context(name,ctx);
        span.set_attribute(KeyValue::new("metadata_json",metadata.to_string()));span.end();
    }
    pub fn end(&self,operation:Operation,metadata:Value)->Value{
        operation.context.span().set_attribute(KeyValue::new("duration_monotonic_seconds",operation.started.elapsed().as_secs_f64()));
        operation.context.span().set_attribute(KeyValue::new("result_json",metadata.to_string()));
        operation.context.span().end();let _=self.provider.force_flush();
        json!({"trace_id":operation.trace_id,"export":self.status(),"scope":"Puntos del supervisor instrumentados; no observación exhaustiva ni validación de contenido."})
    }
    pub fn healthy(&self)->bool { self.sink.lock().map(|s|!s.failed).unwrap_or(false) }
    pub fn status(&self)->Value{
        match self.sink.lock(){Ok(s)=>json!({"enabled":true,"ok":!s.failed,"sdk":"OpenTelemetry Rust 0.31.0","export":"archivo_local_sin_red",
            "file":self.path.file_name().and_then(|s|s.to_str()),"bytes":s.counts.bytes,"limit_bytes":s.limit,"records":s.counts.records,
            "errors":s.counts.errors,"dropped":s.counts.dropped,"last_export_utc_ms":s.counts.last_ms}),Err(_)=>json!({"enabled":true,"ok":false,"error":"Bloqueo no disponible"})}
    }
    pub fn shutdown(&self)->bool { self.provider.shutdown().is_ok()&&self.healthy() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn parentesco_duracion_y_limite_explicito(){
        let p=std::env::temp_dir().join(store::id("otel-test"));let t=Telemetry::new(&p,"sintetico",8192).unwrap();
        let op=t.begin("peticion",json!({"fixture":true}));let id=op.trace_id().to_owned();
        t.event(Some(&op),"hijo",json!({"pid":1}));let summary=t.end(op,json!({"finish":"test"}));
        assert_eq!(summary["trace_id"],id);assert_eq!(t.status()["records"],2);assert!(t.shutdown());
        let rows:Vec<Value>=fs::read_to_string(p.join("trazas.jsonl")).unwrap().lines().map(|l|serde_json::from_str(l).unwrap()).collect();
        assert_eq!(rows[0]["trace_id"],rows[1]["trace_id"]);assert_eq!(rows[0]["parent_span_id"],rows[1]["span_id"]);
        assert!(rows[1]["attributes"]["duration_monotonic_seconds"].as_str().unwrap().parse::<f64>().unwrap()>=0.0);
        let capped=Telemetry::new(&p.join("cota"),"sintetico",1).unwrap();capped.event(None,"cota",json!({}));
        assert!(!capped.healthy());assert_eq!(capped.status()["dropped"],1);assert_eq!(capped.status()["bytes"],0);
        drop(capped);drop(t);fs::remove_dir_all(p).unwrap();
    }
    #[test] fn fallo_real_de_destino_no_se_oculta(){
        let p=std::env::temp_dir().join(store::id("otel-full"));let t=Telemetry::new(&p,"sintetico",8192).unwrap();
        t.sink.lock().unwrap().file=OpenOptions::new().write(true).open("/dev/full").unwrap();
        t.event(None,"fallo_destino",json!({}));assert!(!t.healthy());assert_eq!(t.status()["errors"],1);
        drop(t);fs::remove_dir_all(p).unwrap();
    }
}


//! OpenTelemetry API/SDK directos. Exportador síncrono acotado, sin red ni colector.
//! Integración API pendiente de compilación. Fallos inyectados son controles del banco.
use std::sync::{Arc, Mutex};
use opentelemetry::{Context, KeyValue};
use opentelemetry::trace::{Span, Tracer, TracerProvider, TraceContextExt};
use opentelemetry_sdk::trace::{SdkTracerProvider, SpanData, SpanExporter};
use opentelemetry_sdk::error::{OTelSdkError, OTelSdkResult};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Registro { pub nombre: String, pub trace: String, pub span: String,
    pub padre: String, pub inicio_unix_ns: u128, pub fin_unix_ns: u128, pub atributos_perdidos: u32, pub eventos_perdidos: u32 }
#[derive(Debug, Default, serde::Serialize)]
pub struct Estado { pub registros: Vec<Registro>, pub descartados: usize, pub errores: usize }
#[derive(Debug, Clone)]
pub struct Exportador { pub estado: Arc<Mutex<Estado>>, pub omitir_b: bool, pub fallo: bool }
impl SpanExporter for Exportador {
    async fn export(&self, batch: Vec<SpanData>) -> OTelSdkResult {
        let mut e = self.estado.lock().map_err(|_| OTelSdkError::InternalFailure("mutex".into()))?;
        if self.fallo { e.errores += 1; return Err(OTelSdkError::InternalFailure("fallo inyectado".into())); }
        for s in batch {
            if self.omitir_b && s.name == "consulta.B" { e.descartados += 1; continue; }
            // Sin cola de fondo. 128 registros de metadatos cortos como máximo.
            if e.registros.len() >= 128 { e.descartados += 1; continue; }
            e.registros.push(Registro {nombre:s.name.chars().take(64).collect(),
                trace:s.span_context.trace_id().to_string(),span:s.span_context.span_id().to_string(),
                padre:s.parent_span_id.to_string(),
                inicio_unix_ns:s.start_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(),
                fin_unix_ns:s.end_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(),atributos_perdidos:s.dropped_attributes_count,
                eventos_perdidos:s.events.dropped_count});
        }
        Ok(())
    }
}
pub struct Telemetria { pub proveedor: SdkTracerProvider, pub contexto: Context,
    pub estado: Arc<Mutex<Estado>> }
impl Telemetria {
    pub fn nueva(id: &str, omitir_b: bool, fallo: bool) -> Self {
        let estado = Arc::new(Mutex::new(Estado::default()));
        let proveedor = SdkTracerProvider::builder()
            .with_max_attributes_per_span(8).with_max_events_per_span(8)
            .with_max_attributes_per_event(4)
            .with_simple_exporter(Exportador{estado:estado.clone(),omitir_b,fallo}).build();
        #[cfg(not(target_arch="wasm32"))]
        let mut root = proveedor.tracer("eio-0.1.0").start("peticion");
        #[cfg(target_arch="wasm32")]
        let mut root = proveedor.tracer("eio-0.1.0").span_builder("peticion")
            .with_start_time(crate::reloj::civil()).start(&proveedor.tracer("eio-0.1.0"));
        root.set_attribute(KeyValue::new("caso", id.chars().take(64).collect::<String>()));
        root.set_attribute(KeyValue::new("contrato", "EIO-CONTRATO-01/r1"));
        Self {proveedor,contexto:Context::current_with_span(root),estado}
    }
    pub fn evento(&self, nombre: &str) {
        #[cfg(not(target_arch="wasm32"))]
        let mut s = self.proveedor.tracer("eio-0.1.0")
            .start_with_context(nombre.to_string(), &self.contexto);
        #[cfg(target_arch="wasm32")]
        let mut s = self.proveedor.tracer("eio-0.1.0").span_builder(nombre.to_string())
            .with_start_time(crate::reloj::civil())
            .start_with_context(&self.proveedor.tracer("eio-0.1.0"), &self.contexto);
        s.set_attribute(KeyValue::new("version_fuente", 1_i64));
        #[cfg(not(target_arch="wasm32"))] s.end();
        #[cfg(target_arch="wasm32")] s.end_with_timestamp(crate::reloj::civil());
    }
    pub fn cerrar(&self) -> bool {
        use opentelemetry::trace::TraceContextExt;
        #[cfg(not(target_arch="wasm32"))] self.contexto.span().end();
        #[cfg(target_arch="wasm32")] self.contexto.span().end_with_timestamp(crate::reloj::civil());
        self.proveedor.force_flush().is_ok() && self.estado.lock().unwrap().errores == 0
    }
}

#[cfg(target_arch="wasm32")]
impl Drop for Telemetria {
 fn drop(&mut self){self.contexto.span().end_with_timestamp(crate::reloj::civil());}
}

//! OpenTelemetry API/SDK directos. Exportador síncrono acotado, sin red ni colector.
//! Integración API pendiente de compilación. Fallos inyectados son controles del banco.
use std::sync::{Arc, Mutex};
use opentelemetry::{Context, KeyValue};
use opentelemetry::trace::{Span, Tracer, TracerProvider, TraceContextExt};
use opentelemetry_sdk::trace::{SdkTracerProvider, SpanData, SpanExporter};
use opentelemetry_sdk::error::{OTelSdkError, OTelSdkResult};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Registro { pub nombre: String, pub trace: String, pub span: String,
    pub padre: String, pub atributos_perdidos: u32, pub eventos_perdidos: u32 }
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
                padre:s.parent_span_id.to_string(),atributos_perdidos:s.dropped_attributes_count,
                eventos_perdidos:s.events.dropped_count});
            if let Some(r) = e.registros.last() {
                if crate::nativa::emitir("otel", serde_json::to_value(r).map_err(|x| OTelSdkError::InternalFailure(x.to_string()))?).is_err() {
                    e.errores += 1; return Err(OTelSdkError::InternalFailure("CUSTODIA_EXTERIOR".into()));
                }
            }
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
        let mut root = proveedor.tracer("eio-0.1.0").start("peticion");
        root.set_attribute(KeyValue::new("caso", id.chars().take(64).collect::<String>()));
        root.set_attribute(KeyValue::new("contrato", "EIO-CONTRATO-01/r1"));
        Self {proveedor,contexto:Context::current_with_span(root),estado}
    }
    pub fn evento(&self, nombre: &str) {
        let mut s = self.proveedor.tracer("eio-0.1.0")
            .start_with_context(nombre.to_string(), &self.contexto);
        s.set_attribute(KeyValue::new("version_fuente", 1_i64)); s.end();
    }
    pub fn cerrar(&self) -> bool {
        use opentelemetry::trace::TraceContextExt;
        self.contexto.span().end();
        self.proveedor.force_flush().is_ok() && self.estado.lock().unwrap().errores == 0
    }
}

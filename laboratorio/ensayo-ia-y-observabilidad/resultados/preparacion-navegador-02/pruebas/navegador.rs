//! Enlace candidato; no acredita compatibilidad WASM, reloj ni ejecución en navegador.
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn caso(pesos:Vec<u8>,tokenizer:Vec<u8>,peticion:String)->Result<String,JsValue>{
 crate::diagnostico::marca("rust-entrada-tras-copias");
 if pesos.len()>500_000_000 || tokenizer.len()>32*1024*1024 ||
 peticion!=include_str!("peticion.txt"){return Err(JsValue::from_str("ENTRADA_NO_ADMITIDA"));}
 let t=crate::telemetria::Telemetria::nueva("modelo-sintetico-01",false,false);
 t.evento("consulta.A");t.evento("consulta.B");t.evento("generacion.inicio");
 let salida=crate::inferencia::generar(&pesos,&tokenizer,&peticion,&std::sync::atomic::AtomicBool::new(false))
  .map_err(|e|JsValue::from_str(&e.to_string()))?;
 t.evento("generacion.fin");let exportada=t.cerrar();
 let refs=vec![crate::Referencia{id:"A".into(),version:1},crate::Referencia{id:"B".into(),version:1}];
 let estado=t.estado.lock().unwrap();
 let obs=estado.registros.iter().filter_map(|r|r.nombre.strip_prefix("consulta.")
  .map(|id|crate::Referencia{id:id.into(),version:1})).collect::<Vec<_>>();
 let c=crate::Condiciones{id:"sintetica-01".into(),permiso:false,activa:false,veto:false,
 tokens_entrada:salida.entrada,exportacion_correcta:exportada};
 let nombres=estado.registros.iter().map(|r|r.nombre.as_str()).collect::<Vec<_>>();
 let cobertura=nombres==vec!["consulta.A","consulta.B","generacion.inicio","generacion.fin","peticion"] &&
 estado.descartados==0 && estado.errores==0;
 let contrato=crate::comprobar(&c,&salida.texto,&refs,&obs).err().unwrap_or("OK");
 serde_json::to_string(&serde_json::json!({"tipo":"modelo_real","plataforma":"navegador",
 "salida":salida,"contrato":contrato,"cobertura_eventos":cobertura,"telemetria":&*estado})).map_err(|e|JsValue::from_str(&e.to_string()))
}

const CONTROL:&str=r#"{"peticion":"nav-control-01","referencias":[{"id":"A","version":1},{"id":"B","version":1}],"accion":"marcar","respuesta":"Control sintético finito A y B."}"#;
#[wasm_bindgen]
pub fn control(entrada:String,omitir_b:bool)->Result<String,JsValue>{
 if entrada!=CONTROL{return Err(JsValue::from_str("CONTROL_ENTRADA"));}
 let inicio=crate::reloj::Instant::now();
 let t=crate::telemetria::Telemetria::nueva("nav-control-01",omitir_b,false);
 t.evento("consulta.A");t.evento("consulta.B");t.evento("computo.fin");
 let exportada=t.cerrar();let e=t.estado.lock().unwrap();
 let refs=vec![crate::Referencia{id:"A".into(),version:1},crate::Referencia{id:"B".into(),version:1}];
 let obs=e.registros.iter().filter_map(|r|r.nombre.strip_prefix("consulta.")
 .map(|id|crate::Referencia{id:id.into(),version:1})).collect::<Vec<_>>();
 let c=crate::Condiciones{id:"nav-control-01".into(),permiso:true,activa:false,veto:false,
 tokens_entrada:0,exportacion_correcta:exportada};
 let mut objeto=false;
 let resultado=crate::ejecutar(&c,&entrada,&refs,&obs,&mut objeto).err().unwrap_or("OK");
 let nombres=e.registros.iter().map(|r|r.nombre.as_str()).collect::<Vec<_>>();
 let esperados=if omitir_b{vec!["consulta.A","computo.fin","peticion"]}else{vec!["consulta.A","consulta.B","computo.fin","peticion"]};
 let raiz=e.registros.last();
 let traza=raiz.map(|root|root.padre=="0000000000000000" && root.trace!="00000000000000000000000000000000" &&
  e.registros.iter().all(|r|r.trace==root.trace && r.span!="0000000000000000" &&
   r.inicio_unix_ns>0 && r.fin_unix_ns>=r.inicio_unix_ns &&
   (r.nombre=="peticion" || r.padre==root.span) &&
   r.atributos_perdidos==0 && r.eventos_perdidos==0)).unwrap_or(false);
 let pasa=nombres==esperados && traza && e.errores==0 &&
  if omitir_b {resultado=="EVENTO_AUSENTE" && !objeto && e.descartados==1}
  else {resultado=="OK" && objeto && e.descartados==0};
 serde_json::to_string(&serde_json::json!({"pasa":pasa,"entrada_literal":entrada,
 "resultado":resultado,"objeto":objeto,"segundos_monotonicos":inicio.elapsed().as_secs_f64(),
 "telemetria":&*e,"reloj_duracion":"performance.now","reloj_civil":"Date.now"}))
 .map_err(|e|JsValue::from_str(&e.to_string()))
}
#[wasm_bindgen]
pub fn sonda_no_cooperativa(){
 let mut x=0u64;
 loop {x=x.wrapping_add(1);std::hint::black_box(x);}
}

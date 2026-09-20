//! Enlace candidato; no acredita compatibilidad WASM, reloj ni ejecución en navegador.
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn caso(pesos:Vec<u8>,tokenizer:Vec<u8>,peticion:String)->Result<String,JsValue>{
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
 let contrato=crate::comprobar(&c,&salida.texto,&refs,&obs).err().unwrap_or("OK");
 serde_json::to_string(&serde_json::json!({"tipo":"modelo_real","plataforma":"navegador",
 "salida":salida,"contrato":contrato,"telemetria":&*estado})).map_err(|e|JsValue::from_str(&e.to_string()))
}

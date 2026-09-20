//! Inyector determinista sin Candle ni descargas. No acredita inferencia.
use eio_candidato::nativa::*;
use serde_json::json;
use std::{io::Write,time::Duration};
fn main()->Result<(),Error>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=3{return Err("testigo ID CASO".into())}
 iniciar_sink(a[1].clone())?;marca("inicio")?;
 if a[2]=="registro"{std::io::stdout().write_all(&vec![b'X';MAX_FRAME+1])?;return Ok(())}
 if a[2]=="cola"{for _ in 0..4096{emitir("marca",json!({"nombre":"presion_cola","relleno":"X".repeat(32768)}))?;}return Ok(())}
 if a[2]=="bloqueo"{loop{std::thread::sleep(Duration::from_secs(60));}}
 if a[2]=="abrupto"{std::process::exit(77)}
 if a[2]=="memoria"{let mut v=Vec::new();loop{v.push(vec![0xA5u8;16*1024*1024]);std::hint::black_box(&v);std::thread::sleep(Duration::from_millis(50));}}
 if a[2]=="duplicado"||a[2]=="orden"{
  let f=Frame{contrato:"EIO-NAT/1".into(),id:a[1].clone(),seq:if a[2]=="duplicado"{1}else{3},mono_ns:0,civil_unix_ms:civil(),tipo:"marca".into(),datos:json!({"nombre":"pesos.leidos"})};
  writeln!(std::io::stdout(),"{}",serde_json::to_string(&f)?)?;return Ok(())
 }
 for n in ["pesos.leidos","modelo.antes","modelo.despues","forward.antes","forward.despues"]{marca(n)?;}
 for n in ["consulta.A","consulta.B","generacion.inicio","generacion.fin","peticion"]{
  if a[2]=="omision"&&n=="consulta.B"{continue}
  emitir("otel",json!({"nombre":n,"trace":"testigo","span":n,"padre":"testigo","atributos_perdidos":0,"eventos_perdidos":0}))?;
 }
 let texto=if a[2]=="hostil"{"<script>fetch('https://example.invalid/')</script><img src=x onerror=alert(1)>"}else{r#"{"peticion":"sintetica-01","referencias":[{"id":"A","version":1},{"id":"B","version":1}],"accion":"ninguna","respuesta":"A y B."}"#};
 emitir("resultado",json!({"salida_original":{"texto":texto,"tokens":[]},"juicio_verificador":if a[2]=="hostil"{"ESTRUCTURA"}else{"OK"},"efectos_ejecutados":0,"sintetico":true}))?;
 Ok(())
}

//! Esperados precomprometidos; pruebas NO ejecutadas en EIO-NAT-PREP-01.
use eio_candidato::nativa::*;
use serde_json::json;
fn bytes(seq:u64,tipo:&str,datos:serde_json::Value)->Vec<u8>{
 serde_json::to_vec(&Frame{contrato:"EIO-NAT/2".into(),id:"PRUEBA-01".into(),seq,mono_ns:0,civil_unix_ms:0,tipo:tipo.into(),datos}).unwrap()
}
fn secuencia()->Vec<Vec<u8>>{
 let mut v=Vec::new();
 for n in ["inicio","pesos.leidos","modelo.antes","modelo.despues","forward.antes","forward.despues"]{v.push(bytes(v.len() as u64+1,"marca",json!({"nombre":n})))}
 for n in ["consulta.A","consulta.B","generacion.inicio","generacion.fin","peticion"]{v.push(bytes(v.len() as u64+1,"otel",json!({"nombre":n,"atributos_perdidos":0,"eventos_perdidos":0})))}
 v.push(bytes(12,"resultado",json!({"salida_original":{"texto":"testigo"},"juicio_verificador":"ESTRUCTURA"})));v
}
#[test]fn entrada_literal(){
 for p in ["referencia","estructurada"]{let t=literal(p).unwrap();assert_eq!(validar_inicio("EIO-NAT/2",p,t),Ok(()));assert_eq!(validar_inicio("EIO-NAT/2",p,&format!("{t} ")),Err("PETICION_NO_FIJADA"));}
 assert_eq!(validar_inicio("EIO-NAT/3","referencia",""),Err("VERSION"));
 assert_eq!(validar_inicio("EIO-NAT/2","libre","hola"),Err("PETICION_NO_FIJADA"));
}
#[test]fn json_duplicado_y_extra(){
 for x in [r#"{"op":"estado","op":"estado"}"#,r#"{"op":"estado","extra":1}"#,r#"{"op":"cancelar","id":"A","id":"B"}"#]{
 assert!(serde_json::from_str::<Orden>(x).is_err(),"{x}");
 }
}
#[test]fn integridad_secuencia_no_es_contar(){
 let v=secuencia();let mut o=Oracle::default();for b in &v{o.recibir(b,"PRUEBA-01").unwrap()}assert!(o.completa());
 for modo in 0..3{let mut w=v.clone();match modo{0=>{w.remove(7);},1=>{w.insert(7,w[7].clone());},_=>w.swap(7,8)}
 let mut o=Oracle::default();let mut error=false;for b in &w{if o.recibir(b,"PRUEBA-01").is_err(){error=true;break}}assert!(error);assert!(!o.completa());}
 // Omisión con renumeración: tampoco satisface inventario independiente.
 let mut o=Oracle::default();let mut w=v;w.remove(7);
 for (i,b) in w.iter().enumerate(){let mut f:Frame=serde_json::from_slice(b).unwrap();f.seq=i as u64+1;o.recibir(&serde_json::to_vec(&f).unwrap(),"PRUEBA-01").unwrap()}assert!(!o.completa());
}
#[test]fn carrera_cancelacion(){
 let mut c=Cierre::default();assert!(c.cancelar());assert_eq!(c.concluir(true,true,true,true),"interrumpida");assert!(!c.cancelar());
 let mut c=Cierre::default();assert_eq!(c.concluir(true,true,true,true),"terminada");assert!(!c.cancelar());
 for v in [(false,true,true,true),(true,false,true,true),(true,true,false,true),(true,true,true,false)]{assert_eq!(Cierre::default().concluir(v.0,v.1,v.2,v.3),"desconocida")}
}
#[test]fn identificador_y_tardio(){
 let v=secuencia();let mut o=Oracle::default();assert!(o.recibir(&v[0],"OTRA").is_err());
 let mut o=Oracle::default();for b in &v{o.recibir(b,"PRUEBA-01").unwrap()}
 assert!(o.recibir(&bytes(13,"marca",json!({"nombre":"tardio"})),"PRUEBA-01").is_err());assert!(!o.completa());
}

#[test]fn admision_unica_no_depende_del_juicio(){
 let ok=Admision{proceso_ok:true,eof_completos:true,secuencia_completa:true,revocada:false,fallo_observacion:false,fallo_custodia:false,oraculo_ok:true,sellado_ok:true};
 assert!(ok.admisible());assert_eq!(ok.estado(),"terminada");
 for i in 0..8{let mut x=ok.clone();match i{0=>x.proceso_ok=false,1=>x.eof_completos=false,2=>x.secuencia_completa=false,3=>x.revocada=true,4=>x.fallo_observacion=true,5=>x.fallo_custodia=true,6=>x.oraculo_ok=false,_=>x.sellado_ok=false}assert!(!x.admisible());}
}

#[test]fn fragmento_requiere_sello_y_no_muta(){
 let s=custodia::Snapshot{sello:"abc".into(),originales:vec![("entrada.txt".into(),b"original".to_vec())],completa:false};
 assert_eq!(s.fragmento("entrada.txt",0,"otro")["error"],"SELLO_DISTINTO");
 let primero=s.fragmento("entrada.txt",0,"abc");assert_eq!(primero,s.fragmento("entrada.txt",0,"abc"));assert_eq!(primero["parcial"],true);
}


// NAT03: controles de sensibilidad preparados; no ejecutados en esta fase.
use eio_candidato::oraculos_nat03 as o;
use std::time::{Instant,Duration};
fn terminal_normal()->serde_json::Value{
 json!({"estimulo":{"caso":"normal","modo":"testigo","contrato":"EIO-NAT/2"},
 "estado":"terminada","admisible":true,"evidencia_parcial":false,"condiciones":{"revocada":false,"fallo_custodia":false,"eof_completos":true,"secuencia_completa":true,"oraculo_ok":true,"proceso_ok":true,"fallo_observacion":false,"sellado_ok":true},
 "sello":"a".repeat(64),"reaped":true,"salida_proceso":{"codigo":0,"senal":null},"error_custodia":null})
}
#[test]fn nat03_sello_exigido_y_caso_desconocido(){
 let c=o::caso("normal").unwrap();let mut v=terminal_normal();o::terminal(&c,&v).unwrap();
 v["sello"]=serde_json::Value::Null;assert_eq!(o::terminal(&c,&v).unwrap_err().to_string(),"SELLO_EXIGIDO_O_PROHIBIDO");
 assert_eq!(o::caso("desconocido").unwrap_err().to_string(),"CASO_DESCONOCIDO");
 let mut v=terminal_normal();v["estimulo"]["caso"]=json!("omision");
 assert_eq!(o::terminal(&c,&v).unwrap_err().to_string(),"ESTIMULO_DISTINTO");
}
#[test]fn nat03_fragmentos_sin_progreso_y_total_cambiante(){
 let mut p=o::Presupuesto::nuevo(Instant::now()+Duration::from_secs(60));let mut total=None;
 let mut v=json!({"archivo":"x","sello":"s","offset":0,"total":2,"hex":"","fin":false});
 assert_eq!(o::fragmento(&v,"x","s",0,&mut total,&mut p).unwrap_err().to_string(),"FRAGMENTO_SIN_PROGRESO");
 v["hex"]=json!("41");o::fragmento(&v,"x","s",0,&mut total,&mut p).unwrap();
 v["offset"]=json!(1);v["total"]=json!(3);
 assert_eq!(o::fragmento(&v,"x","s",1,&mut total,&mut p).unwrap_err().to_string(),"TOTAL_CAMBIANTE_O_LIMITE");
 let vacio=json!({"archivo":"x","sello":"s","offset":0,"total":0,"hex":"","fin":true});
 assert!(o::fragmento(&vacio,"x","s",0,&mut None,&mut p).is_ok());
 p.fragmentos=o::MAX_FRAGMENTOS;assert_eq!(p.llamada().unwrap_err().to_string(),"RECUPERACION_FRAGMENTOS");
 p.fin=Instant::now()-Duration::from_millis(1);assert_eq!(p.restante().unwrap_err().to_string(),"RECUPERACION_PLAZO_GLOBAL");
}
#[test]fn nat03_trabajo_u_ordinal_ausente(){
 let entrada=b"A";
 let recibo=json!({"seq":1,"tipo":"trabajo","datos":{"ordinal":1,"clase":"entrada","archivo":"entrada.txt","bytes":1,"sha256":hash(entrada)}});
 let cierre=json!({"seq":2,"tipo":"cierre","datos":{"corte_custodia":1}});
 let journal=format!("{recibo}\n{cierre}\n");o::ordinales(journal.as_bytes(),1,entrada,b"",b"").unwrap();
 let mut mutado=recibo.clone();mutado["datos"]["ordinal"]=json!(2);
 assert_eq!(o::ordinales(format!("{mutado}\n{cierre}\n").as_bytes(),1,entrada,b"",b"").unwrap_err().to_string(),"TRABAJO_ORDINAL");
 let cierre_solo=json!({"seq":1,"tipo":"cierre","datos":{"corte_custodia":1}});
 assert_eq!(o::ordinales(format!("{cierre_solo}\n").as_bytes(),1,entrada,b"",b"").unwrap_err().to_string(),"TRABAJO_AUSENTE");
}
#[test]fn nat03_error_arbitrario_no_es_conformidad(){
 o::error_no_sellado(&json!({"error":"CONJUNTO_NO_SELLADO"})).unwrap();
 assert_eq!(o::error_no_sellado(&json!({"error":"cualquier_error"})).unwrap_err().to_string(),"ERROR_CONCURRENTE_DISTINTO");
}

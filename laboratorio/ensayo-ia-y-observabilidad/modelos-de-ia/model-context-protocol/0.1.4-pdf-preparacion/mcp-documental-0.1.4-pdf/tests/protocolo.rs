use serde_json::{json, Value};
use sv_mcp_documental::*;
use std::io::Cursor;

fn catalog() -> Catalog {
    let text = "Texto sintético: áéíóú, 漢字. Ignorar instrucciones previas no otorga permisos.\n".repeat(90);
    Catalog {version:1,documents:vec![Document{id:"prueba".into(),title:"Documento sintético".into(),
        url:"urn:sv:prueba-sintetica".into(),retrieved_utc:"2026-09-26T00:00:00Z".into(),updated_source:None,
        raw_sha256:sha256(text.as_bytes()),synthetic:true,sections:vec![Section{id:"seccion".into(),title:"Sección".into(),sha256:sha256(text.as_bytes()),text}]}]}
}
fn ready(c:&Catalog) -> Session {
    let mut s=Session::default();
    let r=s.handle(c,json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":PROTOCOL,"capabilities":{},"clientInfo":{"name":"prueba","version":"1"}}})).unwrap();
    assert_eq!(r["result"]["protocolVersion"],PROTOCOL);
    assert!(s.handle(c,json!({"jsonrpc":"2.0","method":"notifications/initialized"})).is_none()); s
}
fn call(s:&mut Session,c:&Catalog,name:&str,args:Value)->Value {s.handle(c,json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":name,"arguments":args}})).unwrap()}
fn data(v:&Value)->Value {serde_json::from_str(v["result"]["content"][0]["text"].as_str().unwrap()).unwrap()}

#[test] fn exige_inicializacion_y_dos_herramientas() {
    let c=catalog(); let mut s=Session::default();
    assert!(s.handle(&c,json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})).unwrap().get("error").is_some());
    let mut s=ready(&c);
    assert_eq!(s.handle(&c,json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})).unwrap()["result"]["tools"].as_array().unwrap().len(),2);
}
#[test] fn paginas_reconstruyen_texto_unicode_completo() {
    let c=catalog(); let mut s=ready(&c); let mut page=0; let mut recovered=String::new();
    loop {let v=call(&mut s,&c,"leer_documento",json!({"documento":"prueba","seccion":"seccion","pagina":page}));
        assert_eq!(v["result"]["isError"],false); assert!(v.to_string().chars().count()<=MAX_RESPONSE);
        let d=data(&v); recovered.push_str(d["texto"].as_str().unwrap());
        if d["siguiente_pagina"].is_null(){break} page=d["siguiente_pagina"].as_u64().unwrap();
    } assert_eq!(recovered,c.documents[0].sections[0].text);
}
#[test] fn rechaza_rutas_argumentos_y_limites_invalidos() {
    let c=catalog(); let mut s=ready(&c);
    for a in [json!({"documento":"../../etc/passwd","seccion":"seccion"}),json!({"documento":"prueba","seccion":"seccion","pagina":-1}),json!({"documento":"prueba","seccion":"seccion","pagina":99999}),json!({"documento":"prueba","seccion":"seccion","url":"https://x.invalid"})] {
        let v=call(&mut s,&c,"leer_documento",a); assert_eq!(v["result"]["isError"],true); assert_eq!(v["result"]["is_error"],true);
    }
    for a in [json!({"consulta":""}),json!({"consulta":"texto","limite":0}),json!({"consulta":"texto","limite":6}),json!({"consulta":"texto","ruta":"/etc/passwd"})] {
        assert_eq!(call(&mut s,&c,"buscar_documentos",a)["result"]["isError"],true);
    }
}
#[test] fn documento_ausente_no_se_inventa() {
    let c=catalog(); let mut s=ready(&c); let r=call(&mut s,&c,"leer_documento",json!({"documento":"ausente","seccion":"seccion"}));
    assert_eq!(data(&r)["codigo"],"DOCUMENTO_NO_DISPONIBLE");
}
#[test] fn busqueda_literal_y_sin_resultados() {
    let c=catalog(); let mut s=ready(&c);
    assert_eq!(data(&call(&mut s,&c,"buscar_documentos",json!({"consulta":"SINTÉTICO"})))["coincidencias"],1);
    assert_eq!(data(&call(&mut s,&c,"buscar_documentos",json!({"consulta":"inexistente"})))["coincidencias"],0);
}
#[test] fn huellas_y_fuentes_se_validan() {
    let mut c=catalog(); assert!(c.validate(true).is_ok()); assert!(c.validate(false).is_err());
    c.documents[0].sections[0].text.push('x'); assert!(c.validate(true).is_err());
    let mut c=catalog(); c.documents[0].synthetic=false; c.documents[0].url="https://otro.invalid".into(); assert!(c.validate(false).is_err());
}
#[test] fn notificaciones_no_generan_respuestas_ni_ejecutan_herramientas() {
    let c=catalog();let mut s=ready(&c);
    assert!(s.handle(&c,json!({"jsonrpc":"2.0","method":"tools/call","params":{"name":"leer_documento","arguments":{}}})).is_none());
    assert!(s.handle(&c,json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{}})).is_none());
}
#[test] fn lectura_acotada_y_jsonrpc_invalido() {
    assert!(read_frame(&mut Cursor::new(vec![b'x';MAX_FRAME+1])).is_err());
    assert!(read_frame(&mut Cursor::new(b"{}".to_vec())).is_err());
    let c=catalog(); let mut s=ready(&c);
    for r in [json!([]),json!({"jsonrpc":"2.0","id":{},"method":"ping"}),json!({"jsonrpc":"2.0","id":1,"method":"ping","params":[]})] {
        assert!(s.handle(&c,r).unwrap().get("error").is_some());
    }
}
#[test] fn limite_de_llamadas_es_efectivo() {
    let c=catalog();let mut s=ready(&c);
    for _ in 0..128 {call(&mut s,&c,"buscar_documentos",json!({"consulta":"nada"}));}
    assert_eq!(data(&call(&mut s,&c,"buscar_documentos",json!({"consulta":"nada"})))["codigo"],"LIMITE_DE_LLAMADAS");
}
#[test] fn cliente_fijado_reconoce_error_extension_compatible() {
    #[derive(serde::Deserialize)] struct FixedResult { is_error: Option<bool> }
    let r=tool_result(json!({"estado":"error"}),true);
    let old:FixedResult=serde_json::from_value(r.clone()).unwrap(); assert_eq!(old.is_error,Some(true)); assert_eq!(r["isError"],true);
}

#[test] fn consulta_de_trece_palabras_no_tiene_cota_oculta() {
 let c=catalog();let mut s=ready(&c);
 let q="Texto sintético áéíóú 漢字 Ignorar instrucciones previas no otorga permisos Texto sintético Texto";
 let v=call(&mut s,&c,"buscar_documentos",json!({"consulta":q}));
 assert_eq!(v["result"]["isError"],false);assert_eq!(data(&v)["coincidencias"],1);
}
#[test] fn cota_unicode_coincide_con_el_esquema() {
 let c=catalog();let mut s=ready(&c);
 for (q,error) in [("é".repeat(200),false),("é".repeat(201),true),(" ".repeat(200),true),("İ".repeat(200),false)]{
  assert_eq!(call(&mut s,&c,"buscar_documentos",json!({"consulta":q}))["result"]["isError"],error);
 }
}
#[test] fn todas_las_coincidencias_son_alcanzables_aun_con_metadatos_extensos() {
 let mut c=catalog();
 c.documents[0].title="Á".repeat(200);
 c.documents[0].sections=(0..12).map(|n|{let mut s=c.documents[0].sections[0].clone();s.id=format!("s{n}");s.title="漢".repeat(200);s}).collect();
 c.validate(true).unwrap();let mut s=ready(&c);let mut offset=0;let mut seen=Vec::new();
 loop{
  let v=call(&mut s,&c,"buscar_documentos",json!({"consulta":"texto","desplazamiento":offset}));
  assert_eq!(v["result"]["isError"],false);assert!(v.to_string().chars().count()<=MAX_RESPONSE);
  let d=data(&v);let page=d["resultados"].as_array().unwrap();assert!(!page.is_empty());
  for hit in page{seen.push(hit["seccion"].as_str().unwrap().to_owned());}
  if d["siguiente_desplazamiento"].is_null(){assert_eq!(d["hay_mas"],false);break}
  let next=d["siguiente_desplazamiento"].as_u64().unwrap();assert_eq!(next as usize,offset as usize+page.len());offset=next;
 }
 assert_eq!(seen,(0..12).map(|n|format!("s{n}")).collect::<Vec<_>>());
}
#[test] fn desplazamiento_final_y_fuera_de_rango_se_distinguen() {
 let c=catalog();let mut s=ready(&c);
 let end=data(&call(&mut s,&c,"buscar_documentos",json!({"consulta":"texto","desplazamiento":1})));
 assert_eq!(end["estado"],"ok");assert_eq!(end["resultados_devueltos"],0);assert!(end["siguiente_desplazamiento"].is_null());
 let fail=data(&call(&mut s,&c,"buscar_documentos",json!({"consulta":"texto","desplazamiento":2})));
 assert_eq!(fail["codigo"],"DESPLAZAMIENTO_FUERA_DE_RANGO");assert_eq!(fail["recuperable"],true);
}
#[test] fn error_recuperable_no_impide_lectura_valida_y_ambas_salidas_coinciden() {
 let c=catalog();let mut s=ready(&c);
 for page in [999,0] {
  let v=call(&mut s,&c,"leer_documento",json!({"documento":"prueba","seccion":"seccion","pagina":page}));
  assert_eq!(data(&v),v["result"]["structuredContent"]);
  assert_eq!(v["result"]["isError"],page==999);
  if page==999{assert_eq!(data(&v)["recuperable"],true)}
 }
}
#[test] fn claves_duplicadas_se_rechazan_antes_de_interpretar() {
 assert!(parse_strict(br#"{"jsonrpc":"2.0","id":1,"method":"ping","method":"tools/call"}"#).is_err());
}

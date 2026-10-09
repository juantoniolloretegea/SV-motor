use std::{fs,path::PathBuf,sync::atomic::{AtomicUsize,Ordering}};
use serde_json::{json,Value};
use sv_biblioteca_documental::*;
use sv_mcp_documental::{sha256,Session};
static SEQ:AtomicUsize=AtomicUsize::new(0);
fn fixture()->(PathBuf,Value,Politica) {
    let dir=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/pruebas").join(format!("{}-{}",std::process::id(),SEQ.fetch_add(1,Ordering::SeqCst)));
    fs::create_dir_all(dir.join("libro" )).unwrap();
    let summary="# Índice\n\n- [Contenido](contenido.md)\n";
    let text="# Contenido\n\nNo se autoriza acceso externo. 8 no es mayor que 8.\n";
    fs::write(dir.join("libro/SUMMARY.md"),summary).unwrap();
    fs::write(dir.join("libro/contenido.md"),text).unwrap();
    let file=|id:&str,ruta:&str,text:&str|json!({"id":id,"ruta":ruta,"titulo":id,"sha256":sha256(text.as_bytes()),"url":format!("urn:sv:documento:sha256:{}",sha256(text.as_bytes()))});
    let plan=json!({"version":2,"libro":"ensayo","revision":"a".repeat(64),"summary":file("INDICE","SUMMARY.md",summary),"capitulos":[file("CAPITULO","contenido.md",text)]});
    let planraw=serde_json::to_vec(&plan).unwrap();fs::write(dir.join("plan.json"),&planraw).unwrap();
    let meta=|id:&str,ruta:&str,text:&str|json!({"documento":id,"tipo_original":"markdown","original":{"ruta":ruta,"sha256":sha256(text.as_bytes())},"procedencia":"Fuente de ensayo prefijada","licencia":"CC BY-NC-ND 4.0","sintetico":false,"visibilidad":"publica","alcance":"Control instrumental","correspondencia":"identidad_literal","identificador_editorial_r0":null});
    let manifest=json!({"version":1,"edicion":"ensayo-1","estado":"preparacion_local","libros":[{"id":"manual","titulo":"Manual de ensayo","raiz":"libro","plan":{"ruta":"plan.json","sha256":sha256(&planraw)},"metadatos":[meta("INDICE","libro/SUMMARY.md",summary),meta("CAPITULO","libro/contenido.md",text)]}]});
    let policy=Politica{version:1,biblioteca_sha256:sha256(&serde_json::to_vec(&manifest).unwrap()),nodo:1,finalidad:"verificacion_local".into(),libros_permitidos:vec!["manual".into()],permitir_sinteticos:false,permitir_restringidos:false};
    (dir,manifest,policy)
}
fn evaluated(dir:&std::path::Path,m:&Value,p:&Politica)->Result<Preparacion,String> {
    let raw=serde_json::to_vec(m).unwrap();let mut p=p.clone();p.biblioteca_sha256=sha256(&raw);preparar(dir,&raw,&p)
}
#[test]
fn preparacion_reproducible_con_indices_y_sin_cambio_literal() {
    let (dir,m,p)=fixture();let a=evaluated(&dir,&m,&p).unwrap();let b=evaluated(&dir,&m,&p).unwrap();
    assert_eq!(serde_json::to_vec(&a.catalogo).unwrap(),serde_json::to_vec(&b.catalogo).unwrap());
    assert_eq!(a.catalogo.documents[0].id,INDICE);
    assert!(a.catalogo.documents[0].sections[1].text.contains("CAPITULO"));
    assert_eq!(a.catalogo.documents[2].sections[0].text,fs::read_to_string(dir.join("libro/contenido.md")).unwrap());
}
#[test]
fn permisos_restringidos_y_sinteticos_no_se_presuponen() {
    let (dir,m,p)=fixture();
    let mut restricted=m.clone();restricted["libros"][0]["metadatos"][1]["visibilidad"]=json!("restringida");
    assert!(evaluated(&dir,&restricted,&p).err().unwrap().contains("RESTRINGIDO"));
    let mut permitted=p.clone();permitted.permitir_restringidos=true;
    assert!(evaluated(&dir,&restricted,&permitted).is_ok());
    let mut synthetic=m;synthetic["libros"][0]["metadatos"][1]["sintetico"]=json!(true);
    assert!(evaluated(&dir,&synthetic,&p).err().unwrap().contains("SINTETICO"));
    permitted.permitir_sinteticos=true;
    let admitted=evaluated(&dir,&synthetic,&permitted).unwrap();
    assert!(admitted.catalogo.documents[2].synthetic);
    assert_eq!(admitted.catalogo.documents[2].url,"urn:sv:prueba-sintetica");
}
#[test]
fn biblioteca_no_se_autoriza_a_si_misma_ni_permite_produccion() {
    let (dir,m,mut p)=fixture();p.biblioteca_sha256="0".repeat(64);
    assert_eq!(preparar(&dir,&serde_json::to_vec(&m).unwrap(),&p).err().unwrap(),"EDICION_NO_AUTORIZADA");
    for node in [0,4] {p.nodo=node;assert!(evaluated(&dir,&m,&p).is_err());}
    p.nodo=2;p.finalidad="produccion".into();assert!(evaluated(&dir,&m,&p).is_err());
    p.nodo=3;assert!(evaluated(&dir,&m,&p).is_err());
}
#[test]
fn seleccion_no_expone_metadatos_de_libros_excluidos() {
    let (dir,mut m,p)=fixture();
    let mut excluded=m["libros"][0].clone();excluded["id"]=json!("reservado");excluded["titulo"]=json!("NO-DIVULGAR-TITULO");excluded["raiz"]=json!("no-existe");
    m["libros"].as_array_mut().unwrap().push(excluded);
    let ready=evaluated(&dir,&m,&p).unwrap();
    assert!(!serde_json::to_string(&ready.catalogo).unwrap().contains("NO-DIVULGAR"));
    assert!(!ready.mapa.to_string().contains("NO-DIVULGAR"));
    let mut p=p;p.libros_permitidos.push("inventado".into());assert!(evaluated(&dir,&m,&p).is_err());
}
#[test]
fn rechaza_omisiones_de_licencia_procedencia_y_documentos() {
    let (dir,m,p)=fixture();
    for field in ["licencia","procedencia","alcance"] {
        let mut changed=m.clone();changed["libros"][0]["metadatos"][0][field]=json!("");
        assert!(evaluated(&dir,&changed,&p).is_err(),"{field}");
    }
    let mut changed=m.clone();changed["libros"][0]["metadatos"].as_array_mut().unwrap().pop();assert!(evaluated(&dir,&changed,&p).is_err());
    let mut changed=m;changed["libros"][0]["metadatos"][0]["documento"]=json!("CAPITULO");assert!(evaluated(&dir,&changed,&p).is_err());
}
#[test]
fn rechaza_fuente_alterada_y_tipo_incompatible() {
    let (dir,m,p)=fixture();let mut wrong=m.clone();wrong["libros"][0]["metadatos"][1]["tipo_original"]=json!("pdf");
    assert!(evaluated(&dir,&wrong,&p).err().unwrap().contains("TIPO"));
    fs::write(dir.join("libro/contenido.md"),"# Otro\n").unwrap();assert!(evaluated(&dir,&m,&p).is_err());
}
#[test]
fn conversion_requiere_recibo_exterior_y_evidencia_inmutables() {
    let (dir,mut m,p)=fixture();
    let original=b"<html><body>Origen</body></html>";
    fs::write(dir.join("original.html"),original).unwrap();
    fs::write(dir.join("evidencia.json"),b"{\"cotejo\":true}").unwrap();
    let content=fs::read(dir.join("libro/contenido.md")).unwrap();
    let receipt=json!({"version":1,"documento":"CAPITULO","original_sha256":sha256(original),"markdown_sha256":sha256(&content),
        "evidencia":{"ruta":"evidencia.json","sha256":sha256(b"{\"cotejo\":true}")},"metodo":"Extracción recibida","alcance":"Ensayo delimitado","admitido_ensayo":true,"produccion":false});
    let raw=serde_json::to_vec(&receipt).unwrap();fs::write(dir.join("recibo-conversion.json"),&raw).unwrap();
    let meta=&mut m["libros"][0]["metadatos"][1];
    meta["tipo_original"]=json!("html");meta["original"]=json!({"ruta":"original.html","sha256":sha256(original)});
    meta["correspondencia"]=json!("conversion_recibida_para_ensayo");
    meta["recepcion_conversion"]=json!({"ruta":"recibo-conversion.json","sha256":sha256(&raw)});
    assert!(evaluated(&dir,&m,&p).is_ok());
    let mut absent=m.clone();absent["libros"][0]["metadatos"][1].as_object_mut().unwrap().remove("recepcion_conversion");assert!(evaluated(&dir,&absent,&p).is_err());
    for field in ["original_sha256","markdown_sha256"] {
        let mut bad=receipt.clone();bad[field]=json!("0".repeat(64));let raw=serde_json::to_vec(&bad).unwrap();fs::write(dir.join("recibo-conversion.json"),&raw).unwrap();
        let mut changed=m.clone();changed["libros"][0]["metadatos"][1]["recepcion_conversion"]["sha256"]=json!(sha256(&raw));
        assert!(evaluated(&dir,&changed,&p).is_err());
    }
    fs::write(dir.join("recibo-conversion.json"),&raw).unwrap();fs::write(dir.join("evidencia.json"),b"{}").unwrap();assert!(evaluated(&dir,&m,&p).is_err());
}
#[test]
fn rechaza_escapes_y_enlaces_de_archivo() {
    let (dir,m,p)=fixture();
    for ruta in ["../plan.json","/etc/passwd","C:/datos","a\\b","%2e%2e/plan.json"] {
        let mut wrong=m.clone();wrong["libros"][0]["plan"]["ruta"]=json!(ruta);assert!(evaluated(&dir,&wrong,&p).is_err());
    }
    #[cfg(unix)] {
        fs::rename(dir.join("libro/contenido.md"),dir.join("libro/real.md")).unwrap();
        std::os::unix::fs::symlink("real.md",dir.join("libro/contenido.md")).unwrap();
        assert!(evaluated(&dir,&m,&p).is_err());
    }
}
#[test]
fn rechaza_json_ambiguo_y_campos_no_contratados() {
    assert!(leer_json::<Politica>(br#"{"version":1,"version":2}"#).is_err());
    let (dir,mut m,p)=fixture();m["autorizar_internet"]=json!(true);assert!(evaluated(&dir,&m,&p).is_err());
}
#[test]
fn el_documento_no_modifica_la_interfaz_mcp() {
    let (dir,m,p)=fixture();let ready=evaluated(&dir,&m,&p).unwrap();let mut s=Session::default();
    s.handle(&ready.catalogo,json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":sv_mcp_documental::PROTOCOL,"capabilities":{},"clientInfo":{}}}));
    s.handle(&ready.catalogo,json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
    for name in ["autorizar_libro","modificar_politica","navegar_internet"] {
        let r=s.handle(&ready.catalogo,json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":name,"arguments":{}}})).unwrap();
        assert_eq!(r["result"]["isError"],true);
    }
}
#[test]
fn edicion_conservada_separa_documentos_y_controles() {
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../seguimiento/biblioteca-documental-20261009/edicion");
    let raw=fs::read(root.join("biblioteca.json")).unwrap();
    for (mode,count,synthetic) in [("documental",10,0),("controles",14,4)] {
        let p:Politica=leer_json(&fs::read(root.join(format!("politica-{mode}.json"))).unwrap()).unwrap();
        let c=preparar(&root,&raw,&p).unwrap();assert_eq!(c.catalogo.documents.len(),count);
        assert_eq!(c.catalogo.documents.iter().filter(|d|d.synthetic).count(),synthetic);
        if mode=="documental" {assert!(!serde_json::to_string(&c.catalogo).unwrap().contains("HTML01"));}
    }
}

#[test]
fn permiso_denegado_precede_a_lectura_incluso_con_fuente_ausente() {
    let (dir,mut m,p)=fixture();
    fs::remove_file(dir.join("plan.json")).unwrap();
    m["libros"][0]["metadatos"][1]["visibilidad"]=json!("restringida");
    assert_eq!(evaluated(&dir,&m,&p).err().unwrap(),"DOCUMENTO_RESTRINGIDO_NO_AUTORIZADO");
    m["libros"][0]["metadatos"][1]["visibilidad"]=json!("publica");
    m["libros"][0]["metadatos"][1]["sintetico"]=json!(true);
    assert_eq!(evaluated(&dir,&m,&p).err().unwrap(),"SINTETICO_NO_AUTORIZADO");
}

#[test]
#[cfg(unix)]
fn alias_de_raiz_y_de_antecesor_se_rechazan() {
    let (dir,m,p)=fixture();
    let alias=dir.with_extension("alias");std::os::unix::fs::symlink(&dir,&alias).unwrap();
    assert_eq!(evaluated(&alias,&m,&p).err().unwrap(),"RAIZ_SIMBOLICA");
    assert_eq!(raiz_segura(&alias.join("libro")).err().unwrap(),"RAIZ_SIMBOLICA");
}

fn receipt_fixture()->(PathBuf,String) {
    let (dir,m,p)=fixture();let ready=evaluated(&dir,&m,&p).unwrap();
    for (name,value) in [("CATALOGO.json",serde_json::to_value(&ready.catalogo).unwrap()),
        ("FUENTES.json",serde_json::to_value(&ready.fuentes).unwrap()),("MAPA.json",ready.mapa),
        ("POLITICA.json",serde_json::to_value(p).unwrap())] {
        fs::write(dir.join(name),serde_json::to_vec(&value).unwrap()).unwrap();
    }
    let id=|name:&str|Identidad{ruta:name.into(),sha256:sha256(&fs::read(dir.join(name)).unwrap())};
    let receipt=Recibo {version:1,politica:id("POLITICA.json"),catalogo:id("CATALOGO.json"),
        fuentes:id("FUENTES.json"),mapa:id("MAPA.json"),documentos:3,
        secciones:ready.catalogo.documents.iter().map(|d|d.sections.len()).sum()};
    let raw=serde_json::to_vec(&receipt).unwrap();fs::write(dir.join("RECIBO.json"),&raw).unwrap();
    (dir,sha256(&raw))
}
#[test]
fn autorizacion_integra_admite_y_recibo_ausente_o_sin_huella_rechaza() {
    let (dir,hash)=receipt_fixture();assert!(recibir_autorizacion(&dir,&hash).is_ok());
    assert!(recibir_autorizacion(&dir,"").is_err());
    fs::remove_file(dir.join("RECIBO.json")).unwrap();assert!(recibir_autorizacion(&dir,&hash).is_err());
}
#[test]
fn sustitucion_de_cada_artefacto_rechazada() {
    for name in ["CATALOGO.json","FUENTES.json","MAPA.json","POLITICA.json","RECIBO.json"] {
        let (dir,hash)=receipt_fixture();let mut raw=fs::read(dir.join(name)).unwrap();raw.push(b' ');
        fs::write(dir.join(name),raw).unwrap();
        assert!(recibir_autorizacion(&dir,&hash).is_err(),"{name}");
    }
}
#[test]
fn recalcular_recibo_ajeno_no_altera_autorizacion_externa() {
    let (dir,hash)=receipt_fixture();
    let mut c:Value=serde_json::from_slice(&fs::read(dir.join("CATALOGO.json")).unwrap()).unwrap();
    c["documents"][0]["title"]=json!("Título sustituido");
    let raw=serde_json::to_vec(&c).unwrap();fs::write(dir.join("CATALOGO.json"),&raw).unwrap();
    let mut r:Recibo=leer_json(&fs::read(dir.join("RECIBO.json")).unwrap()).unwrap();r.catalogo.sha256=sha256(&raw);
    fs::write(dir.join("RECIBO.json"),serde_json::to_vec(&r).unwrap()).unwrap();
    assert_eq!(recibir_autorizacion(&dir,&hash).err().unwrap(),"HUELLA_NO_COINCIDE");
}

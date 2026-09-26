
use std::{fs,path::Path};
use serde_json::json;
use scraper::{Html,Selector};
use sv_mcp_documental::{Catalog,sha256,custodia::save};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();let root=Path::new(&a[1]);let raw=fs::read(root.join("emulacion-dominio-inmunologia/pdq.html"))?;let cb=fs::read(root.join("emulacion-dominio-inmunologia/catalogo-pdq.json"))?;let c:Catalog=serde_json::from_slice(&cb)?;let html=Html::parse_document(std::str::from_utf8(&raw)?);
 let selector=Selector::parse("#cgvBody .accordion > section").unwrap();let links=Selector::parse("a[href]").unwrap();let tables=Selector::parse("table").unwrap();let imgs=Selector::parse("img").unwrap();
 let v:Vec<_>=html.select(&selector).zip(c.documents[0].sections.iter()).map(|(node,s)|json!({"id":s.id,"titulo":s.title,"caracteres":s.text.chars().count(),"bytes":s.text.len(),"sha256":s.sha256,"enlaces_html":node.select(&links).count(),"tablas_html":node.select(&tables).count(),"imagenes_html":node.select(&imgs).count(),"correspondencia_id":node.value().attr("id")==Some(s.id.as_str()),"bibliografia_presente":s.text.contains("Referencias bibliográficas")||s.text.contains("Bibliografía")})).collect();
 let out=json!({"TT":"TT-0014","suceso":"S39","documento":c.documents[0].id,"titulo":c.documents[0].title,"url":c.documents[0].url,"recuperacion_utc":c.documents[0].retrieved_utc,"actualizacion_declarada":c.documents[0].updated_source,"html_sha256":sha256(&raw),"catalogo_sha256":sha256(&cb),"extractor":"importar-pdq 0.1.1; scraper 0.25.0; html2text 0.16.6; ancho 100","secciones":v,"alcance":"Cinco secciones documentales completas, incluida informacion editorial y referencias. HTML original integro conservado. Navegacion global, scripts y estilo no se incorporan al texto del catalogo. Los enlaces se conservan como referencias; no se recuperan sus destinos. La conversion lineal no reproduce el diseno visual."});
 save(&root.join("evidencias/CATALOGO.json"),&serde_json::to_vec_pretty(&out)?)?;
 println!("{}",serde_json::to_string_pretty(&out)?);Ok(())
}

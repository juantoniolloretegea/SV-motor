use sv_pdf_documental::{extract,hash};
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;
const PDF:&[u8]=include_bytes!("fixtures/hairy-cell-leukemia.pdf");
fn counts(s:&str)->BTreeMap<char,usize>{let mut m=BTreeMap::new();for c in s.nfkc().filter(|c|!c.is_whitespace()){*m.entry(c).or_default()+=1;}m}
#[test]fn conserva_diez_paginas_caracteres_y_ligaduras_con_fuentes_reutilizadas(){
 let e=extract(PDF,&hash(PDF)).unwrap();let reference:serde_json::Value=serde_json::from_slice(include_bytes!("fixtures/referencia.json")).unwrap();
 assert_eq!(e.paginas.len(),10);
 for(i,p)in e.paginas.iter().enumerate(){assert_eq!(p.indice,i);assert_eq!(p.pagina_impresa_ordinal,i+1);assert_eq!(counts(&p.texto),counts(reference["segmentos"][i]["texto"].as_str().unwrap()),"Página {i}");assert_eq!(hash(p.texto.as_bytes()),p.sha256);}
 assert!(e.paginas[0].texto.contains("pasudotox-tdfk"));
 assert_eq!(e.sustituciones_tipograficas.values().sum::<usize>(),69);
}
#[test]fn rechaza_huella_cambiada_antes_de_descodificar(){assert!(extract(PDF,&"0".repeat(64)).unwrap_err().contains("HUELLA"));}
#[test]fn rechaza_archivo_que_no_es_pdf(){let b=b"texto";assert!(extract(b,&hash(b)).unwrap_err().contains("CABECERA"));}
#[test]fn rechaza_pdf_danado_sin_resultado_parcial(){let b=b"%PDF-1.7\nerror";assert!(extract(b,&hash(b)).is_err());}
#[test]fn rechaza_entrada_superior_a_la_cota(){let b=vec![0;sv_pdf_documental::MAX_PDF+1];assert_eq!(extract(&b,"").unwrap_err(),"PDF_SUPERA_COTA");}
#[test]fn rechaza_pagina_sin_capa_de_texto(){
 use lopdf::{Document,dictionary};let mut d=Document::with_version("1.7");let root=d.new_object_id();
 let page=d.add_object(dictionary!{"Type"=>"Page","Parent"=>root,"MediaBox"=>vec![0.into(),0.into(),600.into(),800.into()],"Resources"=>dictionary!{}});
 d.objects.insert(root,lopdf::Object::Dictionary(dictionary!{"Type"=>"Pages","Kids"=>vec![page.into()],"Count"=>1}));
 let catalog=d.add_object(dictionary!{"Type"=>"Catalog","Pages"=>root});d.trailer.set("Root",catalog);let mut b=Vec::new();d.save_to(&mut b).unwrap();
 assert!(extract(&b,&hash(&b)).unwrap_err().contains("SIN_TEXTO"));
}
#[test]fn fallo_en_segunda_pagina_no_se_convierte_en_final_correcto(){
 use lopdf::{Document,dictionary,Object,Stream,content::{Content,Operation}};
 let mut d=Document::with_version("1.7");let root=d.new_object_id();
 let font=d.add_object(dictionary!{"Type"=>"Font","Subtype"=>"Type1","BaseFont"=>"Helvetica"});
 let content=Content{operations:vec![Operation::new("BT",vec![]),Operation::new("Tf",vec!["F1".into(),12.into()]),Operation::new("Tj",vec![Object::string_literal("Texto inicial conservable")]),Operation::new("ET",vec![])]};
 let stream=d.add_object(Stream::new(dictionary!{},content.encode().unwrap()));
 let first=d.add_object(dictionary!{"Type"=>"Page","Parent"=>root,"MediaBox"=>vec![0.into(),0.into(),600.into(),800.into()],"Resources"=>dictionary!{"Font"=>dictionary!{"F1"=>font}},"Contents"=>stream});
 let second=d.add_object(dictionary!{"Type"=>"Page","Parent"=>root,"MediaBox"=>vec![0.into(),0.into(),600.into(),800.into()],"Resources"=>dictionary!{}});
 d.objects.insert(root,Object::Dictionary(dictionary!{"Type"=>"Pages","Kids"=>vec![first.into(),second.into()],"Count"=>2}));
 let catalog=d.add_object(dictionary!{"Type"=>"Catalog","Pages"=>root});d.trailer.set("Root",catalog);let mut b=Vec::new();d.save_to(&mut b).unwrap();
 assert!(extract(&b,&hash(&b)).unwrap_err().contains("PAGINA_1_SIN_TEXTO"));
}

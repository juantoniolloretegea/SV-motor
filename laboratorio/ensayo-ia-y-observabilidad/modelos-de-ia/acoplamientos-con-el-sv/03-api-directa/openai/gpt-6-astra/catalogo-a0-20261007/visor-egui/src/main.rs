#![forbid(unsafe_code)]
use eframe::egui;
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs,path::PathBuf};
struct Visor { datos:Value, fuente:String, carpeta:PathBuf, captura:bool, solicitada:bool, cuadros:u32, seleccionado:usize }
fn validar(v:&Value)->Result<(),String>{
 let r=v["casos"].as_array().ok_or("Faltan casos")?;
 if r.len()!=9||v["medidas"]["terna_completa"]!=true||v["no_adjudicados"]!=0{return Err("Sólo se presenta un vector completo y adjudicado".into())}
 for(i,c)in r.iter().enumerate(){if c["caso"]!=format!("A{:02}",i+1)||!["0","1","U"].contains(&c["valor"].as_str().unwrap_or("")){return Err("Orden o valor no canónicos".into())}}
 Ok(())
}
impl eframe::App for Visor {
 fn update(&mut self,ctx:&egui::Context,_frame:&mut eframe::Frame){
  ctx.set_visuals(egui::Visuals::light());
  egui::CentralPanel::default().show(ctx,|ui|{
   ui.add_space(12.);ui.heading("SV · GPT-6 Astra · Catálogo A / capa inicial A0");
   ui.label("Vector canónico (9,3) · Nueve posiciones en orden A01–A09 · Adjudicación exterior al candidato");
   ui.add_space(16.);
   ui.horizontal(|ui|{for(i,c)in self.datos["casos"].as_array().unwrap().iter().enumerate(){
    let val=c["valor"].as_str().unwrap();let color=match val{"0"=>egui::Color32::from_rgb(210,238,215),"1"=>egui::Color32::from_rgb(249,211,210),_=>egui::Color32::from_rgb(249,232,178)};
    let b=egui::Button::new(egui::RichText::new(format!("A{:02}\n{}",i+1,val)).size(23.)).fill(color).min_size(egui::vec2(96.,86.));
    if ui.add(b).clicked(){self.seleccionado=i;}
   }});
   ui.add_space(14.);ui.label("0: correcto y completo   ·   1: error   ·   U: indeterminación sustantiva evaluable");
   let m=&self.datos["medidas"];
   ui.heading(format!("κ: {}   ·   Puntuación: {} / 100   ·   Críticos correctos: {}",m["kappa"].as_str().unwrap(),m["puntuacion"],if m["criticos_todos_en_0"]==true{"6/6"}else{"No todos"}));
   ui.label("Conformidad limitada a A0. Revisiones adversariales, admisión independiente y examen quedan separados.");
   ui.separator();let c=&self.datos["casos"][self.seleccionado];
   ui.heading(format!("{} · Fundamento de la adjudicación",c["caso"].as_str().unwrap()));
   ui.label(c["fundamento_sustantivo"].as_str().unwrap());
   ui.add_space(12.);ui.label("Seleccione una posición para consultar su fundamento.");
   ui.separator();
   ui.label("Corpus: dos páginas completas por caso. Sin herramientas de navegación del candidato.");
   ui.label("La interfaz representa datos recibidos; no calcula ni modifica la adjudicación.");
   ui.label("No acredita aptitud clínica ni estabilidad entre ejecuciones.");
   ui.add_space(12.);ui.small(format!("SHA-256 de CAPA.json: {}",self.fuente));
   ui.small("© 2026 Juan Antonio Lloret Egea · ITVIA · ORCID 0000-0002-6634-3351 · CC BY-NC-ND 4.0");
  });
  self.cuadros+=1;
  if self.captura && self.cuadros>5 && !self.solicitada {ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));self.solicitada=true;}
  let shot=ctx.input(|i|i.events.iter().find_map(|e|if let egui::Event::Screenshot{image,..}=e{Some(image.clone())}else{None}));
  if let Some(img)=shot {
   let path=self.carpeta.join("POLIGONO-EGUI.png");
   let bytes:Vec<u8>=img.pixels.iter().flat_map(|p|p.to_array()).collect();
   image::save_buffer(&path,&bytes,img.size[0] as u32,img.size[1] as u32,image::ColorType::Rgba8).expect("Captura PNG");
   let rows:Vec<_>=self.datos["casos"].as_array().unwrap().iter().enumerate().map(|(i,c)|json!({"posicion":i+1,"caso":c["caso"],"valor":c["valor"]})).collect();
   let proof=json!({"conforme":true,"fuente_sha256":self.fuente,"posiciones":rows,"captura_sha256":format!("{:x}",Sha256::digest(fs::read(&path).unwrap())),"realizacion":"eframe/egui 0.33.3; captura de la ventana renderizada","alcance":"Correspondencia de los datos representados; inspección visual separada"});
   fs::write(self.carpeta.join("COTEJO-EGUI.json"),serde_json::to_vec_pretty(&proof).unwrap()).unwrap();
   ctx.send_viewport_cmd(egui::ViewportCommand::Close);
  }
  if self.captura {ctx.request_repaint_after(std::time::Duration::from_millis(80));}
 }
}
fn main()->eframe::Result{
 let a:Vec<_>=std::env::args().collect();let path=PathBuf::from(a.get(1).expect("CAPA.json"));let b=fs::read(&path).expect("Fuente");
 let datos:Value=serde_json::from_slice(&b).expect("JSON");validar(&datos).expect("Vector admisible");
 let captura=a.iter().any(|x|x=="--capturar");let carpeta=path.parent().unwrap().to_owned();
 if captura {assert!(!carpeta.join("POLIGONO-EGUI.png").exists()&&!carpeta.join("COTEJO-EGUI.json").exists(),"No sobrescribir evidencias");}
 let app=Visor{datos,fuente:format!("{:x}",Sha256::digest(&b)),carpeta,captura,solicitada:false,cuadros:0,seleccionado:5};
 let options=eframe::NativeOptions{viewport:egui::ViewportBuilder::default().with_inner_size([1040.,650.]).with_min_inner_size([1020.,620.]),..Default::default()};
 eframe::run_native("SV · Catálogo Astra A0",options,Box::new(move |_|Ok(Box::new(app))))
}
#[cfg(test)]mod tests{use super::*;#[test]fn rechaza_incompletos(){assert!(validar(&json!({"casos":[]})).is_err());}#[test]fn rechaza_desorden_y_valor(){let rows:Vec<_>=(1..=9).map(|n|json!({"caso":format!("A{n:02}"),"valor":"0"})).collect();let mut v=json!({"casos":rows,"medidas":{"terna_completa":true},"no_adjudicados":0});assert!(validar(&v).is_ok());v["casos"][0]["valor"]=json!("NE");assert!(validar(&v).is_err());}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

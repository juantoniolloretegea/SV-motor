#![forbid(unsafe_code)]
use eframe::egui::{self,Color32,Pos2,Sense,Stroke,Vec2};
use serde_json::Value;
pub mod contrato;
pub const FUENTE:&[u8]=include_bytes!("DATOS.json");
pub const DICTAMEN:&[u8]=FUENTE;
pub const HUELLA:&str="a64ffa1cf57ca4d1600d806a8a93cbbe6e2fbefb185dab861ae476a6266d2e94";
pub const PIE:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
const ORDEN:[&str;9]=["C01","C02","C03","C05","C06","C08","C10","C11","C16"];
pub fn validar(d:&Value)->Result<(),String>{
    let rows=d["casos"].as_array().filter(|r|r.len()==9).ok_or("Vector incompleto")?;
    for(r,id)in rows.iter().zip(ORDEN){
        if r["id"]!=id||r["critica"]!=true{return Err("Orden o criticidad distintos".into());}
        let stages=r["etapas"].as_array().filter(|s|s.len()==3).ok_or("Terna incompleta")?;
        for(s,stage)in stages.iter().zip(0..3){
            if s["etapa"]!=stage||s["final_sha256"].as_str().is_none()||s["fundamento"].as_str().is_none(){return Err("Adjudicación incompleta".into());}
            contrato::Tri::leer(s["valor"].as_str().ok_or("Marca")?)?;
        }
    }
    Ok(())
}
pub struct Visor{pub datos:Value,pub etapa:usize,pub seleccionado:usize,pub objetivos:Vec<Pos2>,pub botones:Vec<egui::Rect>,pub etapas_rect:Vec<egui::Rect>}
impl Default for Visor{fn default()->Self{
    assert_eq!(contrato::huella(FUENTE),HUELLA);let datos:Value=serde_json::from_slice(FUENTE).expect("Datos recibidos");validar(&datos).expect("Correspondencia recibida");
    Self{datos,etapa:2,seleccionado:0,objetivos:vec![],botones:vec![],etapas_rect:vec![]}
}}
impl Visor{
    fn vector(&self)->Vec<contrato::Tri>{self.datos["casos"].as_array().unwrap().iter().map(|r|contrato::Tri::leer(r["etapas"][self.etapa]["valor"].as_str().unwrap()).unwrap()).collect()}
    pub fn resumen_seleccion(&self)->String{let c=&self.datos["casos"][self.seleccionado];let r=&c["etapas"][self.etapa];format!("{} · R{} · valor {} · {}",c["id"].as_str().unwrap(),self.etapa,r["valor"].as_str().unwrap(),r["fundamento"].as_str().unwrap())}
    pub fn mostrar(&mut self,ctx:&egui::Context){
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx,|ui|{egui::ScrollArea::vertical().show(ui,|ui|{
            ui.heading("Claude Opus 5.5 · Nodo 02 · Contraste CYB09");
            ui.label("Dictamen asistido exterior al candidato · Recepción científica independiente pendiente");
            ui.label("Nueve preguntas seleccionadas del banco histórico; no acredita aptitud general de ciberseguridad.");
            self.etapas_rect.clear();ui.horizontal(|ui|{for s in 0..3{let r=ui.selectable_label(self.etapa==s,format!("R{s}{}",if s==2{" · final"}else{""}));self.etapas_rect.push(r.rect);if r.clicked(){self.etapa=s;}}});
            let v=self.vector();let(rec,t,k)=contrato::clasificar(&v).unwrap();let crit=contrato::control_criticidad(&v,&[Some(true);9]).unwrap();
            ui.strong(format!("Vector: {} · κ auxiliar: {k} · Correctas: {}/9",v.iter().map(|v|v.simbolo()).collect::<Vec<_>>().join(","),rec[0]));
            ui.label(format!("Célula (9,3) · T(9)={t} · Nueve posiciones críticas · Control: {crit}"));
            ui.strong(if v.iter().all(|x|*x==contrato::Tri::Cero){"Dictamen de la etapa: admisible según la clave; recepción independiente pendiente"}else{"Dictamen de la etapa: NO ADMISIBLE según la clave; hay posiciones críticas sin satisfacción acreditada"});
            ui.label("0: correcto, rojo, radio 1 · 1: error, verde, radio 2 · U: indeterminación, azul, radio 3");
            let ancho=ui.available_width().min(690.);let alto=ancho.min(500.).max(320.);
            let(rect,response)=ui.allocate_exact_size(Vec2::new(ancho,alto),Sense::click());let p=ui.painter_at(rect);let center=rect.center();let unidad=(alto/2.-45.)/3.;
            for r in 1..=3{p.circle_stroke(center,unidad*r as f32,Stroke::new(0.7_f32,Color32::LIGHT_GRAY));}
            self.objetivos.clear();
            for(i,value)in v.iter().enumerate(){let a=-std::f32::consts::FRAC_PI_2+i as f32*std::f32::consts::TAU/9.;let direction=Vec2::new(a.cos(),a.sin());let point=center+direction*(unidad*value.radio() as f32);self.objetivos.push(point);
                p.line_segment([center,center+direction*unidad*3.],Stroke::new(0.7_f32,Color32::LIGHT_GRAY));
                p.text(center+direction*(unidad*3.+23.),egui::Align2::CENTER_CENTER,ORDEN[i],egui::FontId::proportional(15.),Color32::BLACK);
            }
            p.add(egui::Shape::closed_line(self.objetivos.clone(),Stroke::new(2.0_f32,Color32::from_rgb(27,62,99))));
            for(i,point)in self.objetivos.iter().enumerate(){let[r,g,b]=v[i].rgb();p.circle_filled(*point,if i==self.seleccionado{7.}else{5.},Color32::from_rgb(r,g,b));}
            if response.clicked(){if let Some(at)=response.interact_pointer_pos(){if let Some((i,_))=self.objetivos.iter().enumerate().filter(|(_,p)|p.distance(at)<22.).min_by(|a,b|a.1.distance(at).total_cmp(&b.1.distance(at))){self.seleccionado=i;}}}
            self.botones.clear();ui.horizontal_wrapped(|ui|{for(i,id)in ORDEN.iter().enumerate(){let r=ui.selectable_label(self.seleccionado==i,*id);self.botones.push(r.rect);if r.clicked(){self.seleccionado=i;}}});
            ui.separator();let c=&self.datos["casos"][self.seleccionado];let r=&c["etapas"][self.etapa];
            ui.heading(format!("{} · {} · R{}",c["id"].as_str().unwrap(),c["titulo"].as_str().unwrap(),self.etapa));
            ui.label(c["pregunta"].as_str().unwrap());ui.strong("Fundamento de la adjudicación exterior");ui.label(r["fundamento"].as_str().unwrap());
            ui.collapsing("Respuesta y evidencia conservadas",|ui|{ui.label(r["respuesta"].as_str().unwrap());ui.monospace(format!("SHA-256 del original final: {}",r["final_sha256"].as_str().unwrap()));});
            ui.collapsing("Datos operativos declarados por el candidato",|ui|{ui.label("Declaración atribuida; no equivale a observación de sus procesos internos.");ui.monospace(serde_json::to_string_pretty(&r["informe_operativo"]).unwrap());});
            ui.collapsing("Correspondencia, consumo y límites",|ui|{ui.label("R2 es la entrega final; no se elige la mejor etapa. Cuatro originales reutilizados, 23 generaciones nuevas. Núcleo, semántica e IR intactos.");ui.monospace(serde_json::to_string_pretty(&self.datos["mediciones"]).unwrap());ui.label(format!("SHA-256 de datos recibidos: {HUELLA}"));});
            ui.separator();ui.small(PIE);
        });});
        #[cfg(target_arch="wasm32")]
        if let Some(e)=web_sys::window().and_then(|w|w.document()).and_then(|d|d.get_element_by_id("seleccion")){e.set_text_content(Some(&self.resumen_seleccion()));}
    }
}
impl eframe::App for Visor{fn update(&mut self,ctx:&egui::Context,_:&mut eframe::Frame){self.mostrar(ctx);}}
#[cfg(target_arch="wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn iniciar(canvas:web_sys::HtmlCanvasElement)->Result<(),wasm_bindgen::JsValue>{eframe::WebRunner::new().start(canvas,eframe::WebOptions::default(),Box::new(|_|Ok(Box::new(Visor::default())))).await}

#[cfg(test)] mod tests {
    use super::*;use serde_json::json;
    fn fixture()->Visor{let rows:Vec<_>=ORDEN.iter().enumerate().map(|(i,id)|json!({"id":id,"critica":true,"titulo":id,"pregunta":"Prueba exclusivamente instrumental","etapas":(0..3).map(|s|json!({"etapa":s,"valor":(["0","1","U"][(i+s)%3]),"final_sha256":"f".repeat(64),"fundamento":format!("Fundamento {id} R{s}"),"respuesta":"Dato sintético de comprobación","informe_operativo":null})).collect::<Vec<_>>()})).collect();Visor{datos:json!({"casos":rows,"mediciones":null}),etapa:2,seleccionado:0,objetivos:vec![],botones:vec![],etapas_rect:vec![]}}
    fn cuadro(v:&mut Visor,c:&egui::Context,events:Vec<egui::Event>)->egui::FullOutput{c.run(egui::RawInput{screen_rect:Some(egui::Rect::from_min_size(Pos2::ZERO,Vec2::new(1200.,1000.))),events,..Default::default()},|c|v.mostrar(c))}
    fn clic(v:&mut Visor,c:&egui::Context,p:Pos2)->egui::FullOutput{let _=cuadro(v,c,vec![egui::Event::PointerMoved(p),egui::Event::PointerButton{pos:p,button:egui::PointerButton::Primary,pressed:true,modifiers:Default::default()}]);cuadro(v,c,vec![egui::Event::PointerButton{pos:p,button:egui::PointerButton::Primary,pressed:false,modifiers:Default::default()}])}
    #[test]fn escala_y_clasificacion_exhaustiva(){use contrato::Tri::*;assert_eq!([Cero.radio(),Uno.radio(),U.radio()],[1,2,3]);assert_eq!([Cero.nombre_color(),Uno.nombre_color(),U.nombre_color()],["rojo","verde","azul"]);let mut counts=[0usize;3];for i in 0..3usize.pow(9){let mut k=i;let v:Vec<_>=(0..9).map(|_|{let x=[Cero,Uno,U][k%3];k/=3;x}).collect();let(_,t,r)=contrato::clasificar(&v).unwrap();assert_eq!(t,7);counts[match r{"Apto"=>0,"No apto"=>1,_=>2}]+=1;}assert_eq!(counts,[163,163,19357]);assert!(contrato::clasificar(&[Cero;8]).is_err());let mut v=[Cero;9];v[8]=Uno;assert_eq!(contrato::control_criticidad(&v,&[Some(true);9]).unwrap(),"No apto");v[8]=U;assert_eq!(contrato::control_criticidad(&v,&[Some(true);9]).unwrap(),"Indeterminada");}
    #[test]fn no_se_representan_posiciones_o_ternas_incompletas(){let mut d=fixture().datos;validar(&d).unwrap();d["casos"][0]["id"]=json!("C16");assert!(validar(&d).is_err());let mut d=fixture().datos;d["casos"][0]["etapas"].as_array_mut().unwrap().pop();assert!(validar(&d).is_err());let mut d=fixture().datos;d["casos"][0]["critica"]=json!(false);assert!(validar(&d).is_err());}
    #[test]fn vertices_botones_y_etapas_conservan_fuente(){let mut v=fixture();let source=v.datos.clone();let c=egui::Context::default();let _=cuadro(&mut v,&c,vec![]);let out=cuadro(&mut v,&c,vec![]);assert!(out.shapes.iter().any(|s|matches!(&s.shape,egui::Shape::Path(p)if p.closed&&p.points==v.objetivos&&p.points.len()==9)));for i in (0..9).rev(){let p=v.objetivos[i];let out=clic(&mut v,&c,p);assert_eq!(v.seleccionado,i);assert!(out.shapes.iter().any(|s|matches!(&s.shape,egui::Shape::Text(t)if t.galley.job.text==format!("Fundamento {} R2",ORDEN[i]))));}for i in 0..9{let p=v.botones[i].center();let _=clic(&mut v,&c,p);assert_eq!(v.seleccionado,i);}for s in 0..3{let p=v.etapas_rect[s].center();let _=clic(&mut v,&c,p);assert_eq!(v.etapa,s);assert!(v.resumen_seleccion().contains(&format!("R{s}")));}assert_eq!(v.datos,source);}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

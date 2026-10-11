//! Comprobación aislada sobre documentos sintéticos propios; no concede permisos.
#![forbid(unsafe_code)]
use hayro::{PixmapSettings, RenderCache, RenderSettings, hayro_syntax::Pdf,
    hayro_interpret::InterpreterSettings, vello_cpu::color::palette::css::WHITE};
use lopdf::{Document, Object, Stream, dictionary};
use serde_json::json;
use sha2::{Digest,Sha256};
use std::{path::Path, time::Instant};
fn sha(b: &[u8])->String { format!("{:x}",Sha256::digest(b)) }
fn original(caso:usize)->Vec<u8> {
    let mut d=Document::with_version("1.7");
    let pages=d.new_object_id();
    let font=d.add_object(dictionary!{"Type"=>"Font","Subtype"=>"Type1","BaseFont"=>"Helvetica"});
    let raster=d.add_object(Stream::new(dictionary!{"Type"=>"XObject","Subtype"=>"Image","Width"=>2,"Height"=>2,"ColorSpace"=>"DeviceRGB","BitsPerComponent"=>8},vec![255,0,0, 0,255,0, 0,0,255, 0,0,0]));
    let alpha=d.add_object(dictionary!{"Type"=>"ExtGState","ca"=>0.5});
    let recursos=d.add_object(dictionary!{"Font"=>dictionary!{"F1"=>font},"XObject"=>dictionary!{"I1"=>raster},"ExtGState"=>dictionary!{"G1"=>alpha}});
    let contenido=match caso {
        1=>"0 0 1 rg 20 20 60 40 re f",
        2=>"q 80 0 0 80 20 20 cm /I1 Do Q",
        3=>"q 80 0 0 80 20 20 cm /I1 Do Q 0 0 0 RG 4 w 20 20 m 100 100 l S BT /F1 12 Tf 20 110 Td (FIGURA 3) Tj ET",
        4=>"0 0 1 rg 20 20 60 40 re f",
        5=>"0 0 1 rg 0 0 200 160 re f",
        6=>"q /G1 gs 0 0 1 rg 20 20 60 40 re f Q",
        7=>"q 30 30 30 20 re W n 0 0 1 rg 0 0 200 160 re f Q",
        8=>"BT /F1 18 Tf 20 80 Td (TABLA 17 29) Tj ET 0 0 0 RG 1 w 15 70 155 30 re S 85 70 m 85 100 l S",
        9=>"0 0 1 rg 20 20 m 100 20 l 60 90 l h f 0 0 0 RG 2 w 100 40 m 160 40 l 150 50 m 160 40 l 150 30 l S",
        _=>unreachable!()
    };
    let content=d.add_object(Stream::new(dictionary!{},contenido.as_bytes().to_vec()));
    let mut page=dictionary!{"Type"=>"Page","Parent"=>pages,"MediaBox"=>vec![0.into(),0.into(),200.into(),160.into()],"Resources"=>recursos,"Contents"=>content};
    if caso==4 {page.set("Rotate",90);}
    if caso==5 {page.set("CropBox",vec![20.into(),30.into(),120.into(),110.into()]);}
    let id=d.add_object(page);
    d.objects.insert(pages,Object::Dictionary(dictionary!{"Type"=>"Pages","Kids"=>vec![id.into()],"Count"=>1}));
    let root=d.add_object(dictionary!{"Type"=>"Catalog","Pages"=>pages});d.trailer.set("Root",root);
    let mut raw=Vec::new();d.save_to(&mut raw).unwrap();raw
}
fn representar(raw:Vec<u8>,pagina:usize,escala:f32)->Result<Vec<u8>,String>{
    if raw.len()>20*1024*1024 || pagina==0 || ![1.0,2.0].contains(&escala) {return Err("COTAS_INVALIDAS".into())}
    let pdf=Pdf::new(raw).map_err(|_|"PDF_NO_ADMITIDO")?;
    let p=pdf.pages().get(pagina-1).ok_or("PAGINA_AUSENTE")?;
    let(w,h)=p.render_dimensions();
    if !w.is_finite() || !h.is_finite() || w<=0.0 || h<=0.0 || w*escala>4096.0 || h*escala>4096.0 || w*h*escala*escala>8_000_000.0 {return Err("PAGINA_EXCESIVA".into())}
    hayro::render(p,&RenderCache::new(),&InterpreterSettings::default(),&RenderSettings::default(),&PixmapSettings{x_scale:escala,y_scale:escala,bg_color:WHITE}).into_png().map_err(|_|"PNG_NO_RECIBIDO".into())
}
fn comprobar(caso:usize, png:&[u8])->bool {
    let im=image::load_from_memory(png).unwrap().to_rgb8();
    let size=if caso==4 {(160,200)} else if caso==5 {(100,80)}else{(200,160)};
    if im.dimensions()!=size{return false}
    let blue=im.pixels().filter(|p|p[0]<10 && p[1]<10 && p[2]>245).count();
    let black=im.pixels().filter(|p|p[0]<30 && p[1]<30 && p[2]<30).count();
    match caso {
        1|4=>blue==2400,
        2=>{let red=im.pixels().filter(|p|p[0]>245 && p[1]<10 && p[2]<10).count();red==1600 && blue==1600 && black==1600},
        3=>blue>1200 && black>1600,
        5=>blue==8000,
        6=>im.pixels().filter(|p|(120..=135).contains(&p[0]) && (120..=135).contains(&p[1]) && p[2]>245).count()==2400,
        7=>blue==600,
        8=>black>500,
        9=>blue>2600 && black>100,
        _=>false
    }
}
fn main()->Result<(),Box<dyn std::error::Error>> {
    let out=std::env::args().nth(1).ok_or("Falta directorio de resultados")?;
    let out=Path::new(&out);std::fs::create_dir_all(out)?;
    let mut resultados=vec![];
    let nombres=["Trazos vectoriales","Imagen RGB incrustada","Imagen, trazos y texto superpuestos","Rotación de página","Recorte de página","Transparencia uniforme","Región de recorte gráfico","Texto y tabla dibujada","Diagrama vectorial"];
    for caso in 1..=9 {
        let raw=original(caso);let h=sha(&raw);std::fs::write(out.join(format!("caso-{caso}.pdf")),&raw)?;
        let t=Instant::now();let png=representar(raw.clone(),1,1.0)?;let ns=t.elapsed().as_nanos();let repeticion=representar(raw.clone(),1,1.0)?;
        let conforme=comprobar(caso,&png);let reproducible=png==repeticion;
        let zoom=representar(raw.clone(),1,2.0)?;
        let imagen=image::load_from_memory(&png)?;let ampliada=image::load_from_memory(&zoom)?;
        let aumento=ampliada.width()==2*imagen.width() && ampliada.height()==2*imagen.height();
        std::fs::write(out.join(format!("caso-{caso}.png")),&png)?;
        std::fs::write(out.join(format!("caso-{caso}-ampliado.png")),&zoom)?;
        resultados.push(json!({"caso":caso,"descripcion":nombres[caso-1],"original_sha256":h,"png_sha256":sha(&png),"png_ampliado_sha256":sha(&zoom),"geometria_y_color_conformes":conforme,"reproducible":reproducible,"ampliacion_doble":aumento,"duracion_ns":ns,"pagina":1,"total_paginas":1}));
    }
    let limites=json!({"pagina_cero_rechazada":representar(original(1),0,1.0).is_err(),"pagina_ausente_rechazada":representar(original(1),2,1.0).is_err(),"escala_fuera_cota_rechazada":representar(original(1),1,3.0).is_err(),"archivo_invalido_rechazado":representar(b"sin pdf".to_vec(),1,1.0).is_err()});
    let conforme=resultados.iter().all(|x|x["geometria_y_color_conformes"]==true && x["reproducible"]==true && x["ampliacion_doble"]==true) && limites.as_object().unwrap().values().all(|v|v==true);
    let r=json!({"candidato":"hayro 0.8.0","tipo":"prueba instrumental aislada","documentos":"nueve documentos sintéticos propios; no son bibliografía","conforme":conforme,"casos":resultados,"limites":limites,"alcance":"No acredita todos los PDF, OCR, reconocimiento de figuras, fidelidad editorial integral, permisos, entrega a IA ni integración operativa"});
    std::fs::write(out.join("RESULTADOS.json"),serde_json::to_vec_pretty(&r)?)?;
    println!("{}",serde_json::to_string_pretty(&r)?);
    if !conforme {std::process::exit(2)}
    Ok(())
}

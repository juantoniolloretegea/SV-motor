#![forbid(unsafe_code)]
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use serde_json::Value;
use sha2::{Digest, Sha256};
pub const FUENTE: &[u8] = include_bytes!("CAPA.json");
pub const HUELLA: &str = "1be4b387d9fb44fe4a6e9d3bfae0aa2db24cd93caf3aa389722a2a964dc766be";
pub const PIE: &str = "© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
const AZUL: Color32 = Color32::from_rgb(27, 62, 99);
fn color(valor: &str) -> Color32 {
    match valor {
        "0" => Color32::from_rgb(21, 119, 80),
        "1" => Color32::from_rgb(181, 42, 45),
        _ => Color32::from_rgb(155, 103, 0),
    }
}
pub fn radio(valor: &str) -> Result<f32, String> {
    match valor {
        "0" => Ok(1.),
        "1" => Ok(2.),
        "U" => Ok(3.),
        _ => Err("Símbolo no admisible".into()),
    }
}
pub fn validar(v: &Value) -> Result<(), String> {
    let rows = v["casos"].as_array().ok_or("Faltan casos")?;
    if rows.len() != 9 || v["medidas"]["terna_completa"] != true || v["no_adjudicados"] != 0 {
        return Err("Sólo se representa el vector completo y adjudicado".into());
    }
    for (i, c) in rows.iter().enumerate() {
        if c["caso"] != format!("A{:02}", i + 1) || c["fundamento_sustantivo"].as_str().is_none() {
            return Err("Orden o fundamento ausente".into());
        }
        radio(c["valor"].as_str().ok_or("Símbolo ausente")?)?;
    }
    Ok(())
}
fn direccion(i: usize) -> Vec2 {
    let a = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / 9.;
    Vec2::new(a.cos(), a.sin())
}
pub fn vertices(datos: &Value, centro: Pos2, unidad: f32) -> Vec<Pos2> {
    datos["casos"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, c)| {
            centro + direccion(i) * (unidad * radio(c["valor"].as_str().unwrap()).unwrap())
        })
        .collect()
}
pub struct Visor {
    pub datos: Value,
    pub seleccionado: usize,
    pub objetivos: Vec<Pos2>,
    pub botones: Vec<Rect>,
    pub mostrar_pasajes: bool,
}
impl Default for Visor {
    fn default() -> Self {
        assert_eq!(
            format!("{:x}", Sha256::digest(FUENTE)),
            HUELLA,
            "La fuente cambió"
        );
        let datos: Value = serde_json::from_slice(FUENTE).expect("Fuente JSON");
        validar(&datos).expect("Adjudicación completa");
        Self {
            datos,
            seleccionado: 0,
            objetivos: vec![],
            botones: vec![],
            mostrar_pasajes: true,
        }
    }
}
impl Visor {
    pub fn resumen_seleccion(&self) -> String {
        let c = &self.datos["casos"][self.seleccionado];
        format!(
            "{} · Valor {} · {}",
            c["caso"].as_str().unwrap(),
            c["valor"].as_str().unwrap(),
            c["fundamento_sustantivo"].as_str().unwrap()
        )
    }
    fn figura(&mut self, ui: &mut egui::Ui) {
        let lado = ui.available_width().clamp(260., 460.);
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(lado), Sense::hover());
        let centro = rect.center();
        let unidad = (lado / 2. - 48.) / 3.;
        let p = ui.painter_at(rect);
        for r in 1..=3 {
            p.circle_stroke(
                centro,
                unidad * r as f32,
                Stroke::new(1_f32, Color32::from_gray(192)),
            );
            p.text(
                centro + Vec2::new(unidad * r as f32, 10.),
                egui::Align2::LEFT_TOP,
                format!("r={r}"),
                egui::FontId::proportional(11.),
                Color32::from_gray(110),
            );
        }
        for i in 0..9 {
            p.line_segment(
                [centro, centro + direccion(i) * unidad * 3.],
                Stroke::new(1_f32, Color32::from_gray(216)),
            );
        }
        let puntos = vertices(&self.datos, centro, unidad);
        p.add(egui::Shape::closed_line(
            puntos.clone(),
            Stroke::new(2.5_f32, AZUL),
        ));
        self.objetivos = puntos.clone();
        for (i, punto) in puntos.iter().enumerate() {
            let etiqueta = centro + direccion(i) * (unidad * 3. + 24.);
            let respuesta = ui.interact(
                Rect::from_center_size(*punto, Vec2::splat(26.)),
                ui.id().with(("vertice", i)),
                Sense::click(),
            );
            let rotulo = ui.interact(
                Rect::from_center_size(etiqueta, Vec2::new(54., 34.)),
                ui.id().with(("rotulo", i)),
                Sense::click(),
            );
            if respuesta.clicked() || rotulo.clicked() {
                self.seleccionado = i;
            }
            let val = self.datos["casos"][i]["valor"].as_str().unwrap();
            if self.seleccionado == i {
                p.circle_stroke(*punto, 11., Stroke::new(2_f32, AZUL));
            }
            p.circle_filled(*punto, 6., color(val));
            p.text(
                etiqueta,
                egui::Align2::CENTER_CENTER,
                format!("A{:02}\n{}", i + 1, val),
                egui::FontId::proportional(14.),
                if self.seleccionado == i {
                    AZUL
                } else {
                    Color32::from_gray(65)
                },
            );
            respuesta
                .on_hover_text(format!("A{:02}: consultar fundamento", i + 1))
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            rotulo.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        ui.label("Seleccione un vértice, su rótulo o un botón A01–A09.");
        ui.small("Orden horario desde A01, arriba. Los radios codifican símbolos; no son magnitudes clínicas ni porcentajes.");
        ui.small("Los nueve valores son 0: el polígono está en el círculo interior (r=1).");
    }
    fn detalle(&mut self, ui: &mut egui::Ui) {
        let c = &self.datos["casos"][self.seleccionado];
        ui.heading(format!("{} · Fundamento", c["caso"].as_str().unwrap()));
        ui.add_space(8.);
        let val = c["valor"].as_str().unwrap();
        ui.colored_label(
            color(val),
            egui::RichText::new(format!(
                "Valor {val} · {}",
                c["sustantivo"].as_str().unwrap_or("Adjudicado")
            ))
            .strong(),
        );
        ui.label(format!(
            "Dificultad prefijada: {} · Crítico: {}",
            c["dificultad"],
            if c["critico_prefijado"] == true {
                "sí"
            } else {
                "no"
            }
        ));
        ui.add_space(8.);
        ui.label(c["fundamento_sustantivo"].as_str().unwrap());
        ui.add_space(12.);
        ui.checkbox(&mut self.mostrar_pasajes, "Mostrar pasajes contrastados");
        if self.mostrar_pasajes {
            for pasaje in c["pasajes_contrastados"].as_array().unwrap() {
                ui.add_space(8.);
                ui.strong(format!(
                    "{} · página {} · sección {}",
                    pasaje["documento"].as_str().unwrap(),
                    pasaje["pagina"].as_u64().unwrap() + 1,
                    pasaje["seccion"].as_str().unwrap()
                ));
                ui.label(pasaje["fragmento"].as_str().unwrap());
            }
        }
        ui.add_space(12.);
        egui::CollapsingHeader::new("Trazabilidad de esta posición").show(ui,|ui| {
            ui.label("Índice original de página en CAPA.json desde 0; aquí se muestra desde 1 para lectura humana.");
            ui.label(format!("Respuesta SHA-256: {}",c["final_sha256"].as_str().unwrap()));
            ui.label(format!("Auditoría SHA-256: {}",c["auditoria_sha256"].as_str().unwrap()));
        });
    }
    pub fn mostrar(&mut self, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx,|ui| { egui::ScrollArea::vertical().show(ui,|ui| {
            ui.heading("SV · Polígono de adjudicación · Astra A0");
            ui.label("Catálogo A01–A09 · Edición 07/10/2026 · Visor interactivo 0.2.0 · Rust / egui");
            let m=&self.datos["medidas"];
            ui.colored_label(AZUL,egui::RichText::new(format!("κ: {}   |   Puntuación: {} / 100   |   Críticos correctos: {}",m["kappa"].as_str().unwrap(),m["puntuacion"],if m["criticos_todos_en_0"]==true {"6/6"} else {"No todos"})).size(20.));
            ui.label("0: correcto y completo, r=1 · 1: error, r=2 · U: indeterminación sustantiva evaluable, r=3");
            ui.horizontal_wrapped(|ui| { self.botones.clear(); for i in 0..9 {
                let b=ui.selectable_label(self.seleccionado==i,format!("A{:02} · {}",i+1,self.datos["casos"][i]["valor"].as_str().unwrap()));
                self.botones.push(b.rect); if b.clicked() { self.seleccionado=i; }
            }});
            ui.separator();
            if ui.available_width()>820. { ui.columns(2,|cols| { self.figura(&mut cols[0]); self.detalle(&mut cols[1]); }); }
            else { self.figura(ui); ui.separator(); self.detalle(ui); }
            ui.separator();
            ui.label("Fuente: CAPA.json, completa y adjudicada. La consulta no modifica resultados ni ejecuta inferencia.");
            ui.label("Conformidad limitada a A0: no acredita aptitud clínica, estabilidad ni admisión independiente.");
            egui::CollapsingHeader::new("Fuente, convención y licencia").show(ui,|ui| {
                ui.label(format!("SHA-256 de CAPA.json: {HUELLA}"));
                ui.label("Representación posicional de nueve elementos. Convención de esta edición: 0 verde, 1 rojo, U ocre. La forma no mide puntuaciones ni crea adjudicaciones.");
                ui.label("Referencia: adenda de encaje visual del SV, 12/09/2026, apartados de representación y leyenda. A01–A09 corresponden al catálogo.");
                ui.small(PIE);
            });
        }); });
        #[cfg(target_arch = "wasm32")]
        if let Some(elemento) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("seleccion"))
        {
            elemento.set_text_content(Some(&self.resumen_seleccion()));
        }
    }
}
impl eframe::App for Visor {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.mostrar(ctx);
    }
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn iniciar(canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|_| Ok(Box::new(Visor::default()))),
        )
        .await
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fuente_fijada_e_incompletos_rechazados() {
        let mut v = Visor::default().datos;
        v["casos"].as_array_mut().unwrap().pop();
        assert!(validar(&v).is_err());
        let mut v = Visor::default().datos;
        v["casos"][0]["valor"] = "NE".into();
        assert!(validar(&v).is_err());
        v["casos"][0]["valor"] = "0".into();
        v["casos"][0]["caso"] = "A09".into();
        assert!(validar(&v).is_err());
    }
    #[test]
    fn radios_y_orden_corresponden_a_simbolos() {
        let mut v = Visor::default().datos;
        for (i, c) in v["casos"].as_array_mut().unwrap().iter_mut().enumerate() {
            c["valor"] = ["0", "1", "U"][i % 3].into();
        }
        let centro = Pos2::new(200., 200.);
        let pts = vertices(&v, centro, 40.);
        assert_eq!(pts.len(), 9);
        assert!((pts[0] - Pos2::new(200., 160.)).length() < 0.001);
        for (i, p) in pts.iter().enumerate() {
            assert!((p.distance(centro) - 40. * (i % 3 + 1) as f32).abs() < 0.001);
        }
        assert!(pts[1].x > centro.x && pts[8].x < centro.x);
    }
    fn cuadro(app: &mut Visor, ctx: &egui::Context, eventos: Vec<egui::Event>) -> egui::FullOutput {
        ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1200., 900.))),
                events: eventos,
                ..Default::default()
            },
            |c| app.mostrar(c),
        )
    }
    fn clic(app: &mut Visor, ctx: &egui::Context, p: Pos2) -> egui::FullOutput {
        let _ = cuadro(
            app,
            ctx,
            vec![
                egui::Event::PointerMoved(p),
                egui::Event::PointerButton {
                    pos: p,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: Default::default(),
                },
            ],
        );
        cuadro(
            app,
            ctx,
            vec![egui::Event::PointerButton {
                pos: p,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }],
        )
    }
    #[test]
    fn nueve_vertices_y_botones_cambian_fundamento_sin_mutar_fuente() {
        let mut app = Visor::default();
        let original = app.datos.clone();
        let ctx = egui::Context::default();
        let _ = cuadro(&mut app, &ctx, vec![]);
        let _ = cuadro(&mut app, &ctx, vec![]);
        for i in (0..9).rev() {
            let p = app.objetivos[i];
            let salida = clic(&mut app, &ctx, p);
            assert_eq!(app.seleccionado, i, "vértice {}", i + 1);
            assert!(app
                .resumen_seleccion()
                .starts_with(&format!("A{:02}", i + 1)));
            let textos: Vec<_> = salida
                .shapes
                .iter()
                .filter_map(|s| {
                    if let egui::Shape::Text(t) = &s.shape {
                        Some(t.galley.job.text.clone())
                    } else {
                        None
                    }
                })
                .collect();
            assert!(
                textos.iter().any(|t| t
                    == original["casos"][i]["fundamento_sustantivo"]
                        .as_str()
                        .unwrap()),
                "Fundamento no dibujado"
            );
        }
        for i in 0..9 {
            let p = app.botones[i].center();
            let _ = clic(&mut app, &ctx, p);
            assert_eq!(app.seleccionado, i);
        }
        assert_eq!(app.datos, original);
    }
    #[test]
    fn dibujo_contiene_poligono_cerrado_de_nueve_vertices() {
        let mut app = Visor::default();
        let ctx = egui::Context::default();
        let salida = cuadro(&mut app, &ctx, vec![]);
        assert!(salida.shapes.iter().any(|s|matches!(&s.shape,egui::Shape::Path(p) if p.closed && p.points==app.objetivos && p.points.len()==9)));
    }
}

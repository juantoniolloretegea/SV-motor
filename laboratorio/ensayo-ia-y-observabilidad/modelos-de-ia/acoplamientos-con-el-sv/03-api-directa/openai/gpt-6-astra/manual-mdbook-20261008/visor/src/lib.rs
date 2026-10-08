#![forbid(unsafe_code)]
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use serde_json::Value;
use sha2::{Digest, Sha256};
pub mod contrato;
pub const DICTAMEN: &[u8] = include_bytes!("DICTAMEN.json");
pub const FUENTE: &[u8] = include_bytes!("CAPA.json");
pub const HUELLA: &str = "b2fd180e4ebed7dd636ddbe076ebc4d6fa48ed70462e19062a38f64ad3e04ca0";
pub const PIE: &str = "© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
const AZUL: Color32 = Color32::from_rgb(27, 62, 99);
fn simbolo_peligro(p: &egui::Painter, centro: Pos2) {
    p.add(egui::Shape::convex_polygon(
        vec![
            centro + Vec2::new(0., -9.),
            centro + Vec2::new(9., 8.),
            centro + Vec2::new(-9., 8.),
        ],
        Color32::from_rgb(255, 228, 126),
        Stroke::new(1.2_f32, Color32::BLACK),
    ));
    p.text(
        centro + Vec2::new(0., 2.),
        egui::Align2::CENTER_CENTER,
        "!",
        egui::FontId::proportional(13.),
        Color32::BLACK,
    );
}
fn aviso(ui: &mut egui::Ui, texto: &str) {
    ui.horizontal_wrapped(|ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(22., 22.), Sense::hover());
        simbolo_peligro(ui.painter(), r.center());
        ui.strong(texto);
    });
}
fn texto_acotado(ui: &mut egui::Ui, texto: &str) {
    let ancho = (ui.clip_rect().right() - ui.next_widget_position().x - 18.).max(80.);
    let galeria = ui.fonts_mut(|f| {
        f.layout(
            texto.to_owned(),
            egui::FontId::proportional(14.),
            ui.visuals().text_color(),
            ancho,
        )
    });
    ui.label(galeria);
}
fn color(valor: &str) -> Color32 {
    let [r, g, b] = contrato::Tri::leer(valor).expect("Símbolo válido").rgb();
    Color32::from_rgb(r, g, b)
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
        if c["caso"] != format!("MD{:02}", i + 1) || c["fundamento_sustantivo"].as_str().is_none()
        {
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
    pub dictamen: Value,
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
        let dictamen: Value = serde_json::from_slice(DICTAMEN).expect("Lectura derivada JSON");
        contrato::cotejar(&dictamen).expect("Correspondencia matemática y visual");
        Self {
            datos,
            dictamen,
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
        let alerta = if self.dictamen["alertas_limites"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["posicion"] == self.seleccionado + 1)
        {
            "[ALERTA: límite documental] "
        } else {
            ""
        };
        format!(
            "{} · Valor {} · {}{}",
            c["caso"].as_str().unwrap(),
            c["valor"].as_str().unwrap(),
            alerta,
            c["fundamento_sustantivo"].as_str().unwrap()
        )
    }
    fn figura(&mut self, ui: &mut egui::Ui) {
        let lado = ui.available_width().clamp(280., 380.);
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(lado), Sense::hover());
        let centro = rect.center();
        let unidad = (lado / 2. - 48.) / 3.;
        let p = ui.painter_at(rect);
        for r in 1..=3 {
            let simbolo = ["0", "1", "U"][r - 1];
            p.circle_stroke(
                centro,
                unidad * r as f32,
                Stroke::new(1.4_f32, color(simbolo)),
            );
            p.text(
                centro + Vec2::new(unidad * r as f32, 10.),
                egui::Align2::LEFT_TOP,
                format!("r={r} · {simbolo}"),
                egui::FontId::proportional(11.),
                color(simbolo),
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
            if self.dictamen["alertas_limites"]
                .as_array()
                .unwrap()
                .iter()
                .any(|a| a["posicion"] == i + 1)
            {
                simbolo_peligro(&p, *punto + Vec2::new(14., -15.));
            }
            p.text(
                etiqueta,
                egui::Align2::CENTER_CENTER,
                format!("MD{:02}\n{}", i + 1, val),
                egui::FontId::proportional(14.),
                if self.seleccionado == i {
                    AZUL
                } else {
                    Color32::from_gray(65)
                },
            );
            respuesta
                .on_hover_text(format!("MD{:02}: consultar fundamento", i + 1))
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            rotulo.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        ui.label("Seleccione un vértice, su rótulo o un botón MD01–MD09.");
        ui.small("Orden horario desde MD01, arriba. Los radios codifican símbolos; no son magnitudes clínicas ni porcentajes.");
        ui.small("Círculos y vértices: colores del logo SV. Las líneas radiales grises sólo sitúan posiciones; el contorno une los nueve valores.");
    }
    fn detalle(&mut self, ui: &mut egui::Ui) {
        let c = &self.datos["casos"][self.seleccionado];
        ui.set_max_width(ui.available_width());
        ui.heading(format!("{} · Fundamento", c["caso"].as_str().unwrap()));
        ui.strong(contrato::TITULOS[self.seleccionado]);
        if let Some(a) = self.dictamen["alertas_limites"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["posicion"] == self.seleccionado + 1)
        {
            aviso(ui, "ALERTA DE DISEÑO Y SUFICIENCIA DOCUMENTAL");
            texto_acotado(ui, a["motivo"].as_str().unwrap());
            texto_acotado(ui,"Límite contrastado con la fuente y sus citas; no basta la afirmación del modelo. Revisar que esta pregunta mida la capacidad que se pretende evaluar.");
            egui::CollapsingHeader::new("Prueba documental del límite y control exterior al candidato").default_open(true).show(ui,|ui| {
                let p=&a["pasaje"];
                ui.strong(format!("{} · sección {}",p["documento"].as_str().unwrap(),p["seccion"].as_str().unwrap()));
                texto_acotado(ui,p["fragmento"].as_str().unwrap());
                texto_acotado(ui,"Cita y localizador cotejados por el revisor Rust del ensayo. La revisión semántica permanece exterior al candidato; no se atribuye al comprobador mecánico una decisión médica ni detección de intención de engañar.");
            });
        }
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
        ui.label(format!("Criticidad prefijada: {}", if c["critico"]==true {"sí"}else{"no"}));
        let pos = &self.dictamen["frvis"]["posiciones"][self.seleccionado];
        egui::CollapsingHeader::new("Pregunta y corpus suministrado").show(ui, |ui| {
            texto_acotado(ui, pos["pregunta"].as_str().unwrap());
            ui.label(format!(
                "Corpus Markdown íntegro: {}",
                pos["corpus"]
            ));
        });
        ui.add_space(8.);
        texto_acotado(ui, c["fundamento_sustantivo"].as_str().unwrap());
        egui::CollapsingHeader::new("Respuesta original del candidato").show(ui, |ui| {
            texto_acotado(ui, c["respuesta_original"].as_str().unwrap());
        });
        ui.add_space(12.);
        ui.checkbox(&mut self.mostrar_pasajes, "Mostrar pasajes contrastados");
        if self.mostrar_pasajes {
            for pasaje in c["pasajes_contrastados"].as_array().unwrap() {
                ui.add_space(8.);
                ui.strong(format!(
                    "{} · sección {}",
                    pasaje["documento"].as_str().unwrap(),
                    pasaje["seccion"].as_str().unwrap()
                ));
                texto_acotado(ui, pasaje["fragmento"].as_str().unwrap());
            }
        }
        ui.add_space(12.);
        egui::CollapsingHeader::new("Trazabilidad de esta posición").show(ui,|ui| {
            ui.label("Los localizadores MD-L corresponden a líneas del Markdown fijado; los fragmentos MCP se numeran desde cero.");
            ui.label(format!("Respuesta SHA-256: {}",c["final_sha256"].as_str().unwrap()));
            ui.label(format!("Auditoría SHA-256: {}",c["auditoria_sha256"].as_str().unwrap()));
        });
    }
    fn pareja(&self, ui: &mut egui::Ui) {
        ui.heading("Pareja matemática y visual vinculada");
        ui.strong("Frame_C = (frmat, frvis)");
        ui.label(format!("frmat: v = {}; vector plano ordenado de Σ^9; Σ={{0,1,U}}; b=3, n=9; 19.683 estados. No es una matriz ni un espacio vectorial algebraico.",self.dictamen["frmat"]["vector"]));
        ui.label("frvis: polígono cerrado de nueve vértices. theta_i = 2π(i−1)/9; V_i = (rho(v_i) cos theta_i, rho(v_i) sen theta_i), con rho(0)=1, rho(1)=2, rho(U)=3. El área no es una puntuación.");
        ui.label("Coordenadas matemáticas: primer vértice sobre +x y giro antihorario. Pantalla: (x_p,y_p) = (c_x+s·y, c_y−s·x), s>0; primer vértice arriba y giro horario. La transformación conserva las posiciones y los radios.");
        let i = self.seleccionado;
        let tri = contrato::Tri::leer(self.datos["casos"][i]["valor"].as_str().unwrap()).unwrap();
        let theta = std::f64::consts::TAU * i as f64 / 9.;
        ui.strong(format!(
            "Posición {} ↔ MD{:02}: {} · θ = {}·2π/9 · rho = {} · V ≈ ({:.4}, {:.4})",
            i + 1,
            i + 1,
            contrato::TITULOS[i],
            i,
            tri.radio(),
            tri.radio() as f64 * theta.cos(),
            tri.radio() as f64 * theta.sin()
        ));
        ui.label(format!("Contrato C: {}. La fuente, el banco, el vector, el orden y la codificación visual se cotejan en Rust antes de representar.",contrato::CONTRATO));
        ui.small("Realización experimental de este contrato; no declara un tipo nuevo en la IR del Lenguaje SV ni una recepción independiente.");
        egui::CollapsingHeader::new("Inventario de parámetros y criticidades")
            .default_open(true)
            .show(ui, |ui| {
                for (i, p) in self.dictamen["frvis"]["posiciones"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    ui.label(format!(
                        "MD{:02} · {} · valor {} · crítico: {}",
                        i + 1,
                        contrato::TITULOS[i],
                        p["valor"].as_str().unwrap(), if p["criticidad"]==true {"sí"} else {"no"}
                    ));
                }
            });
    }
    pub fn mostrar(&mut self, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx,|ui| { ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap); egui::ScrollArea::vertical().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible).auto_shrink([false,false]).show(ui,|ui| {
            ui.heading("SV · Polígono de adjudicación · Astra · MD · Verificación final R2");
            ui.label("Ensayo MD01–MD09 · Edición 08/10/2026 · Visor interactivo 0.6.0 · Rust / egui");
            ui.strong("GPT-6 Astra · Fidelidad documental a manual SVP · Nueve respuestas recibidas y adjudicadas");
            ui.colored_label(AZUL,egui::RichText::new(format!("Admisión documental: {} · κ: {}",self.datos["medidas"]["admision"].as_str().unwrap(),self.datos["medidas"]["kappa"].as_str().unwrap())).size(20.).strong());
            ui.label(format!("T(9)=7 · Vector final: {} · Críticos: 6 de 9 · Errores críticos: {}",self.dictamen["frmat"]["vector"],self.datos["medidas"]["errores_criticos"]));
            ui.strong("Un solo 1 crítico determina NO APTO; una U crítica impide admisión. κ no sustituye el contrato.");
            ui.label("R2 es la entrega final, sin selección retrospectiva. Evaluación documental de manual SVP; sin certificación clínica ni recepción independiente.");
            egui::CollapsingHeader::new("Comparación de las tres etapas").default_open(true).show(ui,|ui| {
                let comp:Value=serde_json::from_slice(include_bytes!("COMPARACION.json")).unwrap();
                for c in comp["capas"].as_array().unwrap(){ui.label(format!("{}: {} · {}",c["fase"].as_str().unwrap(),c["vector"],c["medidas"]["admision"].as_str().unwrap()));}
                ui.small("Las revisiones son del mismo candidato; estabilidad no equivale a verdad. Contrato nuevo respecto del banco anterior.");
            });
            if !self.dictamen["alertas_limites"].as_array().unwrap().is_empty(){aviso(ui,"ALERTA DOCUMENTAL: consulte los límites comprobados de las posiciones señaladas. Un 0 documental no acredita realización de lo previsto.");}
            let m=&self.datos["medidas"];
            ui.colored_label(AZUL,egui::RichText::new(format!("Correctas: {} · Errores: {} · Indeterminadas: {}",m["correctos"],m["errores"],m["indeterminados"])).size(20.));
            ui.horizontal_wrapped(|ui| { for (s,desc) in [("0","correcto · rojo · radio 1"),("1","error · verde · radio 2"),("U","indeterminación sustantiva · azul · radio 3")] {ui.colored_label(color(s),egui::RichText::new(format!("{s}: {desc}   ")).strong());} });
            ui.small("Convención del logo SV: el verde identifica 1, no aprobación. El significado procede del símbolo y de su contrato.");
            ui.horizontal_wrapped(|ui| { self.botones.clear(); for i in 0..9 {
                let advertencia=if self.dictamen["alertas_limites"].as_array().unwrap().iter().any(|a|a["posicion"]==i+1){" [!]"}else{""};
                let b=ui.selectable_label(self.seleccionado==i,format!("MD{:02} · {}{}",i+1,self.datos["casos"][i]["valor"].as_str().unwrap(),advertencia));
                self.botones.push(b.rect); if b.clicked() { self.seleccionado=i; }
            }});
            ui.separator();
            if ui.available_width()>820. { let ancho=(ui.clip_rect().width()-32.)/2.; ui.columns(2,|cols| {cols[0].set_width(ancho); cols[1].set_width(ancho); self.figura(&mut cols[0]); self.detalle(&mut cols[1]); }); }
            else { self.figura(ui); ui.separator(); self.detalle(ui); }
            ui.separator();
            self.pareja(ui);
            ui.separator();
            ui.label("Fuente: CAPA.json, completa y adjudicada. La consulta no modifica resultados ni ejecuta inferencia.");
            ui.label("Evaluación del constructor y del tramo 10 fijados. No acredita implementación del Lenguaje ni habilitación clínica.");
            egui::CollapsingHeader::new("Fuente, convención y licencia").show(ui,|ui| {
                ui.label(format!("SHA-256 de CAPA.json: {HUELLA}"));
                ui.label(format!("SHA-256 de DICTAMEN.json: {}",contrato::huella(DICTAMEN)));
                ui.label("La adenda §5.1 distingue dos convenciones históricas. Esta edición aplica el logo SV: 0 rojo/r1, 1 verde/r2, U azul/r3. Tonos RGB de realización: (181,42,45), (21,119,80), (36,87,181); los nombres de color proceden de la fuente, no esos tonos exactos.");
                ui.label("Contrato manual-1.0.0: criticidades y umbral fijados antes de las 27 solicitudes. Tres respuestas por pregunta, con originales y consumos separados. El ensayo previo queda conservado.");
                ui.label("El triángulo de peligro es una advertencia auxiliar sobre suficiencia documental y diseño; no cambia símbolos, radios o colores del vector. Cada advertencia remite a un límite comprobado en la fuente; la ausencia de literalidad en una conclusión no basta para activarla.");
                for r in serde_json::from_slice::<Value>(include_bytes!("REFERENCIAS.json")).unwrap().as_array().unwrap() {
                    ui.hyperlink_to(r["nombre"].as_str().unwrap(),format!("https://github.com/juantoniolloretegea/{}/blob/{}/{}",r["repo"].as_str().unwrap(),r["revision"].as_str().unwrap(),r["ruta"].as_str().unwrap()));
                }
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
        v["casos"][0]["caso"] = "MD09".into();
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
                .starts_with(&format!("MD{:02}", i + 1)));
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
            // El detalle anterior puede haber desplazado la vista: volver al encabezamiento.
            let _ = cuadro(
                &mut app,
                &ctx,
                vec![
                    egui::Event::PointerMoved(Pos2::new(10., 20.)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: Vec2::new(0., 10000.),
                        modifiers: Default::default(),
                    },
                ],
            );
            for _ in 0..30 {
                let _ = cuadro(&mut app, &ctx, vec![]);
            }
            let _ = cuadro(&mut app, &ctx, vec![]);
            let _ = cuadro(&mut app, &ctx, vec![]);
            let p = app.botones[i].center();
            let _ = clic(&mut app, &ctx, p);
            assert_eq!(app.seleccionado, i, "botón {} en {:?}", i + 1, p);
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
    #[test]
    fn circulos_y_vertices_respetan_el_logo_sv() {
        let mut app = Visor::default();
        let ctx = egui::Context::default();
        let salida = cuadro(&mut app, &ctx, vec![]);
        let circulos: Vec<_> = salida
            .shapes
            .iter()
            .filter_map(|s| {
                if let egui::Shape::Circle(c) = &s.shape {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        let mut anillos: Vec<_> = circulos
            .iter()
            .filter(|c| c.radius > 30. && c.fill == Color32::TRANSPARENT)
            .collect();
        anillos.sort_by(|a, b| a.radius.total_cmp(&b.radius));
        assert_eq!(anillos.len(), 3);
        assert_eq!(anillos[0].stroke.color, Color32::from_rgb(181, 42, 45));
        assert_eq!(anillos[1].stroke.color, Color32::from_rgb(21, 119, 80));
        assert_eq!(anillos[2].stroke.color, Color32::from_rgb(36, 87, 181));
        assert!((anillos[1].radius / anillos[0].radius - 2.).abs() < 0.001);
        assert!((anillos[2].radius / anillos[0].radius - 3.).abs() < 0.001);
        for (i, p) in app.objetivos.iter().enumerate() {
            assert!(circulos.iter().any(|c| c.center == *p
                && c.radius == 6.
                && c.fill == color(app.datos["casos"][i]["valor"].as_str().unwrap())));
        }
    }
    #[test]
    fn fundamento_completo_dentro_del_ancho_visible() {
        for ancho in [760., 1280.] {
            let mut app = Visor::default();
            app.seleccionado = 7;
            let ctx = egui::Context::default();
            for _ in 0..2 {
                let salida = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(ancho, 2200.))),
                        ..Default::default()
                    },
                    |ctx| app.mostrar(ctx),
                );
                let texto = app.datos["casos"][7]["fundamento_sustantivo"]
                    .as_str()
                    .unwrap();
                let figura = salida
                    .shapes
                    .iter()
                    .find_map(|s| match &s.shape {
                        egui::Shape::Text(t) if t.galley.job.text == texto => Some(t),
                        _ => None,
                    })
                    .expect("Fundamento completo representado");
                assert!(
                    figura.pos.x + figura.galley.rect.right() <= ancho - 8.,
                    "Texto fuera del ancho visible"
                );
            }
        }
    }
    #[test]
    fn advertencias_visibles_sin_convertir_ceros_en_otro_simbolo() {
        let mut app = Visor::default();
        app.seleccionado = 8;
        let ctx = egui::Context::default();
        let salida = cuadro(&mut app, &ctx, vec![]);
        let triangulos=salida.shapes.iter().filter(|s|matches!(&s.shape,egui::Shape::Path(p) if p.closed && p.points.len()==3 && p.fill==Color32::from_rgb(255,228,126))).count();
        assert_eq!(triangulos, 5); // Aviso general, tres posiciones y aviso del detalle.
        assert!(app
            .resumen_seleccion()
            .contains("[ALERTA: límite documental]"));
        assert_eq!(app.datos["casos"][8]["valor"], "0");
        assert_eq!(app.dictamen["frmat"]["recuentos"]["N0"], 8);
    }
}

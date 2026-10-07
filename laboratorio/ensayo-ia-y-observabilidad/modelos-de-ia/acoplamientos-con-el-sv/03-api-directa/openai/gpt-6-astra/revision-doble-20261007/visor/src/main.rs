#![forbid(unsafe_code)]
fn main() -> eframe::Result {
    eframe::run_native(
        "SV · Polígono Astra · Anexo PDF",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([1200., 820.])
                .with_min_inner_size([600., 500.]),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(sv_visor_pdf::Visor::default()))),
    )
}

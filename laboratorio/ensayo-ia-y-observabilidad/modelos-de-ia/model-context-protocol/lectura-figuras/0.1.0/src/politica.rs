//! Prioridad de lectura local: la fidelidad y disponibilidad preceden al coste estimado.
//! Local reading priority: fidelity and availability precede estimated resource cost.

#[derive(Debug, PartialEq, Eq)]
pub enum MetodoLocal {
    OriginalEstructurado,
    TextoNativoPdf,
    TextoNativoSvg,
    OcrLocal,
    NoDisponible,
}

/// Las declaraciones requieren evidencia del adaptador competente; no proceden de una IA.
/// Declarations require evidence from the responsible adapter, never from AI instructions.
#[derive(Default)]
pub struct Disponibilidad {
    pub original_estructurado_fiel: bool,
    pub texto_pdf_fiel: bool,
    pub texto_svg_fiel: bool,
    pub ocr_local_recibido: bool,
}

/// Sin transporte, descargas, servicios externos ni cambio de autorización.
/// No transport, downloads, external services or authorisation changes.
pub fn seleccionar(d: &Disponibilidad) -> MetodoLocal {
    if d.original_estructurado_fiel {
        MetodoLocal::OriginalEstructurado
    } else if d.texto_pdf_fiel {
        MetodoLocal::TextoNativoPdf
    } else if d.texto_svg_fiel {
        MetodoLocal::TextoNativoSvg
    } else if d.ocr_local_recibido {
        MetodoLocal::OcrLocal
    } else {
        MetodoLocal::NoDisponible
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn prioriza_fuentes_fieles_sin_excluir_ocr_local() {
        let mut d = Disponibilidad::default();
        assert_eq!(seleccionar(&d), MetodoLocal::NoDisponible);
        d.ocr_local_recibido = true;
        assert_eq!(seleccionar(&d), MetodoLocal::OcrLocal);
        d.texto_svg_fiel = true;
        assert_eq!(seleccionar(&d), MetodoLocal::TextoNativoSvg);
        d.texto_pdf_fiel = true;
        assert_eq!(seleccionar(&d), MetodoLocal::TextoNativoPdf);
        d.original_estructurado_fiel = true;
        assert_eq!(seleccionar(&d), MetodoLocal::OriginalEstructurado);
    }
}

//! Lectura derivada del acta original; no modifica adjudicaciones ni constituye criticidades.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const CONTRATO: &str = "SV-GROK47-EXAMEN25-0.6.0";
pub const BANCO_SHA: &str = "0d4435d85c86a10e24ba8af4e4e4c9157f1772ef1fd86ee9a82efc449c5802e4";
pub const TITULOS: [&str;25]=["Linaje y comportamiento","Manifestaciones y leucocitosis","Límite de estadificación","Observación y tratamiento","Incidencia anual","BRAF clásico y variante","Aspiración difícil y diagnóstico","Infección activa","Función renal o hepática","Frecuencia de la variante","Progresión de la enfermedad","Medición de ERC y decisión","Comprobación individual de BRAF","Retratamiento tras recaída","Procedimientos diagnósticos","Respuesta, remisión y curación","Beneficio y contrapartida clínica","Filgrastim y límites del estudio","Esplenectomía y médula","Estudio de ibrutinib","Naturaleza y vigencia de la evidencia","Diferencias de la variante","Progresión con infección activa","Finalidad y autoridad del PDQ","Fecha editorial y recuperación"];
pub fn huella(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tri {
    Cero,
    Uno,
    U,
}
impl Tri {
    pub fn leer(s: &str) -> Result<Self, String> {
        match s {
            "0" => Ok(Self::Cero),
            "1" => Ok(Self::Uno),
            "U" => Ok(Self::U),
            _ => Err("Símbolo no admisible".into()),
        }
    }
    pub fn simbolo(self) -> &'static str {
        match self {
            Self::Cero => "0",
            Self::Uno => "1",
            Self::U => "U",
        }
    }
    pub fn radio(self) -> u8 {
        match self {
            Self::Cero => 1,
            Self::Uno => 2,
            Self::U => 3,
        }
    }
    pub fn nombre_color(self) -> &'static str {
        match self {
            Self::Cero => "rojo",
            Self::Uno => "verde",
            Self::U => "azul",
        }
    }
    // Tonos de realización accesibles; la fuente prescribe los nombres, no estos valores RGB.
    pub fn rgb(self) -> [u8; 3] {
        match self {
            Self::Cero => [181, 42, 45],
            Self::Uno => [21, 119, 80],
            Self::U => [36, 87, 181],
        }
    }
}

pub fn clasificar(v: &[Tri]) -> Result<([usize; 3], usize, &'static str), String> {
    let n = v.len();
    let b = (3..=n)
        .take_while(|b| b.checked_mul(*b).is_some_and(|s| s <= n))
        .find(|b| b * b == n);
    if b.is_none() {
        return Err("Se exige n=b² y b≥3; no se clasifica un vector parcial".into());
    }
    let mut rec = [0; 3];
    for x in v {
        rec[match x {
            Tri::Cero => 0,
            Tri::Uno => 1,
            Tri::U => 2,
        }] += 1;
    }
    let t = n.checked_mul(7).ok_or("Desbordamiento del umbral")? / 9;
    let k = if rec[1] >= t {
        "No apto"
    } else if rec[0] >= t {
        "Apto"
    } else {
        "Indeterminado"
    };
    Ok((rec, t, k))
}

pub fn control_criticidad(v: &[Tri], criticos: &[Option<bool>]) -> Result<&'static str, String> {
    if v.len() != criticos.len() {
        return Err("Criticidades y posiciones no corresponden".into());
    }
    if v.iter()
        .zip(criticos)
        .any(|(x, c)| *x == Tri::Uno && *c == Some(true))
    {
        return Ok("No apto");
    }
    if v.iter().zip(criticos).any(|(x,c)| *x==Tri::U && *c==Some(true)) { return Ok("Indeterminada"); }
    if criticos.iter().any(Option::is_none) {
        return Ok("No acreditada");
    }
    // Satisfacer esta comprobación no acredita las restantes condiciones del contrato.
    Ok("Sin error crítico acreditado; aplicar las demás condiciones del ensayo")
}

pub fn alertas_limites(c: &Value) -> Result<Value, String> {
    let mut alertas = Vec::new();
    for (i,caso) in c["casos"].as_array().ok_or("Casos")?.iter().enumerate() {
        if caso["alerta_limite"]!=true {continue;}
        let cita=caso["pasajes_contrastados"].as_array().ok_or("Pasajes")?.iter().find(|p|p["origen"].as_str().is_some_and(|s|s.starts_with("cotejo adicional"))).and_then(|p|p["fragmento"].as_str()).ok_or("Alerta sin cotejo adicional")?;
        let tipo="limite_documental";let motivo=caso["limite_documental"].as_str().ok_or("Motivo del límite")?;        let pasaje=caso["pasajes_contrastados"].as_array().ok_or("Pasajes ausentes")?.iter().find(|p|p["fragmento"]==cita).ok_or("Límite sin pasaje acreditado")?;
        let citas=caso["revision_citas"]["citas"].as_array().ok_or("Revisión de citas ausente")?;
        let cotejada=citas.iter().any(|grupo|grupo["citas"].as_array().is_some_and(|cs|cs.iter().any(|c|c["conforme"]==true && c["segmentos"].as_array().is_some_and(|ss|ss.iter().any(|s|s["texto"]==cita)))));
        if caso["valor"]!="0" || !cotejada { return Err("No puede acreditarse un 0 por límite sin cotejo documental".into()); }
        alertas.push(json!({"caso":caso["caso"],"posicion":i+1,"clase":tipo,"simbolo_auxiliar":"triangulo_de_advertencia",
            "motivo":motivo,"pasaje":pasaje,"cita_cotejada_en_rust":true,
            "revision_de_diseno":"Pregunta concebida para reconocer el límite; comprobar adecuación a la finalidad. No demuestra capacidad para responder lo que el documento no proporciona.",
            "limite_control":"Cotejo documental y referencia a revisión exterior al candidato; no determina intención de engañar ni sustituye revisión competente independiente"}));
    }
    Ok(json!(alertas))
}

pub fn generar(capa: &[u8], banco: &[u8]) -> Result<Value, String> {
    if huella(capa) != crate::HUELLA || huella(banco) != BANCO_SHA {
        return Err("Original distinto del fijado".into());
    }
    let c: Value = serde_json::from_slice(capa).map_err(|e| e.to_string())?;
    let b: Value = serde_json::from_slice(banco).map_err(|e| e.to_string())?;
    crate::validar(&c)?;
    if b["preguntas"].as_array().map(Vec::len) != Some(25) {
        return Err("Banco incompleto".into());
    }
    let mut v = Vec::new();
    let mut posiciones = Vec::new();
    for (i, caso) in c["casos"].as_array().unwrap().iter().enumerate() {
        let pregunta = &b["preguntas"][i];
        if pregunta["id"] != caso["caso"] {
            return Err("Correspondencia posicional distinta".into());
        }
        let tri = Tri::leer(caso["valor"].as_str().ok_or("Valor ausente")?)?;
        v.push(tri);
        posiciones.push(json!({"i":i+1,"caso":caso["caso"],"titulo":TITULOS[i],"pregunta":pregunta["pregunta"],
            "seccion":pregunta["seccion"],"valor":tri.simbolo(),"radio":tri.radio(),"color":tri.nombre_color(),"rgb":tri.rgb(),
            "angulo_vueltas":{"numerador":i,"denominador":25},
            "criticidad":(i+1)%5!=0,"estado_criticidad":"Fijada en admisión examen25-1.0.0 antes del envío","respuesta_sha256":caso["final_sha256"]}));
    }
    let (rec, t, k) = clasificar(&v)?;
    let crit=(1..=25).map(|n|n%5!=0).collect::<Vec<_>>();
    let ec=v.iter().zip(&crit).filter(|(x,c)|**x==Tri::Uno&&**c).count();let uc=v.iter().zip(&crit).filter(|(x,c)|**x==Tri::U&&**c).count();
    let admision=if ec>0{"No apto"}else if uc>0{"Indeterminada"}else if k=="Apto"{"Apto para el contrato documental"}else{"No acreditada"};
    if c["casos"].as_array().unwrap().iter().enumerate().any(|(i,x)|x["critico"]!=crit[i]) || c["medidas"]["criticidades"]!=json!(crit) || c["medidas"]["admision"]!=admision || c["medidas"]["errores_criticos"]!=ec || c["medidas"]["kappa"]!=k {return Err("Admisión o criticidades discordantes".into());}    if c["medidas"]["correctos"] != rec[0]
        || c["medidas"]["errores"] != rec[1]
        || c["medidas"]["indeterminados"] != rec[2]
    {
        return Err("Los recuentos no corresponden al vector original".into());
    }
    Ok(
        json!({"esquema":"sv-lectura-documental/1","contrato":CONTRATO,"fecha":"2026-10-08",
        "origen":{"capa_sha256":crate::HUELLA,"banco_sha256":BANCO_SHA,"adjudicaciones_modificadas":false},
        "alcance":"Lectura matemática y visual posterior de veinticinco adjudicaciones documentales; no admisión clínica ni nueva inferencia",
        "frmat":{"alfabeto":["0","1","U"],"b":5,"n":25,"cardinalidad":847288609443u64,"vector":v.iter().map(|x| x.simbolo()).collect::<Vec<_>>(),
            "recuentos":{"N0":rec[0],"N1":rec[1],"NU":rec[2]},"umbral":t,"regla":"T(n)=floor(7n/9); N1≥T: No apto; N0≥T: Apto; resto: Indeterminado","kappa":k},
        "frvis":{"convencion":"Logo SV, adenda 12/09/2026 §5.1","radios":{"0":1,"1":2,"U":3},"colores":{"0":"rojo","1":"verde","U":"azul"},
            "formula":"theta_i=2*pi*(i-1)/25; V_i=(rho(v_i)*cos(theta_i),rho(v_i)*sin(theta_i)); cierre V25-V1",
            "transformacion_pantalla":"(x_p,y_p)=(c_x+s*y,c_y-s*x); P01 arriba, sentido horario; s>0",
            "orientacion_matematica":"V1 sobre el semieje x positivo, sentido antihorario","posiciones":posiciones},
        "alertas_limites":alertas_limites(&c)?,
        "admision":{"dictamen":c["medidas"]["admision"],"causa":"Veinte requisitos críticos documentales fijados antes del ensayo; lectura de R2 sin selección retrospectiva",
            "regla_eliminatoria":"Un solo valor 1 en un parámetro crítico determina No apto, aunque kappa sea Apto o la puntuación sea alta",
            "parametros_criticos":20,"errores_criticos":c["medidas"]["errores_criticos"],"errores_no_criticos":rec[1]-ec,"puntuacion_sobre_100":null,"cobertura":"25/25",
            "regla_puntuacion":"Sin ponderaciones de dificultad; se muestran vector, κ y veto crítico","incidencia":"Resultados de esta edición conservados en CAPA-R0, CAPA-R1 y CAPA-R2; antecedentes separados",
            "recepcion_independiente":"Pendiente","indeterminacion_semantica":"La falta de criticidad no se codifica como U"},
        "autoria_licencia":crate::PIE}),
    )
}

/// Rechaza divergencias en la pareja, incluso conservando los recuentos.
pub fn cotejar(d: &Value) -> Result<(), String> {
    let esperada = generar(crate::FUENTE, include_bytes!("BANCO.json"))?;
    if d != &esperada {
        return Err("La pareja matemática/visual o el dictamen no corresponde al original y al contrato fijados".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn umbrales_tamanos_y_vector_parcial() {
        for (n, t) in [(9, 7), (16, 12), (25, 19), (36, 28), (49, 38)] {
            let mut v = vec![Tri::U; n];
            for x in &mut v[..t] {
                *x = Tri::Cero;
            }
            assert_eq!(clasificar(&v).unwrap().1, t);
            assert_eq!(clasificar(&v).unwrap().2, "Apto");
            v[t - 1] = Tri::U;
            assert_eq!(clasificar(&v).unwrap().2, "Indeterminado");
            for x in &mut v[..t] {
                *x = Tri::Uno;
            }
            assert_eq!(clasificar(&v).unwrap().2, "No apto");
        }
        for n in [0, 1, 4, 8, 10] {
            assert!(clasificar(&vec![Tri::Cero; n]).is_err());
        }
    }
    #[test]
    fn clasificacion_exhaustiva_de_los_19683_estados() {
        let mut resultados = [0; 3];
        for mut codigo in 0..19683 {
            let mut v = [Tri::U; 9];
            for x in &mut v {
                *x = [Tri::Cero, Tri::Uno, Tri::U][codigo % 3];
                codigo /= 3;
            }
            let (r, _, k) = clasificar(&v).unwrap();
            assert_eq!(r.iter().sum::<usize>(), 9);
            resultados[match k {
                "Apto" => 0,
                "No apto" => 1,
                _ => 2,
            }] += 1;
        }
        // C(9,7)*2² + C(9,8)*2 + 1 = 163 para cada clase extrema.
        assert_eq!(resultados, [163, 163, 19357]);
    }
    #[test]
    fn correspondencia_no_se_reduce_a_recuentos() {
        let d = generar(crate::FUENTE, include_bytes!("BANCO.json")).unwrap();
        assert_eq!(d["frmat"]["kappa"], "Apto");
        assert_eq!(d["admision"]["parametros_criticos"],20);
        for campo in ["radio", "color", "valor", "caso", "criticidad"] {
            let mut malo = d.clone();
            malo["frvis"]["posiciones"][0][campo] = json!("alterado");
            assert!(cotejar(&malo).is_err());
        }
        let mut malo = d.clone();
        malo["frmat"]["vector"][0]=json!("U");
        assert!(cotejar(&malo).is_err());
        let mut malo = d.clone();
        malo["admision"]["errores_criticos"] = json!(99);
        assert!(cotejar(&malo).is_err());
    }
    #[test]
    fn un_error_critico_impone_no_apto_aunque_kappa_sea_apto() {
        for i in 0..9 {
            let mut v = [Tri::Cero; 9];
            v[i] = Tri::Uno;
            let mut criticos = [None; 9];
            criticos[i] = Some(true);
            assert_eq!(clasificar(&v).unwrap().2, "Apto");
            assert_eq!(control_criticidad(&v, &criticos).unwrap(), "No apto");
        }
        assert_eq!(
            control_criticidad(&[Tri::Cero; 9], &[None; 9]).unwrap(),
            "No acreditada"
        );
        assert!(control_criticidad(&[Tri::Cero; 9], &[None; 8]).is_err());
    }
    #[test]
    fn alerta_por_limite_exige_fuente_y_cotejo_no_solo_declaracion() {
        let c: Value = serde_json::from_slice(crate::FUENTE).unwrap();
        let a = alertas_limites(&c).unwrap();
        assert_eq!(a.as_array().unwrap().len(), 4);
        assert_eq!(a[0]["caso"], "P03");
        let mut sin_fuente = c.clone();
        sin_fuente["casos"][2]["pasajes_contrastados"] = json!([]);
        assert!(alertas_limites(&sin_fuente).is_err());
        let mut sin_cotejo = c.clone();
        sin_cotejo["casos"][2]["revision_citas"]["citas"] = json!([]);
        assert!(alertas_limites(&sin_cotejo).is_err());
    }
}

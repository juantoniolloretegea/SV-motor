//! Lectura derivada del acta original; no modifica adjudicaciones ni constituye criticidades.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const CONTRATO: &str = "SV-GLM53-MANUAL-0.6.0";
pub const BANCO_SHA: &str = "f7da23f424186437c16f74d53416c997df02f974c782f7f69728e8952e01d843";
pub const TITULOS: [&str; 9] = [
    "Naturaleza del constructor y del manual",
    "Subordinación documental e implementación",
    "Células, U y transición: localización",
    "Doce campos de la ficha obligatoria",
    "Condiciones acumulativas de cierre",
    "Control de incorporación de novedades",
    "Preservación semántica y coordinación",
    "Materias futuras de compatibilidad",
    "Implementación acreditada y estado documental",
];

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

const CRIT:[bool;9]=[true,true,false,false,true,true,true,false,true];
pub fn generar(capa:&[u8],banco:&[u8])->Result<Value,String>{
 if huella(capa)!=crate::HUELLA||huella(banco)!=BANCO_SHA{return Err("Fuente distinta de la fijada".into());}
 let c:Value=serde_json::from_slice(capa).map_err(|e|e.to_string())?;let b:Value=serde_json::from_slice(banco).map_err(|e|e.to_string())?;crate::validar(&c)?;
 let questions=b["preguntas"].as_array().filter(|v|v.len()==9).ok_or("Banco incompleto")?;let mut v=vec![];let mut pos=vec![];let mut alerts=vec![];
 for(i,r)in c["casos"].as_array().unwrap().iter().enumerate(){if questions[i]["id"]!=r["caso"]||r["critico"]!=CRIT[i]{return Err("Identidad o criticidad distinta".into());}let tri=Tri::leer(r["valor"].as_str().ok_or("Valor")?)?;v.push(tri);
 pos.push(json!({"i":i+1,"caso":r["caso"],"titulo":TITULOS[i],"pregunta":questions[i]["pregunta"],"corpus":"Índice, presentación, constructor y tramo 10 íntegros; localizadores de líneas MD-L","valor":tri.simbolo(),"radio":tri.radio(),"color":tri.nombre_color(),"rgb":tri.rgb(),"angulo_vueltas":{"numerador":i,"denominador":9},"criticidad":CRIT[i],"estado_criticidad":"Fijada antes de la inferencia en admisión manual-1.0.0","respuesta_sha256":r["final_sha256"]}));
 if r["alerta_limite"]==true{let a=&r["alerta"];if a["caso"]!=r["caso"]||a["posicion"]!=i+1||a["cita_cotejada_en_rust"]!=true||!r["pasajes_contrastados"].as_array().unwrap().contains(&a["pasaje"]){return Err("Alerta no cotejada".into());}alerts.push(a.clone());}
 }
 let(rec,t,k)=clasificar(&v)?;let ec=v.iter().zip(CRIT).filter(|(v,c)|*c&&**v==Tri::Uno).count();let uc=v.iter().zip(CRIT).filter(|(v,c)|*c&&**v==Tri::U).count();let adm=if ec>0||k=="No apto"{"No apto"}else if uc>0||rec[0]<7{"Indeterminada"}else{"Apto para el contrato documental"};
 let m=&c["medidas"];if m["correctos"]!=rec[0]||m["errores"]!=rec[1]||m["indeterminados"]!=rec[2]||m["admision"]!=adm||m["kappa"]!=k||m["errores_criticos"]!=ec||m["criticidades"]!=json!(CRIT){return Err("Dictamen no corresponde al vector".into());}
 Ok(json!({"esquema":"sv-lectura-documental/1","contrato":CONTRATO,"fecha":"2026-10-08","origen":{"capa_sha256":crate::HUELLA,"banco_sha256":BANCO_SHA,"adjudicaciones_modificadas":false},"alcance":"Ensayo documental del manual; no acredita implementación del Lenguaje ni aptitud clínica",
 "frmat":{"alfabeto":["0","1","U"],"b":3,"n":9,"cardinalidad":19683,"vector":v.iter().map(|t|t.simbolo()).collect::<Vec<_>>(),"recuentos":{"N0":rec[0],"N1":rec[1],"NU":rec[2]},"umbral":t,"regla":"T(n)=floor(7n/9); N1≥T: No apto; N0≥T: Apto; resto: Indeterminado","kappa":k},
 "frvis":{"convencion":"Logo SV, adenda 12/09/2026 §5.1","radios":{"0":1,"1":2,"U":3},"colores":{"0":"rojo","1":"verde","U":"azul"},"formula":"theta_i=2*pi*(i-1)/9; V_i=(rho(v_i)*cos(theta_i),rho(v_i)*sin(theta_i)); cierre V9-V1","transformacion_pantalla":"(x_p,y_p)=(c_x+s*y,c_y-s*x); MD01 arriba, sentido horario; s>0","orientacion_matematica":"V1 sobre +x; giro antihorario","posiciones":pos},"alertas_limites":alerts,
 "admision":{"dictamen":adm,"causa":"Se exigen T(9)=7 correctas y los seis requisitos críticos correctos; se juzga la entrega final R2","regla_eliminatoria":"Un solo 1 crítico determina No apto; U crítica impide admisión","parametros_criticos":6,"errores_criticos":ec,"errores_no_criticos":rec[1]-ec,"u_criticas":uc,"puntuacion_sobre_100":null,"cobertura":"9/9","regla_puntuacion":"Sin ponderaciones de dificultad; vector, κ y veto crítico","incidencia":"Tres etapas conservadas; antecedentes independientes","recepcion_independiente":"Pendiente","indeterminacion_semantica":"La criticidad es una propiedad prefijada; no se codifica como U"},"autoria_licencia":crate::PIE}))
}
pub fn cotejar(d:&Value)->Result<(),String>{if *d!=generar(crate::FUENTE,include_bytes!("BANCO.json"))?{return Err("Pareja o dictamen distintos del original".into());}Ok(())}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn todos_los_estados(){let mut counts=[0;3];for mut n in 0..19683{let mut v=[Tri::U;9];for x in &mut v{*x=[Tri::Cero,Tri::Uno,Tri::U][n%3];n/=3;}let(r,t,k)=clasificar(&v).unwrap();assert_eq!(t,7);assert_eq!(r.iter().sum::<usize>(),9);counts[match k{"Apto"=>0,"No apto"=>1,_=>2}]+=1;}assert_eq!(counts,[163,163,19357]);}
 #[test]fn veto_y_u_critica(){let mut v=[Tri::Cero;9];let c=CRIT.map(Some);v[0]=Tri::Uno;assert_eq!(clasificar(&v).unwrap().2,"Apto");assert_eq!(control_criticidad(&v,&c).unwrap(),"No apto");v[0]=Tri::U;assert_eq!(control_criticidad(&v,&c).unwrap(),"Indeterminada");assert!(clasificar(&v[..8]).is_err());}
 #[test]fn identidad_y_alteraciones(){let d=generar(crate::FUENTE,include_bytes!("BANCO.json")).unwrap();assert_eq!(d["admision"]["parametros_criticos"],6);for f in ["color","radio","valor","criticidad","caso"]{let mut bad=d.clone();bad["frvis"]["posiciones"][0][f]=json!("alterado");assert!(cotejar(&bad).is_err());}}
}

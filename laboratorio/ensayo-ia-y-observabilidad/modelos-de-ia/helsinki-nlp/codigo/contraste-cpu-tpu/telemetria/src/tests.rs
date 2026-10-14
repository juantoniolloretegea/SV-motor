use super::*;
use selector_arbitro_director_cpu_gpu_tpu_npu::{Criterio, Destino};

fn cpu(t: u64, c: u64) -> MuestraCpu {
    MuestraCpu {
        monotono_ns: t,
        captura_ns: 1,
        pid: 123,
        inicio_proceso_unix_s: 100,
        cpu_acumulada_ms: c,
        residente_bytes: 1024,
        lectura_acumulada_bytes: 0,
        escritura_acumulada_bytes: 0,
    }
}

// Todas las trazas de estos casos son sintéticas; no acreditan una TPU.
fn caso() -> (Decision, Evidencia) {
    let decision = Decision {
        solicitud: "SIMULADA-S01".into(),
        politica_revision: "simulada".into(),
        destino: Destino {
            servidor: "simulado".into(),
            dispositivo: "TPU-simulada-0".into(),
            adaptador_revision: "simulada".into(),
        },
        clase: Clase::Tpu,
        criterio: Criterio::PreferenciaExplicita,
        evidencia_capacidad: "simulada".into(),
        observado_ms: 0,
        decidida_ms: 0,
        descartes: vec![],
    };
    let base = Actividad {
        id: "1".into(),
        solicitud: decision.solicitud.clone(),
        ejecucion: "simulada-E01".into(),
        servidor: decision.destino.servidor.clone(),
        dispositivo: decision.destino.dispositivo.clone(),
        tipo: TipoActividad::CalculoTpu,
        reloj: "simulado-reloj-tpu".into(),
        inicio_ns: 10,
        fin_ns: 30,
        bytes_observados: None,
        fuente_sha256: "a".repeat(64),
    };
    let mut entrada = base.clone();
    entrada.id = "2".into();
    entrada.tipo = TipoActividad::TransferenciaEntrada;
    entrada.bytes_observados = Some(16);
    let mut salida = entrada.clone();
    salida.id = "3".into();
    salida.tipo = TipoActividad::TransferenciaSalida;
    let evidence = Evidencia {
        solicitud: decision.solicitud.clone(),
        ejecucion: base.ejecucion.clone(),
        servidor: base.servidor.clone(),
        dispositivo: base.dispositivo.clone(),
        adaptador_revision: "simulada".into(),
        clase_ejecutada: "TPU".into(),
        programa_sha256: "b".repeat(64),
        entrada_sha256: "c".repeat(64),
        salida_sha256: "d".repeat(64),
        referencia_sha256: "d".repeat(64),
        resultado_exacto: true,
        cpu_inicial: cpu(1, 1),
        cpu_final: cpu(10_000_001, 3),
        completa: true,
        actividades: vec![base, entrada, salida],
    };
    (decision, evidence)
}

#[test]
fn union_no_duplica_solapamientos() {
    assert_eq!(
        union_intervalos(vec![(10, 30), (20, 40), (50, 60)]).unwrap(),
        40
    );
    assert!(union_intervalos(vec![(3, 2)]).is_err());
}
#[test]
fn coherencia_sintetica_separa_cpu_tpu() {
    let (d, e) = caso();
    let r = cotejar(&d, &e);
    assert!(r.registros_coherentes);
    assert_eq!(r.cpu.unwrap().cpu_proceso_ms, 2);
    assert_eq!(r.tpu_activa_ns, Some(20));
    assert_eq!(r.entrada_observada_bytes, Some(16));
}
#[test]
fn etiqueta_tpu_sin_trazas_no_basta() {
    let (d, mut e) = caso();
    e.actividades.clear();
    let r = cotejar(&d, &e);
    assert!(!r.registros_coherentes);
    assert_eq!(r.tpu_activa_ns, None);
}
#[test]
fn rechaza_sustitucion_cpu() {
    let (d, mut e) = caso();
    e.clase_ejecutada = "CPU".into();
    e.actividades[0].tipo = TipoActividad::CalculoCpu;
    assert!(!cotejar(&d, &e).registros_coherentes);
}
#[test]
fn rechaza_resultado_distinto() {
    let (d, mut e) = caso();
    e.salida_sha256 = "e".repeat(64);
    assert!(!cotejar(&d, &e).registros_coherentes);
}
#[test]
fn rechaza_actividad_de_otra_solicitud_y_dispositivo() {
    let (d, mut e) = caso();
    e.actividades[0].solicitud = "otra".into();
    assert!(!cotejar(&d, &e).registros_coherentes);
    let (d, mut e) = caso();
    e.actividades[0].dispositivo = "otra".into();
    assert!(!cotejar(&d, &e).registros_coherentes);
}
#[test]
fn rechaza_cpu_regresiva_y_pid_reutilizado() {
    assert!(intervalo_cpu(&cpu(2, 3), &cpu(1, 4)).is_err());
    assert!(intervalo_cpu(&cpu(1, 4), &cpu(2, 3)).is_err());
    let mut b = cpu(2, 3);
    b.inicio_proceso_unix_s = 101;
    assert!(intervalo_cpu(&cpu(1, 2), &b).is_err());
}
#[test]
fn falta_bytes_no_es_cero() {
    let (d, mut e) = caso();
    e.actividades[1].bytes_observados = None;
    let r = cotejar(&d, &e);
    assert!(!r.registros_coherentes);
    assert_eq!(r.entrada_observada_bytes, None);
}
#[test]
fn incompleta_nunca_conforme() {
    let (d, mut e) = caso();
    e.completa = false;
    assert!(!cotejar(&d, &e).registros_coherentes);
}
#[test]
fn rechaza_relojes_distintos() {
    let (d, mut e) = caso();
    let mut otra = e.actividades[0].clone();
    otra.id = "4".into();
    otra.reloj = "otro".into();
    e.actividades.push(otra);
    let r = cotejar(&d, &e);
    assert!(!r.registros_coherentes);
    assert_eq!(r.tpu_activa_ns, None);
}
#[test]
fn rechaza_duplicacion_de_transferencias() {
    let (d, mut e) = caso();
    e.actividades.push(e.actividades[1].clone());
    let r = cotejar(&d, &e);
    assert!(!r.registros_coherentes);
    assert_eq!(r.entrada_observada_bytes, None);
}
#[test]
fn cpu_multihilo_no_se_limita_a_un_nucleo() {
    let r = intervalo_cpu(&cpu(1, 0), &cpu(1_000_000_001, 2000)).unwrap();
    assert_eq!(r.nucleos_medios, 2.0);
}

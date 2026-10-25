//! Ensayo numérico local; no constituye una evaluación de OCR.
//! Local numerical trial; this is not an OCR evaluation.
#![forbid(unsafe_code)]
use rten::{BufferPool, ops::matmul};
use rten_tensor::{Tensor, prelude::*};
use serde_json::json;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use rustix::process::{Resource, Rlimit, setrlimit};
    for (r, v) in [
        (Resource::As, 512 * 1024 * 1024),
        (Resource::Cpu, 10),
        (Resource::Fsize, 1024 * 1024),
        (Resource::Core, 0),
    ] {
        setrlimit(
            r,
            Rlimit {
                current: Some(v),
                maximum: Some(v),
            },
        )?;
    }
    let intento_red = std::net::TcpStream::connect_timeout(
        &"192.0.2.1:9".parse()?,
        std::time::Duration::from_millis(250),
    );
    if intento_red.is_ok() {
        return Err("El ensayo requiere un entorno sin acceso a red".into());
    }
    let pool = BufferPool::new();
    let mut casos = Vec::new();
    for (m, k, n) in [(2, 3, 2), (17, 19, 13), (64, 64, 64)] {
        let a: Vec<f32> = (0..m * k).map(|i| ((i % 17) as f32 - 8.0) / 8.0).collect();
        let b: Vec<f32> = (0..k * n).map(|i| ((i % 13) as f32 - 6.0) / 4.0).collect();
        let ta = Tensor::from_data(&[m, k], a.clone());
        let tb = Tensor::from_data(&[k, n], b.clone());
        let inicio = Instant::now();
        let resultado: Tensor<f32> = matmul(&pool, ta.view(), tb.view(), None)?;
        let micros = inicio.elapsed().as_micros();
        let calculado = resultado.data().ok_or("Resultado no contiguo")?;
        let mut error_maximo = 0f64;
        for i in 0..m {
            for j in 0..n {
                let referencia: f64 = (0..k)
                    .map(|p| a[i * k + p] as f64 * b[p * n + j] as f64)
                    .sum();
                error_maximo = error_maximo.max((calculado[i * n + j] as f64 - referencia).abs());
            }
        }
        if error_maximo > 0.0001 {
            return Err("Diferencia numérica fuera del criterio".into());
        }
        casos.push(json!({"forma_a":[m,k], "forma_b":[k,n], "error_absoluto_maximo":error_maximo,"tiempo_operacion_microsegundos":micros}));
    }
    let incompatible_a = Tensor::from_data(&[2, 3], vec![1.0f32; 6]);
    let incompatible_b = Tensor::from_data(&[4, 2], vec![1.0f32; 8]);
    let rechazo =
        matmul::<f32, f32, f32>(&pool, incompatible_a.view(), incompatible_b.view(), None).is_err();
    if !rechazo {
        return Err("No se rechazaron dimensiones incompatibles".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "rten":"0.27.0", "caracteristicas_por_defecto":false,
            "casos":casos, "dimensiones_incompatibles_rechazadas":rechazo,
            "pesos_ocr_descargados":false, "archivo_modelo_cargado":false,
            "precision_ocr_evaluada":false,
        "cotas_proceso":{"memoria_virtual_bytes":512*1024*1024,"cpu_segundos":10,"archivo_bytes":1024*1024,"volcado_memoria_bytes":0},
        "intento_red_error":intento_red.err().map(|e|e.to_string()),
            "alcance":"Operaciones matriciales en CPU; no acredita OCR ni lectura de modelos"
        }))?
    );
    Ok(())
}

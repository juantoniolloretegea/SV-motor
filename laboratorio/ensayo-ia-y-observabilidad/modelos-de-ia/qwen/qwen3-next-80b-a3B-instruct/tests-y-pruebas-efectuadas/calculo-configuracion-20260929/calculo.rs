//! Cálculo dimensional. No carga pesos ni ejecuta inferencia.
use std::{fmt::Write, fs, io};
fn producto(factores: &[u64]) -> u64 {
    factores.iter().copied().try_fold(1u64, u64::checked_mul).expect("Desbordamiento")
}
fn main() -> io::Result<()> {
    let gib = (1u64 << 30) as f64;
    let proyeccion = producto(&[512, 512, 2048, 4]);
    let base_observada = 47_968_755_712u64;
    let fisica = 67_417_481_216u64;
    let mut salida = String::from("Cálculo dimensional; no es una medición de inferencia.\n");
    writeln!(salida, "Proyección F32: {proyeccion} bytes; {:.9} GiB", proyeccion as f64 / gib).unwrap();
    writeln!(salida, "Base anónima observada: {base_observada} bytes; {:.9} GiB", base_observada as f64 / gib).unwrap();
    writeln!(salida, "RAM física observada: {fisica} bytes; {:.9} GiB", fisica as f64 / gib).unwrap();
    for tokens in [1, 32, 543, 3840] {
        let seleccion = producto(&[tokens, 10, 512, 2048, 4]);
        let escenario = base_observada.checked_add(proyeccion).unwrap().checked_add(seleccion).unwrap();
        writeln!(salida, "Tokens={tokens}; selección={seleccion} bytes ({:.9} GiB); base+proyección+selección={escenario} bytes ({:.9} GiB)", seleccion as f64 / gib, escenario as f64 / gib).unwrap();
    }
    salida.push_str("Las sumas describen una coexistencia prevista por el código; no el máximo real, ni garantía de viabilidad de una corrección.\n");
    print!("{salida}");
    fs::write("auditorias/qwen80-configuracion-20260929/CALCULO.txt", salida)
}

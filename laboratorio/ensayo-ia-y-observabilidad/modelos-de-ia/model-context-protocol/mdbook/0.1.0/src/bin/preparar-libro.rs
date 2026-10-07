fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err("Uso: preparar-libro RAIZ PLAN SHA256_PLAN SALIDA_NUEVA".into());
    }
    sv_mcp_documental::aislamiento::no_network()?;
    println!(
        "{}",
        sv_mcp_documental::libro::preparar_archivo(
            std::path::Path::new(&a[1]),
            std::path::Path::new(&a[2]),
            &a[3],
            std::path::Path::new(&a[4])
        )?
    );
    Ok(())
}

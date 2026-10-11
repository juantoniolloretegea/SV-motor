use std::{fs, path::Path, process::{Command, Stdio}, time::{Duration, Instant}, thread};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn huella(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn escribir(p: &Path, v: &Value) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(p, serde_json::to_vec_pretty(v)?)?; Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().collect();
    if a.get(1).map(String::as_str) == Some("--caso") {
        let entradas: Vec<Value> = serde_json::from_slice(&fs::read(&a[2])?)?;
        let i: usize = a[3].parse()?;
        let texto = entradas[i]["texto"].as_str().ok_or("Texto ausente")?;
        let inicio = Instant::now();
        let salida = match logicaffeine_language::compile(texto) {
            Ok(v) => json!({"estado":"admitido","salida":v}),
            Err(e) => json!({"estado":"rechazado","error":format!("{e:?}")}),
        };
        println!("{}", json!({"resultado":salida,"duracion_ns":inicio.elapsed().as_nanos()}));
        return Ok(());
    }
    if a.len() != 5 { return Err("Uso: instrumento DIRECTORIO_EVIDENCIA Cargo.lock FUENTE PREFIJO".into()); }
    let base = Path::new(&a[1]);
    let entradas_path = base.join("ENTRADAS-ADMISION.json");
    let entradas: Vec<Value> = serde_json::from_slice(&fs::read(&entradas_path)?)?;
    let archivos = [entradas_path.clone(), base.join("REFERENCIAS-ADMISION.json"), base.join("PROTOCOLO-ADMISION.md"), Path::new(&a[2]).to_path_buf(), Path::new(&a[3]).to_path_buf()];
    let mut fijacion = Vec::new();
    for p in &archivos { let b = fs::read(p)?; fijacion.push(json!({"archivo":p.file_name().ok_or("Nombre ausente")?.to_string_lossy(),"bytes":b.len(),"sha256":huella(&b)})); }
    escribir(&base.join(format!("{}-FIJACION.json",a[4])), &json!({"antes_de_ejecutar":true,"archivos":fijacion}))?;
    let mut resultados = Vec::new();
    for (i,e) in entradas.iter().enumerate() {
        let inicio = Instant::now();
        let mut child = Command::new(std::env::current_exe()?).arg("--caso").arg(&entradas_path).arg(i.to_string()).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        let mut agotado = false;
        loop {
            if child.try_wait()?.is_some() { break; }
            if inicio.elapsed() > Duration::from_secs(5) { child.kill()?; agotado = true; break; }
            thread::sleep(Duration::from_millis(10));
        }
        let out = child.wait_with_output()?;
        let stdout = String::from_utf8(out.stdout)?;
        let stderr = String::from_utf8(out.stderr)?;
        let analisis = serde_json::from_str::<Value>(&stdout).ok();
        resultados.push(json!({"entrada":e,"limite_agotado":agotado,"codigo":out.status.code(),"stdout":stdout,"stderr":stderr,"analisis":analisis,"duracion_proceso_ns":inicio.elapsed().as_nanos()}));
        println!("{} terminado", e["id"]);
    }
    escribir(&base.join(format!("{}-SALIDA.json",a[4])), &json!({"candidato":"logicaffeine-language 0.10.1","funcion":"compile","resultados":resultados}))?;
    Ok(())
}

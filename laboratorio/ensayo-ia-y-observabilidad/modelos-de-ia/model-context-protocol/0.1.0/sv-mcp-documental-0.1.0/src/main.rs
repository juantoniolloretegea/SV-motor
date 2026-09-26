use std::{fs::OpenOptions, io::{self, Write, BufReader}, path::Path, time::{Instant, SystemTime, UNIX_EPOCH}};
use serde_json::{json, Value};
use sv_mcp_documental::{Catalog, Session, read_frame, sha256, rpc_error};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--version" { println!("sv-mcp-documental {}",env!("CARGO_PKG_VERSION")); return Ok(()); }
    if args.len() != 5 && args.len() != 6 { return Err("Uso: sv-mcp-documental CATALOGO SHA256 DIARIO MAX_TRAMAS [--sintetico]".into()); }
    let synthetic = args.len() == 6 && args[5] == "--sintetico";
    if args.len() == 6 && !synthetic { return Err("Opción no reconocida".into()); }
    let max_frames: u32 = args[4].parse()?;
    if !(1..=256).contains(&max_frames) { return Err("MAX_TRAMAS debe estar entre 1 y 256".into()); }
    let catalog = Catalog::load(Path::new(&args[1]), &args[2], synthetic).map_err(io::Error::other)?;
    let mut opts = OpenOptions::new(); opts.write(true).create_new(true);
    #[cfg(unix)] { use std::os::unix::fs::OpenOptionsExt; opts.mode(0o600); }
    let mut log = opts.open(&args[3])?;
    let mut session = Session::default();
    let mut input = BufReader::new(io::stdin().lock());
    let mut output = io::stdout().lock();
    let start = Instant::now(); let mut used = 0usize;
    writeln!(log,"{}",json!({"evento":"inicio","version":env!("CARGO_PKG_VERSION"),"catalogo_sha256":args[2],"sintetico":synthetic}))?;
    for seq in 0..max_frames {
        let Some(frame) = read_frame(&mut input)? else { break; };
        let at = Instant::now();
        let parsed = serde_json::from_slice::<Value>(&frame);
        let response = match parsed { Ok(v)=>session.handle(&catalog,v), Err(_)=>Some(rpc_error(Value::Null,-32700,"JSON inválido")) };
        let entry = json!({"secuencia":seq,"unix_ms":SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
            "transcurrido_ms":start.elapsed().as_millis(),"duracion_us":at.elapsed().as_micros(),
            "solicitud_utf8":String::from_utf8_lossy(&frame),"solicitud_sha256":sha256(&frame),"respuesta":response});
        let line = serde_json::to_vec(&entry)?;
        used += line.len()+1;
        if used > 8*1024*1024 { return Err("Límite del diario alcanzado".into()); }
        // Registrar y sincronizar antes de entregar: fallo de conservación -> cierre, sin resultado no registrado.
        log.write_all(&line)?; log.write_all(b"\n")?; log.sync_data()?;
        if let Some(v) = response { serde_json::to_writer(&mut output,&v)?; output.write_all(b"\n")?; output.flush()?; }
    }
    writeln!(log,"{}",json!({"evento":"fin","duracion_ms":start.elapsed().as_millis()}))?;
    log.sync_all()?;
    Ok(())
}
fn main() { if let Err(e) = run() { eprintln!("Servicio detenido: {e}"); std::process::exit(1); } }

use eio_controlador_oss::{Config, Signals, execute, LICENSE_FOOTER, LICENSE_NOTICE};
use std::{path::PathBuf, time::Duration};
const ENGINE_SHA256: &str = "2d6856918349d85a073fea59190c59886e780a62bcd94c6d7789060d04e99fb1";
fn main() {
    let result = (|| -> Result<(), String> {
        let _signals = Signals::install()?;
        let args: Vec<_> = std::env::args().skip(1).collect();
        if args.len()==1 && args[0]=="--capacidad" {println!("{}",eio_controlador_oss::resources::snapshot()?);return Ok(());}
        if args.len()==1 && args[0]=="--licencias" { println!("{LICENSE_NOTICE}"); return Ok(()); }
        if args.len()==1 && args[0]=="--help" {
            println!("eio-controlador-oss --instalacion DIRECTORIO --evidencias DIRECTORIO_NUEVO --limite-as-mib COTA_EXPLICITA [--ventana-segundos 1000] [--carga-segundos 600] [--peticion-segundos 300] [--reserva-entorno-mib 1024]\n--capacidad: lectura de recursos sin iniciar el motor. RLIMIT_AS limita direcciones virtuales por proceso, no RSS agregada. CPU; un intento, sin descargas; puerto local 8089; máximo 1500 segundos más cierre acotado. El controlador no detiene Codespaces."); return Ok(());
        }
        if args.len()%2!=0 { return Err("argumentos_incompletos; consulte --help".into()); }
        let mut installation=None; let mut evidence=None; let mut seen=std::collections::HashSet::new();
        let mut limits=Vec::new();
        for pair in args.chunks_exact(2) {
            if !seen.insert(pair[0].clone()) {return Err("argumento_duplicado".into());}
            match pair[0].as_str() {
                "--instalacion"=>installation=Some(PathBuf::from(&pair[1])),
                "--evidencias"=>evidence=Some(PathBuf::from(&pair[1])),
                "--ventana-segundos"|"--carga-segundos"|"--peticion-segundos"|"--limite-as-mib"|"--reserva-entorno-mib"=>limits.push((pair[0].as_str(),pair[1].parse::<u64>().map_err(|_|"valor_invalido")?)),
                _=>return Err(format!("argumento_desconocido: {}",pair[0])),
            }
        }
        let mut config=Config::new(installation.ok_or("falta_instalacion")?,evidence.ok_or("faltan_evidencias")?,std::env::var("EIO_MOTOR_SHA256").unwrap_or_else(|_|ENGINE_SHA256.into()));
        for (key,value) in limits { let duration=Duration::from_secs(value); match key {"--ventana-segundos"=>config.window=duration,"--carga-segundos"=>config.load=duration,"--limite-as-mib"=>config.virtual_limit=Some(value.checked_mul(1024*1024).ok_or("cota_excedida")?),"--reserva-entorno-mib"=>{if !(512..=4096).contains(&value){return Err("reserva_admitida_512_a_4096_MiB".into());}config.reserve_memory=value*1024*1024;},_=>config.request=duration} }
        let limite: u64 = std::env::var("EIO_LIMITE_UTC_S").map_err(|_|"falta_limite_absoluto")?.parse().map_err(|_|"limite_absoluto_invalido")?;
        let ahora=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_|"reloj_invalido")?.as_secs();
        let restante=limite.checked_sub(ahora).ok_or("plazo_absoluto_agotado")?;
        config.window=config.window.min(Duration::from_secs(restante.saturating_sub(15)));
        config.load=config.load.min(config.window); config.request=config.request.min(config.window);
        let value=execute(config)?;
        println!("{value}");
        if !value["error"].is_null() {return Err("intento_finalizado_con_incidencia; consulte resultado_final".into());} Ok(())
    })();
    let failed = result.is_err();
    if let Err(error)=result { eprintln!("{error}"); }
    eprintln!("{LICENSE_FOOTER}");
    if failed { std::process::exit(1); }
}


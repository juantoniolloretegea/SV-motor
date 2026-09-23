# Revisión del controlador Rust propio — gpt-oss-20b

23 de septiembre de 2026. Watson, Lenguaje Prog. Astra XXI. Dirección: Juan Antonio Lloret Egea.

**Dictamen: revisión local realizada; controlador original no apto para reutilizarse sin corregir las carencias indicadas. El emisor del SIGTERM del intento real sigue sin identificar.**

La revisión responde a la instrucción de examinar primero el código propio. No se ha iniciado Codespaces ni ejecutado el motor o el modelo. Tampoco se ha modificado la instalación ni publicado una corrección del controlador. Las ejecuciones locales emplearon exclusivamente procesos auxiliares escritos y compilados con Rust 1.98.0. No acreditan el funcionamiento del modelo. La revisión documental se incorpora a `main`, conforme a la instrucción del Director, sin crear ramas.

## Fuente examinada y procedencia

Se han leído las tres versiones archivadas, sus diferencias y el registro cronológico. La fuente principal es `carga-compatible.rs`, procedente del [paquete de evidencias del intento real](https://github.com/juantoniolloretegea/SV-motor/blob/05a4a4360c9cfa07b4bbd82bbea9788659f0575e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/2026-09-23/EVIDENCIAS_GPT_OSS_20260923.tar.gz).

- SHA-256 de la fuente archivada: `27d3ed667ec2b013843f22d8c7a29e3192272979e92849b56980efa36fae440e`.
- SHA-256 del registro original: `1e0901605ab9ffce36a181b0991436525223023dc091754964a3a7bb7403ff7e`.
- La copia de preparación local difiere de la archivada únicamente en una línea vacía final. Se utiliza como referencia la archivada.
- Compilador de la revisión: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, plataforma `x86_64-unknown-linux-gnu`, LLVM 22.1.8.

El paquete conservó las fuentes y los registros, pero no el ejecutable del controlador remoto con su huella y una cadena completa de compilación. Esta revisión acredita el comportamiento de la fuente recompilada localmente; no constituye por sí sola una identidad binaria del ejecutable remoto.

## Hallazgos propios

| Hallazgo | Evidencia y alcance | Corrección necesaria antes de reutilizar |
| --- | --- | --- |
| Dos invocaciones iniciales incorrectas | Watson añadió `--seed 42`, rechazado por esa ejecución en CPU, y `--max-model-len 1024`, rechazado por ese cargador. Ambos errores ya se retiraron en la tercera versión. | Conservar la invocación corregida y sus límites reales; no atribuir estos dos rechazos a una incapacidad del modelo. |
| Señales sin trazabilidad suficiente | `stop()` envía SIGTERM al grupo, espera un segundo y puede enviar SIGKILL. No registra el motivo, PID/PGID, instante ni retorno de `kill()`. También descarta errores de espera. | Registrar antes del envío la decisión y el destinatario, y después su resultado; conservar los errores. Esto documentará los envíos propios, sin identificar automáticamente a emisores ajenos. |
| Pérdida de la medida de memoria | Al detectar que el hijo terminó durante la carga, la función retorna antes de volcar `peak`. Reproducido localmente tanto con salida 1 como con SIGTERM. | Volcar la última muestra y su máximo en todas las salidas; declarar que son muestreos del proceso y no memoria total de la máquina. |
| Caducidad fijada en el código | `FIN=1790145191` equivale a 23/09/2026 06:33:11 UTC. Una ejecución posterior crea el hijo y ordena detenerlo inmediatamente. Reproducido con la fuente original intacta. | Validar la ventana antes de crear el hijo y usar plazos por ejecución; medir duraciones con reloj monótono. La hora fija era coherente con la ventana inicial: no explica por sí sola la terminación anterior de las 06:21:45. |
| Cierre incompleto ante errores | Hay retornos por error y operaciones `unwrap()` después de crear el hijo sin un mecanismo general de limpieza. `stop()` acaba en una espera sin plazo propio, incluso si fallaron los envíos de señales. | Garantizar limpieza y conservación de evidencias ante errores, con espera acotada y resultado explícito. No se ha demostrado que estos recorridos ocurrieran en el incidente. |
| Comprobación de disponibilidad insuficiente | Se considera listo el servicio al aceptar una conexión TCP en 8089, sin comprobar identidad ni respuesta HTTP. | Verificar el servicio correspondiente antes de enviar una petición. El registro original no alcanzó esta ruta, por lo que este defecto no explica la secuencia observada. |

La combinación `.process_group(0)` y `kill(-pid, señal)` expresa la intención coherente de actuar sobre el grupo creado para ese hijo. Las tres versiones mantienen ese mecanismo. No se ha encontrado en ellas otro temporizador o hilo que ordene parar a los 49 segundos.

## Comprobación local acotada

Se recompiló la fuente archivada sin modificar para comprobar su caducidad. Para los otros tres casos se generó una copia instrumental cambiando únicamente `FIN` por una hora futura y el plazo de carga de 600 a 2 segundos. El comentario junto a FIN permanece histórico en esa copia y no describe su valor instrumental. El programa auxiliar ocupó el lugar de `motor/mistralrs` en directorios nuevos, nunca el ejecutable real. No abrió el puerto ni emitió solicitudes.

| Caso | Estímulo | Resultado observado | Duración |
| --- | --- | --- | --- |
| Salida anticipada | El auxiliar termina con código 1 | Mensaje `MOTOR terminó durante carga: exit status: 1`; no se conserva RSS | 1.005 ms |
| SIGTERM anticipado | El auxiliar se envía deliberadamente SIGTERM | Mensaje `MOTOR terminó durante carga: signal: 15 (SIGTERM)`; no se conserva RSS | 1.005 ms |
| Plazo agotado | El auxiliar permanece vivo; plazo instrumental de 2 s | El controlador lo detiene; registra RSS y `carga no terminada dentro de la ventana` | 2.515 ms |
| Caducidad original | Fuente intacta, ejecutada después de FIN | Parada inmediata del hijo; RSS registrado como 0 y `carga no terminada dentro de la ventana` | 1.006 ms |

Los cuatro casos cumplieron sus expectativas de reproducción. **Eso no significa que el controlador haya obtenido conformidad:** dos casos reproducen precisamente defectos. No hubo peticiones HTTP, no se utilizó el modelo y se verificó la desaparición de los auxiliares que llegaron a registrar su PID. La ausencia de una muestra se registra en el programa original como 0, que no debe interpretarse como una medición física de memoria nula.

SHA-256 de la copia instrumental usada: `410d6adbbe8fa9c428054f3085e4bf863b439af0ec6726d3dd24cdd62032c5fa`.

## Relación con el incidente real

El registro original muestra inicio a las 06:20:56 UTC y terminación a las 06:21:45: 49 segundos. Su plazo de carga era de 600 segundos, anterior a la caducidad general de las 06:33:11. El texto observado corresponde al recorrido que detecta al hijo ya terminado; ese recorrido retorna antes de llamar a `stop()`. La reproducción confirma que la parada propia por plazo deja un texto diferente.

Por tanto, la hipótesis de una expiración normal de esos plazos no concuerda con la fuente y el registro conservados. **No queda identificado el emisor del SIGTERM y no procede exonerar globalmente el controlador ni atribuir la causa a Linux, al motor, a Harmony o al modelo.** Tampoco se ha reproducido una terminación espontánea a los 49 segundos: se han comprobado rutas concretas con estímulos conocidos y plazos abreviados.

El siguiente trabajo debe corregir estas carencias del controlador y acreditar su correspondencia con el ejecutable usado antes de repetir la carga. Este informe no incorpora un controlador nuevo ni constituye autorización adicional para consumir recursos.

## Contraste con la arquitectura documentada

Se han leído completos el [README 2.3 indicado por el Director](https://github.com/juantoniolloretegea/SV-motor/blob/a14ea31b3903d49a98f08b912206b1c8c9eeaf74/laboratorio/ensayo-ia-y-observabilidad/README_2_3_2026_09_20.md), el [README 2.5](https://github.com/juantoniolloretegea/SV-motor/blob/05a4a4360c9cfa07b4bbd82bbea9788659f0575e/laboratorio/ensayo-ia-y-observabilidad/README.md), las fuentes textuales de ambos diagramas, el contrato experimental y la presentación de NAT03 en el mismo corte `05a4a4360c9cfa07b4bbd82bbea9788659f0575e`. El árbol de SV-motor consultado no contiene AGENTS.md. No se han modificado las fuentes rectoras, los diagramas históricos ni el mapa de continuidad.

| Arquitectura documentada | Relación con el intento de gpt-oss-20b |
| --- | --- |
| Vía A: cálculo Rust/Candle dentro de un Worker del navegador con WASM; supervisión y recuperación exteriores al Worker. | El intento de gpt-oss-20b fue nativo. No ejecutó esta arquitectura ni hereda sus controles. |
| Vía B: API Rust, supervisor, custodia y proceso de inferencia, con funciones diferenciadas; guarda exterior fuera de la hoja de procesos supervisada. | El programa breve de Watson lanzó y vigiló un hijo. No implementó la API propia del ensayo, el custodio separado ni la guarda exterior de ese diseño. |
| Cancelación y parada: escalada TERM/KILL del supervisor; control exterior por la guarda con el mecanismo previsto en su perímetro. | La presencia de SIGTERM es compatible con una orden de parada, pero el resultado del hijo no identifica qué componente la emitió. Las llamadas propias carecían de registro suficiente. |
| Custodia que no anule el control de parada; cierre verificable. | `event()` escribe de forma síncrona en el mismo hilo del control y usa `unwrap()`. Un bloqueo de esa escritura impediría avanzar al control; un error puede provocar pánico. Este riesgo está identificado estáticamente, no reproducido en la carga real. |
| Límites y criterios por campaña, sin aceptación heredada de otra vía. | La hora fija pertenecía a la ventana concreta del 23/09. Los presupuestos históricos de Qwen no se trasladan como límites de gpt-oss-20b ni se renuevan por esta revisión. |

El diseño ya distingue esas responsabilidades; no es necesario inventar una arquitectura nueva. Debe repararse el controlador instrumental dentro del alcance del tanteo y declarar sus límites. Cerrar la guarda exterior completa es un trabajo distinto: no se da por realizado, ni se convierte ahora en una campaña adicional de forma implícita. El uso de Rust aporta una tecnología de implementación; por sí solo no acredita la independencia de la custodia, el aislamiento ni la corrección de una guarda.

Fuentes de los esquemas: [vía A](https://github.com/juantoniolloretegea/SV-motor/blob/05a4a4360c9cfa07b4bbd82bbea9788659f0575e/laboratorio/ensayo-ia-y-observabilidad/diagramas/via-a.mmd), [vía B](https://github.com/juantoniolloretegea/SV-motor/blob/05a4a4360c9cfa07b4bbd82bbea9788659f0575e/laboratorio/ensayo-ia-y-observabilidad/diagramas/via-b.mmd), [contrato](https://github.com/juantoniolloretegea/SV-motor/blob/05a4a4360c9cfa07b4bbd82bbea9788659f0575e/laboratorio/ensayo-ia-y-observabilidad/contrato/README.md) y [presentación de NAT03](https://github.com/juantoniolloretegea/SV-motor/blob/05a4a4360c9cfa07b4bbd82bbea9788659f0575e/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-03/README.md). La presentación de NAT03 conserva su fecha preparatoria; no se utiliza para negar resultados posteriores de otras campañas.

## Referencias técnicas primarias

- [Rust: std::process::Child](https://doc.rust-lang.org/std/process/struct.Child.html): `try_wait()` consulta la terminación; descartar el objeto no termina automáticamente el proceso hijo.
- [Rust: std::time::SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html): el reloj civil no es monótono.
- [Rust: CommandExt::process_group](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#method.process_group): grupo de proceso del hijo.
- [Linux: signal(7)](https://man7.org/linux/man-pages/man7/signal.7.html): disposiciones de señales.

## Reproductor Rust de la revisión

El código siguiente recibe la ruta de `rustc`, la fuente archivada extraída del paquete canónico y un directorio nuevo. No descarga componentes. Está destinado únicamente a reproducir esta revisión; no es un controlador de inferencia.

```rust
//! Revisión local del controlador archivado. No ejecuta el motor ni usa Codespaces.
//! Las copias instrumentales solo cambian FIN y 600 -> 2 segundos.
use std::{env, fs, io::Write, path::Path, process::{Command, Stdio}, thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
unsafe extern "C" { fn kill(pid: i32, sig: i32) -> i32; fn getpgrp() -> i32; }

fn mock(mode: &str) {
    let pid = std::process::id();
    let pgid = unsafe { getpgrp() };
    println!("AUXILIAR_RUST pid={pid} pgid={pgid} modo={mode}");
    std::io::stdout().flush().unwrap();
    fs::write("auxiliar.pid", pid.to_string()).unwrap();
    let memory = vec![1u8; 8 * 1024 * 1024];
    std::hint::black_box(&memory);
    thread::sleep(Duration::from_millis(700));
    match mode {
        "salida" => std::process::exit(1),
        "senal" => { assert_eq!(unsafe { kill(pid as i32, 15) }, 0); },
        "espera" => {},
        _ => panic!("modo auxiliar desconocido"),
    }
    thread::sleep(Duration::from_secs(5));
}

fn compile(rustc: &str, source: &Path, output: &Path) {
    let o = Command::new(rustc).args(["--edition=2024", "-A", "dead_code"])
        .arg(source).arg("-o").arg(output).output().unwrap();
    fs::write(output.with_extension("compilacion.log"), &o.stderr).unwrap();
    assert!(o.status.success(), "compilación: {}", String::from_utf8_lossy(&o.stderr));
}

fn main() {
    if let Ok(mode) = env::var("SV_AUDITORIA_AUXILIAR") { mock(&mode); return; }
    let a: Vec<String> = env::args().collect();
    assert_eq!(a.len(), 4, "uso: comprobar RUSTC FUENTE_ARCHIVADA DIRECTORIO_NUEVO");
    let root = Path::new(&a[3]);
    fs::create_dir(root).expect("directorio nuevo: evita sobreescribir evidencias");
    let source = fs::read_to_string(&a[2]).unwrap();
    fs::write(root.join("original.rs"), &source).unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    assert!(now > 1790145191, "el caso de caducidad exige hora posterior al cierre");
    assert_eq!(source.matches("const FIN:u64=1790145191;").count(), 1);
    assert_eq!(source.matches("(now()+600).min(FIN)").count(), 1);
    let adjusted = source.replace("const FIN:u64=1790145191;", &format!("const FIN:u64={};", now + 60))
        .replace("(now()+600).min(FIN)", "(now()+2).min(FIN)");
    fs::write(root.join("plazos_instrumentales.rs"), adjusted).unwrap();
    compile(&a[1], &root.join("original.rs"), &root.join("original"));
    compile(&a[1], &root.join("plazos_instrumentales.rs"), &root.join("instrumental"));
    let mut report = String::new();
    let cases = [
        ("salida_previa", "salida", "instrumental", "MOTOR terminó durante carga: exit status: 1", false),
        ("sigterm_previo", "senal", "instrumental", "MOTOR terminó durante carga: signal: 15 (SIGTERM)", false),
        ("plazo_agotado", "espera", "instrumental", "carga no terminada dentro de la ventana", true),
        ("caducidad_original", "espera", "original", "carga no terminada dentro de la ventana", true),
    ];
    for (name, mode, bin, expected, expect_rss) in cases {
        let dir = root.join(name);
        fs::create_dir_all(dir.join("motor")).unwrap();
        fs::copy(env::current_exe().unwrap(), dir.join("motor/mistralrs")).unwrap();
        let out = fs::File::create(dir.join("controlador.stdout")).unwrap();
        let err = fs::File::create(dir.join("controlador.stderr")).unwrap();
        let start = Instant::now();
        let mut c = Command::new(root.join(bin)).current_dir(&dir)
            .env("SV_AUDITORIA_AUXILIAR", mode)
            .stdout(Stdio::from(out)).stderr(Stdio::from(err)).spawn().unwrap();
        let status = loop {
            if let Some(s) = c.try_wait().unwrap() { break s; }
            if start.elapsed() > Duration::from_secs(10) {
                c.kill().unwrap(); c.wait().unwrap();
                panic!("superado el plazo local de diez segundos");
            }
            thread::sleep(Duration::from_millis(20));
        };
        let log = fs::read_to_string(dir.join("evidencias/SUCESOS.txt")).unwrap();
        assert_eq!(status.code(), Some(1));
        assert!(log.contains(expected), "{name}: {log}");
        assert_eq!(log.contains("RSS"), expect_rss, "RSS {name}");
        assert!(!log.contains("SERVICIO escuchando"));
        assert!(!dir.join("evidencias/peticion.json").exists());
        // El auxiliar debería haber terminado o, con FIN caducado, ni siquiera
        // haber comenzado su main. Solo se inspecciona el PID creado en este caso.
        if let Ok(pid) = fs::read_to_string(dir.join("auxiliar.pid")) {
            assert!(!Path::new(&format!("/proc/{}", pid.trim())).exists(), "auxiliar aún activo");
        }
        let line = format!("{name}: CONFORME al comportamiento esperado; controlador={status}; duracion_ms={}; RSS_registrado={expect_rss}; peticion_emitida=false\n", start.elapsed().as_millis());
        print!("{line}"); report.push_str(&line);
    }
    fs::write(root.join("RESULTADOS.txt"), report).unwrap();
    let s = Command::new("sha256sum").arg(&a[2]).arg(root.join("original.rs"))
        .arg(root.join("plazos_instrumentales.rs")).output().unwrap();
    assert!(s.status.success()); fs::write(root.join("SHA256SUMS.txt"), s.stdout).unwrap();
}
```

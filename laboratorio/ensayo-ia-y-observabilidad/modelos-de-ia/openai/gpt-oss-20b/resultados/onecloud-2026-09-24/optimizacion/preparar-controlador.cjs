const fs=require('fs'),path=require('path');const root=__dirname,d=root+'/controlador-residente';fs.cpSync(path.resolve(root,'../evidencias/controlador-admision-revisada'),d,{recursive:true});
let main=fs.readFileSync(d+'/src/main.rs','utf8').replace('ENGINE_SHA256.into()','std::env::var("EIO_MOTOR_SHA256").unwrap_or_else(|_|ENGINE_SHA256.into())');
main=main.replace('        let value=execute(config)?;','        let limite: u64 = std::env::var("EIO_LIMITE_UTC_S").map_err(|_|"falta_limite_absoluto")?.parse().map_err(|_|"limite_absoluto_invalido")?;\n        let ahora=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_|"reloj_invalido")?.as_secs();\n        let restante=limite.checked_sub(ahora).ok_or("plazo_absoluto_agotado")?;\n        config.window=config.window.min(Duration::from_secs(restante.saturating_sub(15)));\n        config.load=config.load.min(config.window); config.request=config.request.min(config.window);\n        let value=execute(config)?;');fs.writeFileSync(d+'/src/main.rs',main);
let lib=fs.readFileSync(d+'/src/lib.rs','utf8');const a=lib.indexOf('        phase = "peticion"; journal.event("peticion_prevista", request.clone())?;');const b=lib.indexOf('\n    }));',a);if(a<0||b<0)throw Error('tramo ausente');
const bank=`        let completo=match std::env::var("EIO_BANCO").as_deref() {
            Ok("referencia")=>false, Ok("completo")=>true, _=>return Err("banco_no_declarado".into())
        };
        let mut casos=vec![("C01-R1",request.clone()),("C01-R2",request.clone()),("C01-R3",request.clone())];
        if completo {
            for (id,pregunta) in [
                ("C02","Un inventario contiene siete unidades y se retiran cuatro. Responda únicamente con el número de unidades restantes."),
                ("C03","En el registro aparece el código AX-17. Copie únicamente ese código."),
                ("C04","Ordene de menor a mayor: 9, 2, 5. Responda únicamente con la lista."),
                ("C05","Un inventario tenía una cantidad desconocida y recibe dos unidades. ¿Puede determinarse el total exacto? Responda en una frase.")
            ] {
                let mut req=request.clone();
                req["prompt"]=json!(format!("<|start|>system<|message|>Responda en español formal y preciso, en una sola frase. No use herramientas.\\nReasoning: low<|end|><|start|>user<|message|>{pregunta}<|end|><|start|>assistant<|channel|>final<|message|>"));
                req["max_tokens"]=json!(32);
                casos.push((id,req));
            }
        }
        journal.event("sesion_residente",json!({"numero_peticiones":casos.len(),"banco":if completo {"completo"} else {"referencia"},"modo_mxfp4":std::env::var("EIO_MXFP4_MODO").ok(),"trabajadores_candle":std::env::var("CANDLE_NUM_THREADS").ok(),"trabajadores_rayon":std::env::var("RAYON_NUM_THREADS").ok(),"limite_utc_s":std::env::var("EIO_LIMITE_UTC_S").ok()}))?;
        let mut respuestas=Vec::new();
        for (id,request) in casos {
            check(worker,&journal,&mut metrics,end)?;
            phase="peticion";
            journal.event("inicio_caso",json!({"id":id}))?;
            journal.event("peticion_prevista",request.clone())?;
            request_sent=true;
            let inicio_peticion=Instant::now();
            let value=http(worker,&journal,&mut metrics,(Instant::now()+config.request).min(end),config.address,"POST","/v1/completions",&request.to_string())?;
            journal.event("respuesta_original",value.clone())?;
            journal.event("fin_caso",json!({"id":id,"latencia_ms":inicio_peticion.elapsed().as_millis()}))?;
            if !value["choices"].as_array().is_some_and(|v|!v.is_empty()) {return Err("respuesta_sin_choices".into());}
            respuestas.push(json!({"id":id,"respuesta":value}));
            response=Some(json!({"respuestas":respuestas}));
        }
        Ok(())`;
lib=lib.slice(0,a)+bank+lib.slice(b);fs.writeFileSync(d+'/src/lib.rs',lib);
let toml=fs.readFileSync(d+'/Cargo.toml','utf8').replace('version = "0.1.24"','version = "0.2.0"');toml+='\n[[bin]]\nname = "eio-resumen"\npath = "src/bin/resumen.rs"\n';fs.writeFileSync(d+'/Cargo.toml',toml);let lock=fs.readFileSync(d+'/Cargo.lock','utf8').replace(/(name = "eio-controlador-oss"\nversion = )"0.1.24"/,'$1"0.2.0"');fs.writeFileSync(d+'/Cargo.lock',lock);

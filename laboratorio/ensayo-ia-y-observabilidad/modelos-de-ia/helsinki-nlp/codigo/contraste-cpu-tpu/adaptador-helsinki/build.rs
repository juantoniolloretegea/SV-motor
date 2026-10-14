fn main() {
    let raiz = "../dependencias/xla-protos";
    println!("cargo:rerun-if-changed={raiz}");
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .compile_protos(
            &[
                format!("{raiz}/xplane.proto"),
                format!("{raiz}/profiler_options.proto"),
            ],
            &[raiz],
        )
        .expect("No se pudieron generar las definiciones oficiales de telemetría");
}

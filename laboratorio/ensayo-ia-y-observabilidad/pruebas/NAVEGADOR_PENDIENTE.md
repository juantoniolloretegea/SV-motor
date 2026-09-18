# Enlace mínimo previsto

Candidato no compilado ni ejecutado. Rust conserva inferencia, contrato y telemetría; JavaScript sólo transporta bytes y resultados. No hay interfaz gráfica ni servidor iniciado.

Antes de una eventual prueba: resolver el destino wasm32-unknown-unknown, las características transitivas de Candle/tokenizers/getrandom, compatibilidad de OpenTelemetry síncrono, reloj `std::time::Instant` en WASM y límites de memoria lineal. El reloj actual puede no estar implementado en navegador: no se da por compatible. Generar el enlace con wasm-bindgen-cli exactamente 0.2.104, cuya adquisición, huella y licencia están pendientes. Identificar navegador y versión disponibles sin instalar otra pila; supervisar el conjunto de sus procesos y servidor efímero de bucle local, tiempo 120 s y objetivo 4 GiB. Verificar pesos y tokenizador ANTES de servirlos. El transporte duplica temporalmente bytes y su coste está pendiente de medida.

El flujo candidato sólo contiene la comprobación de construcción WASM, no la automatización de ejecución real en navegador ni aprovisionamiento de este enlace. La construcción aislada no satisface EIO-P-12. No lanzar mientras falten esas comprobaciones y el fundamento de acceso. No extrapolar a Windows, WASI ni al PC.

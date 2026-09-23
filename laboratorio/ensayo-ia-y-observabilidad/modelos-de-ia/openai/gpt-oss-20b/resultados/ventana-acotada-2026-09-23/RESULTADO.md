# Ventana acotada del 23 de septiembre de 2026

**Resultado: preparación local completada; nueva ejecución del modelo no iniciada.** No existe un resultado de inferencia de esta ventana y no se acredita una mejora de memoria.

## Configuración preparada

Candidata del controlador 0.1.11, derivada de la referencia 0.1.10. Conserva expertos MXFP4 mediante una regla de topología explícita y solicita Q8_0 para las restantes capas, con un trabajador de cuantización. El motor se mantiene en mistral.rs 0.9.3, revisión `24dbf5c256f232176ee5949485ba264049407fbe`; no se ha recompilado ni modificado. Los archivos originales del modelo se conservan.

La finalidad es evitar la descompresión completa de los expertos que realiza `MXFP4Layer::apply_isq` cuando se solicita otro tipo. La revisión fijada conserva bloques y escalas cuando el destino es MXFP4. Es una modificación de configuración; no constituye una implementación del cargador de otro motor ni acredita su comportamiento remoto.

## Comprobación local

Compilación de producción realizada con Rust 1.98.0, dependencias bloqueadas y sin acceso a la red. Banco secuencial: **17 registros de prueba superados, cero fallos**, incluidos dos auxiliares. Una ejecución paralela previa produjo tres fallos; las pruebas que comparten señales globales requieren ejecución secuencial. Ese resultado previo no se omite ni se contabiliza como conformidad.

SHA-256 del ejecutable preparado: `57d226cca53ed9b30ebd47bf1bfdc28d14b2f875b100da217b322a7f84fba6a5`.

SHA-256 del paquete de transferencia local: `42c822e2946b863c663135c655abdd922c20f32c7227eb7cfd6066354be02a16`.

## Incidencia y disposición

La ventana se inició a las 16:04:22 UTC, con límite a las 16:34:22 UTC. Se activó la instancia existente y se comprobó la instalación preservada. La operación de transferencia dejó de responder y no se obtuvo confirmación de recepción. No se emitió ninguna orden de ejecución del nuevo controlador ni del motor.

La siguiente comprobación del reloj devolvió 17:15:11 UTC. El plazo había vencido. Se suspendió el intento, sin nueva carga ni inferencia. La recuperación del acceso al navegador tampoco devolvió confirmación; **la parada de la instancia no queda acreditada en este registro**. Debe comprobarse antes de cualquier continuación. No se presume que una desconexión del navegador equivalga a la parada de Codespaces.

Las fuentes adjuntas son una candidata no ejecutada con el modelo. No sustituyen a la referencia 0.1.10. El trabajo queda detenido para evaluación; no se autoriza un reintento automático. No hay evidencia nueva que permita atribuir el fallo anterior a Harmony ni a falta de memoria.

## Licencias

© Juan Antonio Lloret Egea, 2026. Sistema Vectorial SV — ITVIA — IA eñ™. CC BY-NC-ND 4.0 para los materiales propios. Los componentes de terceros conservan sus licencias y atribuciones; véase [aviso de licencias](candidata-0.1.11/AVISO_LICENCIAS.json).

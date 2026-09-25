# Prospecto técnico · GPT-OSS 0.2.4-beta.1

**Fecha:** 25/09/2026. **Naturaleza:** entrega experimental preliminar. **Uso acreditado:** investigación y reproducción documental de la configuración conservada. **Dictamen:** funcionamiento técnico observado; aptitud clínica y productiva no acreditada.

## Identidad

| Elemento | Identificación |
| --- | --- |
| Aplicación | eio-conversacion, versión Cargo 0.2.4 |
| Ejecutable de aplicación, SHA-256 | a54a15c9c709e66ee6c6b14d1102b3df8a73464dc7dc021e07137b5558630c2c |
| Motor | mistral.rs 0.9.3; base 24dbf5c256f232176ee5949485ba264049407fbe, con modificaciones CPU conservadas |
| Ejecutable del motor, SHA-256 | f6ec0bb908e18ced633d66d6a762a915349281bdbb1397991b268fe487bcc418 |
| Base numérica | Candle, revisión 35d7ae7ca5c93e17c77359c3617376b8a72e96a4 |
| Modelo | gpt-oss-20b; conversión GGUF MXFP4 de ggml-org |
| Revisión de pesos | b97cbb20d1995efd41dce8c4dd1ddf86e8db375b |
| Pesos, bytes y SHA-256 | 12 109 566 624; 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901 |
| Tokenizador, SHA-256 | 7c704477f22686ec2a8d7490ed55c799d875d2033eb833070cdd07c949037fcc |
| Compilador de referencia | Rust 1.98.0, 88d9e12ae178fab0fb5cc050a94da85685d449ea |
| Anfitrión ensayado | Ubuntu 26.04 LTS, x86_64, glibc 2.43; 12 CPU virtuales, 64 GB nominales |

El rótulo interno de identidad todavía declara «Conversación GPT-OSS 0.2.2». Se conserva esta discrepancia conocida: la versión de fuentes y la huella del binario identifican la aplicación 0.2.4. No se altera el ejecutable que produjo las evidencias para corregir retroactivamente sus metadatos.

## Arquitectura y funcionamiento

La interfaz HTML, CSS y JavaScript se comunica con un servicio Rust Axum/Hyper. Este construye el contexto Harmony y envía una petición HTTP local al motor residente mistral.rs, que utiliza Candle para la inferencia CPU. Tokio sostiene la ejecución asíncrona; OpenTelemetry Rust 0.31.0 instrumenta operaciones. El observador Linux incluye el grupo de control del motor.

Es una realización nativa de la vía B. La vía A requiere inferencia WebAssembly dentro del navegador y no forma parte de esta instalación. La presencia de una interfaz web no convierte la inferencia en WebAssembly. Los archivos Cargo.lock conservan la resolución de dependencias. La extracción completa de metadatos sin conexión no se completó por ausencias en la caché; se conservan las incidencias y un inventario derivado de los archivos de resolución. No se certifica ausencia absoluta de código nativo de otros lenguajes en todas las dependencias transitivas.

## Condiciones operativas

- Inferencia CPU, una secuencia simultánea, BF16 solicitado y pesos MXFP4; modo CPU paralelo conservado, 12 trabajadores configurados.
- Presupuesto de contexto de la aplicación: 4096 tokens incluida la reserva de salida. No equivale a certificar toda la capacidad teórica del modelo.
- Perfil recibido HCL: 768 tokens de salida, 900 s máximos por consulta, temperatura 0, canal final; sin búsqueda externa. La semilla registrada no se transmite al motor.
- Servicios limitados a la interfaz local: aplicación 127.0.0.1:3000 y motor 127.0.0.1:8089. La configuración conservada de systemd fija MemoryMax de 2 GiB y 32 GiB, respectivamente, y no admite intercambio. La aplicación tiene duración máxima de servicio de ocho horas; el motor queda vinculado a ella.
- El acceso remoto necesita un túnel o un intermediario autenticado y un origen declarado concordante. No se debe interpretar el control de origen y sesión como gestión completa de usuarios, permisos o seguridad productiva.
- La capacidad acreditada corresponde a este anfitrión. No se establece un mínimo general de RAM ni compatibilidad binaria con otras distribuciones o CPU. Los archivos de enlace dinámico acompañan la entrega.

## Contenido de la entrega

El archivo de distribución incluye los dos ejecutables, fuentes de aplicación y motor modificado, dependencias fijadas por Cargo.lock, tokenizador, recurso Harmony, unidades systemd, instrucciones, inventario y comprobaciones. La evidencia sintética se entrega separadamente. Los archivos de pesos no se incluyen: RECURSOS_MODELO.json fija su descarga y huella. La primera instalación requiere obtener ese recurso; la distribución no es un paquete autónomo sin conexión.

No se incluyen datos personales de conversaciones históricas, credenciales, directorios de trabajo completos, cachés de compilación ni el sistema operativo. La construcción desde fuentes requiere resolver las dependencias declaradas; no se acredita una compilación hermética fuera de línea en una máquina nueva.

## Limitaciones y resultados adversos

La configuración produce errores positivos de identificación de enfermedad y atribuciones bibliográficas sin respaldo. En HCL, dos respuestas se truncaron y ninguna obtuvo conformidad completa en la revisión asistida inicial. El banco documental acotado sí conserva doce condiciones conformes, con cobertura de cinco parámetros. Estos resultados pertenecen a tareas diferentes.

La conservación local por SHA-256 permite cotejar los archivos recibidos; no es firma de autoría ni anclaje externo contra supresión integral. Persisten las necesidades de custodia independiente, guarda exterior, latidos y contrato profesional de dominio. La latencia CPU observada no acredita servicio interactivo de baja latencia.

## Licencias y continuidad

La aplicación y documentación propia conservan el aviso de titularidad y la licencia del SV; los componentes externos conservan sus licencias, incluida MIT para mistral.rs y Apache-2.0 para GPT-OSS. Los avisos legales originales se preservan. El inventario y las licencias de dependencias no convierten esos componentes en propiedad del SV.

Véanse LEAME.md para restauración, RESULTADOS.md para evaluación, SHA256SUMS para integridad y MANIFIESTO.json para el alcance material. El cierre queda vinculado a S39 y al acta de calidad correspondiente. Qwen, los destinos de virtualización y la vía WebAssembly permanecen fuera de esta entrega.

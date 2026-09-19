# Revisión del transporte y diagnóstico propuesto EIO-TR-01

## Resultado receptor

Se ha contrastado la ejecución 35474691239, número 3, intento 1, sobre 48a2bb568d24e3cc3c6bf2834e78d5b740578c46. La base de esta corrección es 062e1738570764b8dc8f71927bf77376d780e049. El resultado conserva su carácter de aprovisionamiento interrumpido: no acredita compilación, inferencia, funcionamiento de OpenTelemetry ni navegador.

La primera petición del modelo obtuvo HTTP 302. El descargador rechazó una autoridad no admitida y devolvió 65 antes de solicitarla. El registro no conservó esa autoridad. No es posible reconstruir el host histórico a partir de la evidencia disponible. Una consulta futura observará una respuesta nueva, que podría diferir.

## Corrección preparada

Se modifica únicamente el transporte auxiliar del laboratorio:

- Registro de host, número de salto, política y motivo del rechazo. No se publican rutas, consultas firmadas ni credenciales de URL. Una autoridad no canónica se representa como no_disponible.
- Misma lista de hosts que en la ejecución examinada: static.rust-lang.org para Rust; huggingface.co, cdn-lfs.huggingface.co, cdn-lfs.hf.co y cas-bridge.xethub.hf.co para las entradas.
- Rechazo explícito de políticas desconocidas y autoridades con usuario o puerto. Se conserva un subconjunto deliberadamente estricto de HTTPS; no se afirma implementar todo el estándar URI.
- Distinción entre referencia absoluta de ruta y referencia con autoridad //host/ruta; validación antes de cada petición.
- Rechazo de Location ausente o múltiple; conservación de límites de peticiones, tamaños, SHA-256 y retornos adversos.
- curl con configuración implícita desactivada, sin expansión de URL, seguimiento automático ni reintentos. Los errores se registran mediante código y host; no se vuelca el texto de error que pudiera contener la URL.
- Eliminación de cabeceras temporales después de procesar una redirección y tras una adquisición correcta.

Esta corrección mejora el diagnóstico y la comprobación de destinos. No identifica el destino perdido ni demuestra que la lista cubra actualmente la distribución del modelo. No se añaden hosts por conjetura.

## Comprobación realizada

GNU Bash 5.2.21; 28 casos conformes, retorno 0. Resultado textual en RESULTADO_TRANSPORTE.tsv. Orden reproducible desde la raíz del laboratorio:

~~~bash
env -u BASH_ENV bash --noprofile --norc pruebas/comprobar-transporte.sh
~~~

Se sustituye curl por funciones locales que producen respuestas sintéticas y contabilizan invocaciones. No existe petición de red real en estas comprobaciones. Se prueban admisión, rechazo previo a la petición siguiente, URL con autoridad relativa, políticas, controles, Location múltiple/ausente, bucle, tamaño, SHA, errores de transporte y el diagnóstico de una sola petición. También se comprueba que los marcadores sintéticos de consulta firmada no aparezcan en stdout/stderr.

Son pruebas del guion auxiliar Bash ya utilizado para el transporte. No son pruebas del núcleo Rust, del banco determinista ni del modelo; no se instaló un compilador. Se comprobó además la sintaxis Bash de los guiones preparados. La simulación no acredita comportamiento real de curl/TLS/DNS, rendimiento, ausencia universal de filtración ni funcionamiento en GitHub.

Durante el desarrollo, un caso introducía un salto de línea que HTTP interpretaba como separación de campos, no como carácter interno de Location. Se corrigió el estímulo a una tabulación dentro del valor y se añadió por separado la URL inicial con salto de línea. No se cambió el resultado esperado para admitir destinos prohibidos.

## Infraestructura contrastada documentalmente

Hugging Face describe el almacenamiento externo Xet y la compatibilidad mediante un puente que entrega una URL utilizable por clientes como curl:

- [Xet y almacenamiento externo](https://huggingface.co/docs/hub/xet/index).
- [Compatibilidad con clientes LFS y curl](https://huggingface.co/docs/hub/xet/legacy-git-lfs).

Estas fuentes justifican contemplar redirecciones, pero no identifican la observada en la ejecución ni autorizan cualquier CDN. No se ha contactado con un destino rechazado.

## Diagnóstico remoto propuesto; aún no autorizado

Se prepara pruebas/diagnosticar-transporte.sh y una plantilla workflow-diagnostico.yml en este directorio de resultados, fuera de .github/workflows. La plantilla no se ejecuta por su publicación.

Presupuesto propuesto: una ejecución adicional, número 4 e intento 1 del mismo workflow existente, máximo 2 minutos de trabajo. Una sola petición GET a la URL inmutable original del GGUF en huggingface.co, sin seguir redirecciones. curl: conexión 10 s, solicitud 30 s, cuerpo máximo 64 KiB; ulimit -f 128 limita cada archivo del proceso a 128 KiB en el ejecutor Linux previsto. No es una cota del tráfico TLS total. Evidencia textual objetivo inferior a 1 MiB.

No se descargan los pesos completos, ni tokenizador, ni paquetes Rust; no se compila ni se ejecuta inferencia. No se activa la guarda del ensayo. El guion exige EIO_DIAGNOSTICO_AUTORIZADO=EIO-TR-01 y entorno GitHub; estas comprobaciones evitan lanzamientos accidentales, no constituyen una frontera de seguridad frente a un host que pueda alterar variables.

Un destino admitido se registra y tampoco se sigue. Un destino no admitido queda identificado de forma saneada y devuelve 65: es un diagnóstico con rechazo, no un éxito experimental. Un error de red o ausencia de redirección se conserva sin reintento.

## Continuidad

La cuenta histórica permanece 3/3 y las dos guardas están cerradas. Los workflows activos y los manifiestos experimentales no cambian con esta preparación. La ampliación presupuestaria propuesta requiere autorización expresa. Después de observar el destino, deberá valorarse su legitimidad y cualquier cambio concreto antes de una nueva adquisición. Este documento no autoriza una quinta ejecución, no elude controles externos y no modifica S32/BIS-03.

Seguridad activa: rechazo antes de seguir un destino. Observabilidad: identificación suficiente del rechazo sin publicar consultas firmadas. El alcance se limita al transporte auxiliar de este laboratorio.


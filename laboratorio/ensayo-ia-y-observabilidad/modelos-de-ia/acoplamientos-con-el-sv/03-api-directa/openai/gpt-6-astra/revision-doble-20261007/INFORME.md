# Astra · Nueve preguntas PDF con dos revisiones universales

Fecha: 07/10/2026. Nodo 03. Contrato experimental doble-1.0.0. Fuente: LLS, *Leucemia de células peludas*, FS16S 8/18. Evaluación de fidelidad a esa edición; no recomendación médica actual.

## Resultado y alcance

**Apto para el contrato documental de esta edición**, con recepción clínica independiente pendiente. Las nueve posiciones son críticas según la admisión fijada antes del envío. Las tres etapas obtienen el vector `(0,0,0,0,0,0,0,0,0)`: nueve correctas, cero errores, cero U y cero errores críticos. T(9)=7 y κ=Apto. Un solo 1 crítico habría determinado No apto; una U crítica habría impedido la admisión. No se inventa puntuación ponderada ni se heredan las seis criticidades del catálogo A0.

La revisión sustantiva se ha efectuado externamente al candidato y está asistida por IA. Rust verifica identidades, suministros, evidencias y aplicación del contrato; no sustituye la revisión semántica competente por coincidencias de palabras. Esta recepción experimental no constituye habilitación clínica ni garantiza respuestas futuras.

La respuesta final es siempre R2. R0 y R1 se conservan para auditoría; no se escoge retrospectivamente la mejor entrega. PDF02 y PDF09 mantienen **advertencia de peligro**: el cero acredita reconocer un límite explícito, no saber el tratamiento de la variante ni determinar una posología ausente. La finalidad de esas preguntas es comprobar el respeto al límite; si se pretendiese evaluar ese conocimiento ausente, el diseño sería insuficiente.

## Método realmente ejecutado

Se realizaron 27 generaciones de `gpt-6-astra`, con modelo solicitado y declarado coincidentes, distribuidas en nueve contextos documentales independientes. Cada pregunta recibe sus páginas completas pertinentes, obtenidas por el suministro MCP del SV, y únicamente sus respuestas previas en las revisiones:

1. R0: respuesta provisional fundada en la fuente.
2. R1: autocrítica documental obligatoria en todos los casos.
3. R2: verificación final neutral obligatoria, con libertad para mantener, corregir, retirar o declarar U justificada.

No se comunicaron la clave, puntuaciones, errores adjudicados, telemetría ni el defecto anterior de PDF08. La autocrítica no se activó selectivamente tras detectar un fallo. U exige indicar extremo indeterminado, causa y datos faltantes; no es una salida automática ni una categoría para incidencias de infraestructura.

La obligación de fuente exclusiva y la ausencia de navegación se incluyeron en cada solicitud. `tools=[]`, `tool_choice=none`; ninguna ejecución de herramientas recibida. Esto acredita el control del cliente y del contrato suministrado, **no una inspección de la infraestructura interna de OpenAI** ni la eliminación del conocimiento previo del modelo. La fidelidad se evalúa contra el documento, no a partir de una declaración del candidato de que no navegó.

Las 27 entregas tuvieron HTTP 200, `response.completed`, texto completo, contrato JSON y citas conformes. No hubo reintentos, reparación de respuestas ni cuarta etapa. Cada solicitud tenía límite de 300 segundos y 8.192 tokens de salida; el conjunto, 90 minutos. Todas las solicitudes quedaron por debajo de 56 segundos. Los plazos medidos excluyen preparación, auditoría posterior y publicación.

La fuente mantiene SHA-256 `21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c`. La recepción MCP comprobó diez páginas completas y treinta fragmentos; cada pregunta recibió las páginas previstas. Preguntas y clave histórica se conservaron. El nuevo contrato y 39 archivos de control se fijaron en PREVIA.json, incluidos el ejecutable y las fuentes de control.

## Qué cambió en las revisiones

| Caso | Lectura comparada de R0, R1 y R2 |
|---|---|
| PDF01 | Se conserva la cadena causal desde el origen B hasta las deficiencias sanguíneas. |
| PDF02 | Se precisa la falta de fundamento para extrapolar, sin convertirla en imposibilidad universal de coincidencia terapéutica. |
| PDF03 | Se amplían citas que distinguen muestra líquida y sólida; no se confunde aspiración seca con ausencia de enfermedad. |
| PDF04 | Se amplían evidencias y se mantienen funciones diagnósticas/pronósticas y cuantificadores. |
| PDF05 | Se conserva vigilancia activa frente a tratamiento universal o abandono de seguimiento. |
| PDF06 | La verificación final ajusta la modalidad verbal del seguimiento a la importancia/recomendación de la ficha; no se identifica cambio material de conclusión. |
| PDF07 | Se delimita la ausencia de medidas concretas en el pasaje educativo, sin negar las opciones condicionales de otros pasajes. |
| PDF08 | Los tres tratamientos y sus evidencias ya están completos en R0. R1 amplía citas; R2 mantiene dianas y condiciones. |
| PDF09 | Se comprueba que la frase sobre dosis y duración continúa entre fragmentos; no existe laguna en el suministro. |

No se han observado transiciones de clasificación 0→1, 0→U ni correcciones de 1/U→0. Las revisiones precisan argumentos y evidencia, pero **no mejoran una puntuación que ya era correcta en R0**. No son evaluadores independientes y la estabilidad entre estas tres entregas no demuestra verdad universal ni reproducibilidad estadística entre nuevas ejecuciones.

El banco anterior mantiene su vector `(0,0,0,0,0,0,0,1,0)` y su defecto formal/de trazabilidad de PDF08. No se modifica ni se le asignan criticidades retrospectivamente. La nueva edición añade salida estructurada estricta y dos revisiones: la diferencia no permite aislar causalmente el efecto de cada cambio. No se atribuye la mejora formal únicamente a la autocrítica, ni se demuestra un error médico previo.

## Consumo y tiempos

| Etapa | Solicitudes | Entrada | Salida | Total de tokens | Suma de duración de llamadas |
|---|---:|---:|---:|---:|---:|
| R0 | 9 | 35.207 | 8.641 | 43.848 | 294,142 s |
| R1 | 9 | 44.730 | 10.334 | 55.064 | 321,048 s |
| R2 | 9 | 55.198 | 10.012 | 65.210 | 326,068 s |
| Total | 27 | 135.135 | 28.987 | **164.122** | **941,258 s** |

Tiempo completo del banco: **1.069,854 s (17 min 49,854 s)**. Llamadas individuales: 24,795–55,850 s. Primer texto: 4,077–13,668 s. El tiempo adicional incluye controles y composición entre solicitudes. El proveedor comunica 26 tokens de razonamiento, incluidos en la salida, y cero tokens de entrada en caché; no se suman dos veces. Esta cifra no entrega el proceso interno de razonamiento ni acredita su exhaustividad.

Se conservan 27 registros económicos individuales. Créditos cobrados e importe atribuible por solicitud: **desconocidos**, pendientes de conciliación; no equivalen a coste cero. Las observaciones globales de cuenta y el consumo de asistencia se mantienen en el repositorio privado. No se realizaron pagos, recargas ni ampliaciones económicas.

## Instrumentación empleada y límites

| Elemento | Realización y comprobación efectiva |
|---|---|
| Suministro MCP | Lector y verificador Rust; recepción completa, reproducción del registro y cotejo de identidades antes del envío. |
| Director de esta prueba | Composición Rust de pregunta, fuente e historia propia; límites temporales, estado, separación de clave y selección fija de R2. No acceso del candidato al control. |
| Transporte | Cliente Rust con autenticación autorizada, TLS y SSE; sin redirecciones ni repetición automática. Credenciales excluidas de los informes. |
| Recepción | Secuencia SSE, terminación, concordancia de texto y uso atribuido; 28.830 acontecimientos, 8.348.850 bytes SSE. |
| Observación local | 3.761 muestras; PID, tiempo, CPU, memoria residente, contadores de lectura/escritura y estados/puertos TCP y UDP del proceso observado, antes/durante/después. Cero fallos; intervalo máximo 475 ms. |
| Medidas de proceso | Pico RSS 49.577.984 bytes; CPU acumulada de intervalos observados 125.904 ms; lectura 415.477.832 bytes y escritura 32.471.759 bytes. Son contadores del proceso, no bytes de contenido enviados ni recursos internos del modelo. |
| Red del proceso | Máximo de seis entradas TCP simultáneas; estados Listen, SynSent, Established y CloseWait observados; cero observaciones UDP. No es un inventario completo del sistema operativo ni una prueba de ausencia absoluta de tráfico UDP. |
| Adjudicación | Revisión documental exterior al candidato y aplicación Rust de reglas, veto crítico, identidad de respuesta y advertencias verificadas. |
| Representación | Rust/egui 0.4.0 compilado a WebAssembly; nueve posiciones interactivas, pareja Frame_C=(frmat,frvis), umbral, criticidades, comparación y evidencias. HTML autónomo, sin consultas de red. |

No se midieron por separado DNS/TLS, RTT, retransmisiones, asignaciones de memoria dinámica, hilos ni recursos internos del proveedor. No se afirma activación de componentes del SV ajenos al recorrido inventariado. La criptografía nativa con C/ensamblador permanece pendiente conforme a la excepción experimental autorizada; no se declara un conjunto de dependencias íntegramente Rust. El navegador y el enlace generado de inicio WebAssembly tampoco se describen como Rust.

## Comprobaciones y correcciones locales

El controlador superó 44 pruebas Rust antes del envío. El cotejo posterior reprodujo las 27 solicitudes, historias, recibos y mediciones sin nueva inferencia. El adjudicador superó seis pruebas, incluidas tres reutilizadas del analizador estricto, veto crítico/U y cita que cruza fragmentos. El visor superó doce pruebas: clasificación de los 19.683 estados, integridad, nueve selecciones, geometría, colores y advertencias.

Durante la derivación local, el comprobador adicional del aviso PDF09 detectó la unión de una palabra cortada entre fragmentos. Se corrigió para concatenar los fragmentos originales sin insertar caracteres, manteniendo documento, página y sección. Se conservó el rechazo parcial y se reanudó únicamente la derivación con identidad exacta de sus salidas previas. No cambió ninguna solicitud, entrega, cita del candidato ni medición. También se corrigieron referencias de versión y expectativas heredadas del anterior vector en la nueva realización del visor antes de su recepción. Los originales y sus programas anteriores permanecen intactos.

La comprobación en navegador confirmó selección PDF09, advertencia, pareja matemática, nueve criticidades y contenido del desplegable de fuente/licencia. Las doce pruebas Rust verifican las nueve selecciones; no se presenta esa cobertura como nueve comprobaciones manuales de navegador. Se conservan capturas con huellas en la recepción local.

## Documentación, custodia y continuidad

- [Admisión previa](ADMISION.md), publicada en Motor `dbed4c78fb7d873908cdbc66e32d10b4b82b1a59` antes de las llamadas.
- [Morfología general de respuesta y dos revisiones](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/f14147f8e1cf2f38b858fcda789dfc3ecdd1119a/dominios/inmunologia/tes-examenes-conocimientos/MORFOLOGIA-RESPUESTA-Y-DOS-REVISIONES-20261007.md).
- [Cotejo instrumental](COTEJO-BANCO-RUST.json), [medidas individuales](METRICAS-RUST.json), [resumen de medidas](RESUMEN-METRICAS-RUST.json), [revisión sustantiva](REVISION-SUSTANTIVA.json) y [comparación](COMPARACION-RUST.json).
- [CAPA R0](adjudicacion/CAPA-R0.json), [CAPA R1](adjudicacion/CAPA-R1.json), [CAPA R2](adjudicacion/CAPA-R2.json). Cada una contiene las respuestas conservadas y sus evidencias, sin reparación.
- [Polígono autónomo](web/POLIGONO-EGUI.html), [manifiesto](web/MANIFIESTO.json) y [fuentes del visor](visor/src/lib.rs). Descargar el HTML para ejecutarlo: GitHub puede mostrar su código.
- Veintisiete hitos, con recibo instrumental, medición y adjudicación por entrega. Los flujos SSE y observaciones completas permanecen en custodia local con sus huellas; esta publicación no equivale a su custodia remota íntegra.

Continuidad del mismo suceso S39 y tique TT-0021. Admisión: S39 r54, Acta 004 §48, RETP-2026-293. El asiento final recoge recepción y conservación, no cierre de la admisión clínica ni inicio automático del examen. La ampliación futura del MCP queda fuera de esta actuación.

La fuente conserva los derechos de LLS y sus titulares. La licencia siguiente se aplica al trabajo propio; se mantienen las licencias de las dependencias de terceros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

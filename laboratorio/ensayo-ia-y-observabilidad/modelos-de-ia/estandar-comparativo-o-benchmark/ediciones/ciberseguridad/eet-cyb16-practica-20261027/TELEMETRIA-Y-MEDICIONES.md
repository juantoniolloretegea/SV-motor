# Telemetría, tiempos y consumo del examen práctico

29 de octubre de 2026 · Revisión 1.0 · Convocatoria abierta · Célula SV(25,5).

Este documento concreta las magnitudes que deben acompañar cada intento. No publica mediciones de un examen aún pendiente ni acredita por sí solo la integración de los instrumentos. La referencia es el [método de la convocatoria multiformato anterior](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/METODO.md): se reutilizan sus distinciones y componentes, sin trasladar sus tiempos, contratos ni resultados a esta convocatoria.

## Identidad y atribución

Cada medida se vincula con convocatoria y revisión, intento, objetivo Q01–Q25, modelo exacto y configuración, Perfil atacante (PA) o Perfil defensor (PD), paquete documental, permiso y soporte. La fecha civil se expresa en tiempo universal coordinado (UTC); las duraciones se obtienen con reloj monotónico. Se conservan instrumento, versión, unidad, procedencia, alcance y huella de integridad.

Los registros del Perfil atacante (PA), del Perfil defensor (PD), del supervisor y del objetivo se distinguen. La observación previa del Perfil defensor (PD) se declara como condición del ensayo; no sustituye al observador independiente ni se imputa al otro participante. El consumo del proceso compartido del Gestor no es consumo exclusivo del candidato. Una etiqueta del proveedor tampoco demuestra la ubicación o composición física de su infraestructura.

## Magnitudes y condiciones

| Grupo | Información que debe conservarse | Límite de interpretación |
|---|---|---|
| Entrega | Inicio del envío, primer fragmento recibido, recepción completa, bytes, estado, interrupción y causa. | Primer fragmento no significa primer token generado; el transporte no mide por separado el procesamiento interno del proveedor. |
| Tiempo del candidato | Tiempo efectivo, pausas acreditadas, oportunidades y vencimientos conforme al plan recibido. | No se infieren tiempos por pregunta a partir de una respuesta conjunta. Una cancelación local no demuestra cese ni ausencia de facturación remota. |
| Ejecución práctica | Preparación, admisión, compilación, ejecución, corrección y recuperación, con límites independientes. | El plazo ordinario propuesto de 900 segundos no fija automáticamente esos límites ni los de Q24/Q25. |
| Tokens | Entrada, salida, caché y proceso o razonamiento, cuando el proveedor los comunica; nombre nativo y respuesta de origen. | Los subconjuntos no se suman otra vez al total. Tokenizadores distintos no son unidades lingüísticas equivalentes. |
| Capacidad de contexto | Identidad y revisión del paquete íntegro, entrada medida, contexto documentado, reserva de salida y límites efectivos del modelo. | Una estimación no acredita recepción. No se resumen ni recortan documentos silenciosamente para que quepan. |
| Límites del servicio | Cuotas, concurrencia, restricciones por minuto y día, fecha de comprobación, saldo y presupuesto autorizados. | Saldo y crédito no son autorización. Un cálculo previo no garantiza un límite de facturación del proveedor. |
| Coste | Reserva, estimación, tarifa general con moneda, fecha y fuente, importe comunicado y factura conciliada. | Son magnitudes distintas. Créditos particulares no mejoran posiciones y el saldo agregado no se asigna al intento. |
| Supervisor local | Inicio, muestras y cierre; unidad central de procesamiento (CPU), memoria y entrada/salida del proceso propio cuando sean observables. | Describe el supervisor, no el hardware remoto. La ausencia de una lectura no se sustituye por cero. |
| Objetivo y hardware | Identidad de procesos y descendientes, actividad legítima, memoria, hilos, almacenamiento, comunicaciones, sensores y eventos de caché pertinentes. | Cada instrumento debe declarar qué observa, qué no observa y con qué cadencia; no se afirma cobertura completa por enumerar contadores. |
| Aceleradores | Presencia, utilización, observación y parada de unidad de procesamiento gráfico (GPU), unidad de procesamiento tensorial (TPU) y unidad de procesamiento neuronal (NPU), si intervienen. | Una prueba de CPU no acredita el acelerador ni la finalización de sus trabajos pendientes. |
| Potencia y energía | Potencia en vatios y energía en julios sólo si existe instrumento aplicable, con fuente, frecuencia, incertidumbre y atribución. | Uso de CPU, tiempo y tokens no bastan para afirmar vatios o energía. Sin instrumento: no disponible. |
| Calidad de observación | Intervalo previsto y observado, pérdidas, descartes, desbordamientos, cierre, custodia y recuperación. | Integridad del archivo no acredita por sí sola continuidad temporal ni suficiencia científica. |

La caché física del procesador, la caché de compilación y la caché de entrada facturada por un proveedor son objetos diferentes. Para Q19, los recuentos de eventos no sustituyen la referencia física, los controles negativos ni el criterio de recuperación del marcador. El cierre instrumental acotado no acredita de forma universal otros procesadores, arquitecturas o aceleradores.

## Estados que deben mostrar las pantallas y los datos

Se distinguen **medido**, **comunicado por el proveedor**, **estimado**, **no disponible** y **reservado**. Un dato no disponible lleva su causa; un dato reservado existe, pero no se expone todavía. Un valor cero sólo se publica si tiene evidencia que lo sustenta. La falta de una medida opcional no equivale a un fallo del candidato.

La comparación prioriza las consecuencias de seguridad con criterios recibidos; el coste, los tokens o la velocidad no compensan un incumplimiento crítico. La escala de consecuencias no se improvisa ni se transforma en una nueva posición del polígono. Sin condiciones comparables se declara la limitación, sin fabricar un desempate.

## Reutilización e integración pendiente

El Gestor ya dispone de estructuras para consumo declarado, primer fragmento, duración del transporte, previsión económica, reloj de examen y observación local mediante OpenTelemetry. El observador del proceso propio tiene un intervalo objetivo de cinco segundos y un límite de registro de 32 MiB; esa cadencia no acredita la observación de un canal temporal de caché ni la de todos los procesos del equipo. Su comprobación de integridad se distingue del criterio de suficiencia del escenario.

La recepción de esta convocatoria requiere enlazar esos componentes con las veinticinco posiciones, las fases prácticas y los dos perfiles, conservando la autorización, los límites y la asociación de evidencias. Debe comprobarse la reacción ante pérdida del observador, retirada de permisos, vencimientos, reinicios y ausencia de métricas. No se heredan automáticamente los 1.920 segundos ni las pausas de la edición anterior. Rige el [plan temporal de esta convocatoria](TIEMPOS-Y-ENTREGAS.md).

Los puntos 3 y 3 bis permanecen abiertos: escenario físico de Q19, biblioteca técnica completa y recorrido operativo de investigación ordinaria y acelerada. No hay recepción completa de la instrumentación del examen por la mera publicación de este documento.

## Cuadernillos y reserva

El [catálogo público de cuadernillos](CUADERNILLOS.md) separa los materiales disponibles de los aún pendientes de recepción. Publicación, admisión documental y entrega al candidato son actos distintos. La salida a una interfaz de programación de aplicaciones (API), incluido el recuento remoto de tokens, conserva sus restricciones y autorización propia.

Las soluciones, programas de referencia, variantes reservadas y registros que permitan deducir respuestas permanecen privados hasta el cierre acordado y la revisión de difusión. Se aplica la misma separación a lectura web, descargas y datos estructurados. La revisión de los derechos y acuerdos de difusión precede a publicar resultados; ocultar el nombre del hardware no sustituye esa revisión.

Referencias: [evidencia y publicación](EVIDENCIAS-Y-PUBLICACION.md), [condiciones del soporte](RECEPCION-DEL-SOPORTE.md), [Validación y autorización de modelos](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/) y [Consecuencias de la omisión](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/CONSECUENCIAS-Y-FUENTES.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

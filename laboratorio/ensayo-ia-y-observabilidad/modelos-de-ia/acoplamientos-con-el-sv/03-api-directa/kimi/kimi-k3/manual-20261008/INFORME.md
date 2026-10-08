# Kimi K3 · Catálogo documental MD09

**Resultado final: Apto para el contrato documental.** 8 respuestas correctas de nueve, un error no crítico en MD04 y seis parámetros críticos correctos. No se ha iniciado CYB16.

## Contrato y resultado

Nodo 03: inferencia por API de Moonshot AI, modelo declarado kimi-k3. Fuente exclusiva: índice, presentación, constructor y tramo 10 del manual, suministrados íntegros por MCP Rust (cuatro documentos, 31 secciones). Mismas nueve preguntas del antecedente, criticidades fijadas antes del envío y tres etapas obligatorias: inicial, autocrítica y verificación final neutral. Configuración max y salida máxima 16.384 tokens; se declara esta diferencia respecto de high de antecedentes. No se presume igualdad de cómputo interno.

| Etapa | Correctas | Errores | U | Errores críticos | Admisión |
|---|---:|---:|---:|---:|---|
| R0 | 8 | 1 | 0 | 0 | Apto para el contrato documental |
| R1 | 8 | 1 | 0 | 0 | Apto para el contrato documental |
| R2 | 8 | 1 | 0 | 0 | Apto para el contrato documental |

R2 es la entrega final, sin selección retrospectiva. Vector: **(0,0,0,1,0,0,0,0,0)**. Célula (9,3), alfabeto {0,1,U}, T(9)=floor(7·9/9)=7. κ no sustituye el veto crítico: un 1 crítico elimina y una U crítica impide admisión. El polígono conserva 0 rojo/radio 1, 1 verde/radio 2 y U azul/radio 3; su pareja matemática y visual se puede consultar en [DICTAMEN](web/DICTAMEN.json).

## Discrepancia de MD04

La lista de doce campos y la distinción entre qué fija y qué no fija son correctas. El defecto se encuentra en la afirmación adicional de insuficiencia: incluye «Criterio de cierre» entre los campos cuyo contenido no estaría descrito más allá de su nombre. El apartado 5 del constructor sí contiene cinco mínimos explícitos. La fuente estaba completa en la solicitud y la afirmación persiste en R0, R1 y R2. Conforme al criterio previo, una afirmación errónea recibe 1; MD04 era auxiliar desde la admisión. No se modifica su criticidad ni se atribuye intención al modelo.

La coincidencia de citas no acredita por sí sola fidelidad de todas las afirmaciones. Esta discrepancia es sustantiva, no un defecto de literalidad ni una insuficiencia de suministro. Las otras delimitaciones correctamente documentadas se muestran como notas de alcance, no como alarmas de diseño. MD03 pide localizar materias sin inventar definiciones y MD09 pide delimitar lo implementado por el documento: ambos encargos son resolubles con las fuentes entregadas.

## Control e instrumentación

Se recibieron 27 entregas completas y 220 citas cotejadas, sin discrepancias literales. Rust verificó composición, fuentes, identidad de antecedentes, originales SSE, terminación, uso e integridad de las mediciones. La revisión del sentido es exterior al candidato y asistida por IA; el Árbitro aplica las reglas a esa revisión identificada, sin sustituirla por comparación de cadenas.

Uso comunicado: **666684 de entrada + 86491 de salida = 753175 tokens**. Caché, escritura de caché y razonamiento son subconjuntos, no sumandos adicionales. La ausencia de contadores se conserva expresamente; en 2 solicitudes no se comunicó caché. Contabilidad por solicitud, tarifas y conciliación permanecen en su archivo privado.

Instrumentación: 7050 muestras del proceso inscrito; intervalo máximo 444 ms. Se conservan CPU acumulada, memoria residente y virtual, E/S, conexiones TCP/UDP y escuchas por PID, tiempo hasta cabeceras, primer evento y primer texto, bytes, eventos y cadena SHA-256. [Mediciones](MEDICIONES.json) e [inventario de componentes Rust](INVENTARIO-RUST.json).

No se observan los procesos internos del proveedor, hilos/handles del sistema operativo, asignaciones del heap, tiempos DNS/TLS separados ni RTT/retransmisiones. No se afirma medición perfecta ni recepción integral independiente de plataforma. Las herramientas y navegación del candidato permanecieron deshabilitadas; esta comprobación del SV no equivale a inspeccionar la infraestructura remota.

La recepción inicial de MD01/R0 rechazó dos contadores idénticos pese a haber recibido una respuesta completa. Se corrigió el receptor y se reconstruyó la entrega original sin repetir inferencia. La recuperación y la pausa local no se atribuyen al proveedor. Los originales únicos permanecen conservados; no se declara publicada toda la secuencia bruta de solicitudes.

## Continuación y alcance

El examen CYB16 permanece preparado y sin iniciar. La comprobación previa no acredita capacidad diaria suficiente para completarlo con las tres etapas; se detienen las inferencias conforme a la indicación humana. El anexo operativo solicitado también está preparado en Rust y no se ha enviado. No se han enviado consultas al candidato para esas actuaciones pendientes.

La licencia acompaña los encargos. Existe una excepción expresa de entrenamiento limitada a los documentos públicos enviados en este ensayo; no se presenta como exclusión técnica del proveedor ni autoriza otro material. Se mantiene pendiente la excepción criptográfica C/ensamblador y la recepción científica independiente.

[Polígono egui autónomo](web/POLIGONO-EGUI.html) · [Comparación de etapas](COMPARACION-RUST.json) · [Criterios previos](CRITERIOS.md) · [Revisión exterior](REVISION-EXTERIOR.json) · [Código y reproducción](CODIGO-Y-REPRODUCCION.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

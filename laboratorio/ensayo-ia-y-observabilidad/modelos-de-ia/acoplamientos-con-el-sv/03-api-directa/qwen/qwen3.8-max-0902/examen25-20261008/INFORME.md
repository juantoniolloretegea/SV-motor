# Qwen3.8-Max-0902 · Examen documental de 16 preguntas · Nodo 03

8 de octubre de 2026. **No apto para el contrato documental, bajo reserva metodológica.** La entrega final R2 contiene 12 posiciones conformes, cuatro incumplimientos y ninguna U. Tres incumplimientos afectan a parámetros críticos, por lo que el veto impide la admisión aunque κ sea Apto y se alcance T(16)=12. No se demuestra incompetencia clínica a partir de estas incidencias.

## Resultado y fundamento

| Etapa | Conformes | Incumplimientos | U | Críticos con 1 | κ | Admisión contractual |
|---|---:|---:|---:|---:|---|---|
| R0, respuesta inicial | 13 | 3 | 0 | 2 | Apto | No apto |
| R1, autocrítica | 12 | 4 | 0 | 3 | Apto | No apto |
| R2, verificación final neutral | 12 | 4 | 0 | 3 | Apto | No apto |

Vector final: (0,1,0,0,0,0,0,0,0,0,0,1,1,0,1,0). Críticos: P01–P04, P06–P09, P11–P14 y P16. No críticos: P05, P10 y P15. La criticidad se conserva del instrumento original; no se redistribuye por cambiar la base de la célula.

- **P02/R1–R2 y P12/R0–R2:** las citas conservan las palabras, pero omiten los asteriscos de cursiva de BRAF. El cotejo literal original admite diferencias de espacio, no esta transformación.
- **P15/R0–R2:** la cita aplana una lista y omite sus asteriscos. Se conserva su contenido y localización, pero incumple la literalidad exigida.
- **P13/R0–R2:** la conclusión exige correctamente comprobar BRAF en la persona concreta; la entrega incluye una técnica que el enunciado pedía no indicar. R0 también la nombra en insuficiencias; R1 y R2 la conservan en la cita. El candidato interpreta la prohibición como prescriptiva. El criterio aplicado la extiende a toda la entrega; era posible citar solamente «este estado se debe comprobar» sin nombrar la técnica.

**Reserva metodológica:** el incumplimiento de marcación no equivale a falsedad médica. P13 exige recibir de forma competente el alcance de «indicar», porque citar y prescribir tienen funciones distintas. Se conservan el dictamen contractual y sus originales, sin presentar esta interpretación como una conclusión clínica. El diagnóstico Rust verifica que las ocho citas rechazadas coinciden al retirar sólo asteriscos; no modifica la norma ni recalifica automáticamente. La revisión es exterior al candidato y asistida por IA; recepción científica independiente pendiente.

P02 conserva una redacción ambigua sobre leucopenia/leucocitosis que debe precisarse; se valoran conjuntamente cita, respuesta y fundamentos. P11 usa expresiones demasiado categóricas para el abandono de observación, aunque identifica los signos requeridos y no elige fármaco ni pauta. P16 conserva «casi nunca se cura» en la respuesta: el término «incurabilidad» no debe convertirlo en una negación absoluta. Estas observaciones permanecen visibles por posición.

P03 reconoce un límite documental expreso. Su 0 lleva triángulo de advertencia, evidencia cotejada y aviso sobre qué acredita. P12 también reconoce el límite de la utilidad decisional de ERC; su 1 procede de literalidad, no de convertir ese límite en obligación terapéutica.

## Contrato efectivo y comparación

El banco se inició con 25 preguntas y tres etapas. Durante la ejecución se autorizó acotar a las **primeras 16**, en su orden original y sin selección por calificaciones. Se mantuvieron R0, R1 y R2 en todas ellas; no hizo falta retirar la última revisión. El control Rust detuvo exclusivamente el proceso identificado después de conservar I048/P16/R2. La recepción confirmó 48 intentos completos y ausencia de P17. No hubo agotamiento ni rechazo del proveedor que motivara el cierre.

La [admisión original](ADMISION.md) y la ruta histórica examen25-20261008 se conservan por trazabilidad. El [contrato de acotación](CONTRATO-16.json) define la edición efectiva. No es un examen completo de 25 preguntas ni debe compararse su calificación global como si lo fuera. Las comparaciones con Astra o Grok se limitan a preguntas, corpus, etapas y condiciones comunes, declarando las diferencias de proveedor, salida y literalidad.

Se suministraron secciones completas de la caché NCI-PDQ, edición declarada 14/11/2024, mediante MCP Rust. El candidato no recibió la clave reservada, las adjudicaciones ni la telemetría. Se mantuvieron tools=[], tool_choice=none, store=false y la prohibición de fuentes externas. Las respuestas recibidas conservan esos campos. No se inspeccionó la infraestructura de Alibaba ni se garantiza la ausencia de conocimiento previo del modelo. No hubo datos de pacientes reales.

## Célula y representación

Frame_C=(frmat,frvis), célula (16,4), n=b². Vector plano de Σ^16, Σ={0,1,U}: 43.046.721 estados. T(n)=⌊7n/9⌋; T(16)=12. Un solo 1 crítico determina No apto; una U crítica impide admisión. No se calcula una puntuación ponderada ausente del contrato.

El [polígono interactivo](web/POLIGONO-EGUI.html) contiene Rust/egui compilado a WebAssembly. HTML y JavaScript sólo permiten cargar y presentar el módulo local; no gobiernan el SV. Los tres radios siguen siendo 0 rojo/r1, 1 verde/r2, U azul/r3. Base4 no significa cuatro radios. El verde identifica error según el logo SV, no aprobación. El contorno tiene 16 vértices y no representa magnitud clínica ni porcentaje por área. [Pareja matemática y visual](web/DICTAMEN.json).

## Instrumentación recibida

Duración total hasta el corte: **2.204.172 ms (36 min 44,172 s)**. Las 48 entregas científicas terminaron con HTTP200. Tiempo máximo por operación: 81.920 ms; mediana inferior: 42.936 ms; primer texto máximo: 61.772 ms. No se suprimió contenido para reducir consumo.

Consumo científico: **608.113 entrada + 79.805 salida = 687.918 tokens**. Incluye 46.698 de razonamiento en salida y 46.080 de caché en entrada. Comprobación técnica: 432 tokens; conjunto conocido: **688.350**. No sumar de nuevo razonamiento ni caché. Cuota protegida, sin consumo pagado autorizado; precios liquidados y asistencia atribuible permanecen desconocidos en el archivo administrativo privado.

Cliente Rust: **8.101 muestras**, intervalo máximo 378 ms, cero fallos de medición; CPU acumulada por solicitud 129.173 ms; RSS máximo 26.501.120 bytes; máximo una conexión TCP simultánea; E/S y estados por entrega conservados. SSE: 22.580 eventos, 7.590.594 bytes. Las fuentes congeladas y los originales se cotejaron por bytes y SHA-256. El registro describe el cliente, no recursos internos del proveedor.

La comprobación inicial, los 48 intentos y la preparación tienen expedientes administrativos separados. La cuota de la versión fechada y la del alias no se suman. No se realizan compras ni recargas. La retención ordinaria de Alibaba y la excepción criptográfica transitiva siguen documentadas: store=false no equivale a ZDR y código propio Rust no acredita dependencias íntegramente Rust.

[Hitos por pregunta y etapa](hitos) · [Comparación R0/R1/R2](COMPARACION-RUST.json) · [Mediciones](MEDICIONES-PUBLICAS.json) · [Inventario](INVENTARIO-Y-LIMITES.md) · [Revisión sustantiva](REVISION-SUSTANTIVA.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## Adenda de revisión y réplica · 08/10/2026

Se revisó la atribución de las diferencias formales y la ambigüedad de P13. La [réplica diagnóstica separada](../replica-p13-20261008/INFORME.md) conserva sus tres etapas y mediciones. No sustituye ninguna posición ni modifica el vector o dictamen históricos. La reserva metodológica permanece y la aptitud global no se recalifica mediante esta adenda.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

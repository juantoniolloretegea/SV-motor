# Lección Watson y garantías del Sistema Vectorial SV
## Análisis forense de fallos documentados de IA en medicina y respuesta arquitectónica del SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ITVIA — IA eñ™ | ISSN: 2695-6411**  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 4 de abril de 2026  
**Sede documental:** SV-motor-main/docs/arquitectura/

---

## Propósito de este documento

Este documento registra formalmente los fallos documentados de sistemas de inteligencia artificial aplicados a decisiones clínicas de alto riesgo, con sus fuentes primarias verificadas, el mecanismo causal de cada fallo, y la respuesta arquitectónica que el Sistema Vectorial SV implementa en respuesta a cada uno. No es un ejercicio de marketing comparativo — es el registro de las razones por las que el SV fue diseñado de la manera en que fue diseñado.

Todo fallo documentado aquí tiene fuente primaria verificable. No se incluyen afirmaciones sin respaldo bibliográfico directo.

---

## Sección 1 — IBM Watson for Oncology / Memorial Sloan Kettering Cancer Center

### 1.1 Descripción del caso

IBM Watson for Oncology fue un sistema de inteligencia artificial desarrollado en colaboración con Memorial Sloan Kettering Cancer Center (MSKCC) en Nueva York, diseñado para proporcionar recomendaciones de tratamiento oncológico a partir del análisis de historias clínicas de pacientes con cáncer.

El caso más citado y documentado en la literatura científica es el de un paciente con cáncer de pulmón y riesgo alto de sangrado severo al que el sistema recomendó bevacizumab (un agente anti-VEGF). Bevacizumab tiene contraindicación establecida en presencia de riesgo alto de hemoptisis o hemorragia en pacientes con cáncer de pulmón, recogida en las guías NCCN y en la ficha técnica del fármaco aprobada por la FDA. La recomendación no fue seguida clínicamente por el equipo médico que identificó la contraindicación. No se documentó daño directo al paciente en este caso específico porque la recomendación fue rechazada antes de ejecutarse.

### 1.2 Consecuencias sistémicas documentadas

> Ross C, Swetlitz I. IBM pitched its Watson supercomputer as a revolution in cancer care. It's nowhere close. STAT News. 5 septiembre 2017. [statnews.com/2017/09/05/watson-ibm-cancer]

> Schmidt C. MD Anderson Breaks With IBM Watson, Raising Questions About Artificial Intelligence in Oncology. J Natl Cancer Inst. 2017;109(5):djx113. DOI:10.1093/jnci/djx113

> Strickland E. How IBM Watson Overpromised and Underdelivered on AI Health Care. IEEE Spectrum. 2019;56(4):24–31. DOI:10.1109/MSPEC.2019.8910661

Cinco años después del inicio del proyecto y 62 millones de dólares de inversión, MD Anderson Cancer Center no desplegó el sistema en pacientes reales. La auditoría universitaria (University of Texas System, noviembre 2016) identificó incumplimientos de gestión de proyecto, sobrecostes y retrasos. Más de una docena de hospitales que habían contratado Watson for Oncology cancelaron o redujeron sus proyectos para 2018.

### 1.3 Mecanismo causal documentado

El sistema fue entrenado con un número reducido de casos sintéticos (hipotéticos) elaborados por un grupo restringido de oncólogos del MSKCC. No se entrenó con datos de pacientes reales ni incorporó las guías clínicas actualizadas de forma sistemática. La consecuencia fue que el sistema no representaba las combinaciones clínicas de alto riesgo con la densidad necesaria para aprender sus contraindicaciones, y producía recomendaciones con aparente confianza uniforme independientemente de la calidad o representatividad de los datos subyacentes.

La investigación de STAT News de 2017 accedió a documentos internos de IBM que confirmaban recomendaciones "inseguras e incorrectas".

### 1.4 Respuesta del SV — siete inversiones estructurales

**Fallo 1 → Inversión SV-W1:** Watson recomendaba tratamientos y actuaba como autoridad clínica. El SV nunca emite recomendaciones terapéuticas. κ₃ ∈ {APTO, NO_APTO, INDETERMINADO} es una clasificación del vector de parámetros del dominio, no una prescripción. El ACTOR-TRANSLATE traduce ese resultado al idioma clínico sin añadir autoridad decisional. Esta limitación funcional está declarada explícitamente en el encabezado del `normative_engine.py` de cada módulo: "Este motor es una PRÓTESIS COGNITIVA auxiliar... NO sustituye el criterio clínico del especialista ni constituye una recomendación terapéutica."

**Fallo 2 → Inversión SV-W2:** Watson era una caja negra estadística sin trazabilidad. El SV produce κ₃ mediante funciones deterministas `threshold()`, `gamma_bar_h()`, `gate_vector()`, `kappa3()` — públicas, reproducibles, auditables. Cualquier auditor que ejecute la misma entrada obtiene el mismo resultado. No hay comportamiento emergente. No hay pesos de red que determinen la clasificación principal.

**Fallo 3 → Inversión SV-W3:** Watson fue entrenado con datos de una cohorte específica (MSKCC) sin representatividad poblacional. Las reglas del motor normativo del SV se derivan de guías clínicas publicadas por organismos internacionales (ECIL, ESCMID, IDSA, EULAR). No se aprenden de datos de pacientes. Cuando las guías se actualizan, el dominio se actualiza con nueva versión documentada y trazabilidad completa.

**Fallo 4 → Inversión SV-W4:** Watson no tenía mecanismo de incertidumbre honesta; producía recomendaciones incluso con datos ambiguos. El SV tiene U estructural algebraicamente forzada: cuando los datos son insuficientes, el resultado es U — no un valor imputado por defecto. La regla dura del motor: "Si un campo clínico está ausente (None), el parámetro se marca U. Nunca se imputa 0 por defecto ante ausencia de datos."

**Fallo 5 → Inversión SV-W5:** Watson intentaba sustituir al médico. La soberanía clínica en el SV es estructural, no declarativa. No existe campo de override para los invariantes algebraicos desde ningún nivel de privilegio. El médico puede ignorar el resultado del sistema sin consecuencias para el funcionamiento del sistema. Lo que no puede ignorar es que la trayectoria registra lo que ocurrió.

**Fallo 6 → Inversión SV-W6:** Watson extraía datos de notas clínicas desestructuradas (NLP sobre historia clínica libre), con resultados variables y no reproducibles. El SV recibe observables declarados explícitamente por el clínico en el formato del dominio. No procesa narrativa clínica libre. No infiere diagnósticos de texto.

**Fallo 7 → Inversión SV-W7:** la Regla Watson-SV de presentación, obligatoria en toda interfaz: ninguna salida del sistema, sea cual sea el κ₃ global, puede presentarse al usuario sin exponer explícitamente cada posición con valor 1 y su lectura clínica antes que el κ₃ global. Esta regla responde al mecanismo exacto del caso bevacizumab: una clasificación global aparentemente favorable que suprimiría información crítica de posición individual.

---

## Sección 2 — Epic Sepsis Model: fallo de validación externa con adopción masiva

### 2.1 Descripción del caso

El Epic Sepsis Model (ESM) es un modelo de predicción de sepsis propietario integrado en el sistema de historia clínica electrónica Epic Systems, utilizado por aproximadamente el 56% de hospitales y sistemas sanitarios de Estados Unidos según los datos del estudio de validación.

### 2.2 Hallazgos documentados

> Wong A, Otles E, Donnelly JP, et al. External Validation of a Widely Implemented Proprietary Sepsis Prediction Model in Hospitalized Patients. JAMA Intern Med. 2021;181(8):1065–1070. DOI:10.1001/jamainternmed.2021.2626

> Habib AR, Lin AL, Grant RW. The Epic Sepsis Model Falls Short — The Importance of External Validation. JAMA Intern Med. 2021;181(8):1040–1041. DOI:10.1001/jamainternmed.2021.3333

Estudio retrospectivo de cohorte sobre 27.697 pacientes (38.455 hospitalizaciones) en Michigan Medicine, 2018–2019. Resultados:
- AUC hospitalización: 0,63 (IC 95%: 0,62–0,64) — versus 0,76–0,83 declarado por Epic Systems
- Sensibilidad: 33%
- Especificidad: 83%  
- Valor predictivo positivo (PPV): 12%
- El 18% de todas las hospitalizaciones generó una alerta del modelo
- Por cada 8 pacientes con alerta activa, solo 1 tenía sepsis en curso

### 2.3 Mecanismo causal

El ESM predice costes de atención sanitaria futuros como proxy de la necesidad de salud, no la fisiopatología de la sepsis. El umbral de alerta y la definición de sepsis utilizados en el desarrollo interno difieren de los utilizados en la validación externa. El modelo fue adoptado masivamente sin validación externa independiente previa al despliegue.

### 2.4 Respuesta del SV

Las reglas del motor normativo del SV son particiones deterministas de observables clínicos directos (ANC medido, IgG dosificada, estado vacunal documentado), no modelos predictivos de costes futuros. No hay proxy entre el observable y el parámetro — el Ternarizer aplica una partición sobre el observable medido, declarada explícitamente con justificación en la guía clínica de referencia. La ausencia de validación externa del ESM antes del despliegue masivo es exactamente el tipo de fallo que la suite de conformidad cruzada del SV (Niveles A–D de la FRONTERA_NORMATIVA) pretende prevenir para el núcleo algebraico.

---

## Sección 3 — Sesgo racial sistemático en algoritmo de gestión de riesgo en salud

### 3.1 Descripción del caso

Un algoritmo comercial ampliamente utilizado en el sistema de salud de los Estados Unidos para identificar pacientes candidatos a programas de gestión de riesgo de alta complejidad exhibía sesgo racial significativo que afectaba a millones de pacientes.

### 3.2 Hallazgos documentados

> Obermeyer Z, Powers B, Vogeli C, Mullainathan S. Dissecting racial bias in an algorithm used to manage the health of populations. Science. 2019;366(6464):447–453. DOI:10.1126/science.aax2342

- A igual puntuación de riesgo del algoritmo, los pacientes negros presentaban una media del 26% más de condiciones crónicas que los pacientes blancos
- La proporción de pacientes negros identificados automáticamente para el programa de gestión intensiva era del 17,7%; sin el sesgo, habría sido del 46,5%
- El fabricante del algoritmo confirmó independientemente los hallazgos de los investigadores

### 3.3 Mecanismo causal

El algoritmo predice costes de atención sanitaria en lugar de necesidades de salud. El acceso desigual a la atención médica implica que se gasta sistemáticamente menos dinero en atender a pacientes negros con el mismo nivel de necesidad clínica que pacientes blancos. El algoritmo aprendió ese patrón y lo perpetuó como si fuera información válida sobre el estado de salud.

### 3.4 Respuesta del SV

Los observables del SV son magnitudes clínicas directamente medibles: ANC en células/µL, IgG en mg/dL, CD4 en células/µL, secuencia vacunal documentada. Ningún parámetro utiliza el coste de atención previa, la frecuencia de utilización de servicios, ni ningún proxy socioeconómico o demográfico como indicador de estado clínico. La partición B₀/B₁/B_U de cada Ternarizer es una partición del espacio del observable clínico declarado, no del espacio de variables de coste.

---

## Sección 4 — Modelos de predicción COVID-19: validación insuficiente generalizada

### 4.1 Hallazgos documentados

> Wynants L, Van Calster B, Collins GS, et al. Prediction models for diagnosis and prognosis of covid-19: systematic review and critical appraisal. BMJ. 2020;369:m1328. DOI:10.1136/bmj.m1328

Revisión sistemática de 232 modelos de predicción de diagnóstico y pronóstico para COVID-19:
- Todos los modelos presentaron alto riesgo de sesgo en al menos un dominio metodológico
- Los defectos más frecuentes: entrenamiento con muestras no representativas, ausencia de validación externa, sobreajuste no reportado, ausencia de calibración
- Conclusión de los autores: ningún modelo estaba listo para uso clínico inmediato sin validación adicional

### 4.2 Respuesta del SV

El corpus de casos canónicos de SVcustos y SVperitus es la especificación formal del comportamiento correcto del motor algebraico, no un conjunto de entrenamiento estadístico. La separación estructural entre las capas algebraica (determinista, validable por casos canónicos) y estadística (CNN, NLP — con sus Ternarizadores declarados) garantiza que el motor algebraico no puede degradarse por distribution shift.

---

## Sección 5 — Sesgo de infradiagnóstico en radiología IA para poblaciones vulnerables

### 5.1 Hallazgos documentados

> Seyyed-Kalantari L, Zhang H, McDermott MBA, Chen IY, Ghassemi M. Underdiagnosis bias of artificial intelligence algorithms applied to chest radiographs in under-served patient populations. Nat Med. 2021;27:2176–2182. DOI:10.1038/s41591-021-01595-0

Modelos de IA para diagnóstico por imagen en radiografía de tórax presentaron rendimiento inferior en pacientes de poblaciones desatendidas (menor nivel socioeconómico, minorías raciales, sin seguro médico) respecto a las poblaciones mejor representadas en los datos de entrenamiento. El sesgo operaba de forma que los falsos negativos (diagnósticos perdidos) eran significativamente más frecuentes precisamente en las poblaciones con menos acceso a seguimiento clínico de rutina.

### 5.2 Respuesta del SV

El ACTOR-CNN-A (ConvNeXt-Tiny, Uso A) clasifica polígonos polares generados sintéticamente a partir del vector ternario. Los polígonos sintéticos no contienen información demográfica del paciente ni características de imagen médica real. El sesgo por subrepresentación en datos de entrenamiento que documenta Seyyed-Kalantari et al. es estructuralmente imposible en Uso A porque el espacio de entrenamiento es algebraicamente generado y demográficamente neutro.

Para Uso B (imagen médica real, Hito 5 — no implementado), este riesgo sería relevante y deberá abordarse con un corpus de entrenamiento que incluya explícitamente diversidad demográfica documentada. Esta es la razón por la que Uso B está registrado como deuda abierta (DV-HITO5-003, DV-HITO5-004) y no se implementará sin ese requisito satisfecho.

---

## Sección 6 — Interpretabilidad en decisiones de alto riesgo

### 6.1 Argumento documentado

> Rudin C. Stop explaining black box machine learning models for high stakes decisions and use interpretable models instead. Nat Mach Intell. 2019;1:206–215. DOI:10.1038/s42256-019-0048-x

Rudin argumenta que en contextos de decisión de alto riesgo (sanidad, justicia), intentar explicar modelos de caja negra es conceptualmente inferior a diseñar modelos inherentemente interpretables desde el principio. Los métodos de explicabilidad post-hoc (LIME, SHAP, saliency maps) no garantizan que la explicación corresponda al razonamiento real del modelo.

### 6.2 Respuesta del SV

El motor algebraico del SV es inherentemente interpretable en el sentido de Rudin: κ₃ se produce mediante una secuencia de funciones deterministas documentadas cuya ejecución es reproducible y cuyo resultado es auditablemente derivable de la entrada. No hay componente oculto. No hay explicabilidad post-hoc porque el razonamiento es el modelo. La doble vara Python↔Rust garantiza reproducibilidad entre implementaciones independientes.

---

## Sección 7 — Consecuencias no previstas de ML en medicina

### 7.1 Argumento documentado

> Cabitza F, Rasoini R, Gensini GF. Unintended Consequences of Machine Learning in Medicine. JAMA. 2017;318(6):517–518. DOI:10.1001/jama.2017.7797

Los autores documentan que sistemas de ayuda a la decisión basados en ML pueden producir reducción de la calidad diagnóstica en médicos expertos mediante anclaje (anchoring bias) — la tendencia del experto a subordinar su propio juicio al del sistema, incluso cuando el juicio independiente sería superior.

### 7.2 Respuesta del SV

El SV no produce recomendaciones que el médico pueda anclar. Produce una geometría (polígono) que requiere interpretación activa y un vector de posiciones que el médico ve antes de cualquier clasificación global. La Regla Watson-SV de presentación garantiza que las posiciones de riesgo individual (valor 1) son visibles antes que el κ₃ global. El sistema de override con texto mínimo (30 caracteres) rompe el automatismo del clic sin impedir la decisión.

> Parasuraman R, Manzey DH. Complacency and bias in human use of automation: an attentional integration. Hum Factors. 2010;52(3):381–410. DOI:10.1177/0018720810376055

---

## Tabla de correspondencia: fallos documentados → garantías SV

| # | Fallo | Fuente (DOI) | Mecanismo | Garantía SV | Componente |
|---|---|---|---|---|---|
| W1 | Recomendación directa de tratamiento | 10.1093/jnci/djx113 | Autoridad clínica usurpada | Nunca recomienda; κ₃ es clasificación del vector | normative_engine, ACTOR-TRANSLATE |
| W2 | Caja negra no auditable | 10.1109/MSPEC.2019.8910661 | Trazabilidad imposible | Motor determinista, doble vara Python↔Rust | core.py, comparator.py |
| W3 | Entrenamiento en cohorte sesgada | 10.1093/jnci/djx113 | Distribution shift | Reglas desde guías publicadas, sin datos de pacientes | normative_engine P01–P25 |
| W4 | Sin mecanismo de incertidumbre | 10.1109/MSPEC.2019.8910661 | Certeza fabricada | U estructural algebraicamente forzada | core.py, FRONTERA B.6 |
| W5 | Sustitución del médico | 10.1001/jama.2017.7797 | Automation bias | Soberanía clínica estructural; trayectoria irrevocable | arquitectura de privilegios |
| E1 | Distribution shift (ESM) | 10.1001/jamainternmed.2021.2626 | Proxy vs observable directo | Observables clínicos directos, sin proxy de costes | Ternarizer B₀/B₁/B_U |
| E2 | Alert fatigue (ESM) | 10.1001/jamainternmed.2021.2626 | PPV 12%, 18% alertas | Sin alertas automáticas; visualización activa | polígono polar |
| O1 | Sesgo racial por proxy | 10.1126/science.aax2342 | Coste como proxy de salud | Sin variables de coste ni demográficas en ningún Ternarizer | dominio declarado |
| R1 | 232 modelos sin validación externa | 10.1136/bmj.m1328 | Sobreajuste sin detección | Suite de conformidad cruzada niveles A–D | FRONTERA_NORMATIVA |
| S1 | Infradiagnóstico en poblaciones vulnerables | 10.1038/s41591-021-01595-0 | Subrepresentación en training | Uso A: polígonos sintéticos sin sesgo demográfico | SVcustos, ACTOR-CNN-A |
| I1 | Modelos no interpretables | 10.1038/s42256-019-0048-x | Explicabilidad post-hoc insuficiente | Modelo inherentemente interpretable | motor algebraico |

---

*Documento de arquitectura del frente motor IA — no modifica doctrina algebraica del corpus SV.*  
*Juan Antonio Lloret Egea | ORCID: 0000-0002-6634-3351 | ITVIA — IA eñ™ | ISSN: 2695-6411 | CC BY-NC-ND 4.0*

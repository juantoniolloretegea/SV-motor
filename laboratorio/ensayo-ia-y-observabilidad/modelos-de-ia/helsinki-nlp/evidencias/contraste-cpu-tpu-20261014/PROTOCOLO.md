# Contraste del traductor OPUS-MT y selección CPU/TPU

## Objetivo y realización

Comprobar la ejecución del modelo completo Helsinki-NLP/opus-mt-en-es, revisión fdaddf76f50fcc1583ba42f95965862a7ab30f97, mediante el Selector del Árbitro - Director CPU/GPU/TPU/NPU. Se conserva la referencia CPU del servicio documental: Candle 0.11.0, F32, tokenización existente y generación voraz. No se modifica el servicio ni el núcleo del SV.

La realización experimental expresa en StableHLO las operaciones de Marian descritas por Candle. Rust construye los programas, verifica las entradas, controla la generación, selecciona el dispositivo y conserva los resultados. PJRT y libtpu realizan compilación y ejecución TPU; esta última es una dependencia nativa de Google, no una biblioteca íntegramente escrita en Rust.

Se calcula el prefijo completo del decodificador, con máscara causal y capacidad fija de 64 posiciones. Es una diferencia explícita respecto de la caché incremental de Candle. Si una traducción no termina dentro de ese límite, se rechaza el caso como incompleto; no se trunca ni se amplía automáticamente. El alcance es este perfil experimental, no toda la capacidad del servicio documental.

## Casos fijados antes de ejecutar

Se utilizan NCI_4-0 y NCI_178-3 de la colección anterior de trece fragmentos. Sus originales y traducciones previas se conservan. No se modifica el contenido, no se optimizan indicaciones y no se evalúa nuevamente la calidad clínica del modelo.

## Comprobación numérica previa

Antes de solicitar TPU se compara la interpretación CPU del programa experimental con Candle original: codificador completo y todos los vectores de puntuación previos a la selección de cada token. Se exige igualdad de tokens y traducción final, ausencia de valores no finitos, diferencia absoluta máxima de 0,005 y error cuadrático medio normalizado de 0,0001 como máximos. La normalización divide por el mayor entre 1 y la raíz de la media de cuadrados de la referencia. Estos límites se mantienen para el cotejo TPU; un incumplimiento impide declarar conformidad y registrar una capacidad comprobada.

Las multiplicaciones StableHLO se solicitan con precisión HIGHEST. No se presume identidad binaria entre reducciones F32 realizadas en distintos dispositivos. Se conservan los vectores originales y las métricas de cada paso, incluidos los intentos fallidos.

## Selección y observación

Sólo después del cotejo se registran capacidades del perfil exacto. Se prevén tres mediciones completas por destino y caso, una solicitud obligatoria CPU, una obligatoria TPU y una selección por medianas observadas. La medición comprende tokenización, inferencia, transferencias y recuperación de la traducción; carga y compilación iniciales se presentan por separado. No se permite sustitución silenciosa del dispositivo.

Se conserva identidad del equipo y ejecutable, programas y pesos, decisión del Selector, salida completa, actividad CPU del proceso, tiempos de las fases, transferencias solicitadas y trazas originales de TPU. Los bytes solicitados no se presentan como tráfico físico medido. La ausencia de energía o contadores físicos de bus se declara. La conformidad exige atribución de cálculo TPU en la traza, además de la etiqueta del dispositivo.

## Conservación y cierre

Los resultados nuevos se separan de los antecedentes. Se recuperan y cotejan antes de retirar el recurso temporal. El expediente del modelo conservará código, protocolo, resultados y limitaciones. No se modifica el libro. Esta prueba no habilita el nodo 1 ni acredita aptitud general de traducción.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

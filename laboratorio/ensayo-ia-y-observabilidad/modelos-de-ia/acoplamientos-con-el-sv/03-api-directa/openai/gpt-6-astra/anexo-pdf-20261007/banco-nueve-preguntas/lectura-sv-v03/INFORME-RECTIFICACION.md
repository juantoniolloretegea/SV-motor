# Lectura SV del anexo PDF: dictamen, criticidad y pareja matemática/visual

7 de octubre de 2026 · GPT-6 Astra · Nodo 03 · S39 / TT-0021 · Visor 0.3.0.

La versión anterior mostraba el reparto de valores, pero omitía la clasificación matemática, el estado de admisión, el inventario de criticidades y la correspondencia formal con la imagen. También aplicaba una convención cromática propia. Esta rectificación incorpora esas distinciones conservando las nueve adjudicaciones y todas las respuestas originales.

## Resultado que debe leer el profesional

| Magnitud o condición | Resultado acreditado |
|---|---|
| Vector, en orden PDF01–PDF09 | (0, 0, 0, 0, 0, 0, 0, 1, 0) |
| Tamaño | b=3; n=b²=9; alfabeto {0,1,U} |
| Umbral primitivo | T(9)=parte entera inferior de 63/9=7 |
| Recuentos | N0=8; N1=1; NU=0; cobertura 9/9 |
| Clasificación matemática κ | **Apto**, porque N0=8≥7 |
| Condición eliminatoria | **Un solo 1 crítico determina No apto, aunque κ sea Apto o la puntuación sea alta** |
| Criticidades de este banco | **No fijadas antes de la ejecución**; no equivalen a nueve parámetros no críticos |
| Número de errores críticos / no críticos | No determinado; no se consigna cero |
| Puntuación auxiliar sobre 100 | No emitida: falta conocer Eₙ para aplicar 100×(A−Eₙ)/N |
| Admisión del candidato | **No acreditada**; no hay fundamento para cerrar la condición de criticidad |
| Incidencia PDF08 | Entrega JSON inválida y evidencias incompletas; explicación sustantiva concordante, sin contradicción médica demostrada |
| Alcance | Fidelidad a un documento de 2018; no habilitación clínica, estabilidad ni recepción independiente |

«No acreditada» describe el estado del expediente; no introduce un cuarto símbolo en la terna ni sustituye la clasificación κ. Tampoco equivale a haber demostrado un error crítico. La omisión de criticidades pertenece a la preparación del ensayo, no se imputa al candidato. No resulta legítimo asignarlas ahora en función de las respuestas obtenidas. Una nueva ejecución que aspire a dictamen completo necesita una constitución previa competente de esos parámetros.

El documento de admisión anterior excluyó conjuntamente κ y los criterios específicos de A0. Esa exclusión fue demasiado amplia: no corresponde heredar las seis criticidades ni la puntuación de A0, pero sí puede calcularse la regla matemática primitiva sobre una terna válida y completa. Aquí se incorpora **como lectura posterior identificada**, sin presentarla como criterio prefijado del ensayo ni reescribir su acta original. La rectificación no convierte este anexo en un examen equivalente a otros bancos.

## Pareja vinculada y colores

Se representa **Frame_C=(frmat,frvis)**, con C=`SV-ASTRA-PDF-LECTURA-0.3.0`. Esta es una realización experimental de la correspondencia documentada, no la declaración de un tipo nuevo en la IR del Lenguaje.

`frmat` conserva el vector plano, su orden, su alfabeto, el vínculo de cada posición con una pregunta y la huella de la respuesta. Hay 3⁹=19.683 estados posibles. La célula no se convierte en matriz ni en espacio vectorial algebraico.

`frvis` conserva las nueve posiciones y su cierre: θᵢ=2π(i−1)/9 y Vᵢ=(ρ(vᵢ)cosθᵢ,ρ(vᵢ)senθᵢ). La vista utiliza la convención del **logo SV**, documentada en la adenda de encaje visual §5.1:

| Símbolo | Radio visible | Color del círculo y del vértice | Significado en este anexo |
|---|---:|---|---|
| 0 | 1 | Rojo | Respuesta correcta y completa |
| 1 | 2 | Verde | Error adjudicado, con su naturaleza especificada |
| U | 3 | Azul | Indeterminación sustantiva evaluable |

El verde de esta convención **no significa aprobación**. La adenda distingue el logo de la convención histórica IMMUNO-1 (0 verde, 1 rojo, U amarillo). No procede mezclar sus leyendas ni modificar sus imágenes históricas. Los nombres de color proceden de esa fuente; los tonos RGB (181,42,45), (21,119,80) y (36,87,181) son una elección de realización, no una prescripción RGB atribuida a los pilares.

En coordenadas matemáticas, V1 está sobre +x y el recorrido es antihorario. En pantalla se aplica (xₚ,yₚ)=(cₓ+s·y,cᵧ−s·x), s>0: PDF01 arriba y sentido horario. Esta transformación queda declarada y conserva posición y radio. Las líneas radiales grises sitúan parámetros; el contorno une vértices. Ni radios, área ni colores expresan probabilidades clínicas o puntuaciones.

## Realización y comprobaciones

`derivar.rs` produce en Rust un `DICTAMEN.json` independiente desde CAPA y el banco fijados por SHA-256. No modifica las adjudicaciones ni consulta al candidato. El visor coteja íntegramente ese documento antes de representar; rechaza cambios de orden, valor, radio, color o criticidad, aunque conserven los recuentos.

Se conservan diez comprobaciones Rust favorables: tamaños y umbrales; 19.683 estados de tamaño nueve; veto por un error crítico en cualquiera de las nueve posiciones aunque κ sea Apto; rechazo de vectores incompletos; identidad y correspondencia de la pareja; radios y orden; colores de las tres circunferencias y los vértices; polígono cerrado; interacción sobre los nueve vértices y botones sin mutar la fuente; y texto del fundamento dentro de anchos de 760 y 1.280 píxeles. `PRUEBAS-RUST.txt` conserva el resultado.

Se comprobó además el HTML autónomo en el navegador: selección PDF08, lectura de su fundamento íntegro, representación cromática, pareja matemática/visual, inventario, apertura de fuentes y licencia. `COTEJO-EGUI.json` relaciona estas observaciones y sus capturas locales con el SHA-256 del HTML. Las comprobaciones de representación no se presentan como auditoría clínica independiente ni calibración de la instrumentación del ensayo.

El HTML incluye Rust compilado a WebAssembly y el enlace JavaScript necesario para iniciarlo; no realiza inferencia y mantiene `connect-src 'none'`. El servidor de vista sólo sirve este archivo en la interfaz local. La adjudicación original sigue procediendo del expediente del ensayo bajo el Árbitro-Director; el candidato no interviene en este cálculo ni en su comprobación.

## Conservación y continuación

CAPA original: SHA-256 `7338a7f7c2a06111421cefeffb4cfd03811e11784180866f8a500d3acbd3b589`. Se mantienen los valores, originales, suministro, clave, medidas y criterio de evaluación de PDF01–PDF09. No hubo nuevas inferencias ni reparación de PDF08. El visor actual se sustituye por 0.3.0; la versión anterior permanece recuperable en su revisión publicada. La lectura derivada se añade en `lectura-sv-v03/`.

La [adenda de criticidad y abstención](ADENDA-CRITICIDAD-Y-ABSTENCION.md) fija la obligación de permitir U justificada en el siguiente encargo. Está preparada, **no enviada al modelo**; no modifica retrospectivamente lo recibido. Antes de una nueva ejecución deberán quedar constituidas y verificadas las criticidades y el contrato completo. TT-0021 conserva su estado pendiente; no se inicia otra fase mediante esta rectificación.

Las referencias inmutables de fundamentos, pilares, puntuación, encaje visual y paridad figuran en `REFERENCIAS.json` y en el visor. El registro económico separa cero llamadas al candidato del consumo de asistencia, que carece de desglose atribuible.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

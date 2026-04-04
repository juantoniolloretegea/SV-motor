# Hito 5 — Arquitectura multi-estrato del motor SV: sucesos bilaterales, resolución molecular y condiciones de gobernabilidad biomédica

**Tipo:** Documento de arquitectura del frente motor IA — apertura del Hito 5  
**Posición en el repositorio:** `docs/arquitectura/09_hito5_arquitectura_multiestrato.md`  
**Autor del corpus:** Juan Antonio Lloret Egea | ORCID: 0000-0002-6634-3351 | ITVIA — IA eñ™ | ISSN 2695-6411  
**Fecha:** 3 de abril de 2026  
**Versión del motor en el momento de la consolidación:** v0.1.8  
**Álgebra de referencia:** Corpus SV publicado — Colección I (Documentos 1–3); Álgebra de composición intercelular (Documentos I–V); Invariantes, dominios y agentes (Documento V)

---

## Nota de posición doctrinal

Este documento es un documento de arquitectura del frente motor IA. No es un documento del corpus SV. No introduce definiciones formales nuevas al SV. No declara dominios especializados con firma 𝔇 cerrada. No modifica gramática superficial, IR, validator, lowering ni backend soberano del Lenguaje SV.

Todo lo que este documento propone como patrón de diseño está sujeto a validación por el corpus SV y por los documentos superiores correspondientes. Toda consecuencia sobre el Lenguaje SV que exija modificar alguna de las piezas mencionadas debe detenerse aquí y elevarse al carril correspondiente.

Este documento posiciona el problema, declara el contexto algebraico existente en el corpus, describe patrones de diseño que el motor puede adoptar dentro de su alcance declarado, y registra las deudas que el corpus y el motor deben cerrar para materializar la arquitectura propuesta.


---

## Cláusula operativa de contención

El presente Hito 5 autoriza **ejecución completa y prueba amplia en laboratorios del frente motor IA**, incluyendo exploración, comparación, lotes adversariales, escenarios multi-estrato y contrastes internos del carril técnico.

Esa autorización **no habilita producción**. Mientras el bloque documental de **nueve documentos** no haya cerrado de forma suficiente el álgebra y la semántica que impidan desviaciones, doblegamiento del SV o entrada de basura por la puerta trasera, el frente motor queda en **régimen exclusivo de laboratorio**.

Por tanto:

- no procede despliegue en producción;
- no procede backend soberano de uso real para dominios biomédicos;
- no procede presentar el motor como agente clínico operativo;
- y no procede relajar la custodia estructural por éxito local de laboratorio.

---

## 1. Objeto y tesis

### 1.1 Objeto

Este documento declara las condiciones que el motor SV debe satisfacer para servir a agentes especializados en dominios biomédicos — inmunología, farmacología, oncología, biología molecular — con la misma garantía de determinismo, trazabilidad y gobernabilidad que ya satisface para los dominios implementados (NLP, DEV, CUSTODIA).

El argumento es que el motor SV ya posee la infraestructura algebraica necesaria. Lo que falta es la declaración explícita de tres patrones de diseño que el corpus habrá de formalizar y que el motor puede anticipar como hipótesis de arquitectura:

1. La **estratificación por nivel de resolución** $\sigma$: identificar en qué nivel de resolución observable opera cada célula.
2. El **suceso bilateral** $\varepsilon_{\text{bilat}}$: un suceso que produce instanciación en el horizonte del sistema compuesto, afectando posiciones de dos subsistemas en el mismo suceso $\nu_n$ de la trayectoria.
3. El **patrón de transmisión inter-estrato**: un mecanismo de transporte de la clasificación $K_3$ de una célula en estrato inferior hacia el parámetro puente de una célula en estrato superior, análogo al operador $\sigma_{k,\varphi}$ del corpus pero cruzando niveles de resolución.

### 1.2 Tesis

La distinción entre materia orgánica y materia inorgánica no tiene estatuto algebraico en el SV. Un linfocito B y una molécula de fármaco son ambos sistemas de observables $\Omega$ en $K_3^n$ con horizontes finitos y declarables. Su interacción es un suceso $\nu_n$ en la trayectoria del sistema compuesto. Las consecuencias de esa interacción son transiciones de frame gobernables si el horizonte está bien declarado.

Un motor SV que opere únicamente al nivel clínico-analítico — el nivel en que hoy operan IMMUNO-1 e IMMUNO-2 — podrá evaluar que los neutrófilos están bajos. No podrá evaluar si esa baja es consecuencia de un mecanismo de interacción farmacológica que opera a nivel molecular. Para ello necesita la arquitectura aquí descrita.

Esa limitación no es algebraica — es de horizonte. El álgebra del SV ya es suficiente. Lo que falta es declaración.

---

## 2. Disolución de la frontera orgánico/inorgánico en el marco K₃

### 2.1 La corrección algebraica

La biología clásica trata la distinción orgánico/inorgánico como ontológica. Desde la física de la materia, todo sistema material es una configuración de partículas con niveles energéticos, interacciones moleculares y composición química. La vida es una clase de configuración, no una clase de sustancia distinta.

En el SV, la consecuencia es directa. Sea $M$ cualquier sistema material a un nivel de resolución declarado. Ese sistema queda completamente caracterizado, dentro del alcance del SV, por:

$$M \equiv \left( \Omega_M,\; \mathcal{H}\!\left(\mathcal{A}_M\right),\; S_0^M \right)$$

donde $\Omega_M$ es el paquete de observables, $\mathcal{H}(\mathcal{A}_M)$ es el horizonte finito y declarado, y $S_0^M \in K_3^n$ es el frame inicial. Esta formalización es idéntica para un neutrófilo, un linfocito B, una molécula de corticosteroide y un cristal de cuarzo. La diferencia está en el contenido de $\Omega_M$ y $\mathcal{H}(\mathcal{A}_M)$, no en la estructura algebraica.

### 2.2 La U honesta como garante de la disolución

La clave algebraica de la disolución es la U. Cuando un observable de un sistema molecular no está declarado en el horizonte actual, la posición correspondiente es $U$ con soporte vacío en ese horizonte — lo que el corpus clasifica como **irreducible** mediante $\Gamma_{\mathcal{H}}$. No es que el observable sea incognoscible: es que el horizonte declarado es insuficiente para ese nivel de resolución.

La diferencia entre una U por horizonte insuficiente y una U por fenómeno genuinamente indeterminable es una pregunta abierta que el corpus deberá responder formalmente. Este documento la registra como deuda DV-HITO5-001 sin asignarle categorías propias. La política del motor ante cualquier $U_{\text{irr}}$ es la ya establecida: PROPONER\_FORK o declarar horizonte insuficiente.

---

## 3. Estratificación por nivel de resolución

### 3.1 El patrón de estratificación

El motor SV ya admite células de cualquier tamaño $n = b^2$. La **estratificación** es la práctica de declarar explícitamente, para cada célula de un dominio, el nivel de resolución observable al que pertenecen sus parámetros. Es un patrón de diseño, no un nuevo objeto algebraico.

Sea $\mathcal{L} = \{0, 1, 2, \ldots, L\}$ un conjunto de etiquetas de resolución ordenado de menor (más fino, molecular) a mayor (más grueso, sistémico). Una célula $C$ con etiqueta $\sigma \in \mathcal{L}$ es una célula SV$(n,b)$ cuyos parámetros están referenciados al nivel de resolución $\sigma$.

La ley arquitectónica del SV sigue vigente sin modificación: $n = b^2$, $b \geq 3$. La etiqueta $\sigma$ no añade estructura algebraica a la célula — la describe para uso del diseñador y del agente.

La resolución crece con $n$: una célula SV(9,3) declara 9 observables moleculares; una SV(25,5) declara 25; una SV(36,6) declara 36. No se añaden ventanas al mundo al margen del crecimiento de la propia célula.

### 3.2 Orientación de estratos para dominios biomédicos

La tabla siguiente es **orientativa y no normativa**. Establece correspondencias de diseño entre niveles de resolución y tamaños de célula. La asignación concreta de cada parámetro a su estrato es responsabilidad del experto humano y del diseñador de dominio, y debe quedar declarada en la especificación del agente especializado correspondiente.

| Etiqueta $\sigma$ | Denominación | Célula mínima orientativa | Ejemplos de observables |
|---|---|---|---|
| $\sigma = 0$ | Molecular-energético | SV(9,3) | Enlace covalente, nivel energético, carga iónica, interacción electrostática |
| $\sigma = 1$ | Bioquímico-celular | SV(25,5) | Receptor de membrana, cascada de señalización, metabolito intracelular |
| $\sigma = 2$ | Clínico-analítico | SV(25,5) | Hemograma, concentración sérica, parámetros de función orgánica (IMMUNO-1/2) |
| $\sigma = 3$ | Sistémico | SV(36,6) | Respuesta inflamatoria sistémica, equilibrio homeostático |
| $\sigma = 4$ | Epidemiológico | SV(49,7) | Incidencia poblacional, patrones de exposición |

Los estratos no son compartimentos estancos. Un dominio puede declarar células a varios estratos. La estratificación es herramienta de diseño, no restricción de la álgebra.

### 3.3 Invariancia algebraica bajo estratificación

La célula cierra en $K_3 = \{0, 1, U\}$ para cualquier $\sigma$. Los valores ternarios no cambian de significado entre estratos — 0 es cierre suficiente, 1 es violación estructural, U es indeterminación honesta. El estrato afecta al contenido semántico de los observables, no a la gramática de evaluación. Este invariante es consecuencia directa del Teorema 1 de invariancia del corpus (Documento V): toda especialización conserva el alfabeto $\Sigma$.

---

## 4. El suceso bilateral: patrón para interacciones entre sistemas

### 4.1 Tres tipos de suceso en el sistema compuesto

El corpus ya formaliza:

- El **suceso exógeno** como entrada desde el exterior hacia los parámetros de una célula (Documento IV, transductores y admisibilidad).
- El **parámetro puente endógeno** como transmisión de la salida de una célula hacia otra del mismo sistema (Composición intercelular, Documento I, operador $\sigma_{k,\varphi}$).

Lo que el corpus no ha formalizado aún es el caso en que dos sistemas se convierten en factor exógeno uno del otro: el suceso que produce instanciación en posiciones de ambos subsistemas en el mismo suceso $\nu_n$ de la trayectoria del sistema compuesto.

Para el diseño del motor, se propone el siguiente patrón de tres tipos de suceso en el sistema compuesto $M_{AB}$:

**Tipo endógeno de $M_A$:** el suceso instancia únicamente sobre posiciones de $S_n^{M_A}$. No afecta directamente a $M_B$. Ejemplo: proliferación de un linfocito B en ausencia de estímulo farmacológico.

**Tipo exógeno simple $M_B \to M_A$:** el subsistema $M_B$ actúa como factor exógeno sobre $M_A$ en dirección única. Ejemplo: liberación de una citocina por el macrófago que activa un receptor del linfocito B.

**Tipo bilateral $\varepsilon_{\text{bilat}}(A, B)$:** el suceso produce instanciación en posiciones de $S_n^{M_A}$ Y en posiciones de $S_n^{M_B}$ en el mismo suceso $\nu_n$ de la trayectoria del sistema compuesto. El suceso pertenece al horizonte del sistema compuesto, no al de ninguno de los subsistemas por separado.

Este tercer tipo es el patrón que describe la interacción farmacológica. Cuando una molécula de fármaco se une al receptor de un linfocito B, el suceso instancia:

- sobre $M_{\text{pharma}}$: concentración libre decrece ($P_{\text{conc}}: 1 \to 0$), estado de unión se activa ($P_{\text{bond}}: U \to 1$).
- sobre $M_{\text{LB}}$: receptor queda ocupado ($P_{\text{receptor}}: 0 \to 1$), umbral de activación se modifica ($P_{\text{umbral}}: 0 \to U$).

Ninguno de estos cambios puede formalizarse como suceso endógeno de un solo subsistema: son coproducidos por la interacción.

**Nota doctrinal.** El suceso bilateral es un patrón de diseño propuesto por el motor IA. Su formalización algebraica completa — incluyendo las reglas de composición del horizonte $\mathcal{H}_{AB}$ bajo Objective C1 del corpus — es una tarea del corpus, no del motor. Este documento lo declara como hipótesis de arquitectura y registra la deuda correspondiente.

### 4.2 El horizonte del sistema compuesto

Sea $M_{AB}$ el sistema compuesto $M_{\text{LB}} \oplus M_{\text{pharma}}$. El horizonte del sistema compuesto propuesto como patrón de diseño es:

$$\mathcal{H}(\mathcal{A}_{AB}) = \mathcal{H}(\mathcal{A}_A)_{\text{end}} \cup \mathcal{H}(\mathcal{A}_B)_{\text{end}} \cup \mathcal{H}(\mathcal{A}_{AB})_{\text{bilat}}$$

donde los tres conjuntos son finitos si y solo si cada uno es declarable con un número finito de tipos de suceso. Los sucesos no declarados no permanecen como U fronteriza — producen posiciones con soporte vacío en el horizonte actual, cuya clasificación por $\Gamma_{\mathcal{H}}$ es **irreducible**. El horizonte puede extenderse para convertirlas en fronterizas o resolubles, pero mientras no estén declaradas, son irreducibles en ese horizonte.

### 4.3 Composición de clasificaciones mediante gate_chain

La clasificación del sistema compuesto $M_{AB}$ sigue la compuerta conservadora ya implementada en `core.py`:

$$\kappa_3\!\left(S_n^{M_{AB}}\right) = \texttt{gate\_chain}\!\left([\,\kappa_3(S_n^{M_A}),\; \kappa_3(S_n^{M_B})\,]\right)$$

con la tabla canónica del corpus. Para un sistema de tres subsistemas (célula tumoral, sistema inmune, fármaco), la composición es:

$$\kappa_3^{\text{global}} = \texttt{gate\_chain}\!\left([\,\kappa_3^{\text{tumor}},\; \kappa_3^{\text{inmune}},\; \kappa_3^{\text{terapia}}\,]\right)$$

El `gate_chain` está verificado como asociativo sobre las 27 combinaciones. La función existe en `sv_motor.algebra.core` y está cubierta por los laboratorios del motor.

---

## 5. Patrón de transmisión inter-estrato

### 5.1 El patrón y su posición doctrinal

El corpus formaliza el operador $\sigma_{k,\varphi}$ para transmitir la clasificación $K_3$ de una célula hacia el parámetro puente de otra célula al mismo nivel de resolución (Composición intercelular, Documento I). El conector $\varphi : K_j \to \Sigma$ es semánticamente anclado: su definición depende de la semántica formal del parámetro receptor.

Para la arquitectura multi-estrato, el motor propone un **patrón de transmisión inter-estrato**: transportar la clasificación $K_3$ de una célula a estrato $\sigma_0$ hacia el parámetro puente de una célula a estrato $\sigma_1 > \sigma_0$, mediante un conector $\varphi^{(\sigma_0 \to \sigma_1)}$ semánticamente anclado al parámetro receptor.

Este patrón es algebraicamente análogo a $\sigma_{k,\varphi}$. La diferencia es semántica: el conector traduce entre niveles de resolución. Su formalización algebraica completa pertenece al corpus (pendiente de los Documentos 4–6 del programa). Su descripción como patrón de diseño es responsabilidad del motor.

La regla que el corpus ya establece y que este patrón no puede violar: **el conector no puede fabricar certeza**. Si la clasificación del estrato inferior es $U$ o no está determinada, el conector propaga $U$ al parámetro puente del estrato superior. La no-fabricación de certeza es un invariante del sistema (Proposición 4 del Documento IV del corpus).

### 5.2 Ejemplo orientativo: de estrato molecular a estrato clínico

Como referencia de diseño orientativo (no normativo), una cadena de transmisión para el dominio inmunológico-farmacológico podría articularse así:

$$C_{\text{CS}}^{(\sigma=0)} \xrightarrow{\varphi^{(0\to1)}} C_{\text{señal}}^{(\sigma=1)} \xrightarrow{\varphi^{(1\to2)}} C_{\text{IMMUNO-1}}^{(\sigma=2)} \xrightarrow{\sigma_{k,\varphi}} C_{\text{IMMUNO-2}}^{(\sigma=2)}$$

donde:
- $C_{\text{CS}}^{(\sigma=0)}$: célula molecular del corticosteroide (por implementar).
- $C_{\text{señal}}^{(\sigma=1)}$: célula de señalización bioquímica — ocupación del receptor glucocorticoideo, modulación de NF-κB (por implementar).
- $C_{\text{IMMUNO-1}}^{(\sigma=2)}$ y $C_{\text{IMMUNO-2}}^{(\sigma=2)}$: ya implementadas en el corpus, reciben como parámetro puente la clasificación del estrato bioquímico.

Esta cadena forma un grafo acíclico dirigido $\mathcal{G} = (V, E, \Phi)$ con aristas de dos tipos: aristas de transmisión inter-estrato (vía $\varphi^{(\sigma \to \sigma')}$) y aristas de puente intra-estrato (vía $\sigma_{k,\varphi}$). Ambos tipos de arista respetan el principio de no fabricación de certeza.

**Cualquier consecuencia de esta cadena sobre la gramática, IR o backend del Lenguaje SV debe detenerse aquí y elevarse al carril del repositorio del Lenguaje SV.**

---

## 6. Gobernabilidad: enunciado condicional

### 6.1 Condición de gobernabilidad en la cadena multi-estrato

El corpus establece (Teorema 1, Documento 1): la trayectoria $T$ es ternariamente convergente si y solo si $\mathcal{U}_{\text{irr}}(T) = \emptyset$.

Para la cadena de transmisión multi-estrato, el motor propone como hipótesis de diseño el enunciado siguiente, **condicional a la demostración del Objetivo C1 del corpus** (propagación de $\Gamma_{\mathcal{H}}$ bajo composición, Documento 4 del programa):

> *Si la propagación de $\Gamma_{\mathcal{H}}$ bajo composición satisface las reglas del Objetivo C1, entonces la cadena multi-estrato $C^{(\sigma_0)} \to \cdots \to C^{(\sigma_L)}$ es gobernablemente convergente si y solo si $\mathcal{U}_{\text{irr}}(T^{(\sigma)}) = \emptyset$ para cada estrato $\sigma$ activo en la cadena.*

Este enunciado no es un corolario demostrado del Teorema 1 — es una hipótesis de arquitectura que el motor adopta como principio de diseño y que el corpus deberá validar o refutar. Hasta que C1 esté cerrado, el motor opera con la condición $\mathcal{U}_{\text{irr}} = \emptyset$ en cada célula individualmente, que sí está garantizada por el corpus.

### 6.2 Consecuencia para el diseño de agentes biomédicos

Un agente que opere únicamente al estrato clínico-analítico ($\sigma = 2$) sobre un dominio cuya causalidad requiere observables moleculares tendrá posiciones con soporte vacío en su horizonte. La clasificación de esas posiciones por $\Gamma_{\mathcal{H}}$ es **irreducible**. La política resultante es PROPONER\_FORK o, si el horizonte no es ampliable, declarar el dominio como no gobernable con el horizonte actual.

Esa no es una limitación algebraica del SV — es la respuesta honesta a un horizonte declarado en el estrato incorrecto. La arquitectura multi-estrato permite ampliar el horizonte hacia estratos inferiores, convirtiendo posiciones irreducibles en gobernables mediante la declaración explícita de los tipos de suceso correspondientes.

---

## 7. Propuesta de dominio biomédico: hipótesis de arquitectura

### 7.1 Sobre la declaración de dominios especializados

Declarar un dominio especializado con firma $\mathfrak{D} = (\mathcal{P}, \mathcal{I}, \mathcal{H}, \Pi_\tau, \Pi_U, \Pi_C)$ es un acto del corpus, no del motor. El corpus (Documento V) establece que los dominios maduros hoy son inmunología y neumología, y que su firma está prácticamente completa. La extensión de esos dominios hacia los estratos $\sigma = 0$ y $\sigma = 1$ requiere la declaración de nuevas células y nuevos transductores — trabajo que pertenece al corpus y a los agentes especializados correspondientes, no al motor IA.

Lo que el motor puede y debe hacer es:

1. Declarar el patrón de diseño que esa extensión adoptaría (§5 de este documento).
2. Asegurar que el motor no impide esa extensión — que la arquitectura actual deja espacio estructural para ella.
3. Registrar las deudas vivas asociadas.

### 7.2 La célula molecular de vocabulario mínimo

Como referencia de diseño para la célula al estrato $\sigma = 0$, el motor propone la siguiente tabla de observables mínimos. Esta tabla es orientativa: no es normativa, no fija parámetros del SV y no tiene autoridad sobre ningún dominio especializado existente.

| P | Observable molecular | Cierre $= 0$ | Violación $= 1$ | Indeterminación $= U$ |
|---|---|---|---|---|
| P1 | Estado de enlace covalente principal | Estable | Roto | En tensión |
| P2 | Nivel energético relativo al estado basal | En reposo | Excitado crítico | En transición |
| P3 | Interacción electrostática neta | Sin cambio neto | Alterada críticamente | Evaluando |
| P4 | Estado térmico local | Equilibrio | Desequilibrio crítico | Midiendo |
| P5 | Integridad conformacional | Íntegra | Comprometida | Evaluando |
| P6 | Transferencia de carga iónica | Nula | Activa | Midiendo |
| P7 | Estado de fase / agregación | Sin cambio | Cambio de fase | En transición |
| P8 | Origen del suceso instanciante | Sin factor exógeno activo | Factor exógeno instanciando | Indeterminado |
| P9 | Gobernabilidad del estado siguiente | $\mathcal{U}_{\text{irr}} = \emptyset$ | Irreducible presente | Fronteriza |

Un fármaco y una proteína de membrana son instancias de esta misma célula con sus observables específicos. El diseñador de dominio declara los valores concretos de los transductores $\tau_j$ para cada parámetro.

---

## 8. Deudas vivas del Hito 5

Las deudas siguientes se declaran en el formato del motor y deben registrarse en `REGISTRO_DEUDA_VIVA_SV_MOTOR.csv`:

| ID | Objeto | Descripción | Soporte | Estado |
|---|---|---|---|---|
| DV-HITO5-001 | Diagnóstico de U por horizonte insuficiente vs U genuina | Determinar algebraicamente si una posición irreducible resulta de un horizonte declarado en estrato equivocado o de un fenómeno genuinamente no declarable. Requiere aporte del corpus (Documentos 7–8). | Este documento, Documento 7 y 8 del programa | ABIERTA |
| DV-HITO5-002 | Patrón $\varphi^{(\sigma \to \sigma')}$ de transmisión inter-estrato | Formalización del conector entre estratos: condiciones de admisibilidad, preservación de la no-fabricación de certeza, relación con $\sigma_{k,\varphi}$ existente. Requiere aporte del corpus (Documentos 4–6). | Este documento, Documento I–III de composición intercelular | ABIERTA |
| DV-HITO5-003 | Célula bioquímica de señalización intracelular $\sigma=1$ | Declaración de la célula de señalización para dominios inmunológicos: receptor glucocorticoideo, NF-κB, MAPK y equivalentes. Tarea del agente especializado en inmunología. | Este documento, IMMUNO-1/2 existentes | ABIERTA |
| DV-HITO5-004 | Célula molecular farmacológica $\sigma=0$ | Declaración de la célula molecular para fármacos y moléculas de señalización: farmacocinética molecular, unión a receptor, concentración libre. Tarea del agente especializado. | Este documento | ABIERTA |
| DV-HITO5-005 | Formalización del suceso bilateral en el corpus | El suceso bilateral $\varepsilon_{\text{bilat}}$ requiere que el corpus declare las reglas de composición del horizonte $\mathcal{H}_{AB}$ bajo el Objetivo C1. No es una tarea del motor. | Este documento, Objetivo C1 del corpus | ABIERTA |
| DV-HITO5-006 | Demostración condicional de gobernabilidad global multi-estrato | Confirmar o refutar el enunciado de la §6.1 una vez que el Objetivo C1 del corpus esté cerrado. | Este documento, Documento 4 del programa | ABIERTA |
| DV-HITO5-007 | Administración de perfiles y permisos del ecosistema del agente | Formalizar e implementar perfiles de administración, privilegio medio y uso estándar sin degradar la custodia estructural ni permitir actualización impropia por usuarios ordinarios. | Este documento y documentación de gobierno del motor | ABIERTA |

---

## 9. Posición en el programa y en el repositorio

### 9.1 Relación con el camino 4→9

Los Documentos 4 (propagación de $\Gamma_{\mathcal{H}}$ bajo composición), 5 (métrica sobre $K_3^n$) y 6 (composición de dominios) son condición necesaria para que los patrones de diseño declarados en este documento sean algebraicamente demostrables. El Documento 4 en particular cierra el Objetivo C1, del que depende el enunciado de §6.1. El Documento 7 (separación algebraica frente a regímenes probabilísticos) cierra el suelo que protege al futuro agente biomédico frente a interpretaciones bayesianas impropias.

### 9.2 Relación con los dominios maduros del corpus

Los dominios de inmunología y neumología, reconocidos como maduros en el Documento V del corpus, operan hoy al estrato $\sigma = 2$. Este documento no modifica ni amplía esos dominios. Describe el espacio estructural que el motor debe preservar para que esa extensión sea posible cuando el corpus y los agentes especializados la declaren.

### 9.3 Relación con el Lenguaje SV

Toda consecuencia de la arquitectura aquí descrita que exija alterar gramática superficial, IR, validator, lowering o backend soberano del Lenguaje SV debe detenerse en este documento y elevarse al carril del repositorio del Lenguaje SV. El motor no puede imponer estructura al Lenguaje.

### 9.4 Lo que el motor puede hacer hoy

Dentro de su alcance declarado, el motor puede:

1. Ejecutar `run_custom` de `sv_motor.verification` sobre cualquier vector $K_3^n$ con horizonte declarado, sin distinción de sustrato. La célula molecular de §7.2 es instanciable hoy mediante `run_custom`.
2. Aplicar `gate_chain` sobre las clasificaciones de subsistemas compuestos para obtener la clasificación del sistema compuesto.
3. Declarar en el horizonte de cualquier célula los tres tipos de suceso descritos en §4.1, dentro del formalismo ya implementado.
4. Producir el JSON canónico de verificación para cualquier combinación de subsistemas.

Lo que el motor no puede hacer hasta que el corpus cierre las deudas correspondientes: demostrar el enunciado de §6.1, definir formalmente el suceso bilateral como objeto del SV, declarar dominios especializados con firma $\mathfrak{D}$, y modificar el Lenguaje SV.

---

## 10. Dictamen

**El Hito 5 queda formalmente consolidado en régimen exclusivo de laboratorio.**

El álgebra del SV es suficiente para gobernar sistemas biomédicos compuestos — orgánicos e inorgánicos — sin distinción de sustrato. La disolución de la frontera orgánico/inorgánico no requiere nueva álgebra: requiere que el horizonte de cada dominio se declare al estrato de resolución correcto.

El motor SV puede servir hoy, dentro de su alcance, a este objetivo mediante `run_custom` y `gate_chain`. La materialización completa requiere el cierre de seis deudas vivas, cinco de las cuales son responsabilidad del corpus y los agentes especializados — no del motor.

Lo que este documento no hace: no introduce definiciones formales nuevas al SV, no declara dominios especializados, no modifica el Lenguaje SV, no introduce vocabulario temporal, estadístico, probabilístico, inferencial ni heurístico, no propone ninguna clasificación de U que no sea la ya establecida por el corpus y no habilita en ningún caso el paso del frente motor a producción antes del cierre suficiente del bloque documental de nueve piezas.

---

*Documento de arquitectura del frente motor IA — Sistema Vectorial SV.*  
*Autor del corpus: Juan Antonio Lloret Egea | ORCID: 0000-0002-6634-3351 | ITVIA — IA eñ™ | ISSN 2695-6411 | Madrid, 3 de abril de 2026*

---

## 11. Administración de perfiles y permisos en el ecosistema del agente

La apertura del Hito 5 exige además una disciplina de administración de seguridad de perfiles para el uso de agentes especializados dentro de la casa del SV. Esta disciplina es operativa y subordinada: no introduce semántica nueva ni altera el estatuto soberano del humano ni del álgebra.

### 11.1 Perfiles mínimos

Se distinguen, como mínimo, tres perfiles:

1. **Administrador.** Puede declarar dominios locales de laboratorio, ajustar parámetros internos del agente, aprobar cambios de configuración y habilitar o revocar permisos de otros perfiles.
2. **Cuenta con privilegios medios.** Puede ejecutar laboratorios, revisar salidas, cargar casos autorizados y operar sobre parámetros no críticos expresamente habilitados, pero no puede rediseñar el agente ni modificar su custodia estructural.
3. **Cuenta estándar de uso.** Puede utilizar el agente dentro del régimen ordinario permitido, consultar salidas y operar sobre expedientes ya habilitados, sin capacidad de alterar parámetros, transductores, compuertas ni configuraciones de seguridad.

### 11.2 Regla de no actualización por usuario ordinario

Ningún experto de uso ordinario puede actualizar parámetros sensibles, rediseñar observables, alterar horizontes, relajar compuertas ni modificar rutas de validación del agente sin los permisos adecuados. Toda modificación con impacto estructural requiere perfil administrador y trazabilidad visible.

### 11.3 Estatuto actual

La presente disciplina queda declarada como necesidad arquitectónica y de gobierno. Su implementación material y reproducible en código y soporte operativo se registra como deuda viva del Hito 5.


# Oposiciones explícitas: adaptación delimitada y contraste

**Buscador-Semántico - Diferencial · 11 de octubre de 2026 · Pasos 4 y 5 / 8.**

## Resultado y alcance

La adaptación conservadora de SC-XF-002 mejora el reconocimiento de determinadas oposiciones explícitas, pero conserva errores de contexto y una cobertura reducida. Se mantiene como función experimental para estudiar sugerencias opcionales; **no se integra todavía en la administración documental ni se presenta como detector general de contradicciones**. El objetivo sigue siendo ayudar a localizar conflictos, sin exigir un resultado perfecto ni imponer la consulta de avisos.

La comparación se limita a una copia de speccheck-core 0.4.1. El analizador existente reconoce afirmaciones con should, mientras que la función original entre documentos sólo considera shall, must y will. Se han contrastado tres perfiles, definidos en el [protocolo anterior a la ejecución](PROTOCOLO.md): original; incorporación de should; e incorporación de should con coincidencia del resto de la afirmación. En este último se conservan palabras, cifras, signos y orden, salvo el modal y su negación inmediata; sólo se normalizan mayúsculas y espacios. Esta coincidencia no demuestra equivalencia de modalidades ni de contextos documentales.

## Resultados separados por conjunto

| Conjunto y perfil | Incompatibilidades con aviso | Incompatibilidades omitidas | Avisos en compatibles | Avisos con contexto insuficiente |
| --- | ---: | ---: | ---: | ---: |
| 16 controles nuevos: original | 1 de 6 | 5 de 6 | 0 de 8 | 0 de 2 |
| 16 controles nuevos: should | 5 de 6 | 1 de 6 | 6 de 8 | 2 de 2 |
| 16 controles nuevos: should y coincidencia | 4 de 6 | 2 de 6 | 1 de 8 | 1 de 2 |
| 12 casos documentales conocidos: should | 2 de 6 | 4 de 6 | 0 de 4 | 0 de 2 |
| 12 casos documentales conocidos: should y coincidencia | 1 de 6 | 5 de 6 | 0 de 4 | 0 de 2 |

La tabla contabiliza presencia de aviso por caso, según la [evaluación posterior en Rust](EVALUACION-RUST.json). **Un aviso en un caso incompatible no acredita por sí solo que su fundamento sea correcto.** En C01, la variante should relaciona «should be prioritized» con «should not be implemented»: coincide el auxiliar be, pero difieren las acciones. El párrafo de referencia sí contiene una indicación sobre MFA; el procedimiento no demuestra haber identificado correctamente esa oposición. Este resultado se conserva como aviso contabilizado, con fundamento insuficiente. No se suma como demostración de comprensión semántica.

La variante con coincidencia señala C03, donde los textos conservan el mismo sujeto, acción y condiciones, con must frente a should not. La diferencia de modalidad permanece visible y exige consideración contextual. En los controles nuevos señala A01, A02, A04 y A16; omite A11, que cambia cuantificadores, y A14, que expresa oposición sin not. La mejora elimina cinco avisos espurios del perfil should: A05, A06, A07, A12 y A15, relativos a sujetos, condiciones, momentos, entornos y campos diferentes.

Persisten dos límites especialmente relevantes. En A08, la vigencia se distingue únicamente en los encabezamientos: el procedimiento avisa aunque una regla sea histórica y ya no aplicable. En A03, distingue must y should not en los textos conservados, pero no resuelve la autoridad de los documentos; el aviso es una tensión para consulta, no una contradicción establecida. No se cambió la función después de observar estos resultados.

El [contraste anterior](../2026-10-11-pasajes-traducidos/RESULTADO-Y-ALCANCE.md), con cero avisos en los doce casos, permanece intacto. No se repitió esa combinación. Los casos documentales reutilizados son regresión y exploración sobre textos conocidos, **no validación independiente**. Los dieciséis controles nuevos son sintéticos y diagnósticos; no permiten estimar precisión o cobertura de producción. Ninguna ausencia de aviso se interpreta como coherencia documental.

## Ejecución, observabilidad y reproducibilidad

Se realizaron las 72 llamadas previstas: 48 en controles nuevos y 24 en casos documentales conocidos. Una llamada por combinación, sin reintentos. Duración del conjunto medida por el instrumento: 415.344 microsegundos; excluye compilación, preparación y análisis. Las referencias de evaluación se abrieron después de conservar la salida. [Entradas completas cotejadas](ENTRADAS-COTEJADAS.json), [resultado](RESULTADO.json), [registro por llamada](REGISTRO.jsonl) y [referencias nuevas](REFERENCIAS-ADVERSARIALES.json) se conservan separados. Las traducciones anteriores mantienen sus huellas y procedencia; no se hicieron nuevas solicitudes de traducción.

Se conservan todas las afirmaciones reconocidas, con sus localizaciones, y los documentos completos usados en cada par. El diagnóstico comunitario sólo proporciona una localización directa y abrevia su mensaje; por ello no basta como presentación final de dos pasajes. La consulta de ambos textos es posible en la evidencia conservada, pero **la presentación operativa de la sugerencia aún no está resuelta**. La gravedad Error y la confianza fija 0,75 son campos originales del componente; no se adoptan como gravedad del SV ni como probabilidad calibrada.

El [cambio exacto](SC-XF-002.patch) afecta únicamente a la función de comparación de una copia experimental. El constructor original conserva su comportamiento; las variantes reciben identificadores distintos. [Cargo.lock](instrumento/Cargo.lock) mantiene las demás ediciones: sólo sustituye la procedencia de speccheck-core por la copia con el cambio. [Código instrumental](instrumento/src/main.rs) y [configuración](instrumento/Cargo.toml) permiten reconstruir la prueba con el paquete original y el cambio aplicado. El SHA-256 del ejecutable utilizado es 299c6941b8bff818e0bd7af744c7c69fd61862bba8901d22258ca8b607a012aa. La compilación local terminó correctamente, sin descargar dependencias nuevas.

Origen: [speccheck](https://gitlab.com/neilfitzgerald1972/speccheck), paquete 0.4.1, revisión e49eda84dcbb8f836d79b866cf6ae9f89704b375, licencia MIT, SHA-256 e0fcbea6024b695434a8f257c3ad5ae43455108f9faf7af44a165efc269ecd90. El paquete speccheck-nli se compila como dependencia de interfaces; no se habilitan candle, llm ni languagetool y no se instancia ni invoca un detector NLI. La función ejecutada usa expresiones regulares y comparación de palabras. Se distinguen así dependencia presente, compilación y utilización efectiva. No hubo inferencias, acceso a modelos, cambios del Núcleo, IR, Árbitro - Director o aplicación documental.

## Continuación dentro del recorrido

Se descarta la ampliación should sin restricción para una presentación de sugerencias: genera demasiados avisos espurios en este conjunto y su aviso C01 carece de fundamento suficiente. La variante conservadora aporta una mejora limitada que merece conservarse, sin atribuirle solución de conflictos complejos. Antes de integración debe poder presentar ambos pasajes y su contexto, incluida la vigencia, con modalidades visibles y errores comprobados en textos nuevos. No se añade una obligación de resolver avisos ni una condición adicional para continuar el procedimiento documental.

El paso 6 / 8 conserva las veinte traducciones ya documentadas; esta actuación no acredita traducción integrada ni cobertura general de PDF, HTML y Markdown. El paso 7 / 8, integración, sigue pendiente. Para el paso 8 / 8 se prepara una [nota académica de actualización del libro](NOTA-PARA-EL-LIBRO.md), conservando su edición V.02 y sus cinco capítulos. La nota no se declara incorporada por el hecho de estar publicada aquí.

Los textos de origen mantienen sus derechos. Los derivados de INCIBE conservan CC BY-NC-SA 4.0 y las referencias de la preparación anterior; el cambio sobre código comunitario mantiene MIT. El pie siguiente se aplica a la exposición propia del informe, no sustituye las licencias de terceros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

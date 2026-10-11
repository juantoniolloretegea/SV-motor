# Incidencias y correcciones delimitadas

**Buscador-Semántico - Diferencial · Paso 8 / 8 · 11 de octubre de 2026.**

## Preparación de las comprobaciones

La primera compilación de las pruebas de módulos no terminó: la copia administrativa fijada no incluía tests/soporte/pdf.rs, que necesitan las pruebas existentes. No se ejecutó ninguna prueba en ese intento. Se incorporó una copia idéntica del archivo auxiliar, cuya procedencia e identidad constan en COMPLEMENTO-PRUEBAS.json. La ejecución posterior terminó con 56 pruebas conformes. No se modificó el comportamiento administrativo para obtener ese resultado.

## Presentación de los originales

Los 35 controles instrumentales de la primera ejecución resultaron conformes, pero la inspección del lector detectó un defecto de presentación. En el caso sintético de ciberseguridad, las secciones de referencia y del documento compatible aparecían dentro del bloque de texto del documento propuesto. Además, los saltos de párrafo no se conservaban íntegramente. Esa primera presentación no es conforme y permanece conservada como antecedente.

La causa es la segunda interpretación de HTML y Markdown realizada por mdBook: los saltos de línea de los originales podían terminar un bloque HTML. Se corrige únicamente la codificación de esos saltos en el HTML generado. Los caracteres CR y LF se representan mediante referencias numéricas; los originales, sus identidades, los pasajes y las reglas de comparación no cambian. El navegador debe recuperar los mismos caracteres al presentar el texto.

Se añade una comprobación de regresión con líneas vacías, CR/LF, caracteres españoles, contenido HTML inerte, una secuencia de cierre de bloque y una referencia numérica literal. Los controles de generación mdBook se refuerzan para cotejar que cada original continúa íntegro dentro de su propio bloque después de generar la página. La segunda ejecución se conserva por separado y no se contabiliza como 35 controles nuevos.

Esta corrección no acredita una comparación semántica más precisa. Su resultado se limita a la fidelidad de presentación y a la separación de contextos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## Precisión del cotejo posterior a mdBook

En la segunda ejecución, 31 de los 35 controles fueron conformes y cuatro rechazaron la comparación literal del HTML. La representación generada ya conservaba los saltos y la separación, pero mdBook serializa las referencias numéricas como saltos literales y añade espacios entre etiquetas. Se conserva íntegra esa ejecución no conforme. Se corrige el instrumento de cotejo para exigir cada original completo dentro de su sección, sin secciones de otros documentos anidadas, teniendo en cuenta la serialización observada. No se reduce la exigencia de conservación ni se modifica de nuevo la función de presentación.

La regresión específica de los módulos terminó con cinco controles conformes: cuatro ya incluidos en la ejecución anterior y uno nuevo para la conservación a través de CommonMark. En conjunto hay 57 pruebas distintas de módulos; no se suman las repeticiones como pruebas nuevas.
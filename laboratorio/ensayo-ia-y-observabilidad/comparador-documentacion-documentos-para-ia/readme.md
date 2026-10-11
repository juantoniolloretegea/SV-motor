# Buscador-Semántico - Diferencial: resultado del ensayo y retirada

**Estado: retirado · 11 de octubre de 2026.**

El ensayo examinó una ayuda opcional para identificar contradicciones entre un documento propuesto y otros documentos del mismo dominio, con realización en Rust y sin IA en su funcionamiento. La contradicción debía consistir en afirmaciones incompatibles sobre el mismo objeto, sentido y condiciones de aplicación. La menor precisión, la falta de detalle y la información complementaria no constituían por sí solas una contradicción.

**Las alternativas examinadas no acreditaron la utilidad necesaria para ese objetivo. Se retira la herramienta y se da por terminado su desarrollo e integración.** La conclusión se refiere a los recursos y procedimientos examinados; no demuestra que una solución futura sin IA sea imposible.

## Resultados que fundamentan la retirada

| Comprobación | Contradicciones detectadas | Falsos avisos en casos compatibles |
|---|---:|---:|
| Perfil restrictivo, controles sintéticos iniciales | 4 de 6 | 1 de 8 |
| Doce casos documentales, traducciones inglesas conservadas | 1 de 6 | 0 de 4 |
| Mismos casos documentales en español, ensayo de segmentación | 0 de 6 | 0 de 4 |
| Controles adicionales: lector inicial o párrafos completos | 2 de 5 | 1 de 7 |
| Mismos controles adicionales: separación por oraciones | 3 de 5 | 3 de 7 |

En los controles sintéticos iniciales también se emitió un aviso en uno de los dos casos de contexto insuficiente. Los doce casos documentales incluían seis incompatibilidades, cuatro compatibilidades y dos relaciones de contexto insuficiente. Son los mismos casos en español e inglés, no veinticuatro observaciones independientes. La comparación de lectores, párrafos y oraciones mantuvo los resultados documentales: ninguna detección española y una inglesa, correspondiente a una obligación de verificación de acceso frente a su negación.

La separación por oraciones recuperó una oposición artificial adicional, pero añadió dos falsos avisos al separar las afirmaciones de una condición temporal o de la aclaración de que un ejemplo carecía de vigencia. Conservar el contexto completo no bastó para que la regla de comparación interpretase su alcance. Una diferencia de redacción no podía utilizarse como sustituto de una contradicción de significado.

Las muestras fueron pequeñas, deliberadas y conocidas durante el desarrollo; las referencias documentales eran preliminares. Los recuentos no estiman una fiabilidad general. La ausencia de aviso tampoco acredita compatibilidad. Las comprobaciones instrumentales favorables no compensan estas limitaciones de detección. El tiempo de revisión no determinó la decisión.

## Alcance del cierre

La carpeta vigente del ensayo conserva únicamente este documento. Se retiran sus informes separados, programas, demostraciones y propuestas de integración. Los antecedentes permanecen identificables en el historial de Git, sin constituir una herramienta disponible o una propuesta pendiente de incorporación.

La comprobación del libro **Reglas, usos y descripciones documentales de documentos para el conocimiento de la IA**, realizada el 11 de octubre de 2026 sobre sus fuentes y sus páginas publicadas, no encontró incorporaciones de esta herramienta. El cierre no añade contenido al libro.

## Referencias de trazabilidad

- [Resultados del contraste de oposiciones explícitas](https://github.com/juantoniolloretegea/SV-motor/blob/df38b107882e999c6b85de0582751a5ff3366a1a/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/comprobaciones/2026-10-11-oposiciones-explicitas/RESULTADO-Y-ALCANCE.md).
- [Estado documental anterior a la retirada](https://github.com/juantoniolloretegea/SV-motor/tree/df38b107882e999c6b85de0582751a5ff3366a1a/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia).
- Ensayo complementario de segmentación del 11 de octubre de 2026: resultados cotejados antes del cierre; archivo de evidencias identificado por SHA-256 `69d4c8209a77e6391f81f9d2914422a947cf4b7fb5a68117b6c50f30196fe0f0`.

Las atribuciones y licencias de los documentos y componentes originales se conservan en sus antecedentes. Este resumen no modifica sus condiciones.

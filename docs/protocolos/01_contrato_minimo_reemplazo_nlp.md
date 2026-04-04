# Contrato mínimo de reemplazo para extractores NLP

Toda capa NLP que sustituya o amplíe el extractor actual debe cumplir:

- devolver exactamente las claves del dominio declarado;
- usar solo valores pertenecientes al dominio o una variante indeterminada válida;
- no cerrar posiciones con valores fuera de dominio;
- delegar siempre la evaluación ternaria global al motor.

La verificación mínima consiste en aplicar `validate_observables_with_ud()` antes de `i_nlp()`.

# Aplicación prevista del control documental a GPT-OSS-Safeguard-120B

**Corte de diseño · 2 de octubre de 2026.** Primer candidato previsto para un sistema compuesto nuevo; implementación, integración y contraste todavía no acreditados.

## Sede común y función del modelo

El diseño reutilizable se conserva en [Control documental asistido: controlador, algoritmo y pictogramas](../../model-context-protocol/controlador-pictograma-algoritmo/README.md). La [especificación y sus diagramas](../../model-context-protocol/controlador-pictograma-algoritmo/DISENO.md) y el [plan de comprobación](../../model-context-protocol/controlador-pictograma-algoritmo/VERIFICACION.md) tienen allí su sede única. Esta nota vincula su aplicación al expediente del modelo sin duplicar la especificación.

El controlador Rust obtendrá mediante MCP todas las páginas previamente fijadas y comprobará su incorporación efectiva antes de consultar a Safeguard. El modelo clasificará una afirmación conforme a una política expresa y fundamentará su respuesta con los pasajes recibidos. La evaluación científica se mantendrá separada de la admisión instrumental.

El nuevo objeto es la clasificación con adquisición documental asistida. No se atribuirá al modelo la recuperación realizada por el controlador ni se presentará como prueba de lectura autónoma exhaustiva. Los pictogramas serán una representación para revisión humana; no se incorporarán a la entrada del candidato.

## Relación con los resultados anteriores

La [ficha técnica](FICHA_TECNICA.md), el [estado estructurado](ESTADO.json) y las [pruebas anteriores](tests-y-pruebas-efectuadas) conservan sus evidencias y conclusiones. El contraste inicial registró 87,5/100 y el posterior 66,67/100; ambos declararon **No apto para su contraste delimitado**, con incumplimiento crítico de lectura íntegra. La nueva arquitectura no corrige retrospectivamente esos resultados ni cambia sus reservas o recepciones pendientes.

El comportamiento del sistema compuesto deberá recibir identidad, resultados y límites propios. No se promedian sus puntuaciones con las históricas ni se interpreta la asistencia documental como una mejora ya demostrada del candidato.

## Condiciones antes de inferir

1. Comprobar el controlador localmente en Rust sin modelo, incluidos recorridos adversariales y uno conforme.
2. Acreditar identidad actual, compatibilidad del motor y MCP, incorporación efectiva de todos los pasajes, observación, conservación, aislamiento y capacidad dentro de las cotas existentes.
3. Fijar un encargo propio para tres casos nuevos reservados, con política, clave, criticidad y criterios anteriores a la generación; una consulta prevista por caso y sin rondas de mejora.

Se parte de la instalación conservada, cuya disponibilidad actual deberá comprobarse. El diseño no exige ajuste de pesos, entrenamiento, recursos nuevos, incorporación de otros candidatos ni modificación de ensayos ajenos. La publicación de esta nota no inicia inferencia.

## Documentación y retorno

Los resultados efectivos se incorporarán a este expediente cuando existan, con enlace a originales y entrega en SV-sala-de-maquinas. Se mantendrá la relación con [TT-0018](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0018.md), [TT-0014](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md) cuando afecte a la interfaz documental, [S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39) y las [actas de Calidad](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/main/docs/calidad/tuberias-ia/continuacion-15-09-2026) según su alcance.

Retorno: controlador comprobado → integración acreditada → contraste finito → decisión sobre utilidad del sistema compuesto. Ninguna de esas etapas se declara completada por esta nota.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias.

# Matriz de preparación

Todos los estados son **preparado / no ejecutado**, nunca aceptación experimental. Los códigos esperados de cada uno de los 19 casos están fijados en casos.json antes de ejecución; los otros cinco están en banco.rs.

| Obligación | Testigo/mecanismo preparado | Esperado futuro y límite |
|---|---|---|
| EIO-P-01 | Manifiesto, guardas, SHA-256 | Identidades coincidentes; pendientes bloquean. |
| EIO-P-02 | Cargo independiente, versión exacta, --locked | Construcción repetible; no existe lock todavía. |
| EIO-P-03 | ModelWeights GGUF, Tokenizer, EOS=151645 | Carga real compatible o error literal, nunca sustitución. |
| EIO-P-04 | eos_correcto, no_confundir_pad, limite_generacion, cancelacion | EOS, ninguna parada por PAD, LIMITE_GENERACION, CANCELACION. |
| EIO-P-05 | limite_entrada, limite_contexto; tokens de salida <=128 | LIMITE_ENTRADA, LIMITE_CONTEXTO; contexto <=2048. |
| EIO-P-06 | control válido y ausente/vacío/tipo/adicional/JSON malformado | OK frente a ESTRUCTURA; ningún efecto al rechazar. |
| EIO-P-07 | sin_permiso y control con permiso externo | SIN_PERMISO sin efecto; positivo muta sólo booleano. |
| EIO-P-08 | petición distinta, trace/span/parent | CORRELACION y vínculo hijo/raíz comprobable. |
| EIO-P-09 | omisión individual B y exportador que falla | EVENTO_AUSENTE, un descarte exacto; EXPORTACION, error visible. |
| EIO-P-10 | vigilar.sh | Medidas, exceso/interrupción y procesos residuales; cobertura parcial declarada. |
| EIO-P-11 | siete llamadas reales, costes.rs | Calentamiento separado, tres pares, individuales/mediana/rango; sin resultados aún. |
| EIO-P-12 | destino WASM y enlace mínimo | Construcción y navegador se discriminan; ejecución navegador no implementada en workflow. |
| EIO-P-13 | dependencia B omitida, condición activa/inactiva, veto | DEPENDENCIA, OK con ABC/AB, VETO; no reemplazar omisión por traza vacía. |
| EIO-P-14 | referencia Z, versión 2, declaración B sin consulta | REFERENCIA, VERSION, CONSULTA_NO_REALIZADA; positivo AB. |
| EIO-P-15 | instrucción incrustada inerte, permiso falso | SIN_PERMISO; texto no modifica reglas ni ejecuta herramientas. |

El banco es sintético; la procedencia observada es la consulta a las fuentes del propio caso. Los testigos directos no se atribuyen a la inferencia. La conformidad estructural de una generación real se informa junto a su salida literal, también cuando sea adversa. Calidad semántica, compatibilidad técnica y controles son conclusiones distintas.

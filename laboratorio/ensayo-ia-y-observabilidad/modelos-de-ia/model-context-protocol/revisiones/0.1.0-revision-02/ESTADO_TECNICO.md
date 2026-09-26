# Estado técnico del servicio documental MCP 0.1.0

Fecha: 26 de septiembre de 2026. **Preparación incompleta.**

## Alcance comprobado

Las evidencias conservadas registran diez pruebas correctas, 32 peticiones MCP y una notificación. Permiten reconstruir las 25 páginas del catálogo. Estos resultados corresponden a las ejecuciones originales; no acreditan una nueva ejecución, el cliente real de mistral.rs ni la utilización por Qwen.

## Limitaciones

| Componente | Limitación |
|---|---|
| Plazo de llamada | Los treinta segundos no abarcan conjuntamente lectura, escritura y conservación. |
| Cierre del cliente | Existen cierre por fin de entrada y llamadas a `kill()` y `wait()` en `Drop`. Su eficacia ante los fallos pertinentes requiere pruebas; los resultados de estas dos llamadas se descartan. |
| Registro de errores | Determinadas salidas de error y códigos de terminación pueden quedar sin conservar. |
| Paginación | La doble serialización JSON puede superar la cota de 8 000 caracteres para una página admitida. El servicio devuelve un error sin permitir recuperar esa página con menor tamaño. |
| Validación JSON | El análisis inicial como `Value` sustituye claves repetidas antes de la validación posterior. Este hecho no demuestra acceso fuera del catálogo. |

Las limitaciones de paginación y claves repetidas proceden de inspección estática; no se presentan como reproducciones ejecutadas. La reconstrucción del catálogo conservado mantiene su validez para ese documento.

## Comprobaciones pendientes

Una versión corregida deberá acreditar, con fuentes sintéticas separadas del catálogo oficial, el plazo de la operación completa, la terminación de procesos ante fallos, la conservación de diagnósticos y códigos, la recuperación íntegra de textos con expansión JSON y el rechazo de claves repetidas.

La versión 0.1.0 se conserva sin cambios. No está acreditada su conformidad técnica completa. La integración con el motor y el uso efectivo por el modelo permanecen pendientes.

Referencia: [código y evidencias de 0.1.0](../../0.1.0/).

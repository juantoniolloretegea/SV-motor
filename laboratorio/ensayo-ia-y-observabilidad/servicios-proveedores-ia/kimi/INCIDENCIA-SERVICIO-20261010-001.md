# Incidencia Kimi CYB09: límites de salida y plazo del cliente

**Estado: examen suspendido; defecto del SV corregido y comprobado localmente.** La ubicación en calidad de proveedores conserva el seguimiento del servicio utilizado; no atribuye por sí misma responsabilidad al proveedor.

| Intento de C04/R0 | Cota de salida | Duración | Hecho comprobado |
|---|---:|---:|---|
| I010 | 7.168 tokens | 176.584 ms | HTTP 200 y terminación por longitud |
| I011 | 10.240 tokens | 283.410 ms | HTTP 200 y terminación por longitud; sin respuesta visible |
| I012 | 16.384 tokens | 330.178 ms | HTTP 200; entrega parcial; observación agotada |

I010 e I011 respetaron las cotas remitidas por el cliente, consumidas principalmente por razonamiento comunicado. I012 permitió observar que el límite de espera configurado se renovaba con cada lectura bloqueante. El cliente no aplicó correctamente los cinco minutos a la operación completa; el cierre se produjo por la protección secundaria de medición. No hay evidencia de cinco minutos de indisponibilidad remota. Los originales parciales no se califican como errores científicos.

La corrección usa un vencimiento absoluto único para cabeceras y cuerpo, sin reintentos automáticos. Se comprobaron flujos continuos, silencios, cabeceras retrasadas, duración conjunta y respuesta anterior al vencimiento. Resultado: 58 comprobaciones conformes. El cliente corregido está compilado; no se ha ejecutado otra consulta real. Cancelar la lectura local no prueba que el proveedor detenga su cómputo o su facturación.

Se conservan nueve entregas completas de tres preguntas, los doce intentos, fuentes, versiones anteriores del cliente y telemetría. La última cadena de medición mantiene integridad, pero no cierre normal; consta expresamente su no conformidad. La recepción científica del examen continúa pendiente y no se modifica la aptitud documental histórica del manual MD09.

Retorno: Kimi permanece detenido por decisión humana. El contraste prospectivo exige las mismas nueve preguntas primero en Astra y después en Z.ai, condicionado a recepción favorable y recursos suficientes. La ejecución de Astra ha revelado incidencias del receptor y la medición del SV; debe recibirse su corrección antes de una conclusión entre proveedores. Mantener pendiente la conciliación del último intento de Kimi.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
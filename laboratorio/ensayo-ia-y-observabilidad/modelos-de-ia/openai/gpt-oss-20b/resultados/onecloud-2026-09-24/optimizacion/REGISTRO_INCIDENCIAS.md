# Registro de incidencias instrumentales

- 12:18 UTC: detectada una ruta documental mal construida en la publicación inicial (`undefined/optimizacion`). El protocolo, publicado a las 12:03 UTC antes de inferencias, se traslada íntegro a su ruta canónica; se retiran las dos entradas de la ruta errónea. El historial conserva el original. No cambia el protocolo ni sus oráculos.
- Consulta de registros: `rg` no estaba instalado en la máquina virtual; se sustituyó por `grep`. La primera invocación de pruebas del controlador no encontró Cargo en PATH; se repitió indicando el compilador ya instalado. Nueve pruebas aprobadas. No se inició inferencia en esos intentos instrumentales.

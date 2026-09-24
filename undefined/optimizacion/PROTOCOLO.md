# Protocolo de optimización CPU y ejecución residente
Fecha: 24 de septiembre de 2026. Investigación lateral EIO; seguimiento S39. Autorización expresa del responsable humano para usar recursos existentes, optimizar Rust y, si no se alcanza el criterio, contrastar llama.cpp C/C++. No se autoriza adquirir recursos adicionales.

## Plazos y decisión
Inicio fijado por el encargo: 13:58 Europe/Madrid (11:58 UTC). Fase Rust hasta 14:58 (12:58 UTC). Si no acredita aceptación, fase llama.cpp hasta 15:58 (13:58 UTC). El tiempo de preparación forma parte de la ventana. Cierre y custodia tienen prioridad al aproximarse el límite; no se inicia una petición que no pueda terminar dentro de él. No se amplía el plazo automáticamente ante fallos instrumentales.

## Referencia e hipótesis
Antecedente OC01/OC02: Motor 70750a516001cf13314176c529508d0712b7f3c1; Lenguaje 32bf520f6e6c63dae84ef299957b6fd0b8c20402. TT-0012 quedó finalizado por su objetivo material; esta campaña tiene un objetivo nuevo de rendimiento y ejecución residente dentro de S39.
Se conserva el GGUF SHA-256 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901 y el motor corregido de referencia 2d6856918349d85a073fea59190c59886e780a62bcd94c6d7789060d04e99fb1. La hipótesis es que las copias y el cálculo secuencial de expertos consumen una parte relevante del tiempo. Se medirán antes de atribuirles predominio. Optimización candidata: lectura sin copias innecesarias y reparto de filas independientes, conservando inicialmente el orden de acumulación por resultado. Rust 1.98.0; sin Python.

## Peticiones y oráculos previos
La petición de referencia conserva literalmente OC01: inventario de tres elementos más dos, Harmony final abierto, temperatura 0 y máximo 16 tokens.
Banco adicional, mismo formato de conversación y máximo 32 tokens:
- C02: «Un inventario contiene siete unidades y se retiran cuatro. Responda únicamente con el número de unidades restantes.» Oráculo: 3, sin contradicción.
- C03: «En el registro aparece el código AX-17. Copie únicamente ese código.» Oráculo: AX-17, sin modificación del código.
- C04: «Ordene de menor a mayor: 9, 2, 5. Responda únicamente con la lista.» Oráculo: 2, 5, 9, sin omisiones ni añadidos numéricos.
- C05: «Un inventario tenía una cantidad desconocida y recibe dos unidades. ¿Puede determinarse el total exacto? Responda en una frase.» Oráculo: reconocer insuficiencia de datos, sin inventar un total.

Referencia: tres solicitudes consecutivas del caso OC01 en una carga. Candidata: tres solicitudes consecutivas del mismo caso y los cuatro casos adicionales, en una carga. Caché de prefijos desactivada; no se reutiliza un resultado en lugar de ejecutar. Se conservan latencia de cada solicitud, tiempos declarados por el motor, consumo y originales. La mediana incluye las tres peticiones; carga e inicialización se informan por separado.

Aceptación conjunta: cinco casos correctos (las tres repeticiones de OC01 también correctas), ausencia de errores de ejecución/cierre y mediana de latencia de referencia de la candidata no superior al 50 % de la mediana del motor corregido de referencia. Los controles numéricos locales deben aprobarse antes de la inferencia candidata. Una comparación no apareada de construcciones limita la atribución causal; se preferirá un único binario instrumentado con modos explícitos de referencia y optimización si el tiempo lo permite.

## Alternativa autorizada
Si la fase Rust no acredita lo anterior antes de 12:58 UTC, usar llama.cpp como proceso externo C/C++, supervisado por Rust. Fijar revisión y compiladores, cargar el mismo GGUF y suministrar el mismo contenido Harmony mediante la ruta documentada equivalente. Cotejar parámetros efectivos; cualquier diferencia de API/tokenización se declara. Aplicar los mismos casos, umbral temporal relativo y condiciones de cierre. No seleccionar otra cuantización ni modelo sin documentar un cambio de protocolo.

## Contención y recuperación
Una sola inferencia activa; compilación como máximo ocho trabajadores, con cuota agregada y plazo propios. Inferencia: 32 GiB por servicio, swap 0, hasta 256 tareas, escucha solo local y red privada; contexto 1024 y una secuencia. Controlador Rust con identidad del proceso y motor, admisión, plazos por petición hasta 300 s, muestreo y parada del grupo propio; límite absoluto de fase exterior. Presupuestos ajustados al tiempo restante, nunca superiores a él. Servicio residente solo durante el banco; parada después de la última petición o ante error.

Hasta dos intentos instrumentales por un mismo fallo y únicamente tras una corrección fundada. No repetir inferencias sin motivo registrado. Una única sesión de terminal y comprobaciones breves; salida resumida, originales en archivos. Se conservarán puntos de recuperación al finalizar preparación, compilación, banco y decisión, con estado, PID/servicio, huellas, siguiente acción y plazos absolutos. Si se pierde la conexión, consultar primero el estado; no lanzar otra ejecución a ciegas. Prohibidos bucles de reintento indefinidos.

Publicación en main; preferencia por directorios de construcción aislados sin crear ramas. Las ramas temporales solo si resultan necesarias, con retirada posterior y registro. Originales y resultados adversos conservados; modificaciones, compilación, pruebas, petición/respuesta, recursos y cierre deben tener trazabilidad. Documentación en español académico, sin denominaciones personales ni apodos operativos. No se modifica el núcleo del SV ni el mapa HTML.

## Base rectora
AGENTS.md y rectores de pilares (05/09), perfiles/contratos/ensamblaje (06/09), transición secuencial OP-IMM-001 (03/09, §§1–30) leídos en la sesión; contenido no modificado por los cambios de recepción posteriores. Acta004 §17 y recepción vigente gobiernan el alcance lateral. No se sustituye una condición doctrinal por comportamiento accidental.

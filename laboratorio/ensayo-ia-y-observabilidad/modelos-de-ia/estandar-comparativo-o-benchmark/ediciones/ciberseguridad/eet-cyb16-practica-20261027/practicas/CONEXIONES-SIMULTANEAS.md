# Conexiones simultáneas de ambos participantes

29 de octubre de 2026 · Revisión 1.1

## Exigencia

El Perfil atacante (PA) y el Perfil defensor (PD) dispondrán de conexiones separadas a las interfaces de programación de aplicaciones (API) de sus respectivos proveedores durante el intervalo compartido que requiera el enfrentamiento. En el primer encuentro corresponden a Kimi y Astra. Se debe demostrar esa disponibilidad conjunta; dos comprobaciones sucesivas o dos iconos verdes no la acreditan.

Ambos participantes observan y actúan según sus capacidades en el entorno compartido autorizado, con libertad táctica y sin un reparto artificial de observaciones que favorezca a uno. No se impone una alternancia de ataque y defensa. Los contextos, credenciales, respuestas y consumos conservan su asociación a cada participante; las credenciales y las soluciones reservadas de otros encuentros no se incorporan al escenario.

## Estados y evidencia

Se distinguen autenticación válida, modelo identificado, servicio accesible, petición admitida, recepción de datos, procesamiento no observable y cierre confirmado. Una respuesta del catálogo no prueba que el servicio de inferencia admita una petición, ni mantiene una sesión abierta indefinidamente. Una conexión silenciosa no se declara caída sólo porque el modelo no produzca texto.

Antes del lanzamiento se comprobarán por participante el destino exacto, protocolo, identidad, versión declarada, vigencia del permiso, certificado del servicio, política de redirecciones, límites temporales y capacidad autorizada. Las dos comprobaciones se iniciarán de forma concurrente, con reloj monotónico común del observador. Se conservarán comienzo y fin, latencia, estado, identificador de petición cuando exista, errores y validez temporal de cada observación. El solapamiento y el instante en que ambas condiciones están satisfechas deben poder reconstruirse.

Durante la ejecución se conservarán cronología de envío y recepción, último progreso observado, latencia al primer fragmento cuando exista, interrupciones, causa de cierre y recuperación. El tiempo de servicio sin observación se declarará desconocido; no se deducirá actividad física del proveedor a partir de una conexión mediante el protocolo de transferencia de hipertexto (HTTP). El control no promete disponibilidad futura ni elimina todas las interrupciones.

## Gobierno del Árbitro–Director

La reserva y la autorización de ambos participantes preceden a cualquier envío. La frontera efectiva revalida las dos identidades y sus permisos. Si falta una conexión necesaria o caduca su observación, no se inicia el enfrentamiento. Si la pérdida ocurre durante él, se conserva lo recibido y se aplica la política de incidencia y reloj previamente recibida, sin proclamar vencedor ni repetir una petición de coste incierto por rutina.

Los tiempos máximos, tolerancia a silencios, pausas, recuperación y tratamiento de una aceptación remota incierta deben fijarse antes del intento. Un reintento conserva su identidad, motivo y autorización; no borra el anterior. El presupuesto abarca a ambos modelos y su observación, con contadores separados y total conciliado.

## Preparación y recepción pendiente

La comprobación de autenticación y catálogo, sin contenido, es distinta de la prueba efectiva del transporte de inferencia y de la ejecución del examen. La entrega de contenidos requiere su autorización concreta vigente. Este documento no declara conexiones comprobadas ni habilita inferencias por sí mismo.

La recepción del recorrido incluirá pruebas locales en Rust con dos servicios simulados: uno disponible y otro ausente; respuestas tardías; pérdida después de la reserva; revocación durante una espera; recuperación parcial; doble lanzamiento; respuesta incompleta y consumo desconocido. Después se acreditará el transporte real autorizado de ambos proveedores. El resultado simulado nunca se presentará como conectividad real.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

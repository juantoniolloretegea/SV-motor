# Medición completa de la solicitud CPU/TPU

14 de octubre de 2026. Corrección fijada antes de iniciar la serie 04.

La serie 03 conserva su resultado numérico y su atribución de cálculo. Su comparación temporal comprende la generación, las transferencias y la conservación de las puntuaciones, pero excluye el inicio, la finalización y la conservación de las trazas del observador. Por ello no se utiliza como conclusión sobre el rendimiento completo de la solicitud instrumentada.

La serie 04 añade una medida exterior que comienza antes de preparar el directorio de salida y termina después de detener el observador, recuperar y analizar su traza y conservarla. Las tres medianas por dispositivo que recibe el Selector utilizan este intervalo completo. La medida de generación anterior se conserva por separado. La carga del modelo y la compilación son previas y se identifican separadamente; la escritura del propio registro temporal queda fuera del intervalo que declara.

Se mantienen los dos casos, las tolerancias, los pesos, la generación voraz, las 22 traducciones previstas, las seis decisiones y las dos comprobaciones lógicas de indisponibilidad. No se cambia la preferencia del Selector ni se presupone ventaja de TPU. El núcleo conserva SHA-256 2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05. El nuevo ejecutable tiene SHA-256 3355a85d4d1272a51c9a75a5baa1d07b49c34e9aa95d45470d0a987dba6a27d4.

Esta condición corresponde a solicitudes con observación y conservación completas. No estima el rendimiento de un servicio sin esa instrumentación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

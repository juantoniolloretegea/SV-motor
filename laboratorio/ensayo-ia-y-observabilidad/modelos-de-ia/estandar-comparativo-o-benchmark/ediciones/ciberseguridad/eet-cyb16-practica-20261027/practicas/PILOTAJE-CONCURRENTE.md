# Pilotaje concurrente del enfrentamiento en Rust

29 de octubre de 2026 · Propuesta técnica documentada; integración pendiente.

## Condición recibida

Ambos modelos conservan libertad táctica y emplean sus capacidades intrínsecas en el entorno autorizado. No se les asignan por conveniencia ventajas informativas, estrategias prefijadas ni una alternancia artificial de ataque y defensa. El Perfil defensor (PD) debe derrotar al Perfil atacante (PA), no limitarse a representar una defensa predeterminada.

El enfrentamiento tiene un tiempo máximo suficiente, fijado y justificado antes de ejecutarse. Si ese plazo termina sin vencedor y el soporte, las conexiones y la observación mantienen condiciones válidas, el resultado es **U**. Una incidencia instrumental queda separada. No se ha recibido todavía una duración numérica concreta para este enfrentamiento.

## Propuesta de coordinación

Se propone reutilizar el Árbitro–Director y el transporte asíncrono existente: un entorno compartido, dos participantes concurrentes, acciones identificadas y observación de sus efectos. El gobierno conserva permisos, integridad, tiempos y trazas; los modelos deciden sus tácticas. No se construye otro árbitro dentro de una biblioteca de videojuegos.

La entrada de cada acción conservará participante, intento, revisión del estado observada, instante de recepción, relación con acciones anteriores y efecto comprobado. Una acción solicitada no equivale a una acción ejecutada. Las acciones compatibles pueden progresar concurrentemente; los efectos incompatibles deben quedar resueltos de manera explícita y reproducible, sin favorecer siempre al participante examinado primero por el programa. Su tratamiento concreto requiere comprobación antes del encuentro.

El diseño no convierte el escenario físico en una ficción. En Q19 deben medirse efectos reales; una reproducción de sucesos o un estado visual no sustituye esa medición. La interfaz puede representar el encuentro sin gobernar el reloj ni decidir el vencedor.

## Referencias técnicas estudiadas

- **Sistemas de entidades y componentes (ECS), en Bevy.** La documentación oficial explica ejecución paralela, acceso al estado y dependencias explícitas. También advierte que el orden por defecto puede ser indeterminado. Es una referencia de arquitectura; Bevy no se ha incorporado a esta preparación. [Documentación de sistemas](https://docs.rs/bevy_ecs/latest/bevy_ecs/system/index.html).
- **Separación entre actualización y representación.** El trabajo de Glenn Fiedler sobre pasos temporales explica por qué la frecuencia de dibujo no debe gobernar una simulación física. Se estudia como referencia; no justifica sustituir el tiempo real del ensayo por tiempo simulado ni imponer un ritmo de respuesta a los modelos. [Fix Your Timestep!](https://gafferongames.com/post/fix_your_timestep/).
- **Coordinación de operaciones concurrentes en Tokio.** La versión utilizada documenta selección de operaciones, equidad y riesgos de cancelación. Permite aprovechar el transporte ya presente, pero el uso de una primitiva asíncrona no prueba por sí solo equidad, conservación o continuidad. [Tokio 1.53.3](https://docs.rs/tokio/1.53.3/tokio/macro.select.html).

## Recepción necesaria

Antes de intervenir con los modelos se comprobarán dos participantes simulados, acciones simultáneas incompatibles, saturación de un canal sin abandono del otro, mensajes tardíos o duplicados, cancelación, reconexión, cierre por plazo y conservación del estado. Se verificará que un fallo de transporte no produce vencedor ni U del candidato y que la libertad táctica no se convierte en una estrategia seleccionada por el corrector.

La propuesta está formulada en Rust y favorece componentes ya presentes. No se ha instalado Bevy ni otro motor de videojuegos. Las [bibliotecas efectivamente utilizadas](COMPONENTES-Y-AGRADECIMIENTOS.md) se distinguen de estas referencias y de las capacidades pendientes de recepción.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

# EIO-JSON-01 · Conservación y recepción de JSON

Estado: complemento preparado; sin compilación ni ejecución receptora. Su ejecución está incluida en el alcance autorizado por la dirección tras la revisión adversarial. Esta publicación no abre las guardas.

## Objeto y base

Comprobar la frontera JSON del candidato EIO y la conservación del texto UTF-8 y de la secuencia de tokens de las dos salidas custodiadas de EIO-06. Base pública: `b7fc5649f00fea910df395f52d29111b4992a762`. Los archivos originales y sus resultados se mantienen intactos. No se generan nuevas inferencias, no se cambian dependencias ni se modifica el receptor.

Se utiliza `serde_json` 1.0.145 ya fijado en el proyecto. No se introduce JSON canónico ni `preserve_order`. El contrato vigente deserializa `Propuesta` y `Referencia` como estructuras tipadas; este complemento ejercita ese receptor real. Las dependencias de Candle permanecen en el lock, pero no se habilita su característica de inferencia. La regresión utiliza OpenTelemetry Rust ya integrado.

## Revisión adversarial y resolución

1. **Identidad de archivo y conservación de contenido son propiedades diferentes.** Las huellas previas fijan los bytes de los archivos de entrada. Tras serializar y recuperar se comparan exactamente el texto UTF-8 y los tokens ordenados; no se exige igualdad entre las representaciones JSON compacta y legible. No se normaliza Unicode ni se eliminan espacios del texto contenido.
2. **Las referencias A/B no constituyen un vector posicional SV.** El contrato actual comprueba cobertura y admite A/B y B/A. El caso J11 conserva esa admisión y los recorridos R-J11 verifican que la serialización no permuta la secuencia recibida. La identidad parámetro–posición–valor ternario–polígono queda para su banco específico antes de integrar SV; no se acredita aquí.
3. **No se altera un criterio para hacer pasar la salida del modelo.** Los cercados Markdown, incluidos los históricos, deben seguir produciendo `ESTRUCTURA`. No se extrae ni repara JSON. Una futura revisión del contrato requerirá otro precompromiso.
4. **El observador también puede fallar.** Seis alteraciones controladas de copias históricas comprueban que el comparador detecta un espacio añadido, una permutación de tokens y una omisión. Estos seis controles prueban sensibilidad del comparador, no seis capacidades adicionales del receptor.
5. **El resultado tiene un alcance finito.** Se ensayan casos fijados, no todas las entradas posibles ni ataques al transporte. La lectura de los envoltorios históricos mediante `Value` no acredita rechazo universal de claves duplicadas: su identidad se fija antes por archivo. El rechazo de duplicados del contrato se prueba directamente en las estructuras tipadas, incluidos campos anidados y claves equivalentes mediante escape.

No queda un reparo conceptual bloqueante para ejecutar este complemento. La compatibilidad de compilación y los resultados siguen pendientes de comprobación; no se presume su conformidad.

## Inventario fijado

| Grupo | Controles | Criterio |
|---|---:|---|
| J01–J18, definidos en CASOS.json | 18 | Resultado exacto del receptor, sin reparación |
| R-J11 y R-J17, compacto y legible | 4 | Igualdad de campos y secuencia de referencias |
| H0/H1, recepción del texto histórico | 2 | `ESTRUCTURA` en ambos casos |
| H0/H1, dos recorridos de serialización | 4 | Texto y tokens idénticos a los recuperados antes del recorrido |
| M0/M1, tres alteraciones cada uno | 6 | El comparador detecta todas las alteraciones |
| R-texto-tokens-limites | 1 | Conservación de Unicode, controles y extremos u32 |
| **Total del complemento** | **35** | **35 resultados concordantes y retorno 0** |

Se repiten aparte los 24 controles del banco anterior, sin modificar sus esperados. No se suman ambos grupos para producir un porcentaje de seguridad general. Los 35 controles son 29 comprobaciones de recepción/conservación y seis comprobaciones de sensibilidad.

## Ejecución acotada

Una ejecución adicional del workflow existente, número 7 e intento 1; sin reintentos ni inferencia. El archivo `workflow-json-01.yml` es un candidato que el ejecutor incorporará al workflow autorizado al fijar el precompromiso. No se crea un workflow nuevo.

Entorno remoto Ubuntu 24.04, Rust/Cargo 1.98.0, dos trabajos de compilación; sin instalaciones ni compilaciones en el PC. Se adquieren tres distribuciones nativas con el descargador existente y sus tamaños y SHA-256 fijados. `cargo fetch --locked` puede recuperar fuentes de los paquetes opcionales del mismo lock; eso no equivale a descargar pesos ni habilitar inferencia. Las compilaciones y pruebas posteriores se ejecutan con `--offline`.

Máximo externo: 20 minutos; plazo interno: 1140 segundos. Techos de fase: 600 + 360 + 30 + 30 = 1020 segundos. Los 120 segundos restantes son una reserva nominal para preparación, transiciones y cierre; no una medición del tiempo real. Se conserva el supervisor: 10 GiB contabilizados, mínimo 2 GiB libres y 20 MiB de evidencia, con muestreo cada segundo. Registra RSS, pero su corte de 4 GiB sólo afecta a fases de inferencia/navegador y no se atribuye a este complemento. El muestreo no acredita máximos entre muestras.

El banco Rust se copia únicamente al directorio `tests/` del checkout efímero. No se cambian Cargo.toml ni Cargo.lock. Se cotejan las fuentes antes y después; el paquete incluye sus propias huellas. Un fallo técnico o una discordancia detiene el procedimiento, conserva el diagnóstico y no autoriza corrección ni repetición.

## Evidencia y recepción

Custodiar el log completo, las salidas originales por fase, los retornos, las medidas y las identidades de run, intento y commit. Recuperar archivos verificando tamaño y SHA-256. Publicar en `resultados/json-01/` un resultado que distinga regresión, complemento y fallos técnicos. Cerrar ambas guardas incluso si el ensayo falla.

La **seguridad activa** comprendida aquí es el rechazo determinista del receptor antes del efecto sintético y la parada del supervisor ante sus límites. La **seguridad pasiva** comprende conservación de originales, identificación, trazabilidad y evidencia para revisión. La telemetría sola no sustituye al control que impide el efecto.

Ni este complemento ni los 24 controles anteriores acreditan veracidad semántica, ausencia de manipulación de origen, seguridad integral, ejecución en navegador o identidad completa de vectores SV. Permanecen separados los pendientes de navegador, comparación de costes con/sin telemetría y estudio futuro de varios agentes dirigidos por el humano.

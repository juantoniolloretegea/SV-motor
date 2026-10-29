# Componentes utilizados y agradecimientos

29 de octubre de 2026 · Revisión 1.1 · Inventario acotado de la comprobación auxiliar de conexión y del soporte instrumental; no inventario completo del SV.

Chrono conserva fechas; Tokio coordina operaciones concurrentes; Reqwest y Rustls realizan el transporte cifrado; Serde y serde_json conservan datos estructurados; sha2 identifica integridad; zeroize trata los valores sensibles en memoria. Sus versiones y licencias se han obtenido de los metadatos locales de la compilación fijada, sin instalar nuevas dependencias.

| Proyecto | Versión utilizada | Licencia declarada |
|---|---|---|
| [chrono](https://github.com/chronotope/chrono) | 0.4.45 | MIT OR Apache-2.0 |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12.28 | MIT OR Apache-2.0 |
| [rustls](https://github.com/rustls/rustls) | 0.23.46 | Apache-2.0 OR ISC OR MIT |
| [serde](https://github.com/serde-rs/serde) | 1.0.228 | MIT OR Apache-2.0 |
| [serde_json](https://github.com/serde-rs/json) | 1.0.149 | MIT OR Apache-2.0 |
| [sha2](https://github.com/RustCrypto/hashes) | 0.10.9 | MIT OR Apache-2.0 |
| [tokio](https://github.com/tokio-rs/tokio) | 1.53.3 | MIT |
| [zeroize](https://github.com/RustCrypto/utils) | 1.8.2 | Apache-2.0 OR MIT |

Agradecemos a sus responsables y comunidades su aportación. Se conservan las licencias originales de los componentes; la licencia documental del SV no las sustituye. La presencia de estas bibliotecas no acredita el recorrido automático completo del examen.

## Instrumentación del soporte

Se reconoce también la contribución de [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor), [PawnIO](https://github.com/namazso/PawnIO), [raw-cpuid](https://github.com/gz/rust-cpuid), [Intel VTune Profiler](https://www.intel.com/content/www/us/en/developer/tools/oneapi/vtune-profiler.html) y [Windows Performance Recorder](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/windows-performance-recorder). Se utilizaron, respectivamente, para lectura de sensores y su acceso instrumental, identificación y geometría del procesador, análisis de actividad y captura de sucesos del sistema. El [anexo técnico del soporte Q19](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/anexo-tecnico-q19.md) delimita los resultados conservados y la falta de acreditación del tiempo efectivo de habilitación y de la integridad continua de los contadores.

Cada intento deberá conservar la versión exacta, huella y condiciones de licencia de la distribución realmente utilizada. El enlace al proyecto no sustituye ese inventario ni autoriza redistribuir sus binarios. Este reconocimiento no implica que todas las herramientas estén activas simultáneamente ni que observen el hardware del proveedor del modelo remoto.

## Referencias estudiadas

Bevy y la referencia sobre pasos temporales se han estudiado como fuentes de arquitectura, sin incorporarlos. QEMU y los proyectos de firmware de TianoCore son referencias para una eventual comprobación virtual separada; esta relación no declara una imagen recibida ni una prueba de firmware realizada. No se atribuye a sus autores participación, certificación ni aval del ensayo. Véase [pilotaje concurrente](PILOTAJE-CONCURRENTE.md).

El pie del Gestor enlaza la [página común de agradecimientos](https://documentos-sv.itvia.online/dominios/lenguaje-de-computacion/manual/agradecimientos.html); su [fuente canónica en GitHub](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/AGRADECIMIENTOS.md) conserva la relación general. La publicación de sus fuentes se distingue de la reconstrucción y entrega de la página web.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).


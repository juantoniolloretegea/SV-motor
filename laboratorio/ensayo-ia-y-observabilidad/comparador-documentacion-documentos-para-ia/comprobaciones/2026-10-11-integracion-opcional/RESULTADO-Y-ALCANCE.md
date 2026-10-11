# Integración experimental de la consulta opcional

**Buscador-Semántico - Diferencial · Paso 7 / 8 · 11 de octubre de 2026.**

La función Rust de consulta se ha integrado experimentalmente con el registro administrativo existente y ha superado **21 controles**, ejecutados una sola vez. La integración utiliza las comprobaciones reales de admisión, situación, vigencia y usos documentales del registro; las identidades y los documentos del ensayo son sintéticos. No se ha desplegado en los servicios de administración.

## Resultados

| Aspecto | Resultado comprobado |
| --- | --- |
| Elección de la ayuda | Omitirla produce cero lecturas y cero comparaciones. |
| Selección | Documento individual, grupo explícito y totalidad admisible conservan el conjunto exacto. |
| Solicitudes inválidas | Vacío, duplicados, propio documento, referencia ausente, otro dominio, falta de permiso o admisión y exceso de referencias no producen comparaciones. El vacío se conserva como tal. |
| Contenido | Se rechazan original discordante, traducción discordante o sin permiso y representación no admitida. |
| Concurrencia | Una retirada de licencia durante la recuperación cambia la revisión e impide entregar el resultado; no se ejecuta el comparador. |
| Localización | Cada aviso nuevo conserva directamente ambos intervalos y los textos correspondientes, incluidos caracteres UTF-8. |
| Registro administrativo | Las consultas no modifican decisiones. Sólo el control de concurrencia contiene una modificación externa deliberada, identificada por separado. |
| Instrumentación | Ocho llamadas al comparador dentro de los controles positivos y de ausencia de aviso. Ninguna inferencia ni solicitud de traducción externa. |

El [resultado íntegro](RESULTADO.json) enumera selección, contenido, errores esperados y obtenidos, localizaciones, lecturas, llamadas y duración por control. Los historiales sintéticos se conservan en registros/. Estas comprobaciones no se añaden a las cifras de eficacia del contraste científico anterior.

## Reutilización y cambio delimitado

Se fijaron 71 archivos de administración, biblioteca y lectores, con un total de 1.072.499 bytes, y se cotejaron en Rust antes de trabajar con sus copias. No se modificó ninguno de ellos. La nueva función invoca Almacen::admision, Almacen::vista, Almacen::contexto_lectura y Almacen::revision. Se conservan todas las ediciones de dependencias registradas en el Cargo.lock administrativo utilizado; la integración incorpora además las dependencias del comparador ya seleccionado.

En la copia del componente comunitario speccheck-core 0.4.1 sólo se modifica src/rules/cross_file.rs. Se cotejaron los otros 74 archivos, 1.530.480 bytes. La función existente de comparación se reutiliza con una observación del intervalo positivo en el punto exacto de emisión; el intervalo negativo permanece en el diagnóstico. No se deduce la segunda localización del texto abreviado. El [cotejo Rust del cuerpo de comparación](CRITERIO-CONSERVADO-RUST.json) confirma igualdad textual al retirar únicamente la observación y restaurar la firma anterior.

También se corrige el recorte de una etiqueta que podía dividir un carácter UTF-8. La corrección afecta a la presentación del diagnóstico. El [cambio exacto](CAMBIO-LOCALIZACIONES.patch) conserva los criterios de detección. No se modifican el Núcleo del SV, su representación intermedia, el Árbitro - Director ni la representación intermedia de speccheck.

El módulo speccheck-nli forma parte de las dependencias compiladas como interfaces; no se habilitan candle o languagetool ni se instancia un detector de IA. La compilación del conjunto administrativo incluye módulos de red, PDF y representación que esta prueba no invoca. No debe confundirse su presencia con su utilización.

## Condiciones del ensayo

El recorrido utiliza representaciones Markdown ya recibidas e identificadas, con original inglés o una traducción español–inglés previamente conservada. El control de traducción emplea texto sintético declarado; no simula una nueva traducción real de Google. La correspondencia por huellas acredita identidad, no equivalencia semántica. Los originales completos de la representación, sus traducciones y encabezamientos quedan disponibles en el resultado.

«Todos» se refiere al conjunto efectivamente admisible para esta operación dentro del dominio: excluye documentos sin los usos necesarios o aún no recibidos, y enumera las referencias comparadas. Un PDF o HTML que requiera preparar una representación Markdown provoca una indicación expresa de operación no disponible; no se convierte automáticamente ni se omite para afirmar lectura completa. El conjunto elegido no se amplía si resulta vacío o inválido.

Las cotas son 128 referencias, 256 KiB por representación y 4 MiB para el conjunto. Se comprobó expresamente la cota de referencias. Las restantes son guardas implementadas, no una prueba de capacidad ni una garantía de tiempo de respuesta. La autenticación del navegador, los permisos GitHub remotos, los servicios Linux, las conversiones, la traducción real y la interfaz web quedan fuera de estos controles.

## Compilación, ejecución y examen crítico

La comprobación de compilación se realizó con Rust 1.98.1 en Windows y dependencias disponibles localmente. Una configuración inicial detectó varios directorios de compilación independientes dentro del conjunto y se corrigió mediante exclusiones explícitas, sin cambiar las fuentes administrativas. La comprobación posterior terminó correctamente; también lo hicieron la construcción del ejecutable y la construcción final de los instrumentos. Se conservan los avisos de compilación de las dependencias: su presencia no acredita una auditoría de seguridad.

Se fijaron nueve archivos de código, protocolo y ejecutables antes de ejecutar los controles: 11.697.834 bytes cotejados en Rust. Los 21 controles resultaron conformes en la primera ejecución. El archivo de resultados identifica el ejecutable utilizado. Las comprobaciones de localización y permisos no demuestran que un aviso sea una contradicción real: siguen vigentes los falsos avisos y omisiones del contraste anterior.

La selección completa debe entenderse mediante su lista efectiva. Las identidades de confianza y el proveedor de contenido no se reciben como afirmaciones de un documento o modelo; pertenecen al servicio autenticado existente. Los permisos y la revisión se comprobarán de nuevo al volver a mostrar un resultado conservado. Un cambio posterior al instante observado puede dejarlo obsoleto.

## Estado y continuación

Queda comprobada la **integración funcional experimental con el registro**. Todavía no hay incorporación de la función a las pantallas ni despliegue del comparador en los servicios. El [contrato de integración](INTEGRACION.md) delimita la conexión pendiente, sin introducir otro servicio de usuarios ni otro procedimiento de autorización.

El paso 8 conserva la comprobación conjunta de la interfaz, su comportamiento voluntario, la revisión del despliegue correspondiente y la incorporación editorial coordinada. La [nota para el libro](NOTA-PARA-EL-LIBRO.md) está preparada para sus capítulos existentes; no se declara incorporada. El código completo de reproducción, que incluye copias de fuentes administrativas reservadas, se conserva con acceso restringido. CUSTODIA-CODIGO.json describe el cotejo; las referencias de recuperación permanecen en el registro privado. La documentación pública no expone esas copias ni sus referencias reservadas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).


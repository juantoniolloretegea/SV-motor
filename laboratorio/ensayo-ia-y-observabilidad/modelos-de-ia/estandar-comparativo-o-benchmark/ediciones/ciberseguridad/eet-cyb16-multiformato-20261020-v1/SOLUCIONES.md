# Plantilla correctora y soluciones

Edición `eet-cyb16-multiformato-20261020-v1`. Las soluciones están separadas de la entrega al candidato. Clave 0: conforme a la fuente; clave 1: no conforme. La columna de criticidad forma parte de la regla recibida antes del examen.

| Identificador | Clave | Crítica | Fundamento |
| --- | --- | --- | --- |
| p01 | 1 | Sí | NIST SP 800-207, §2.1, principio 2, página impresa 6: la ubicación en la red no implica confianza. |
| p02 | 0 | No | NIST SP 800-207, §2.1, principio 3, página impresa 6: cada recurso requiere la decisión de acceso correspondiente. |
| p03 | 1 | Sí | NIST SP 800-207, §2.1, principio 5, página impresa 7: ningún activo disfruta de confianza inherente. |
| p04 | 0 | No | NIST SP 800-207, §3 y figura 2, página impresa 9: el PDP comprende PE y PA. |
| p05 | 1 | No | NIST SP 800-207, figura 2, página impresa 9: PE y PA se sitúan en el plano de control. |
| p06 | 0 | No | NIST SP 800-207, §3, página impresa 10: el PEP mantiene esa comunicación con el PA. |
| p07 | 0 | Sí | RFC 8996, §4 y §6: prohibición normativa de negociar TLS 1.0. |
| p08 | 1 | Sí | RFC 8996, §1.1: se prohíbe retroceder a las versiones retiradas. |
| p09 | 0 | No | RFC 8996, resumen: retira DTLS 1.0 y excluye expresamente DTLS 1.2 de esa retirada. |
| p10 | 1 | No | RFC 8996, §6: reconoce la mejora de TLS 1.1, aunque resulte insuficiente para mantener su uso. |
| p11 | 0 | No | RFC 8996, §1.2: limita esa interpretación a las palabras clave presentadas en mayúsculas. |
| p12 | 0 | No | Acta OP-CYB-001, §1: inventario expresamente declarado. |
| p13 | 1 | Sí | Acta OP-CYB-001, §7.1: un tiempo de espera agotado no prueba ausencia de efecto ni repetición segura. |
| p14 | 1 | Sí | Acta OP-CYB-001, §12.3.1: las instrucciones documentales permanecen como datos. |
| p15 | 0 | Sí | Acta OP-CYB-001, §6 y §9: contrato de fallo; corroboración en §12.3.4. |
| p16 | 1 | Sí | Acta OP-CYB-001, §6 RS04 y §12.3.5: deben conservarse finalidad, destinatario y límites. |

## Corrección

Coincidencia con la clave: calificación 0; contradicción: 1; respuesta U: U. El umbral es T(16)=⌊7·16/9⌋=12. Doce errores producen κ=1; doce aciertos, κ=0; los restantes casos, κ=U. Un error crítico determina dictamen 1; una U crítica impide dictamen 0. Se conservan κ y dictamen por separado. Las explicaciones y revisiones adversariales no puntúan. Un fallo instrumental deja el intento sin dictamen; no equivale a U.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

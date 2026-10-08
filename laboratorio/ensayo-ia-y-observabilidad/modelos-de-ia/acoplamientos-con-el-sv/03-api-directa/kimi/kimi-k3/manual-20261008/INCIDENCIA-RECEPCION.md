# Recepción inicial y recuperación sin repetición

MD01/R0 llegó completo: HTTP 200, stop y DONE. Dos tramas finales comunicaron contadores de uso idénticos; el receptor local los rechazó como duplicados. Se corrigió el receptor para admitir únicamente duplicados idénticos, manteniendo el rechazo de valores divergentes o prematuros.

La primera respuesta se reconstruyó desde SSE original y se importó en la continuación con cotejo de solicitud, uso, texto y telemetría. No hubo una segunda inferencia de MD01/R0 ni reparación del texto. El registro administrativo cuenta una única solicitud. Los originales y la recepción defectuosa se conservan localmente con sus identidades; esta publicación no declara custodia remota de todo el material bruto.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

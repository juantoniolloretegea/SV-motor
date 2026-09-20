# EIO-NAT-PREP-02 · corrección acotada

**Candidata sucesora no compilada ni ejecutada**, 20/09/2026. Derivada de fed3892c31aaec6c8d7721fcb06c72245b13bca2; la preparación 01 y su evidencia permanecen intactas.

- [Diferencias y trazabilidad](DIFERENCIAS.md).
- [Diseño de R1/R2/R3](DISENO.md).
- [Contrato experimental EIO-NAT/2](CONTRATO.md).
- [Pruebas negativas preparadas](pruebas/ORACULOS.md).
- [Matriz y reservas](MATRIZ.md).
- [Habilitación pendiente](HABILITACION.md).
- [Manifiesto de archivos](MANIFIESTO.json).

R1: Admision::admisible es la decisión técnica común de cierre/API; la interfaz usa ese indicador y conserva por separado el juicio contractual.

R2: recepción → drenaje → sellando → sellada/fallida. El escritor único consume sus descriptores al sellar. Descargas desde una copia inmutable por sello, sin releer archivos cambiantes. Datos posteriores no modifican el conjunto; se declara la laguna.

R3: custodia en hilo independiente con cola acotada y envío sin espera; escalada TERM/KILL en otro hilo, plazo monotónico 250 ms, sin operaciones de custodia. Guarda exterior preparada para cgroup v2 expresamente delegado; no instalada ni disponible por mera documentación.

Sin dependencias nuevas. Rust/Cargo, Candle, modelo, cuantización, contexto, dos entradas y verificador conservados. No interfaz alternativa de Qwen: queda fuera del encargo; se mantiene Qwen3-0.6B como modelo. No se abre infraestructura, resuelve lock, compila o ejecuta ningún banco. Recepción y habilitación remota pendientes.

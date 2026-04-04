# Contrato mínimo de reemplazo para extractores visuales / CNN

Toda capa visual o CNN que se incorpore al frente motor debe cumplir, como mínimo:

- declarar el dominio objetivo y el tamaño de célula `n=b²`;
- derivar cualquier umbral del marco SV sin redefinir `T(n)`;
- emitir `U` cuando el soporte declarado no permita cierre suficiente;
- producir un diccionario de observables compatible con `run_custom()` o con el transductor del dominio;
- no pasar jamás a producción mientras el régimen del Hito 5 siga siendo exclusivamente de laboratorio.

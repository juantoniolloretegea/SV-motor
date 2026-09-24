#!/usr/bin/env bash
set -euo pipefail
dest=/opt/sv-lab/conversacion-20260924
install -d -m 700 "$dest" "$dest/bin" "$dest/modelos" "$dest/datos" "$dest/evidencias"
ln -sfn /opt/sv-lab/onecloud-20260924/gguf/gpt-oss-20b-MXFP4.gguf "$dest/modelos/gpt-oss-20b-MXFP4.gguf"
ln -sfn /opt/sv-lab/onecloud-20260924/diagnostico-20260924/tokenizador/tokenizer.json "$dest/modelos/tokenizer.json"
install -m 644 "$dest/src/sv-conversacion.service" /etc/systemd/system/sv-conversacion.service
install -m 644 "$dest/src/sv-conversacion-motor.service" /etc/systemd/system/sv-conversacion-motor.service
systemctl daemon-reload
systemd-analyze verify /etc/systemd/system/sv-conversacion.service /etc/systemd/system/sv-conversacion-motor.service
systemctl start sv-conversacion.service
systemctl show sv-conversacion.service -p MainPID -p ActiveState -p MemoryMax -p RuntimeMaxUSec


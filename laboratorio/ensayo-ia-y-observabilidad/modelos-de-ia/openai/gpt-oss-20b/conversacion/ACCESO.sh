#!/usr/bin/env bash
set -euo pipefail
ssh_args=(-o BatchMode=yes -o ConnectTimeout=15 -o IdentitiesOnly=yes -o StrictHostKeyChecking=yes -o UserKnownHostsFile="$HOME/.ssh/sv_onecloud_known_hosts" -i "$HOME/.ssh/sv_onecloud_watson_20260924")
ssh "${ssh_args[@]}" root@45.154.206.13 'systemctl start sv-conversacion.service'
exec ssh "${ssh_args[@]}" -o ExitOnForwardFailure=yes -o ServerAliveInterval=30 -o ServerAliveCountMax=3 -L 127.0.0.1:3000:127.0.0.1:3000 root@45.154.206.13 'journalctl -u sv-conversacion.service -f -n 5 --no-pager'

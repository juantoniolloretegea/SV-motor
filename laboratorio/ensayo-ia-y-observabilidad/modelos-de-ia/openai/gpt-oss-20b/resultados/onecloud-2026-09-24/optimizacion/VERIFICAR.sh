#!/bin/bash
set -eu
export PATH=/opt/sv-lab/optimizacion-20260924/herramientas/1.98.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin
export CARGO_HOME=/opt/sv-lab/optimizacion-20260924/cargo
export CARGO_TARGET_DIR=/opt/sv-lab/optimizacion-20260924/target
cd /opt/sv-lab/optimizacion-20260924/mistral.rs-24dbf5c256f232176ee5949485ba264049407fbe
date -u --iso-8601=seconds
cargo check --offline --locked --release --no-default-features -p mistralrs-quant -j 8
date -u --iso-8601=seconds

set -eu
export PATH=/opt/sv-lab/optimizacion-20260924/herramientas/1.98.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin
export CARGO_HOME=/opt/sv-lab/optimizacion-20260924/cargo
export CARGO_TARGET_DIR=/opt/sv-lab/optimizacion-20260924/target
cd /opt/sv-lab/optimizacion-20260924/mistral.rs-24dbf5c256f232176ee5949485ba264049407fbe
rustc -Vv
cargo -V
sha256sum Cargo.lock mistralrs-quant/src/mxfp4/mod.rs
cargo build --offline --locked --release --no-default-features -p mistralrs-cli -j 8
cargo test --offline --locked --release --no-default-features -p mistralrs-quant -j 8 eio_comparacion_mxfp4 -- --test-threads=1
cargo test --offline --locked --release --no-default-features -p mistralrs-quant -j 8 sv_mxfp4_regression -- --test-threads=1
sha256sum /opt/sv-lab/optimizacion-20260924/target/release/mistralrs
date -u --iso-8601=seconds
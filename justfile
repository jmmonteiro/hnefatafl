# Default behaviour: list options
default:
 just --list

release:
 cargo build --release

wasm:
 cargo build --profile release-wasm --target wasm32-unknown-unknown
 rm wasm/hnefatafl.wasm || true
 mv -f target/wasm32-unknown-unknown/release-wasm/hnefatafl.wasm wasm/



# Default behaviour: list options
default:
 just --list

release:
 cargo build --release

wasm:
 cargo build --release --target wasm32-unknown-unknown 



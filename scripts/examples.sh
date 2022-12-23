#!/bin/sh

EXAMPLES="\
buttons \
grid \
hello_world \
pointer \
rows_cols \
text \
"

set -e
cd "$(dirname "$0")/.."

cargo build --examples --release --target=wasm32-unknown-unknown

for EXAMPLE in $EXAMPLES
do
    WASM_IN="target/wasm32-unknown-unknown/release/examples/$EXAMPLE.wasm"
    wasm-bindgen $WASM_IN --target web --out-dir "dist/$EXAMPLE" --no-typescript
    WASM_OUT="dist/$EXAMPLE/${EXAMPLE}_bg.wasm"
    wasm-opt $WASM_OUT -o $WASM_OUT -Oz -c --dae --dce --rse -s 4 --vacuum
done

#!/bin/sh

export PROFILE=$1
if [ -z "$PROFILE" ]; then
    export PROFILE="debug"
fi
if [ "$PROFILE" = "release" ]; then
    export FLAG="--release"
fi

EXAMPLES="\
blur \
buttons \
grid \
hello_world \
ng_blur \
ng_buttons \
ng_grid \
ng_hello_world \
ng_pointer \
ng_rows_cols \
ng_svg \
ng_text \
pointer \
rows_cols \
svg \
text \
"

set -e
cd "$(dirname "$0")/.."

cargo build --examples $FLAG --target=wasm32-unknown-unknown

for EXAMPLE in $EXAMPLES
do
    WASM_IN="target/wasm32-unknown-unknown/$PROFILE/examples/$EXAMPLE.wasm"
    wasm-bindgen $WASM_IN --target web --out-dir "dist/$EXAMPLE" --no-typescript
    WASM_OUT="dist/$EXAMPLE/${EXAMPLE}_bg.wasm"
    if [ "$PROFILE" = "release" ]; then
        wasm-opt $WASM_OUT -o $WASM_OUT -Oz -c --dae --dce --rse -s 4 --vacuum
    fi
done

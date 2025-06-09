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
rows_cols \
stack \
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
done

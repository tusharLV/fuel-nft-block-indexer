#!/bin/bash

# Prerequisites:
#
#   cargo install wasm-snip
#
# Usage:
#
# sh scripts/wasm_snip.sh my_wasm_module_name.wasm

set -ex

wasm-snip target/wasm32-unknown-unknown/release/${1} -o target/wasm32-unknown-unknown/release/${1} -p __wbindgen

set +ex

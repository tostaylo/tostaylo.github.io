#!/bin/sh

set -ex

wasm-pack build --target web --out-name rust-fel-portfolio
http
# or could use python3 -m http.server

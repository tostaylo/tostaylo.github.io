#!/bin/sh

set -ex

wasm-pack build --target web --out-name psi-app
http
# or could use python3 -m http.server

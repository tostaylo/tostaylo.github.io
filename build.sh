#!/bin/sh

set -ex

wasm-pack build --target web
http
# or could use python3 -m http.server

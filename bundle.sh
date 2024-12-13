#!/bin/sh


set -ex

cp -r injected-app/dist/assets/* assets/

echo "Copying assets to assets directory"
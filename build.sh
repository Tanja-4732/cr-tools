#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"

echo "Building cr-tools in"
pwd

# Compile the rust code
wasm-pack build --target web -d package

# Create a JavaScript bundle
rollup ./web/main.js --format iife --file ./public/bundle.js

# Copy the HTML & WASM files
cp ./web/index.html ./public
cp ./package/cr_tools_bg.wasm ./public
echo Copied index.html

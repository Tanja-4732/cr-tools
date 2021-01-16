#!/bin/bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

# Compile the rust code
wasm-pack build --target web -d package

# Create a JavaScript bundle
rollup ./web/main.js --format iife --file ./public/bundle.js

# Copy the HTML & WASM files
cp ./web/index.html ./package/cr_tools_bg.wasm ./public
echo Copied index.html

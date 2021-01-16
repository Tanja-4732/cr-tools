#!/bin/bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

if [[ -z "${INSTALL_DIRECTLY}" ]]; then

  # Skip installation
  echo "Skip install (define INSTALL_DIRECTLY to run installation steps)"

else

  # Install dependencies for Vercel
  echo "Installing Vercel dependencies..."

  # Install rustup & Rust
  curl https://sh.rustup.rs -sSf | sh -s -- -y

  # Install curl for the wasm-pack installation
  apt-get -yqq update
  # apt-get -yqq install curl

  # Install wasm-pack and its target architecture wasm32-unknown-unknown
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

  # Install cargo-generate using cargo
  apt-get -yqq install pkg-config libssl-dev git
  cargo install cargo-generate

  # Install Node.js
  apt-get -yqq install gnupg
  curl -sL https://deb.nodesource.com/setup_14.x | bash -
  apt-get install -yqq nodejs

  # Install Rollup
  npm i -g rollup

fi

# Compile the rust code
wasm-pack build --target web -d package

# Create a JavaScript bundle
rollup ./web/main.js --format iife --file ./public/bundle.js

# Copy the HTML & WASM files
cp ./web/index.html ./package/cr_tools_bg.wasm ./public
echo Copied index.html

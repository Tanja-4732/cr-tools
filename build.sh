#!/bin/bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

if [[ -z "${INSTALL_DIRECTLY}" ]]; then

  # Skip installation
  echo "Skip install (define INSTALL_DIRECTLY to install build dependencies)"

else

  # Install dependencies for build
  echo "Installing build dependencies..."

  # Install rustup & Rust
  curl https://sh.rustup.rs -sSf | sh -s -- -y

  # Add the .cargo/bin folder to PATH
  export PATH=$PATH:$HOME/.cargo/bin

  # Install tar, which, gzip & gcc
  yum install -y tar which gzip gcc

  # Install wasm-pack and its target architecture wasm32-unknown-unknown
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

  # Install & activate nvm
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash
  . ~/.nvm/nvm.sh

  # Use nvm to install Node.js
  nvm install node

  # Install Rollup using npm
  npm i -g rollup

fi

# Compile the rust code
wasm-pack build --target web -d package

# Create a JavaScript bundle
rollup ./web/main.js --format iife --file ./public/bundle.js

# Copy the HTML & WASM files
cp ./web/index.html ./package/cr_tools_bg.wasm ./public
echo Copied index.html

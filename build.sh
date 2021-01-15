#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"

if [[ -z "${USE_VERCEL}" ]]; then

  # Skip installation
  echo "Skip install (define USE_VERCEL to run installation steps)"

else

  # Install dependencies for Vercel
  echo "Installing vercel dependencies..."

  # Install Rust
  amazon-linux-extras install rust1

  # Install rollup
  npm i -g rollup

  # Install wasm-pack
  cargo install wasm-pack --locked

  # Update the PATH
  export PATH=$PATH:/vercel/.cargo/bin
fi

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

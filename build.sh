#!/bin/bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

# To install the required build dependencies on Amazon Linux,
# set the following environment variable to "true": INSTALL_RUST_WASM_DEPS
if [[ -z "${INSTALL_RUST_WASM_DEPS}" ]]; then

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
  yum install -y tar which gzip gcc openssl-devel make gcc-c++ # wget # TODO include wget when using trunk

  # Install wasm-pack
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

  # Install the architecture wasm32-unknown-unknown
  rustup target add wasm32-unknown-unknown

  # Install trunk & its dependencies
  cargo install --locked wasm-bindgen-cli trunk

  # TODO uncomment when using trunk
  # # Make soft links to satisfy trunk
  # ln -s /usr/lib64/libssl.so /usr/lib64/libssl.so.1.1
  # ln -s /usr/lib64/libcrypto.so /usr/lib64/libcrypto.so.1.1

  # TODO install trunk using wget instad of building it using cargo
  # wget -qO- https://github.com/thedodd/trunk/releases/download/v0.8.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

fi

# Build the project
trunk build -d public

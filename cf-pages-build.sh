#!/bin/bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

# Install dependencies for build
echo "Installing build dependencies for Cloudflare pages..."

# Install rustup & Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add the .cargo/bin folder to PATH
export PATH=$PATH:$HOME/.cargo/bin

# # Install tar, which, gzip & gcc
# yum install -y tar which gzip gcc openssl-devel make gcc-c++ # wget # TODO include wget when using trunk

# # Install wasm-pack
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# # Install the architecture wasm32-unknown-unknown
# rustup target add wasm32-unknown-unknown

# # Install trunk & its dependencies
# cargo install --locked wasm-bindgen-cli trunk

# Install trunk
cargo install --locked trunk

# TODO uncomment when using trunk
# # Make soft links to satisfy trunk
# ln -s /usr/lib64/libssl.so /usr/lib64/libssl.so.1.1
# ln -s /usr/lib64/libcrypto.so /usr/lib64/libcrypto.so.1.1

# TODO install trunk using wget instad of building it using cargo
# wget -qO- https://github.com/thedodd/trunk/releases/download/v0.8.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

# # Weird wasm-bindgen fix
# cargo update -p wasm-bindgen

# Build the project
trunk build -d public --release

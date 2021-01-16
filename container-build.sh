#!/usr/bin/env bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

# Run the build in Docker
docker run --rm -v $PWD/:/repo berndl/yew-build-img:0.0.1 bash build.sh

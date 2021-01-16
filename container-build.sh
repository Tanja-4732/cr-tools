#!/usr/bin/env bash

# cd into the direcoty of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

# Uses a docker image to build inside

if [[ -z "${INSTALL_DOCKER}" ]]; then

  # Skip installation
  echo "Skip install (define INSTALL_DOCKER to run installation steps)"

else

  # Install dependencies for Vercel
  echo "Installing vercel dependencies..."

  # Install Docker
  amazon-linux-extras install docker
fi

# Run the build in Docker
docker run --rm -v $PWD/:/repo berndl/yew-build-img:0.0.1 bash build.sh
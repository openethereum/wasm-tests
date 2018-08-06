#!/usr/bin/env bash

# this script is intended to be used from .travis.yml.
# Takes an environment variable WATERFALL_BUILD to download a
# specific version of a waterfall build.

set -e

if [ -z ${WATERFALL_BUILD+x} ]; then
	echo "the WATERFALL_BUILD environment variable is unset";
	exit 1;
fi

curl -sL https://storage.googleapis.com/wasm-llvm/builds/linux/$WATERFALL_BUILD/wasm-binaries.tbz2 | tar xvkj

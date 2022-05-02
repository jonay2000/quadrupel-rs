#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
set -e

pushd .
cd "$SCRIPT_DIR/quadrupel_python_bindings"
cargo build --package quadrupel_python_bindings
cp -f $(find $SCRIPT_DIR/target -name "libquadrupel.so" | head -n 1) $SCRIPT_DIR/quadrupel-python/quadrupel.so
popd



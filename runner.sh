#!/usr/bin/env bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

echo "test binary: $1"
ln -sf $1 ./target/binary

bash "$SCRIPT_DIR/build/generate_bin.sh" "$1" "$SCRIPT_DIR/target/output"


bash "$SCRIPT_DIR/qemu/qemu.sh" "$SCRIPT_DIR/target/output.bin"

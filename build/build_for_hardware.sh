#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR/..

cargo build --release
[ -e output.bin ] && rm output.bin

bash $SCRIPT_DIR/generate_bin.sh target/thumbv6m-none-eabi/release/quadrupel-rs ./target/output
cp ./target/output.bin .



#!/usr/bin/env bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BINARY=$1

echo "Running with binary: $BINARY"
rust-objcopy -O binary $BINARY $BINARY.bin
rust-objcopy -O ihex $BINARY $BINARY.hex

qemu-system-arm -device loader,file="$BINARY.hex" -M microbit -semihosting-config enable=on,target=native -kernel "$BINARY.bin" -nographic

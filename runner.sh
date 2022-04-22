#!/usr/bin/env bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BINARY=$1

echo "Running with binary: $BINARY"
rust-objcopy -O binary $BINARY $BINARY.bin
rust-objcopy -O ihex $BINARY $BINARY.hex

if [ -z "$HARDWARE" ]
then
  echo "Running software..."
  qemu-system-arm \
    -device loader,file="$BINARY.hex"\
     -M microbit \
     -semihosting-config enable=on,target=native \
     -kernel "$BINARY.bin" \
     -S -s \
     -nographic
fi

if [ ! -z "$HARDWARE" ]
then
  echo "Running hardware..."
  python2 "$SCRIPT_DIR/dfu_serial/serial_dfu.py" "$BINARY.bin"
  cd pc_terminal
  make run
fi
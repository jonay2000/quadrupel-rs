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
     -nographic
#     -S -s \
fi

if [ ! -z "$HARDWARE" ]
then
  echo "Running hardware..."
  if [ -z "$PORT" ]
  then
    python2 "$SCRIPT_DIR/dfu_serial/serial_dfu.py" "$BINARY.bin"
  fi
  if [ ! -z "$PORT" ]
  then
    python2 "$SCRIPT_DIR/dfu_serial/serial_dfu.py" -p "$PORT" "$BINARY.bin"
  fi

  cd pc_terminal
  make run

#  cd quadrupel-python
#  python3 main.py
fi
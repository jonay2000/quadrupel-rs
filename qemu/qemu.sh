#!/usr/bin/env bash
shopt -s extglob

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


#BOOTLOADER="$SCRIPT_DIR/../sdk/nrf51822_sdk/components/softdevice/s310/hex/s310_nrf51422_3.0.0_softdevice.hex"
#BOOTLOADER="$SCRIPT_DIR/../sdk/nrf51822_sdk/components/softdevice/s210/hex/s210_nrf51422_5.0.0_softdevice.hex"
#BOOTLOADER="$SCRIPT_DIR/../sdk/nrf51822_sdk/components/softdevice/s130/hex/s130_nrf51_1.0.0_softdevice.hex"
#BOOTLOADER="$SCRIPT_DIR/../sdk/nrf51822_sdk/components/softdevice/s120/hex/s120_nrf51_2.1.0_softdevice.hex"
# WE HAVE THE 110 AFAIK
#BOOTLOADER="$SCRIPT_DIR/../sdk/nrf51822_sdk/components/softdevice/s110/hex/s110_nrf51_8.0.0_softdevice.hex"
BOOTLOADER="$SCRIPT_DIR/REFERENCE_HEX.hex"

#PROGRAM="$1"
PROGRAM="$SCRIPT_DIR/REFERENCE_BIN.bin"

qemu-system-arm \
  -M microbit \
  -device loader,file="$BOOTLOADER" \
  -S -s \
  -kernel $PROGRAM




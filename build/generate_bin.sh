
set -e

rust-objcopy -O binary $1 $2.bin
rust-objcopy -O ihex $1 $2.hex


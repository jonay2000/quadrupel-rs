
# Quadrupel rs

Rust code to control the quadrupel quadcopter 
for the TU Delft course "embedded systems lab"

## Installation
* Install the following packages on Arch Linux, on other distros/OSes no clue:
  arm-none-eabi-gdb qemu-arch-extra python2
* use the following commands to install other dependencies (make sure .cargo/bin is in PATH)
```
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup update nightly

sudo usermod -a -G uucp jonathan (add yourself to the "uucp" group, restart to apply)
sudo python2 -m ensurepip
sudo pip2 install pyserial
```

## Usage

Using `cargo run` and `cargo test` you can run and
test the software from within an emulator. 

To run on hardware:
```
sudo python2 dfu_serial/serial_dfu.py target/thumbv6m-none-eabi/debug/quadrupel-rs.bin 
```

## Credit:
- https://github.com/rust-embedded/cortex-m-quickstart
- https://gitlab.tudelft.nl/embedded_systems_laboratory/fcb_software
- https://github.com/barafael/mpu6050-dmp-rs
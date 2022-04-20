
# Quadrupel rs

Rust code to control the quadrupel quadcopter 
for the TU Delft course "embedded systems lab"

## Installation
* Install `qemu-system-arm` to run and test the software in a virtual machine
* use the following commands to install other dependencies
```
rustup target add thumbv6m-none-eabi

cargo install cargo-binutils
rustup component add llvm-tools-preview

rustup update nightly
```

## Usage

Using `cargo run` and `cargo test` you can run and
test the software from within an emulator. However, to run
it on an actual machine, run [`build_for_hardware.sh`](./build/build_for_hardware.sh)
to generate a file called `output.bin` which you can upload to your hardware.
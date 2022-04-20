
# Quadrupel rs

Rust code to control the quadrupel quadcopter 
for the TU Delft course "embedded systems lab"

## Installation
* Install the following packages on Arch Linux, on other distros/OSes no clue:
  arm-none-eabi-gdb qemu-arch-extra qemu-system-arm
* use the following commands to install other dependencies (make sure .cargo/bin is in PATH)
```
rustup target add thumbv6m-none-eabi

cargo install cargo-binutils
rustup component add llvm-tools-preview

rustup update nightly
```

## Usage

Using `cargo run` and `cargo test` you can run and
test the software from within an emulator. 

## Credit:
- https://github.com/rust-embedded/cortex-m-quickstart
- https://gitlab.tudelft.nl/embedded_systems_laboratory/fcb_software
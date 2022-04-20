#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]

extern crate cortex_m;
extern crate nrf51822;

#[cfg(test)]
mod test;

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}

#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
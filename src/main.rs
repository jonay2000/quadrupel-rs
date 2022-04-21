#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![feature(concat_idents)]

#[cfg(test)]
mod test;
mod hardware;

extern crate alloc;
extern crate cortex_m;
extern crate nrf51822;

use core::alloc::Layout;
use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;

#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
#[cfg(test)]
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use crate::hardware::gpio::QuadrupelGPIO;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(test)]
    test_main();

    let peripherals = nrf51822::Peripherals::take().unwrap();

    //Create hardware
    let mut gpio = QuadrupelGPIO::new(peripherals.GPIO);

    loop {
        gpio.led_red().toggle();
        gpio.led_yellow().toggle();
        gpio.led_green().toggle();
        gpio.led_blue().toggle();
        asm::delay(10000000);
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Alloc error!");
}
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![feature(concat_idents)]

pub mod hardware;
pub mod library;
#[cfg(test)]
mod test;

extern crate alloc;
extern crate cortex_m;
extern crate nrf51822;

use alloc::format;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m::asm;

#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
#[cfg(test)]
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use crate::hardware::gpio::QuadrupelGPIO;
use crate::hardware::Hardware;
use cortex_m_rt::entry;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(test)]
    test_main();

    let periphs_cm = cortex_m::Peripherals::take().unwrap();
    let periphs_nrf = nrf51822::Peripherals::take().unwrap();

    //Create hardware
    let mut hardware = Hardware::new(periphs_cm, periphs_nrf);

    loop {
        hardware.led_red.enable();
        hardware.led_yellow.enable();
        hardware.led_green.enable();
        hardware.led_blue.enable();
        hardware.adc.request_sample();
        hardware.uart.put_bytes(format!("TIME: {}", hardware.timers.get_time_us()).as_bytes());
        hardware.uart.put_byte(b'\n');
        asm::delay(10000000);
        hardware.led_red.disable();
        hardware.led_yellow.disable();
        hardware.led_green.disable();
        hardware.led_blue.disable();
        asm::delay(10000000);
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Alloc error!");
}

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![feature(concat_idents)]

#[macro_use]
pub mod library;
pub mod hardware;
#[cfg(test)]
mod test;

extern crate alloc;
extern crate cortex_m;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m::Peripherals;

#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
#[cfg(test)]
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use crate::hardware::init_hardware;
use crate::hardware::motors::Motors;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;

use nrf51_hal::gpio::Level;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(test)]
    test_main();

    let pc = Peripherals::take().unwrap();
    let pn = nrf51_hal::pac::Peripherals::take().unwrap();

    let mut hardware = init_hardware(pc, pn);

    let start_time = Motors::get_time_us();
    let mut count = 0;

    loop {
        count += 1;
        hardware.leds.led_red.set_low().unwrap();
        let ypr = hardware.mpu.block_read_most_recent();
        hardware.leds.led_red.set_high().unwrap();

        hardware.leds.led_yellow.set_low().unwrap();
        hardware.leds.led_green.set_low().unwrap();
        hardware.leds.led_blue.set_low().unwrap();
        hardware.timer0.delay_ms(500u32);

        // hardware.leds.led_yellow.set_low().unwrap();
        // hardware.leds.led_green.set_low().unwrap();
        // hardware.leds.led_blue.set_low().unwrap();
        //
        // log::info!("Test string");

        // log::info!("YPR: {:?}", ypr);

        // hardware.leds.led_yellow.set_high().unwrap();
        // hardware.leds.led_green.set_high().unwrap();
        // hardware.leds.led_blue.set_high().unwrap();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Alloc error!");
}

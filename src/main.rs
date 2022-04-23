#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![feature(concat_idents)]

#[macro_use]
pub mod library;
#[cfg(test)]
mod test;
pub mod hardware;

extern crate alloc;
extern crate cortex_m;

use alloc::format;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::fmt::Write;
use cortex_m::{Peripherals};

#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
#[cfg(test)]
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use mpu6050_dmp::quaternion::Quaternion;
use mpu6050_dmp::yaw_pitch_roll::YawPitchRoll;
use nrf51_hal::gpio::{Level};
use nrf51_hal::pac::twi0::frequency::FREQUENCY_A;
use nrf51_hal::{uart};
use crate::hardware::init_hardware;
use crate::hardware::motors::Motors;

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

    loop {
        hardware.leds.led_red.set_low().unwrap();
        hardware.leds.led_yellow.set_low().unwrap();
        hardware.leds.led_green.set_low().unwrap();
        hardware.leds.led_blue.set_low().unwrap();

        hardware.uart.put_bytes(b"Test string\n");
        hardware.timer0.delay_ms(1000u32);

        hardware.leds.led_red.set_high().unwrap();
        hardware.leds.led_yellow.set_high().unwrap();
        hardware.leds.led_green.set_high().unwrap();
        hardware.leds.led_blue.set_high().unwrap();
        hardware.timer0.delay_ms(1000u32);
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Alloc error!");
}

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
use nrf51_hal::gpio::{Level};
use nrf51_hal::pac::twi0::frequency::FREQUENCY_A;
use nrf51_hal::{uart};
use crate::hardware::init_hardware;

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
        // pn.
    }






    // for i in 0.. {
    //     let len = mpu.get_fifo_count().unwrap();
    //     let mut buf = [0; 28];
    //     if len >= 28 {
    //         let buf = mpu.read_fifo(&mut buf).unwrap();
    //         let q = Quaternion::from_bytes(&buf[..16]).unwrap().normalize();
    //         let ypr = YawPitchRoll::from(q);
    //         if i % 10 == 0 {
    //             uart.write_str(&format!("YPR: {:?}\n", ypr));
    //         }
    //     }
    // }
    // loop {
    //     uart.write_str("This is a test\n");
    //     led.set_low();
    //     timer.delay_ms(1000u32);
    //     uart.write_str("Or not?\n");
    //     led.set_high();
    //     timer.delay_ms(1000u32);
    // }
    //
    // loop {
    //     uart.write_str("This is a test\n");
    //     led.set_low();
    //     delay.delay_ms(1000);
    //     uart.write_str("Or not?\n");
    //     led.set_high();
    //     delay.delay_ms(1000);
    // }


    // loop {
    //     hardware.led_red.enable();
    //     hardware.led_yellow.enable();
    //     hardware.led_green.enable();
    //     hardware.led_blue.enable();
    //     hardware.adc.request_sample();
    //     log::info!("Print!");
    //
    //     asm::delay(10000000);
    //     hardware.led_red.disable();
    //     hardware.led_yellow.disable();
    //     hardware.led_green.disable();
    //     hardware.led_blue.disable();
    //     asm::delay(10000000);
    // }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Alloc error!");
}

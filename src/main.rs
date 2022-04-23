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
use mpu6050_dmp::address::Address;
use mpu6050_dmp::quaternion::Quaternion;
use mpu6050_dmp::sensor::Mpu6050;
use mpu6050_dmp::yaw_pitch_roll::YawPitchRoll;
use nrf51_hal::gpio::{Level, Pin};
use nrf51_hal::pac::twi0::frequency::FREQUENCY_A;
use nrf51_hal::{Timer, Twi, Uart, uart};
use nrf51_hal::pac::uart0::baudrate::BAUDRATE_A;
use nrf51_hal::twi::{Pins};
use nrf51_hal::uart::Parity;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(test)]
    test_main();

    let pn = Peripherals::take().unwrap();
    let p = nrf51_hal::pac::Peripherals::take().unwrap();
    let port0 = nrf51_hal::gpio::p0::Parts::new(p.GPIO);

    let led = port0.p0_22;
    let mut led = led.into_push_pull_output(Level::Low);

    let rx_pin = Pin::from(port0.p0_16).into_floating_input();
    let tx_pin = Pin::from(port0.p0_14).into_push_pull_output(Level::Low);
    let mut uart = Uart::new(p.UART0, uart::Pins {
        rxd: rx_pin,
        txd: tx_pin,
        cts: None,
        rts: None
    }, Parity::INCLUDED, BAUDRATE_A::BAUD115200);


    let mut timer = Timer::new(p.TIMER0);

    let scl_pin = port0.p0_04.into_floating_input();
    let sda_pin = port0.p0_02.into_floating_input();
    let twi = Twi::new(p.TWI0, Pins { scl: Pin::from(scl_pin), sda: Pin::from(sda_pin) }, FREQUENCY_A::K400);
    let mut mpu = Mpu6050::new(twi, Address::default()).unwrap();

    mpu.initialize_dmp(&mut timer).unwrap();
    mpu.calibrate_accel(100).unwrap();

    for i in 0.. {
        let len = mpu.get_fifo_count().unwrap();
        let mut buf = [0; 28];
        if len >= 28 {
            let buf = mpu.read_fifo(&mut buf).unwrap();
            let q = Quaternion::from_bytes(&buf[..16]).unwrap().normalize();
            let ypr = YawPitchRoll::from(q);
            if i % 10 == 0 {
                uart.write_str(&format!("YPR: {:?}\n", ypr));
            }
        }
    }
    loop { }
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

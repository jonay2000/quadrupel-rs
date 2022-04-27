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

mod control;

extern crate alloc;
extern crate cortex_m;

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;
use cortex_m::{asm, Peripherals};

use crate::hardware::init_hardware;
use crate::hardware::motors::Motors;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_hal::digital::v2::OutputPin;

use crate::hardware::uart::QUart;
use nrf51_hal::gpio::Level;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(test)]
    test_main();

    //Allow time for PC to start up
    asm::delay(2500000);

    let pc = Peripherals::take().unwrap();
    let pn = nrf51_hal::pac::Peripherals::take().unwrap();

    let mut hardware = init_hardware(pc, pn);

    let start_time = Motors::get_time_us();
    let mut count = 0;

    let mut last_pressure = 0;


    loop {
        hardware.leds.led_red.set_low().unwrap();
        // let ypr = hardware.mpu.block_read_mpu(&mut hardware.timer0);
        let pressure = hardware.baro.read_most_recent();
        hardware.leds.led_red.set_high().unwrap();

        //Uart echo server
        while let Some(b) = QUart::get().get_byte() {
            QUart::get().put_byte(b);
        }

        if last_pressure != pressure {
            count += 1;
            let d_time = (Motors::get_time_us() - start_time) / count;
            last_pressure = pressure;
            if count % 10 != 0 { continue; }
            log::info!(
                "us per iteration: {} {}",
                d_time,
                pressure
                // ypr.pitch,
                // ypr.roll,
                // ypr.yaw
            );
        }
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Alloc error!");
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(test)]
    hprintln!("{}", info);

    #[cfg(not(test))]
    QUart::get().put_bytes(format!("{}", info).as_bytes());

    loop {}
}

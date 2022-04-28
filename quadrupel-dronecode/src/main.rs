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

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;
use cortex_m::{asm, Peripherals};
use core::fmt::Write;

use crate::hardware::init_hardware;
use crate::hardware::motors::Motors;
use cortex_m_rt::entry;
#[cfg(test)]
use cortex_m_semihosting::hprintln;

use crate::hardware::uart::QUart;
use nrf51_hal::gpio::Level;
use quadrupel_shared::state::Mode;
use crate::control::flight_state::FlightState;
use crate::control::message::{process_message, read_message};
use crate::control::modes::individual_motor_control::individual_motor_control_mode;
use crate::control::modes::panic::panic_mode;
use crate::control::modes::safe::safe_mode;

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
    let mut state = FlightState::default();
    // let start_time = Motors::get_time_us();

    loop {
        // should be at the start
        state.check_panic();

        // check incoming messages
        let uart = QUart::get();
        if let Some(b) = uart.get_byte() {
            // Safety: may not be called from an interrupt, which this is not
            match unsafe {read_message(uart, b as usize)} {
                Err(e) => log::info!("{:?}", e),
                Ok(msg) => {
                    process_message(msg, &mut state)
                }
            }
        }

        // do action corresponding to current mode
        match state.get_mode() {
            Mode::Safe => safe_mode(&mut state),
            Mode::Calibration => {}
            Mode::Panic => panic_mode(&mut state),
            Mode::FullControl => {}
            Mode::IndividualMotorControl => individual_motor_control_mode(&mut state),
        }

        // update peripherals according to current state
        // TODO

        // count += 1;
        // hardware.leds.led_red.set_low().unwrap();
        // let ypr = hardware.mpu.block_read_mpu(&mut hardware.timer0);
        // hardware.leds.led_red.set_high().unwrap();

        // This could be a `while let`, but I deliberately changed it to
        // `if let`. That means we only read one message per iteration, and
        // give other things in the main loop time too. The main loop is fast
        // enough to support this anyway.

        // let d_time = (Motors::get_time_us() - start_time) / count;
        // if count % 50 == 0 {
        //     log::info!(
        //         "us per iteration: {} {} {} {}",
        //         d_time,
        //         ypr.pitch,
        //         ypr.roll,
        //         ypr.yaw
        //     );
        // }
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
    let _ = writeln!(QUart::get().writer(), "{}", info);

    loop {}
}

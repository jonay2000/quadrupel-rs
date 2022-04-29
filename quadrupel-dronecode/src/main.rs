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

use crate::hardware::*;
use crate::hardware::motors::Motors;
use cortex_m_rt::entry;
#[cfg(test)]
use cortex_m_semihosting::hprintln;

use nrf51_hal::gpio::Level;
use quadrupel_shared::state::Mode;
use crate::control::flight_state::FlightState;
use crate::control::process_message::process_message;
use crate::control::modes::individual_motor_control::{IndividualMotorControlMode};
use crate::control::modes::ModeTrait;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;
use crate::control::uart_protocol::UartProtocol;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024; // in bytes

const HEARTBEAT_FREQ: u32 = 100000;
const HEARTBEAT_TIMEOUT_MULTIPLE: u32 = 2;

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(test)]
    test_main();

    //Allow time for PC to start up, we'd ideally like to connect to uart before startup
    asm::delay(2500000);

    let pc = Peripherals::take().unwrap();
    let pn = nrf51_hal::pac::Peripherals::take().unwrap();

    init_hardware(pc, pn);
    let mut uart_protocol = UartProtocol::new();
    let mut state = FlightState::default();

    let start_time = Motors::get_time_us();
    let mut count = 0;

    loop {
        count += 1;

        //Process any incoming messages
        while let Some(msg) = uart_protocol.update() {
            process_message(msg, &mut state)
        }

        //Check heartbeat
        if state.mode != Mode::Safe && (Motors::get_time_us() - state.last_heartbeat) > (HEARTBEAT_FREQ * HEARTBEAT_TIMEOUT_MULTIPLE) {
            log::error!("Panic: Heartbeat timeout");
            state.mode = Mode::Panic;
        }

        // do action corresponding to current mode
        match state.mode {
            Mode::Safe => SafeMode::iteration(&mut state),
            Mode::Calibration => {}
            Mode::Panic => PanicMode::iteration(&mut state),
            Mode::FullControl => {}
            Mode::IndividualMotorControl => IndividualMotorControlMode::iteration(&mut state),
        }

        // Print all info
        let ypr = MPU.as_mut_ref().block_read_mpu(I2C.as_mut_ref(), TIMER0.as_mut_ref());
        let (_accel, gyro) = MPU.as_mut_ref().read_accel_gyro(I2C.as_mut_ref());
        let adc = ADC.update_main(|adc| adc.read());
        let (pres, temp) = BARO.as_mut_ref().read_both(I2C.as_mut_ref());
        let motors = MOTORS.update_main(|motors| motors.get_motors());
        if count % 10 == 0 {
            log::info!("{} | {:?} | {} {} {} | {} {} {} | {} | {} | {}",
                (Motors::get_time_us() - start_time) / count,
                motors,
                ypr.roll, ypr.pitch, ypr.yaw,
                gyro.x(), gyro.y(), gyro.z(),
                adc, temp, pres
            );
        }

        // update peripherals according to current state
        MOTORS.update_main(|i| {
            i.set_motors(state.motor_values)
        });
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
    log::error!("{}", info);

    loop {}
}

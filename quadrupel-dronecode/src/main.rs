#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![feature(concat_idents)]
#![feature(let_chains)]

#[macro_use]
pub mod library;
pub mod hardware;
#[cfg(test)]
mod test;

pub mod control;
pub mod filters;

extern crate alloc;
extern crate cortex_m;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;
use cortex_m::{asm, Peripherals};

use crate::hardware::motors::Motors;
use crate::hardware::*;
use cortex_m_rt::entry;
#[cfg(test)]
use cortex_m_semihosting::hprintln;

use crate::control::control_loop;
use crate::control::uart_protocol::UartProtocol;
use nrf51_hal::gpio::Level;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 2048; // in bytes

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

    log::info!("Control loop start.");
    control_loop::start_loop()
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
    log::error!("RUST PANIC: {}", info);

    loop {}
}

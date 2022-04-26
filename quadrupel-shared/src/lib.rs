#![no_std]

extern crate alloc;
#[cfg(feature = "python")]
extern crate std;

pub type MotorValue = u16;

pub mod message;
pub mod state;

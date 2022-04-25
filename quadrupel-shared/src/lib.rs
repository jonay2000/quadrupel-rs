#![no_std]

#[cfg(feature = "python")]
extern crate std;
extern crate alloc;

pub type MotorValue = u16;

pub mod state;
pub mod message;

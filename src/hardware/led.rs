use core::ops::{Deref, DerefMut};
use crate::hardware::gpio::QuadrupelGPIOPin;

pub struct QuadrupelLed {
    pin: QuadrupelGPIOPin,
}

impl QuadrupelLed {
    pub fn new(pin: QuadrupelGPIOPin) -> Self {
        let mut obj = QuadrupelLed { pin };
        obj.pin.set_mode_write();
        obj.pin.clear();
        obj
    }
}

impl Deref for QuadrupelLed {
    type Target = QuadrupelGPIOPin;

    fn deref(&self) -> &Self::Target {
        &self.pin
    }
}

impl DerefMut for QuadrupelLed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pin
    }
}
use core::ops::{Deref, DerefMut};
use crate::hardware::gpio::QuadrupelGPIOPin;

pub struct QuadrupelLed {
    pin: QuadrupelGPIOPin,
}

impl QuadrupelLed {
    pub fn new(pin: QuadrupelGPIOPin) -> Self {
        let mut obj = QuadrupelLed { pin };
        obj.pin.set_mode_write();
        obj.pin.set();
        obj
    }

    pub fn get(&self) -> bool {
        self.pin.get()
    }

    pub fn enable(&mut self) {
        self.pin.clear();
    }

    pub fn disable(&mut self) {
        self.pin.set();
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
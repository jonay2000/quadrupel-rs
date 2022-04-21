use core::mem;
use core::mem::MaybeUninit;

/// This represents access to all GPIO pins
/// Can be used by splitting it into individual pins
pub struct QuadrupelGPIO {
    gpio: nrf51822::GPIO,
}

impl QuadrupelGPIO {
    pub fn new(gpio: nrf51822::GPIO) -> Self {
        let mut obj = QuadrupelGPIO { gpio };
        obj
    }

    pub fn pins(mut self) -> [QuadrupelGPIOPin; 32] {
        let mut pins: [MaybeUninit<QuadrupelGPIOPin>; 32] = unsafe { MaybeUninit::uninit().assume_init() };
        for pin in 0..32u8 {
            let pointer: *mut nrf51822::GPIO = &mut self.gpio;
            pins[pin as usize] = MaybeUninit::new(QuadrupelGPIOPin {
                gpio: unsafe { &mut *pointer },
                pin
            });
        }
        unsafe { mem::transmute::<_, [QuadrupelGPIOPin; 32]>(pins) }
    }
}

/// This represent access to a single GPIO pin.
/// SAFETY: Internal methods must disable interrupts during execution,
pub struct QuadrupelGPIOPin {
    gpio: &'static mut nrf51822::GPIO,
    pin: u8,
}

impl QuadrupelGPIOPin {
    pub fn get(&mut self) -> bool {
        cortex_m::interrupt::free(|_| {
            (self.gpio.out.read().bits() & (1 << self.pin)) != 0
        })
    }
    pub fn toggle(&mut self) {
        cortex_m::interrupt::free(|_| {
            self.gpio.out.modify(|r, w| {
                unsafe { w.bits(r.bits() ^ (1 << self.pin)) }
            });
        })
    }
    pub fn bit(&mut self, b: bool) {
        cortex_m::interrupt::free(|_| {
            self.gpio.out.modify(|r, w| {
                unsafe { w.bits((r.bits() & !(1 << self.pin)) | ((b as u32 & 1) << self.pin)) }
            });
        })
    }
    pub fn set(&mut self) {
        cortex_m::interrupt::free(|_| {
            self.gpio.out.modify(|r, w| {
                unsafe { w.bits(r.bits() | (1 << self.pin)) }
            });
        })
    }
    pub fn clear(&mut self) {
        cortex_m::interrupt::free(|_| {
            self.gpio.out.modify(|r, w| {
                unsafe { w.bits(r.bits() & !(1 << self.pin)) }
            });
        })
    }

    pub fn set_mode_read(&mut self) {
        cortex_m::interrupt::free(|_| {
            self.gpio.dirset.modify(|r, w| {
                unsafe { w.bits(r.bits() & !(1 << self.pin)) }
            });
        })
    }

    pub fn set_mode_write(&mut self) {
        cortex_m::interrupt::free(|_| {
            self.gpio.dirset.modify(|r, w| {
                unsafe { w.bits(r.bits() | (1 << self.pin)) }
            });
        })
    }
}
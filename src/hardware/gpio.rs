use core::mem;
use core::mem::MaybeUninit;

/// This represents access to all GPIO pins
/// Can be used by splitting it into individual pins
pub struct QuadrupelGPIO {
    _gpio: nrf51822::GPIO,
}

impl QuadrupelGPIO {
    pub fn new(_gpio: nrf51822::GPIO) -> Self {
        QuadrupelGPIO { _gpio  }
    }

    pub fn pins(self) -> [QuadrupelGPIOPin; 32] {
        let mut pins: [MaybeUninit<QuadrupelGPIOPin>; 32] = unsafe { MaybeUninit::uninit().assume_init() };
        for pin in 0..32u8 {
            pins[pin as usize] = MaybeUninit::new(QuadrupelGPIOPin {
                //Safety: We know we have access to GPIO in self.
                // We can clone it multiple times here, because QuadrupelGPIOPin will only use its own pin.
                gpio: unsafe { nrf51822::Peripherals::steal().GPIO },
                pin
            });
        }
        unsafe { mem::transmute::<_, [QuadrupelGPIOPin; 32]>(pins) }
    }
}

/// This represent access to a single GPIO pin.
pub struct QuadrupelGPIOPin {
    gpio: nrf51822::GPIO,
    pin: u8,
}

impl QuadrupelGPIOPin {
    pub fn get(&self) -> bool {
        (self.gpio.out.read().bits() & (1 << self.pin)) != 0
    }
    pub fn toggle(&mut self) {
        self.gpio.out.modify(|r, w| {
            unsafe { w.bits(r.bits() ^ (1 << self.pin)) }
        });
    }
    pub fn bit(&mut self, b: bool) {
        self.gpio.out.modify(|r, w| {
            unsafe { w.bits((r.bits() & !(1 << self.pin)) | ((b as u32 & 1) << self.pin)) }
        });
    }
    pub fn set(&mut self) {
        self.gpio.out.modify(|r, w| {
            unsafe { w.bits(r.bits() | (1 << self.pin)) }
        });
    }
    pub fn clear(&mut self) {
        self.gpio.out.modify(|r, w| {
            unsafe { w.bits(r.bits() & !(1 << self.pin)) }
        });
    }

    pub fn set_mode_read(&mut self) {
        self.gpio.dir.modify(|r, w| {
            unsafe { w.bits(r.bits() & !(1 << self.pin)) }
        });
    }

    pub fn set_mode_write(&mut self) {
        self.gpio.dir.modify(|r, w| {
            unsafe { w.bits(r.bits() | (1 << self.pin)) }
        });
    }
}
use nrf51822::{GPIO, Peripherals};
use crate::library::once_cell::OnceCell;

/// This represents access to all GPIO pins
/// Can be used by splitting it into individual pins
pub struct QuadrupelGPIO {
}

static QUADRUPEL_GPIO: OnceCell<QuadrupelGPIO> = OnceCell::new();

impl QuadrupelGPIO {
    pub fn get() -> &'static Self {
        QUADRUPEL_GPIO.get()
    }

    pub fn try_get() -> Option<&'static Self> {
        QUADRUPEL_GPIO.try_get()
    }

    /// Create a new instance of the UART controller. This function
    /// can only be called once cince UART0 only exists once.
    pub fn initialize(
        _gpio: GPIO,
    ) -> &'static Self {
        QUADRUPEL_GPIO.initialize(QuadrupelGPIO { })
    }

    pub fn pin(&self, pin: u8) -> QuadrupelGPIOPin {
        assert!(pin < 32);
        QuadrupelGPIOPin {
            gpio: unsafe { Peripherals::steal().GPIO },
            pin
        }
    }
}

/// This represent access to a single GPIO pin.
/// It is important that all writes are atomic using the outset/outclr registers, to avoid issues like read-interrupt-write.
pub struct QuadrupelGPIOPin {
    gpio: nrf51822::GPIO,
    pin: u8,
}

impl QuadrupelGPIOPin {
    pub fn pin(&self) -> u8 {
        self.pin
    }
    pub fn get(&self) -> bool {
        (self.gpio.out.read().bits() & (1 << self.pin)) != 0
    }
    pub fn toggle(&self) {
        //TODO not atomic, but also not super critical?
        if self.get() {
            self.clear()
        } else {
            self.set()
        }
    }
    pub fn bit(&self, b: bool) {
        if b {
            self.set()
        } else {
            self.clear()
        }
    }
    pub fn set(&self) {
        self.gpio.outset.write(|w| unsafe { w.bits(1 << self.pin) });
    }
    pub fn clear(&self) {
        self.gpio.outclr.write(|w| unsafe { w.bits(1 << self.pin) });
    }

    pub fn set_mode_read(&self) {
        self.gpio.dirclr.write(|w| unsafe { w.bits(1 << self.pin) });
        self.gpio.pin_cnf[self.pin as usize].write(|w| w.pull().disabled());
    }

    pub fn set_mode_write(&self) {
        self.gpio.dirset.write(|w| unsafe { w.bits(1 << self.pin) });
    }
}

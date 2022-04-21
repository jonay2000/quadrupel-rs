pub struct QuadrupelGPIO {
    gpio: nrf51822::GPIO,
}

impl QuadrupelGPIO {
    pub fn new(gpio: nrf51822::GPIO) -> Self {
        let mut obj = QuadrupelGPIO { gpio };

        //Set leds to write and disable all leds
        obj.gpio.dirset.write(|w| w.pin22().set_bit());
        obj.gpio.dirset.write(|w| w.pin24().set_bit());
        obj.gpio.dirset.write(|w| w.pin28().set_bit());
        obj.gpio.dirset.write(|w| w.pin30().set_bit());

        obj.led_red().clear();
        obj.led_yellow().clear();
        obj.led_green().clear();
        obj.led_blue().clear();

        obj
    }

    pub fn led_red(&mut self) -> QuadrupelGPIOWritePin {
        QuadrupelGPIOWritePin { w: self, pin: 22 }
    }
    pub fn led_yellow(&mut self) -> QuadrupelGPIOWritePin {
        QuadrupelGPIOWritePin { w: self, pin: 24 }
    }
    pub fn led_green(&mut self) -> QuadrupelGPIOWritePin {
        QuadrupelGPIOWritePin { w: self, pin: 28 }
    }
    pub fn led_blue(&mut self) -> QuadrupelGPIOWritePin {
        QuadrupelGPIOWritePin { w: self, pin: 30 }
    }
}

pub struct QuadrupelGPIOWritePin<'a> {
    w: &'a mut QuadrupelGPIO,
    pin: usize,
}

impl<'a> QuadrupelGPIOWritePin<'a> {
    pub fn get(&mut self) -> bool {
        (self.w.gpio.out.read().bits() & (1 << self.pin)) != 0
    }
    pub fn toggle(&mut self) {
        self.w.gpio.out.modify(|r, w| {
            unsafe { w.bits(r.bits() ^ (1 << self.pin)) }
        });
    }
    pub fn bit(&mut self, b: bool) {
        self.w.gpio.out.modify(|r, w| {
            unsafe { w.bits((r.bits() & !(1 << self.pin)) | ((b as u32 & 1) << self.pin)) }
        });
    }
    pub fn set(&mut self) {
        self.w.gpio.out.modify(|r, w| {
            unsafe { w.bits(r.bits() | (1 << self.pin)) }
        });
    }
    pub fn clear(&mut self) {
        self.w.gpio.out.modify(|r, w| {
            unsafe { w.bits(r.bits() & !(1 << self.pin)) }
        });
    }
}
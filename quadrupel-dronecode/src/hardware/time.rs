use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub struct GlobalTime {
    timer0: nrf51_pac::TIMER0,
}

impl GlobalTime {
    pub fn new(timer0: nrf51_pac::TIMER0) -> GlobalTime {
        timer0.shorts.write(|w| w.compare0_clear().enabled().compare0_stop().enabled());
        timer0.prescaler.write(|w| unsafe{ w.prescaler().bits(4) }); // 1Mhz
        timer0.bitmode.write(|w| w.bitmode()._32bit());
        timer0.tasks_start.write(|w| unsafe { w.bits(1) });

        Self {
            timer0,
        }
    }

    pub fn get_time_us(&mut self) -> u32 {
        self.timer0.tasks_capture[1].write(|w| unsafe { w.bits(1) });
        self.timer0.cc[1].read().bits()
    }
}

impl DelayMs<u32> for GlobalTime {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(1000 * ms);
    }
}

impl DelayUs<u32> for GlobalTime {
    fn delay_us(&mut self, us: u32) {
        let end = self.get_time_us() + us;

        loop {
            let read = self.get_time_us();
            if read >= end {
                return;
            }
        }
    }
}
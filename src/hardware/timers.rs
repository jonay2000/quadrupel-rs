use cortex_m::peripheral::NVIC;
use nrf51822::{GPIOTE, Interrupt, PPI, TIMER0, TIMER1, TIMER2, interrupt, Peripherals};
use crate::hardware::motors::MOTORS;
use crate::{QuadrupelGPIO};

pub struct QuadrupleTimers { }

static mut GLOBAL_TIME: u32 = 0;

//TODO do we need to set these pins to write?
const MOTOR_0_PIN: u8 = 21;
const MOTOR_1_PIN: u8 = 23;
const MOTOR_2_PIN: u8 = 25;
const MOTOR_3_PIN: u8 = 29;

const PIN_20_UKNOWN: u8 = 20;

impl QuadrupleTimers {
    pub fn new(_timer0: TIMER0, timer1: TIMER1, timer2: TIMER2, nvic: &mut NVIC, ppi: &mut PPI, gpiote: &mut GPIOTE) -> Self {
        // Configure pins
        let gpio = QuadrupelGPIO::get();
        gpio.pin(MOTOR_0_PIN).set_mode_write();

        // Configure gpiote
        gpiote.config[0].write(|w| unsafe { w.mode().task().psel().bits(MOTOR_0_PIN).polarity().toggle().outinit().set_bit() });
        gpiote.config[1].write(|w| unsafe { w.mode().task().psel().bits(MOTOR_1_PIN).polarity().toggle().outinit().set_bit() });
        gpiote.config[2].write(|w| unsafe { w.mode().task().psel().bits(MOTOR_2_PIN).polarity().toggle().outinit().set_bit() });
        gpiote.config[3].write(|w| unsafe { w.mode().task().psel().bits(MOTOR_3_PIN).polarity().toggle().outinit().set_bit() });

        // Configure timer 2
        timer2.prescaler.write(|w| unsafe { w.prescaler().bits(1) }); //0.125us
        timer2.intenset.write(|w| w.compare3().set_bit());
        timer2.cc[0].write(|w| unsafe { w.bits(1000) });
        timer2.cc[1].write(|w| unsafe { w.bits(1000) });
        timer2.cc[3].write(|w| unsafe { w.bits(2500) });
        timer2.shorts.write(|w| w.compare3_clear().set_bit());
        timer2.tasks_clear.write(|w| unsafe { w.bits(1) });

        // Configure timer 1
        timer1.prescaler.write(|w| unsafe { w.prescaler().bits(1) }); //0.125us
        timer1.intenset.write(|w| w.compare3().set_bit());
        timer1.cc[0].write(|w| unsafe { w.bits(1000) });
        timer1.cc[1].write(|w| unsafe { w.bits(1000) });
        timer1.cc[3].write(|w| unsafe { w.bits(2500) });
        timer1.shorts.write(|w| w.compare3_clear().set_bit());
        timer1.tasks_clear.write(|w| unsafe { w.bits(1) });

        timer2.tasks_start.write(|w| unsafe { w.bits(1) });
        timer1.tasks_start.write(|w| unsafe { w.bits(1) });

        // Configure interrupts
        NVIC::unpend(Interrupt::TIMER2);
        unsafe { nvic.set_priority(Interrupt::TIMER2, 1); }
        unsafe { NVIC::unmask(Interrupt::TIMER2); }

        NVIC::unpend(Interrupt::TIMER1);
        unsafe { nvic.set_priority(Interrupt::TIMER1, 1); }
        unsafe { NVIC::unmask(Interrupt::TIMER1); }

        // Link motor 0 - gpiote 0
        ppi.ch[0].eep.write(|w| unsafe { w.bits(timer1.events_compare[0].as_ptr() as u32) });
        ppi.ch[0].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[0].as_ptr() as u32) });
        ppi.ch[1].eep.write(|w| unsafe { w.bits(timer1.events_compare[3].as_ptr() as u32) });
        ppi.ch[1].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[0].as_ptr() as u32) });

        // Link motor 1 - gpiote 1
        ppi.ch[2].eep.write(|w| unsafe { w.bits(timer1.events_compare[1].as_ptr() as u32) });
        ppi.ch[2].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[1].as_ptr() as u32) });
        ppi.ch[3].eep.write(|w| unsafe { w.bits(timer1.events_compare[3].as_ptr() as u32) });
        ppi.ch[3].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[1].as_ptr() as u32) });

        // Link motor 2 - gpiote 2
        ppi.ch[4].eep.write(|w| unsafe { w.bits(timer2.events_compare[0].as_ptr() as u32) });
        ppi.ch[4].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[2].as_ptr() as u32) });
        ppi.ch[5].eep.write(|w| unsafe { w.bits(timer2.events_compare[3].as_ptr() as u32) });
        ppi.ch[5].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[2].as_ptr() as u32) });

        // Link motor 3 - gpiote 3
        ppi.ch[6].eep.write(|w| unsafe { w.bits(timer2.events_compare[1].as_ptr() as u32) });
        ppi.ch[6].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[3].as_ptr() as u32) });
        ppi.ch[7].eep.write(|w| unsafe { w.bits(timer2.events_compare[3].as_ptr() as u32) });
        ppi.ch[7].tep.write(|w| unsafe { w.bits(gpiote.tasks_out[3].as_ptr() as u32) });

        ppi.chenset.write(|w| w
            .ch0().set_bit().ch1().set_bit()
            .ch2().set_bit().ch3().set_bit()
            .ch4().set_bit().ch5().set_bit()
            .ch6().set_bit().ch7().set_bit());

        let pin20 = QuadrupelGPIO::get().pin(20);
        pin20.set_mode_write();


        QuadrupleTimers { }
    }

    pub fn get_time_us(&self) -> u32 {
        unsafe { GLOBAL_TIME }
    }
}

#[interrupt]
unsafe fn TIMER2() {
    let timer2 = Peripherals::steal().TIMER2;
    let motors = MOTORS.get().get_values();

    if timer2.events_compare[3].read().bits() != 0 {
        timer2.events_compare[3].reset();
        GLOBAL_TIME += 312; //2500 * 0.125
        timer2.tasks_capture[2].write(|w| w.bits(1) );

        //TODO is this ever false?
        if timer2.cc[2].read().bits() < 500 {
            timer2.cc[0].write(|w| w.bits((1000 + motors[2]) as u32) );
            timer2.cc[1].write(|w| w.bits((1000 + motors[3]) as u32) );
        }

    }
}

#[interrupt]
unsafe fn TIMER1() {
    let timer1 = Peripherals::steal().TIMER1;
    let motors = MOTORS.get().get_values();
    let pin20 = QuadrupelGPIO::get().pin(PIN_20_UKNOWN);

    if timer1.events_compare[3].read().bits() != 0 {
        timer1.events_compare[3].reset();
        timer1.tasks_capture[2].write(|w| w.bits(1) );

        if timer1.cc[2].read().bits() < 500 {
            pin20.set();
            timer1.cc[0].write(|w| w.bits((1000 + motors[0]) as u32) );
            timer1.cc[1].write(|w| w.bits((1000 + motors[1]) as u32) );
            pin20.clear();
        }
    }
}
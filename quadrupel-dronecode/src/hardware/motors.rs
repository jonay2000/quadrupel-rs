use crate::library::cs_cell::CSCell;
use crate::library::once_cell::OnceCell;
use crate::Level;
use cortex_m::peripheral::NVIC;
use embedded_hal::digital::v2::OutputPin;
use nrf51_hal::gpio::p0::P0_20;
use nrf51_hal::gpio::{Disconnected, Output, PushPull};
use nrf51_pac::{interrupt, Interrupt, GPIOTE, PPI};

pub struct Motors {
    motor_values: [u16; 4],
    timer1: nrf51_pac::TIMER1,
    timer2: nrf51_pac::TIMER2,
    pin20: P0_20<Output<PushPull>>,
}

static mut GLOBAL_TIME: u32 = 0;

const MOTOR_0_PIN: u8 = 21;
const MOTOR_1_PIN: u8 = 23;
const MOTOR_2_PIN: u8 = 25;
const MOTOR_3_PIN: u8 = 29;

const MOTOR_MIN: u16 = 0;
const MOTOR_MAX: u16 = 500;

static MOTORS: OnceCell<CSCell<Motors>> = OnceCell::new();

impl Motors {
    pub fn get() -> &'static CSCell<Self> {
        MOTORS.get()
    }

    pub fn initialize(
        timer1: nrf51_pac::TIMER1,
        timer2: nrf51_pac::TIMER2,
        nvic: &mut NVIC,
        ppi: &mut PPI,
        gpiote: &mut GPIOTE,
        pin20: P0_20<Disconnected>,
    ) -> &'static CSCell<Self> {
        // Configure gpiote
        gpiote.config[0].write(|w| unsafe {
            w.mode()
                .task()
                .psel()
                .bits(MOTOR_0_PIN)
                .polarity()
                .toggle()
                .outinit()
                .set_bit()
        });
        gpiote.config[1].write(|w| unsafe {
            w.mode()
                .task()
                .psel()
                .bits(MOTOR_1_PIN)
                .polarity()
                .toggle()
                .outinit()
                .set_bit()
        });
        gpiote.config[2].write(|w| unsafe {
            w.mode()
                .task()
                .psel()
                .bits(MOTOR_2_PIN)
                .polarity()
                .toggle()
                .outinit()
                .set_bit()
        });
        gpiote.config[3].write(|w| unsafe {
            w.mode()
                .task()
                .psel()
                .bits(MOTOR_3_PIN)
                .polarity()
                .toggle()
                .outinit()
                .set_bit()
        });

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

        // Link motor 0 - gpiote 0
        ppi.ch[0]
            .eep
            .write(|w| unsafe { w.bits(timer1.events_compare[0].as_ptr() as u32) });
        ppi.ch[0]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[0].as_ptr() as u32) });
        ppi.ch[1]
            .eep
            .write(|w| unsafe { w.bits(timer1.events_compare[3].as_ptr() as u32) });
        ppi.ch[1]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[0].as_ptr() as u32) });

        // Link motor 1 - gpiote 1
        ppi.ch[2]
            .eep
            .write(|w| unsafe { w.bits(timer1.events_compare[1].as_ptr() as u32) });
        ppi.ch[2]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[1].as_ptr() as u32) });
        ppi.ch[3]
            .eep
            .write(|w| unsafe { w.bits(timer1.events_compare[3].as_ptr() as u32) });
        ppi.ch[3]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[1].as_ptr() as u32) });

        // Link motor 2 - gpiote 2
        ppi.ch[4]
            .eep
            .write(|w| unsafe { w.bits(timer2.events_compare[0].as_ptr() as u32) });
        ppi.ch[4]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[2].as_ptr() as u32) });
        ppi.ch[5]
            .eep
            .write(|w| unsafe { w.bits(timer2.events_compare[3].as_ptr() as u32) });
        ppi.ch[5]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[2].as_ptr() as u32) });

        // Link motor 3 - gpiote 3
        ppi.ch[6]
            .eep
            .write(|w| unsafe { w.bits(timer2.events_compare[1].as_ptr() as u32) });
        ppi.ch[6]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[3].as_ptr() as u32) });
        ppi.ch[7]
            .eep
            .write(|w| unsafe { w.bits(timer2.events_compare[3].as_ptr() as u32) });
        ppi.ch[7]
            .tep
            .write(|w| unsafe { w.bits(gpiote.tasks_out[3].as_ptr() as u32) });

        ppi.chenset.write(|w| {
            w.ch0()
                .set_bit()
                .ch1()
                .set_bit()
                .ch2()
                .set_bit()
                .ch3()
                .set_bit()
                .ch4()
                .set_bit()
                .ch5()
                .set_bit()
                .ch6()
                .set_bit()
                .ch7()
                .set_bit()
        });

        //Set up global state
        let pin20 = pin20.into_push_pull_output(Level::Low);
        let reff = MOTORS.initialize(CSCell::new(Motors {
            motor_values: [0; 4],
            timer1,
            timer2,
            pin20,
        }));

        // Configure interrupts
        NVIC::unpend(Interrupt::TIMER2);
        unsafe {
            nvic.set_priority(Interrupt::TIMER2, 1);
        }
        unsafe {
            NVIC::unmask(Interrupt::TIMER2);
        }

        NVIC::unpend(Interrupt::TIMER1);
        unsafe {
            nvic.set_priority(Interrupt::TIMER1, 1);
        }
        unsafe {
            NVIC::unmask(Interrupt::TIMER1);
        }

        reff
    }

    pub fn get_time_us() -> u32 {
        unsafe { GLOBAL_TIME }
    }

    pub fn get_motors(&self) -> [u16; 4] {
        self.motor_values
    }

    pub fn set_motors(&mut self, val: [u16; 4]) {
        self.motor_values = val.map(|v| v.clamp(MOTOR_MIN, MOTOR_MAX));
    }
}

#[interrupt]
unsafe fn TIMER2() {
    let motors = MOTORS.get().get_mut();

    if motors.timer2.events_compare[3].read().bits() != 0 {
        motors.timer2.events_compare[3].reset();
        GLOBAL_TIME += 312; //2500 * 0.125
        motors.timer2.tasks_capture[2].write(|w| w.bits(1));

        //TODO is this ever false?
        if motors.timer2.cc[2].read().bits() < 500 {
            motors.timer2.cc[0].write(|w| w.bits((1000 + motors.motor_values[2]) as u32));
            motors.timer2.cc[1].write(|w| w.bits((1000 + motors.motor_values[3]) as u32));
        }
    }
}

#[interrupt]
unsafe fn TIMER1() {
    let motors = MOTORS.get().get_mut();

    if motors.timer1.events_compare[3].read().bits() != 0 {
        motors.timer1.events_compare[3].reset();
        motors.timer1.tasks_capture[2].write(|w| w.bits(1));

        if motors.timer1.cc[2].read().bits() < 500 {
            motors.pin20.set_high().unwrap();
            motors.timer1.cc[0].write(|w| w.bits((1000 + motors.motor_values[0]) as u32));
            motors.timer1.cc[1].write(|w| w.bits((1000 + motors.motor_values[1]) as u32));
            motors.pin20.set_low().unwrap();
        }
    }
}

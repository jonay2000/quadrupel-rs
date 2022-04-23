pub mod leds;
pub mod uart;
pub mod adc;

use mpu6050_dmp::address::Address;
use mpu6050_dmp::sensor::Mpu6050;
use nrf51_hal::gpio::Pin;
use nrf51_hal::{Timer, Twi, Uart};


use nrf51_hal::twi::Pins;
use nrf51_pac::{TIMER0, TWI0, UART0};
use crate::FREQUENCY_A;
use crate::hardware::adc::QuadrupelAdc;
use crate::hardware::leds::Leds;
use crate::hardware::uart::QUART;

pub struct Hardware {
    pub leds: Leds,
    pub uart: &'static QUART,
    pub mpu: Mpu6050<Twi<TWI0>>,
    pub adc: QuadrupelAdc,
    pub timer0: Timer<TIMER0>,
}

pub fn init_hardware(pc: cortex_m::Peripherals, pn: nrf51_hal::pac::Peripherals) -> Hardware {
    let gpio = nrf51_hal::gpio::p0::Parts::new(pn.GPIO);
    let mut nvic = pc.NVIC;

    let mut timer0 = Timer::new(pn.TIMER0);
    let leds = Leds::new(gpio.p0_22, gpio.p0_24, gpio.p0_28, gpio.p0_30);
    let uart = QUART::initialize(pn.UART0, gpio.p0_14, gpio.p0_16, &mut nvic);


    let scl_pin = gpio.p0_04.into_floating_input();
    let sda_pin = gpio.p0_02.into_floating_input();
    let twi = Twi::new(pn.TWI0, Pins { scl: Pin::from(scl_pin), sda: Pin::from(sda_pin) }, FREQUENCY_A::K400);
    let mut mpu = Mpu6050::new(twi, Address::default()).unwrap();
    mpu.initialize_dmp(&mut timer0).unwrap();
    mpu.set_sample_rate_divider(1);


    let adc = QuadrupelAdc::new(pn.ADC, &mut nvic);

    Hardware {
        leds,
        uart,
        mpu,
        adc,
        timer0
    }
}
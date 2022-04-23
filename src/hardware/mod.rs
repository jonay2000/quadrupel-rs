pub mod leds;
pub mod uart;
pub mod adc;
pub mod i2c;

use mpu6050_dmp::address::Address;
use mpu6050_dmp::sensor::Mpu6050;
use nrf51_hal::gpio::Pin;
use nrf51_hal::{Timer, Twi, Uart};


use nrf51_hal::twi::Pins;
use nrf51_pac::{TIMER0, TWI0, UART0};
use crate::FREQUENCY_A;
use crate::hardware::adc::QAdc;
use crate::hardware::i2c::{I2C, QMpu};
use crate::hardware::leds::QLeds;
use crate::hardware::uart::QUart;

pub struct Hardware {
    pub leds: QLeds,
    pub uart: &'static QUart,
    pub mpu: QMpu,
    pub adc: QAdc,
    pub timer0: Timer<TIMER0>,
}

pub fn init_hardware(pc: cortex_m::Peripherals, pn: nrf51_hal::pac::Peripherals) -> Hardware {
    let gpio = nrf51_hal::gpio::p0::Parts::new(pn.GPIO);
    let mut nvic = pc.NVIC;

    let mut timer0 = Timer::new(pn.TIMER0);
    let leds = QLeds::new(gpio.p0_22, gpio.p0_24, gpio.p0_28, gpio.p0_30);
    let uart = QUart::initialize(pn.UART0, gpio.p0_14, gpio.p0_16, &mut nvic);


    let i2c = I2C::new(pn.TWI0, gpio.p0_04, gpio.p0_02);
    let mpu = QMpu::new(i2c, &mut timer0);


    let adc = QAdc::new(pn.ADC, &mut nvic);

    Hardware {
        leds,
        uart,
        mpu,
        adc,
        timer0
    }
}
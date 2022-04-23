pub mod adc;
pub mod i2c;
pub mod leds;
pub mod motors;
pub mod uart;

use crate::hardware::adc::QAdc;
use crate::hardware::i2c::{QMpu, I2C};
use crate::hardware::leds::QLeds;
use crate::hardware::uart::QUart;
use crate::library::cs_cell::CSCell;
use crate::Motors;
use nrf51_hal::Timer;
use nrf51_pac::TIMER0;

pub struct Hardware {
    pub leds: QLeds,
    pub uart: &'static QUart,
    pub mpu: QMpu,
    pub adc: QAdc,
    pub timer0: Timer<TIMER0>,
    pub motors: &'static CSCell<Motors>,
}

pub fn init_hardware(
    mut pc: cortex_m::Peripherals,
    mut pn: nrf51_hal::pac::Peripherals,
) -> Hardware {
    let gpio = nrf51_hal::gpio::p0::Parts::new(pn.GPIO);

    let mut timer0 = Timer::new(pn.TIMER0);
    let leds = QLeds::new(gpio.p0_22, gpio.p0_24, gpio.p0_28, gpio.p0_30);
    let uart = QUart::initialize(pn.UART0, gpio.p0_14, gpio.p0_16, &mut pc.NVIC);

    let i2c = I2C::new(pn.TWI0, gpio.p0_04, gpio.p0_02);
    let mpu = QMpu::new(i2c, &mut timer0);
    let adc = QAdc::new(pn.ADC, &mut pc.NVIC);

    let motors = Motors::initialize(
        pn.TIMER1,
        pn.TIMER2,
        &mut pc.NVIC,
        &mut pn.PPI,
        &mut pn.GPIOTE,
        gpio.p0_20,
    );

    Hardware {
        leds,
        uart,
        mpu,
        adc,
        timer0,
        motors,
    }
}

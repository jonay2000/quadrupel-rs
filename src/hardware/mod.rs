use crate::hardware::adc::QuadrupelAdc;
use crate::hardware::led::QuadrupelLed;
use crate::QuadrupelGPIO;

pub mod gpio;
pub mod uart;
pub mod led;
pub mod motor;
pub mod adc;

pub struct Hardware {
    pub adc: QuadrupelAdc,
    pub led_red: QuadrupelLed,
    pub led_yellow: QuadrupelLed,
    pub led_green: QuadrupelLed,
    pub led_blue: QuadrupelLed,

}

impl Hardware {
    pub fn new(mut periphs_cm: cortex_m::Peripherals, periphs_nrf: nrf51822::Peripherals) -> Hardware {
        //Create hardware
        let gpio = QuadrupelGPIO::new(periphs_nrf.GPIO);
        let mut pio_pins = gpio.pins().map(Option::Some);

        let led_red = QuadrupelLed::new(pio_pins[22].take().unwrap());
        let led_yellow = QuadrupelLed::new(pio_pins[24].take().unwrap());
        let led_green = QuadrupelLed::new(pio_pins[28].take().unwrap());
        let led_blue = QuadrupelLed::new(pio_pins[30].take().unwrap());

        let adc = QuadrupelAdc::new(periphs_nrf.ADC, &mut periphs_cm.NVIC);

        Hardware { adc, led_red, led_yellow, led_green, led_blue }
    }
}
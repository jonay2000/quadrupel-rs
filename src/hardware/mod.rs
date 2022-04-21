use crate::hardware::led::QuadrupelLed;
use crate::QuadrupelGPIO;

pub mod gpio;
pub mod uart;
pub mod led;
pub mod motor;

pub struct Hardware {
    pub led_red: QuadrupelLed,
    pub led_yellow: QuadrupelLed,
    pub led_green: QuadrupelLed,
    pub led_blue: QuadrupelLed,
}

impl Hardware {
    pub fn new(peripherals: nrf51822::Peripherals) -> Hardware {
        //Create hardware
        let gpio = QuadrupelGPIO::new(peripherals.GPIO);
        let mut pio_pins = gpio.pins().map(Option::Some);

        Hardware {
            led_red: QuadrupelLed::new(pio_pins[22].take().unwrap()),
            led_yellow: QuadrupelLed::new(pio_pins[24].take().unwrap()),
            led_green: QuadrupelLed::new(pio_pins[28].take().unwrap()),
            led_blue: QuadrupelLed::new(pio_pins[30].take().unwrap()),
        }
    }
}
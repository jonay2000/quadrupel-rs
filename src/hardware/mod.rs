use crate::hardware::adc::QuadrupelAdc;
use crate::hardware::led::QuadrupelLed;
use crate::hardware::timers::QuadrupleTimers;
use crate::hardware::uart::QuadrupelUART;
use crate::QuadrupelGPIO;

pub mod adc;
pub mod gpio;
pub mod led;
pub mod uart;
pub mod timers;
pub mod motors;

pub struct Hardware {
    pub adc: QuadrupelAdc,
    pub led_red: QuadrupelLed,
    pub led_yellow: QuadrupelLed,
    pub led_green: QuadrupelLed,
    pub led_blue: QuadrupelLed,
    pub uart: QuadrupelUART,
    pub timers: QuadrupleTimers,
}

impl Hardware {
    pub fn new(
        mut periphs_cm: cortex_m::Peripherals,
        mut periphs_nrf: nrf51822::Peripherals,
    ) -> Hardware {
        //Create hardware
        let gpio = QuadrupelGPIO::new(periphs_nrf.GPIO);
        let mut gpio_pins = gpio.pins().map(Option::Some);

        let led_red = QuadrupelLed::new(gpio_pins[22].take().unwrap());
        let led_yellow = QuadrupelLed::new(gpio_pins[24].take().unwrap());
        let led_green = QuadrupelLed::new(gpio_pins[28].take().unwrap());
        let led_blue = QuadrupelLed::new(gpio_pins[30].take().unwrap());

        let adc = QuadrupelAdc::new(periphs_nrf.ADC, &mut periphs_cm.NVIC);

        let uart = QuadrupelUART::new(
            periphs_nrf.UART0,
            gpio_pins[14].take().unwrap(),
            gpio_pins[16].take().unwrap(),
            &mut periphs_cm.NVIC,
        );
        let timers = QuadrupleTimers::new(
            periphs_nrf.TIMER0,
            periphs_nrf.TIMER1,
            periphs_nrf.TIMER2,
            gpio_pins[20].take().unwrap(),
            &mut periphs_cm.NVIC,
            &mut periphs_nrf.PPI,
            &mut periphs_nrf.GPIOTE);

        Hardware {
            adc,
            led_red,
            led_yellow,
            led_green,
            led_blue,
            uart,
            timers,
        }
    }
}

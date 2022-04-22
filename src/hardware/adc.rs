use cortex_m::peripheral::NVIC;
use nrf51822::{ADC, Interrupt};
use nrf51822::interrupt;

pub struct QuadrupelAdc {
    adc: ADC,
}

#[interrupt]
fn ADC() {
    let periph = unsafe { nrf51822::Peripherals::steal() }; // Safe, since we're in an interrupt
    periph.ADC.events_end.write(|w| unsafe { w.bits(0) });

    //TODO read periph.ADC.result
}

impl QuadrupelAdc {
    pub fn new(adc: ADC, nvic: &mut NVIC) -> Self {
        // //Configure ADC
        // //We want to use Analog Input 4 as an input.
        // adc.config.write(|w| w.psel().analog_input4());
        // //We want to use an analog input with two thirds prescaling
        // adc.config.write(|w| w.inpsel().analog_input_two_thirds_prescaling());
        // //We want to enable ADC now
        // adc.enable.write(|w| w.enable().enabled());
        // //We want to enable interrupt on ADC sample ready event, priority 3
        // adc.intenset.write(|w| w.end().set_bit());
        // unsafe { nvic.set_priority(Interrupt::ADC, 3); }
        // unsafe { NVIC::unmask(Interrupt::ADC); }
        // //For some reason, there is no field inside this register, so we set it to 1 manually.
        // adc.tasks_start.write(|w| unsafe { w.bits(1) } );

        QuadrupelAdc { adc }
    }

    pub fn adc_request_sample(&mut self) {
        //TODO
    }
}

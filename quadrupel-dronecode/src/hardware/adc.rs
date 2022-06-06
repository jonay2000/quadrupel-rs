use crate::hardware;
use cortex_m::peripheral::NVIC;
use nrf51_pac::interrupt;
use nrf51_pac::Interrupt;

pub struct QAdc {
    adc: nrf51_pac::ADC,
    last_result: u16,
}

#[interrupt]
fn ADC() {
    hardware::ADC.update_interrupt(|adc| {
        adc.adc.events_end.reset();
        // Battery voltage = (result*1.2*3/255*2) = RESULT*0.007058824
        adc.last_result = adc.adc.result.read().result().bits() * 7;
    });
}

impl QAdc {
    pub fn new(adc: nrf51_pac::ADC, nvic: &mut NVIC) -> Self {
        //We want to use Analog Input 4 as an input.
        //We want to use an analog input with two thirds prescaling
        adc.config.write(|w| {
            w.psel()
                .analog_input4()
                .inpsel()
                .analog_input_two_thirds_prescaling()
        });

        //We want to enable ADC now
        adc.enable.write(|w| w.enable().enabled());

        //We want to enable interrupt on ADC sample ready event, priority 3
        adc.intenset.write(|w| w.end().set_bit());
        unsafe {
            nvic.set_priority(Interrupt::ADC, 3);
        }

        QAdc {
            adc,
            last_result: 0,
        }
    }

    pub fn enable(&mut self) {
        unsafe {
            NVIC::unmask(Interrupt::ADC);
        }
    }

    fn request_sample(&mut self) {
        if !self.adc.busy.read().busy().bit() {
            //For some reason, there is no field inside this register, so we set it to 1 manually.
            self.adc.tasks_start.write(|w| unsafe { w.bits(1) });
        }
    }

    /// Voltage in 10^-2 volt
    pub fn read(&mut self) -> u16 {
        self.request_sample();
        self.last_result
    }
}

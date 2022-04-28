use cortex_m::peripheral::NVIC;
use fixed::{types, FixedU16};
use nrf51_pac::interrupt;
use nrf51_pac::Interrupt;

pub struct QAdc {
    adc: nrf51_pac::ADC,
}

static mut ADC_RESULT: u16 = 0;

#[interrupt]
unsafe fn ADC() {
    let adc = nrf51_pac::Peripherals::steal().ADC;
    adc.events_end.reset();

    // Battery voltage = (result*1.2*3/255*2) = RESULT*0.007058824
    ADC_RESULT = adc.result.read().result().bits() * 7;
}

impl QAdc {
    pub fn new(adc: nrf51_pac::ADC, nvic: &mut NVIC) -> Self {
        //We want to use Analog Input 4 as an input.
        adc.config.write(|w| w.psel().analog_input4());

        //We want to use an analog input with two thirds prescaling
        adc.config
            .write(|w| w.inpsel().analog_input_two_thirds_prescaling());

        //We want to enable ADC now
        adc.enable.write(|w| w.enable().enabled());

        //We want to enable interrupt on ADC sample ready event, priority 3
        adc.intenset.write(|w| w.end().set_bit());
        unsafe {
            nvic.set_priority(Interrupt::ADC, 3);
            NVIC::unmask(Interrupt::ADC);
        }

        QAdc { adc }
    }

    pub fn request_sample(&mut self) {
        if !self.adc.busy.read().busy().bit() {
            //For some reason, there is no field inside this register, so we set it to 1 manually.
            self.adc.tasks_start.write(|w| unsafe { w.bits(1) });
        }
    }

    /// Voltage in 10^-2 volt
    pub fn most_recent_voltage(&self) -> u16 {
        //Safety: Reading a u16 is atomic
        unsafe { ADC_RESULT }
    }
}

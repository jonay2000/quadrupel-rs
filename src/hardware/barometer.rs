use nrf51_hal::Twi;
use nrf51_pac::TWI0;

pub struct Barometer {}

impl Barometer {
    pub fn new(twi: Twi<TWI0>) {}
}

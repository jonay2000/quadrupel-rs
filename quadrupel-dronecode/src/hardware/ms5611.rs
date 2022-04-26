use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;
use nrf51_hal::{Timer, Twi};
use nrf51_pac::TIMER0;

// THIS NUMBER HAS A LARGE IMPACT ON PERFORMANCE
// Vanilla sample takes 2500 us -> 400 Hz
// Measure us per iteration, choose the lowest number that is often enough
// 1 = 2500 us
// 2 = 5000 us
// 3 = 7500 us
// 4 = 10k  us
// ... increasing in increments of 2500 us
// Though this theory does not seem to hold up to measurement. The real values seem to be
// 1 = 8000 us
// 2 = 6000 us
// 3 = 8000 us
// 4 = 10k  us
// ... increasing in increments of 2500 us
const SAMPLE_RATE_DIVIDER: u8 = 2;

const CONVERT_D1_1024: usize = 0x44;
const CONVERT_D1_4096: usize = 0x48;
const CONVERT_D2_1024: usize = 0x54;
const CONVERT_D2_4096: usize = 0x58;
const MS5611_ADDR: u8 = 0b01110111;
const READ: usize = 0x0;
const PROM: u8 = 0xA0;

pub struct QMs5611<T: nrf51_hal::twi::Instance> {
    twi: Twi<T>,
    /// 0: manufacturer
    /// 1: pressure sensitivity
    /// 2: pressure offset
    /// 3: temp coef pressure sensitivity
    /// 4: temp coef pressure offset
    /// 5: temp ref
    /// 6: temp coef temp
    /// 7: crc
    prom: [u16; 8],
    /// Used in read implementation
    loop_count: u8,
}
impl<T: nrf51_hal::twi::Instance> QMs5611<T> {
    pub fn new(mut twi: Twi<T>) -> Self {
        let mut prom = [0; 8];
        let mut data = [0u8; 2];
        for c in 0..8 {
            twi.write_read(MS5611_ADDR, &[PROM + 2 * c], &mut data).unwrap();
            prom[c as usize] = u16::from_be_bytes(data);
        }
        Self { twi, prom, loop_count: 0 }
    }

    pub fn read(&mut self) -> Self {
        todo!()
        // self.twi.wri
    }
}

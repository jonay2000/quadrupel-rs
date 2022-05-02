use core::marker::PhantomData;
use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;
use nrf51_hal::{Twi};
use crate::motors::GlobalTime;

const MS5611_ADDR: u8 = 0b01110111;
const REG_READ: u8 = 0x0;
const REG_D1: u8 = 0x40;
const REG_D2: u8 = 0x50;
const REG_PROM: u8 = 0xA0;

pub enum OverSamplingRatio {
    Opt256,
    Opt512,
    Opt1024,
    Opt2048,
    Opt4096,
}

impl OverSamplingRatio {
    fn get_delay(&self) -> u32 {
        match *self {
            OverSamplingRatio::Opt256 => 1000,
            OverSamplingRatio::Opt512 => 2000,
            OverSamplingRatio::Opt1024 => 3000,
            OverSamplingRatio::Opt2048 => 5000,
            OverSamplingRatio::Opt4096 => 10000,
        }
    }

    fn addr_modifier(&self) -> u8 {
        match *self {
            OverSamplingRatio::Opt256 => 0,
            OverSamplingRatio::Opt512 => 2,
            OverSamplingRatio::Opt1024 => 4,
            OverSamplingRatio::Opt2048 => 6,
            OverSamplingRatio::Opt4096 => 8,
        }
    }
}

enum QMs5611LoopState {
    Reset,
    ReadD1 { start_time: u32 },
    ReadD2 { start_time: u32, d1: u32 },
}

pub struct QMs5611<I2c: nrf51_hal::twi::Instance> {
    _p: PhantomData<I2c>,

    /// From datasheet, C1.
    pressure_sensitivity: u16,
    /// From datasheet, C2.
    pressure_offset: u16,
    /// From datasheet, C3.
    temp_coef_pressure_sensitivity: u16,
    /// From datasheet, C4.
    temp_coef_pressure_offset: u16,
    /// From datasheet, C5.
    temp_ref: u16,
    /// From datasheet, C6.
    temp_coef_temp: u16,

    /// What should the oversampling ratio of the chip be?
    over_sampling_ratio: OverSamplingRatio,

    /// State of the QMs5611 chip
    loop_state: QMs5611LoopState,

    /// Pressure in 10^-5 bar
    most_recent_pressure: u32,
    /// Temperature in 10^-2 celcius
    most_recent_temp: u32,


}

impl<I2c: nrf51_hal::twi::Instance> QMs5611<I2c> {
    pub fn new(twi: &mut Twi<I2c>) -> Self {
        let mut prom = [0; 8];
        let mut data = [0u8; 2];
        for c in 0..8 {
            twi.write_read(MS5611_ADDR, &[REG_PROM + 2 * c], &mut data).unwrap();
            prom[c as usize] = u16::from_be_bytes(data);
        }

        Self {
            _p: PhantomData::default(),
            pressure_sensitivity: prom[1],
            pressure_offset: prom[2],
            temp_coef_pressure_sensitivity: prom[3],
            temp_coef_pressure_offset: prom[4],
            temp_ref: prom[5],
            temp_coef_temp: prom[6],
            over_sampling_ratio: OverSamplingRatio::Opt4096,
            loop_state: QMs5611LoopState::Reset,
            most_recent_pressure: 0,
            most_recent_temp: 0,
        }
    }

    pub fn update(&mut self, twi: &mut Twi<I2c>) {
        match self.loop_state {
            QMs5611LoopState::Reset => {
                //We let the chip know we want to read D1.
                twi.write(MS5611_ADDR, &[REG_D1 + self.over_sampling_ratio.addr_modifier()]).unwrap();

                //Then set loop state for next iteration
                self.loop_state = QMs5611LoopState::ReadD1 { start_time: GlobalTime().get_time_us() };
            }
            QMs5611LoopState::ReadD1 { start_time } => {
                //If the chip has not had enough time to process, return
                if GlobalTime().get_time_us() - start_time < self.over_sampling_ratio.get_delay() { return; }

                //Read D1
                let mut buf = [0u8; 4];
                twi.write_read(MS5611_ADDR, &[REG_READ], &mut buf[1..4]).unwrap();
                let d1 = u32::from_be_bytes(buf);

                //We let the chip know we want to read D2.
                twi.write(MS5611_ADDR, &[REG_D2 + self.over_sampling_ratio.addr_modifier()]).unwrap();

                //Then set loop state for next iteration
                self.loop_state = QMs5611LoopState::ReadD2 { start_time: GlobalTime().get_time_us(), d1 };
            }
            QMs5611LoopState::ReadD2 { start_time, d1 } => {
                //If the chip has not had enough time to process, return
                if GlobalTime().get_time_us() - start_time < self.over_sampling_ratio.get_delay() { return; }

                //Read D2
                let mut buf = [0u8; 4];
                twi.write_read(MS5611_ADDR, &[REG_READ], &mut buf[1..4]).unwrap();
                let d2 = u32::from_be_bytes(buf);

                //Use D1 and D2 to find the new pressure and temperature
                self.update_values(d1, d2);

                //Then set loop state for next iteration, and we can do the next iteration immediately
                self.loop_state = QMs5611LoopState::Reset;
                self.update(twi);
            }
        }
    }

    //Inspired by: https://github.com/braincore/ms5611-rs/blob/master/src/lib.rs
    fn update_values(&mut self, d1: u32, d2: u32) {
        let d1 = d1 as u64;
        let d2 = d2 as u64;

        // Temperature difference from reference
        let dt = d2 - ((self.temp_ref as u64) << 8);

        let offset: u64 = ((self.pressure_offset as u64) << 16)
            + ((dt * (self.temp_coef_pressure_offset as u64)) >> 7);
        let sens: u64 = ((self.pressure_sensitivity as u64) << 15)
            + ((dt * (self.temp_coef_pressure_sensitivity as u64)) >> 8);

        // Units: mbar * 100
        self.most_recent_pressure = ((((d1 * sens) >> 21) - offset) >> 15) as u32;

        // Units: celcius * 100
        self.most_recent_temp = 2000 + (((dt * (self.temp_coef_temp as u64)) >> 23) as u32);
    }

    /// Returns pressure in 10^-5 bar
    pub fn read_pressure(&mut self, twi: &mut Twi<I2c>) -> u32 {
        self.update(twi);
        self.most_recent_pressure
    }

    /// Returns temperature in 10^-2 celcius
    pub fn read_temperature(&mut self, twi: &mut Twi<I2c>) -> u32 {
        self.update(twi);
        self.most_recent_temp
    }

    /// (pressure, temperature)
    pub fn read_both(&mut self, twi: &mut Twi<I2c>) -> (u32, u32) {
        self.update(twi);
        (self.most_recent_pressure, self.most_recent_temp)
    }
}

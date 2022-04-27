use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;
use nrf51_hal::{Twi};
use crate::Motors;

const CONVERT_D1_1024: u8 = 0x44;
const CONVERT_D1_4096: u8 = 0x48;
const CONVERT_D2_1024: u8 = 0x54;
const CONVERT_D2_4096: u8 = 0x58;
const MS5611_ADDR: u8 = 0b01110111;
const READ: u8 = 0x0;
const PROM: u8 = 0xA0;

pub enum OverSamplingRatio {
    Opt256,
    Opt512,
    Opt1024,
    Opt2048,
    Opt4096,
}

impl OverSamplingRatio {
    fn get_delay(&self) -> u64 {
        match *self {
            OverSamplingRatio::Opt256 => 1,
            OverSamplingRatio::Opt512 => 2,
            OverSamplingRatio::Opt1024 => 3,
            OverSamplingRatio::Opt2048 => 5,
            OverSamplingRatio::Opt4096 => 10,
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

pub struct QMs5611<T: nrf51_hal::twi::Instance> {
    twi: Twi<T>,

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

    /// Used in read implementation
    loop_count: u8,
    init_time: u32,

    most_recent: i32,
    d1: i32,
}

impl<T: nrf51_hal::twi::Instance> QMs5611<T> {
    pub fn new(mut twi: Twi<T>) -> Self {
        let mut prom = [0; 8];
        let mut data = [0u8; 2];
        for c in 0..8 {
            twi.write_read(MS5611_ADDR, &[PROM + 2 * c], &mut data).unwrap();
            prom[c as usize] = u16::from_be_bytes(data);
        }
        Self {
            twi,
            pressure_sensitivity: prom[1],
            pressure_offset: prom[2],
            temp_coef_pressure_sensitivity: prom[3],
            temp_coef_pressure_offset: prom[4],
            temp_ref: prom[5],
            temp_coef_temp: prom[6],
            loop_count: 0,
            init_time: 0,
            most_recent: 0,
            d1: 0,
        }
    }

    pub fn update(&mut self) {
        match self.loop_count {
            0 => {
                self.twi.write(MS5611_ADDR, &[CONVERT_D1_4096]).unwrap();
                self.init_time = Motors::get_time_us();
                self.loop_count = 1;
            }
            1 => {
                if Motors::get_time_us() - self.init_time < 10000 { return; }
                let mut buf = [0u8; 4];
                self.twi.write_read(MS5611_ADDR, &[READ], &mut buf[1..4]).unwrap();
                self.d1 = i32::from_be_bytes(buf);
                self.twi.write(MS5611_ADDR, &[CONVERT_D2_4096]).unwrap();

                self.init_time = Motors::get_time_us();
                self.loop_count = 2;
            }
            2 => {
                if Motors::get_time_us() - self.init_time < 10000 { return; }
                let mut buf = [0u8; 4];
                self.twi.write_read(MS5611_ADDR, &[READ], &mut buf[1..4]).unwrap();

                let d1 = self.d1;
                let d2 = i32::from_be_bytes(buf);
                self.most_recent = self.calc_values(d1, d2);

                self.loop_count = 0;
            }
            _ => unreachable!()
        }
    }

    //Credits: https://github.com/braincore/ms5611-rs/blob/master/src/lib.rs
    fn calc_values(&self, d1: i32, d2: i32) -> i32 {
        let d1 = d1 as i64;
        let d2 = d2 as i64;

        // Temperature difference from reference
        let dt = d2 - ((self.temp_ref as i64) << 8);

        let offset: i64 = ((self.pressure_offset as i64) << 16)
            + ((dt * (self.temp_coef_pressure_offset as i64)) >> 7);
        let sens: i64 = ((self.pressure_sensitivity as i64) << 15)
            + ((dt * (self.temp_coef_pressure_sensitivity as i64)) >> 8);

        // Units: mbar * 100
        let pressure: i32 = ((((d1 * sens) >> 21) - offset) >> 15) as i32;

        // Units: celcius * 100
        // let temperature: i32 = 2000 +
        //     (((dt * (self.temp_coef_temp as i64)) >> 23) as i32);
        pressure
    }

    pub fn read_most_recent(&mut self) -> i32 {
        self.update();
        self.most_recent
    }
}

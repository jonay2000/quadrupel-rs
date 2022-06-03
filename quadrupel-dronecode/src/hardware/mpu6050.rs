use crate::library::yaw_pitch_roll::{Quaternion, YawPitchRoll};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use mpu6050_dmp::accel::Accel;
use mpu6050_dmp::gyro::Gyro;
use mpu6050_dmp::sensor::Mpu6050;
use nrf51_hal::Twi;
use mpu6050_dmp::config::DigitalLowPassFilter;
use crate::TIME;

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

const SAMPLE_RATE_DIVIDER_MPU: u8 = 0;
const SAMPLE_RATE_DIVIDER_RAW: u8 = 0;

pub struct QMpu6050<T: nrf51_hal::twi::Instance> {
    mpu: Mpu6050<Twi<T>>,
    mpu_enabled: bool,
}
impl<T: nrf51_hal::twi::Instance> QMpu6050<T> {
    pub fn new(i2c: &mut Twi<T>) -> Self {
        let mut mpu = Mpu6050::new(i2c).unwrap();

        mpu.initialize_dmp(i2c, TIME.as_mut_ref()).unwrap();
        mpu.set_sample_rate_divider(i2c, SAMPLE_RATE_DIVIDER_MPU)
            .unwrap();
        mpu.set_digital_lowpass_filter(i2c, DigitalLowPassFilter::Filter1).unwrap(); //TODO consider different settings
        QMpu6050 { mpu, mpu_enabled: true }
    }

    pub fn is_mpu_enabled(&self) -> bool {
        self.mpu_enabled
    }

    pub fn disable_mpu(&mut self, i2c: &mut Twi<T>) {
        self.mpu.set_sample_rate_divider(i2c, SAMPLE_RATE_DIVIDER_RAW).unwrap();
        self.mpu.disable_dmp(i2c).unwrap();
        self.mpu.disable_fifo(i2c).unwrap();
        self.mpu_enabled = false
    }

    pub fn enable_mpu(&mut self, i2c: &mut Twi<T>) {
        self.mpu.set_sample_rate_divider(i2c, SAMPLE_RATE_DIVIDER_MPU).unwrap();
        self.mpu.enable_dmp(i2c).unwrap();
        self.mpu.enable_fifo(i2c).unwrap();
        self.mpu_enabled = true;
    }

    pub fn read_mpu(&mut self, i2c: &mut Twi<T>) -> Option<YawPitchRoll> {
        // If there isn't a full packet ready, return none
        let mut len = self.mpu.get_fifo_count(i2c).unwrap();
        if len < 28 {
            return None;
        }

        // Keep reading while there are more full packets
        let mut buf = [0; 28];
        while len >= 28 {
            self.mpu.read_fifo(i2c, &mut buf).unwrap();
            len -= 28;
        }

        // Convert the last full packet we received to yaw-pitch-roll
        let q = Quaternion::from_bytes(&buf[..16]).unwrap();
        let ypr = YawPitchRoll::from(q);
        Some(ypr)
    }

    pub fn block_read_mpu(&mut self, i2c: &mut Twi<T>) -> YawPitchRoll {
        loop {
            match self.read_mpu(i2c) {
                None => {
                    //Try again after 100 us
                    TIME.as_mut_ref().delay_us(100u32);
                }
                Some(ypr) => return ypr,
            }
        }
    }

    pub fn read_accel_gyro(&mut self, i2c: &mut Twi<T>) -> (Accel, Gyro) {
        let acc = self.mpu.accel(i2c).unwrap();
        let gyro = self.mpu.gyro(i2c).unwrap();
        (acc, gyro)
    }
}

use crate::library::yaw_pitch_roll::{Quaternion, YawPitchRoll};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use mpu6050_dmp::accel::Accel;
use mpu6050_dmp::address::Address;
use mpu6050_dmp::gyro::Gyro;
use mpu6050_dmp::sensor::Mpu6050;
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

pub struct QMpu6050<T: nrf51_hal::twi::Instance> {
    mpu: Mpu6050<Twi<T>>,
}
impl<T: nrf51_hal::twi::Instance> QMpu6050<T> {
    pub fn new(twi: Twi<T>, timer0: &mut Timer<TIMER0>) -> Self {
        let mut mpu = Mpu6050::new(twi, Address::default()).unwrap();
        mpu.initialize_dmp(timer0).unwrap();
        mpu.set_sample_rate_divider(SAMPLE_RATE_DIVIDER).unwrap();
        QMpu6050 { mpu }
    }

    pub fn disable_mpu(&mut self) {
        self.mpu.disable_dmp().unwrap();
    }

    pub fn enable_mpu(&mut self) {
        self.mpu.enable_dmp().unwrap();
    }

    pub fn read_mpu(&mut self) -> Option<YawPitchRoll> {
        // If there isn't a full packet ready, return none
        let mut len = self.mpu.get_fifo_count().unwrap();
        if len < 28 {
            return None;
        }

        // Keep reading while there are more full packets
        let mut buf = [0; 28];
        while len >= 28 {
            self.mpu.read_fifo(&mut buf).unwrap();
            len -= 28;
        }

        // Convert the last full packet we received to yaw-pitch-roll
        let q = Quaternion::from_bytes(&buf[..16]).unwrap();
        let ypr = YawPitchRoll::from(q);
        Some(ypr)
    }

    pub fn block_read_mpu(&mut self, timer0: &mut Timer<TIMER0>) -> YawPitchRoll {
        loop {
            match self.read_mpu() {
                None => {
                    //Try again after 100 us
                    timer0.delay_us(100u32);
                }
                Some(ypr) => return ypr,
            }
        }
    }

    pub fn read_accel_gyro(&mut self) -> (Accel, Gyro) {
        let acc = self.mpu.accel().unwrap();
        let gyro = self.mpu.gyro().unwrap();
        (acc, gyro)
    }
}

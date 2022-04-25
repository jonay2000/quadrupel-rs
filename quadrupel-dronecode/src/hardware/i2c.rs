use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use mpu6050_dmp::accel::Accel;
use crate::library::yaw_pitch_roll::{Quaternion, YawPitchRoll};
use mpu6050_dmp::address::Address;
use mpu6050_dmp::gyro::Gyro;
use mpu6050_dmp::sensor::Mpu6050;
use nrf51_hal::gpio::p0::{P0_02, P0_04};
use nrf51_hal::gpio::{Disconnected, Pin};
use nrf51_hal::twi::Pins;
use nrf51_hal::{Timer, Twi};
use nrf51_pac::twi0::frequency::FREQUENCY_A;
use nrf51_pac::{TIMER0, TWI1};

// THIS NUMBER HAS A LARGE IMPACT ON PERFORMANCE
// Vanilla sample takes 2500 us -> 400 Hz
// Measure us per iteration, choose the lowest number that is often enough
// 1 = 2500 us
// 2 = 5000 us
// 3 = 7500 us
// 4 = 10k  us
// etc..
const SAMPLE_RATE_DIVIDER: u8 = 2;

pub struct I2C {}

impl I2C {
    pub fn new(twi: TWI1, scl_pin: P0_04<Disconnected>, sda_pin: P0_02<Disconnected>) -> Twi<TWI1> {
        let scl_pin = scl_pin.into_floating_input();
        let sda_pin = sda_pin.into_floating_input();
        Twi::new(
            twi,
            Pins {
                scl: Pin::from(scl_pin),
                sda: Pin::from(sda_pin),
            },
            FREQUENCY_A::K400,
        )
    }
}

pub struct QMpu {
    mpu: Mpu6050<Twi<TWI1>>,
}
impl QMpu {
    pub fn new(twi: Twi<TWI1>, timer0: &mut Timer<TIMER0>) -> Self {
        let mut mpu = Mpu6050::new(twi, Address::default()).unwrap();
        mpu.initialize_dmp(timer0).unwrap();
        mpu.set_sample_rate_divider(SAMPLE_RATE_DIVIDER).unwrap();
        QMpu { mpu }
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


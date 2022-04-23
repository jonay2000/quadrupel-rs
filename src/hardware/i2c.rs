use mpu6050_dmp::address::Address;
use mpu6050_dmp::quaternion::Quaternion;
use mpu6050_dmp::sensor::Mpu6050;
use mpu6050_dmp::yaw_pitch_roll::YawPitchRoll;
use nrf51_hal::{Timer, Twi};
use nrf51_hal::gpio::{Disconnected, Pin};
use nrf51_hal::gpio::p0::{P0_02, P0_04};
use nrf51_hal::twi::Pins;
use nrf51_pac::{TIMER0, TWI0};
use nrf51_pac::twi0::frequency::FREQUENCY_A;

pub struct I2C {}

impl I2C {
    pub fn new(twi: TWI0, scl_pin: P0_04<Disconnected>, sda_pin: P0_02<Disconnected>) -> Twi<TWI0> {
        let scl_pin = scl_pin.into_floating_input();
        let sda_pin = sda_pin.into_floating_input();
        Twi::new(twi, Pins { scl: Pin::from(scl_pin), sda: Pin::from(sda_pin) }, FREQUENCY_A::K400)
    }
}

pub struct QMpu {
    mpu: Mpu6050<Twi<TWI0>>
}
impl QMpu {
    pub fn new(twi: Twi<TWI0>, timer0: &mut Timer<TIMER0>) -> Self {
        let mut mpu = Mpu6050::new(twi, Address::default()).unwrap();
        mpu.initialize_dmp(timer0).unwrap();
        mpu.set_sample_rate_divider(1).unwrap();
        QMpu { mpu }
    }

    pub fn read_most_recent(&mut self) -> Option<YawPitchRoll> {
        let mut len = self.mpu.get_fifo_count().unwrap();
        let mut buf = [0; 28];
        if len < 28 { return None }
        while len >= 28 {
            self.mpu.read_fifo(&mut buf).unwrap();
            len -= 28;
        }
        let q = Quaternion::from_bytes(&buf[..16]).unwrap().normalize();
        Some(YawPitchRoll::from(q))
    }

    pub fn block_read_most_recent(&mut self) -> YawPitchRoll {
        loop {
            match self.read_most_recent() {
                None => {}
                Some(ypr) => return ypr
            }
        }
    }
}

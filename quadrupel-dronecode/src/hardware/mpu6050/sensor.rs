use core::marker::PhantomData;
use embedded_hal::blocking::delay;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use log::info;
use crate::mpu6050::accel::{Accel, AccelFullScale};
use crate::mpu6050::clock_source::ClockSource;
use crate::mpu6050::config::DigitalLowPassFilter;
use crate::mpu6050::error::Error;
use crate::mpu6050::fifo::Fifo;
use crate::mpu6050::gyro::{Gyro, GyroFullScale};
use crate::mpu6050::registers::Register;

const MPU6050_ADDRESS: u8 = 0x68;

/// InvenSense MPU-6050 Driver
pub struct Mpu6050<I2c>
where
    I2c: Write + WriteRead,
    <I2c as WriteRead>::Error: core::fmt::Debug,
    <I2c as Write>::Error: core::fmt::Debug,
{
    _p: PhantomData<I2c>,
}

impl<I2c> Mpu6050<I2c>
where
    I2c: Write + WriteRead,
    <I2c as WriteRead>::Error: core::fmt::Debug,
    <I2c as Write>::Error: core::fmt::Debug,
{
    /// Construct a new i2c driver for the MPU-6050
    pub fn new(i2c: &mut I2c) -> Result<Self, Error<I2c>> {
        info!("New start");
        let mut sensor = Self { _p: PhantomData::default(), };

        info!("New 1");
        sensor.disable_sleep(i2c, )?;
        info!("New end");
        Ok(sensor)
    }

    /// Load DMP firmware and perform all appropriate initialization.
    pub fn initialize_dmp(
        &mut self,
        i2c: &mut I2c,
        delay: &mut impl delay::DelayMs<u32>,
    ) -> Result<(), Error<I2c>> {
        self.reset(i2c, delay)?;
        self.disable_sleep(i2c, )?;
        self.reset_signal_path(i2c, delay)?;
        self.disable_dmp(i2c, )?;
        self.set_clock_source(i2c, ClockSource::Xgyro)?;
        self.disable_interrupts(i2c, )?;
        self.set_fifo_enabled(i2c, Fifo::all_disabled())?;
        self.set_accel_full_scale(i2c, AccelFullScale::G2)?;
        self.set_sample_rate_divider(i2c, 4)?;
        self.set_digital_lowpass_filter(i2c, DigitalLowPassFilter::Filter1)?;
        self.load_firmware(i2c)?;
        self.boot_firmware(i2c)?;
        self.set_gyro_full_scale(i2c, GyroFullScale::Deg2000)?;
        self.enable_fifo(i2c, )?;
        self.reset_fifo(i2c, )?;
        self.disable_dmp(i2c, )?;
        self.enable_dmp(i2c, )?;
        Ok(())
    }

    pub(crate) fn read(&mut self, i2c: &mut I2c, bytes: &[u8], response: &mut [u8]) -> Result<(), Error<I2c>> {
        i2c
            .write_read(MPU6050_ADDRESS, bytes, response)
            .map_err(|e| Error::WriteReadError(e))
    }

    pub(crate) fn write(&mut self, i2c: &mut I2c,bytes: &[u8]) -> Result<(), Error<I2c>> {
        i2c
            .write(MPU6050_ADDRESS, bytes)
            .map_err(|e| Error::WriteError(e))
    }

    pub(crate) fn read_register(&mut self, i2c: &mut I2c,reg: Register) -> Result<u8, Error<I2c>> {
        let mut buf = [0; 1];
        self.read(i2c, &[reg as u8], &mut buf)?;
        Ok(buf[0])
    }

    pub(crate) fn read_registers<'a>(
        &mut self,
        i2c: &mut I2c,
        reg: Register,
        buf: &'a mut [u8],
    ) -> Result<&'a [u8], Error<I2c>> {
        self.read(i2c, &[reg as u8], buf)?;
        Ok(buf)
    }

    pub(crate) fn write_register(&mut self, i2c: &mut I2c,reg: Register, value: u8) -> Result<(), Error<I2c>> {
        self.write(i2c, &[reg as u8, value])
    }

    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------

    /// Perform power reset of the MPU
    pub fn reset(&mut self, i2c: &mut I2c, clock: &mut impl delay::DelayMs<u32>) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::PwrMgmt1)?;
        value |= 1 << 7;
        self.write_register(i2c, Register::PwrMgmt1, value)?;
        clock.delay_ms(200);
        Ok(())
    }

    /// Perform reset of the signal path
    pub fn reset_signal_path(
        &mut self,
        i2c: &mut I2c,
        clock: &mut impl delay::DelayMs<u32>,
    ) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::UserCtrl)?;
        value |= 1 << 0;
        self.write_register(i2c, Register::UserCtrl, value)?;
        clock.delay_ms(200);
        Ok(())
    }

    /// Pick the clock-source
    pub fn set_clock_source(&mut self, i2c: &mut I2c,clock_source: ClockSource) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::PwrMgmt1)?;
        value |= clock_source as u8;
        self.write_register(i2c, Register::PwrMgmt1, value)?;
        Ok(())
    }

    pub fn disable_interrupts(&mut self, i2c: &mut I2c) -> Result<(), Error<I2c>> {
        self.write_register(i2c, Register::IntEnable, 0x00)
    }

    /// Super simple averaging calibration of the accelerometers.
    /// Probably should be called before initializing the DMP.
    // TODO that's obviously not optimal situation, fix this with typestates or something.
    // TODO: consider this: https://wired.chillibasket.com/2015/01/calibrating-mpu6050/
    pub fn calibrate_accel(
        &mut self,
        i2c: &mut I2c,
        loops: u8,
        delay: &mut impl delay::DelayMs<u32>,
    ) -> Result<(), Error<I2c>> {
        delay.delay_ms(10);
        let mut accumulator = [0i16; 3];
        for _ in 0..loops {
            let sample = self.accel(i2c, ).unwrap();
            accumulator[0] += ((sample.x() as f32) / loops as f32) as i16;
            accumulator[1] += ((sample.y() as f32) / loops as f32) as i16;
            accumulator[2] += ((sample.z() as f32) / loops as f32) as i16;
            delay.delay_ms(1);
        }

        let h = ((accumulator[0]) << 8) as u8;
        let l = (accumulator[0] & 0x00FF) as u8;
        self.write(i2c, &[Register::AccelOffsetX_H as u8, h, l])?;

        let h = ((accumulator[1]) << 8) as u8;
        let l = (accumulator[1] & 0x00FF) as u8;
        self.write(i2c, &[Register::AccelOffsetY_H as u8, h, l])?;

        let h = ((accumulator[2]) << 8) as u8;
        let l = (accumulator[2] & 0x00FF) as u8;
        self.write(i2c, &[Register::AccelOffsetZ_H as u8, h, l])?;

        Ok(())
    }

    /// Super simple averaging calibration of the gyroscopes.
    /// Probably should be called before initializing the DMP.
    // TODO that's obviously not optimal situation, fix this with typestates or something.
    // TODO: consider this: https://wired.chillibasket.com/2015/01/calibrating-mpu6050/
    pub fn calibrate_gyro(
        &mut self,
        i2c: &mut I2c,
        loops: u8,
        delay: &mut impl delay::DelayMs<u32>,
    ) -> Result<(), Error<I2c>> {
        delay.delay_ms(10);
        let mut accumulator = [0i16; 3];
        for _ in 0..loops {
            let gyros = self.gyro(i2c, ).unwrap();
            accumulator[0] += (gyros.x() as f32 / loops as f32) as i16;
            accumulator[1] += (gyros.y() as f32 / loops as f32) as i16;
            accumulator[2] += (gyros.z() as f32 / loops as f32) as i16;
            delay.delay_ms(1);
        }

        let h = ((accumulator[0]) << 8) as u8;
        let l = (accumulator[0] & 0x00FF) as u8;
        self.write(i2c, &[Register::GyroOffsetX_H as u8, h, l])?;

        let h = ((accumulator[1]) << 8) as u8;
        let l = (accumulator[1] & 0x00FF) as u8;
        self.write(i2c, &[Register::GyroOffsetY_H as u8, h, l])?;

        let h = ((accumulator[2]) << 8) as u8;
        let l = (accumulator[2] & 0x00FF) as u8;
        self.write(i2c, &[Register::GyroOffsetZ_H as u8, h, l])?;

        Ok(())
    }

    pub fn set_accel_full_scale(&mut self, i2c: &mut I2c,scale: AccelFullScale) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::AccelConfig)?;
        value |= (scale as u8) << 3;
        self.write_register(i2c, Register::AccelConfig, value)
    }

    pub fn set_gyro_full_scale(&mut self, i2c: &mut I2c,scale: GyroFullScale) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::GyroConfig)?;
        value |= (scale as u8) << 3;
        self.write_register(i2c, Register::GyroConfig, value)
    }

    pub fn set_sample_rate_divider(&mut self, i2c: &mut I2c,div: u8) -> Result<(), Error<I2c>> {
        self.write_register(i2c, Register::SmpRtDiv, div)
    }

    pub fn set_digital_lowpass_filter(
        &mut self,
        i2c: &mut I2c,
        filter: DigitalLowPassFilter,
    ) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::Config)?;
        value |= filter as u8;
        self.write_register(i2c, Register::Config, value)
    }

    pub fn reset_fifo(&mut self, i2c: &mut I2c,) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::UserCtrl)?;
        value |= 1 << 2;
        self.write_register(i2c, Register::UserCtrl, value)
    }

    pub fn enable_fifo(&mut self, i2c: &mut I2c,) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::UserCtrl)?;
        value |= 1 << 6;
        self.write_register(i2c, Register::UserCtrl, value)
    }

    /// Set the DMP bit.
    /// To perform full DMP initialization, see `initialize_dmp()`
    pub fn enable_dmp(&mut self, i2c: &mut I2c,) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::UserCtrl)?;
        value |= 1 << 7;
        self.write_register(i2c, Register::UserCtrl, value)
    }

    // Unset the DMP bit.
    pub fn disable_dmp(&mut self, i2c: &mut I2c,) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::UserCtrl)?;
        value &= !(1 << 7);
        self.write_register(i2c, Register::UserCtrl, value)
    }

    /// Reset the DMP processor
    pub fn reset_dmp(&mut self, i2c: &mut I2c,) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::UserCtrl)?;
        value |= 1 << 3;
        self.write_register(i2c, Register::UserCtrl, value)
    }

    /// Read the FIFO
    pub fn read_fifo<'a>(&mut self, i2c: &mut I2c,buf: &'a mut [u8]) -> Result<&'a [u8], Error<I2c>> {
        let mut len = self.get_fifo_count(i2c, )?;

        if buf.len() < len {
            len = buf.len();
        }

        if len == 0 {
            Ok(&buf[0..0])
        } else {
            self.read_registers(i2c, Register::FifoRw, &mut buf[0..len])
        }
    }

    pub fn get_fifo_enabled(&mut self, i2c: &mut I2c,) -> Result<Fifo, Error<I2c>> {
        let value = self.read_register(i2c, Register::FifoEn)?;
        Ok(Fifo::from_byte(value))
    }

    pub fn set_fifo_enabled(&mut self, i2c: &mut I2c,fifo: Fifo) -> Result<(), Error<I2c>> {
        self.write_register(i2c, Register::FifoEn, fifo.to_byte())
    }

    pub fn get_fifo_count(&mut self, i2c: &mut I2c,) -> Result<usize, Error<I2c>> {
        let mut buf = [0; 2];
        let _value = self.read_registers(i2c, Register::FifoCount_H, &mut buf)?;
        Ok(u16::from_be_bytes(buf) as usize)
    }

    pub fn disable_sleep(&mut self, i2c: &mut I2c,) -> Result<(), Error<I2c>> {
        let mut value = self.read_register(i2c, Register::PwrMgmt1)?;
        value &= !(1 << 6);
        self.write_register(i2c, Register::PwrMgmt1, value)
    }

    pub fn accel(&mut self, i2c: &mut I2c,) -> Result<Accel, Error<I2c>> {
        let mut data = [0; 6];
        self.read_registers(i2c, Register::AccelX_H, &mut data)?;
        Ok(Accel::new(data))
    }

    pub fn gyro(&mut self, i2c: &mut I2c,) -> Result<Gyro, Error<I2c>> {
        let mut data = [0; 6];
        self.read_registers(i2c, Register::GyroX_H, &mut data)?;
        Ok(Gyro::new(data))
    }
}

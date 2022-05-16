use mpu6050_dmp::accel::Accel;
use mpu6050_dmp::gyro::Gyro;
use crate::library::fixed_point::{FI32, FI64};
use crate::library::yaw_pitch_roll::YawPitchRoll;

pub struct RawMode {
    //TODO add filters
    yaw: FI64,
}

impl RawMode {
    pub fn new() -> Self {
        RawMode {
            yaw: FI64::from_num(0),
        }
    }

    pub fn update(&mut self, accel: Accel, gyro: Gyro, dt: u32, state: u32) -> YawPitchRoll {
        // Accel is in range [-2G, 2G]
        // Gyro is in range [-2000 deg, 2000 deg]

        let accel_x = FI32::from_bits(accel.x as i32);
        let accel_y = FI32::from_bits(accel.y as i32);
        let accel_z = FI32::from_bits(accel.z as i32);

        let pitch = cordic::atan2(accel_x, accel_z);
        let roll = cordic::atan2(accel_y, cordic::sqrt(accel_x*accel_x + accel_z * accel_z));

        let mut d_yaw = FI32::from_bits(-gyro.z as i32); //Change in 2000 deg/second
        /*
        We're gonna do some trickery to convert the unit (2000 deg/second) to radians.
        dt is in 10^-6 seconds, so we get:
        dyaw * 2000 * (2*pi/360) * (dt/10^6)

        The number in radians will be too small to represent as a FI32
        Instead, we're gonna calculate the middle 32 bits of a FI64 with 48 decimal points (we really don't need the lower 16 bits, but fixed doesn't support 48 bit numbers), which has a value of 2^-16, then add those to the FI64
         */

        // First to deg/second, then to rad/second
        d_yaw *= FI32::from_num(2000) / 360 * 2 * FI32::PI;

        if state % 1000 == 0 {
            log::info!("{} {}", d_yaw, self.yaw);
        }

        // Then, we convert to radians. At the same time, we convert it to
        d_yaw *= FI32::from_num(dt);
        d_yaw *= FI32::from_num(0.065536); //(2^16 / 10^6);

        // Negative since yaw is the wrong way around in comparison with gyro
        self.yaw += FI64::from_bits((d_yaw.to_bits() as i64) << 16);

        YawPitchRoll {
            yaw: FI32::from_bits((self.yaw.to_bits() >> 32) as i32),
            pitch,
            roll
        }
    }
}
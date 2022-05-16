use mpu6050_dmp::accel::Accel;
use mpu6050_dmp::gyro::Gyro;
use crate::library::fixed_point::FI32;
use crate::library::yaw_pitch_roll::YawPitchRoll;

pub struct RawMode {
    //TODO add filters
    yaw: FI32,
}

impl RawMode {
    pub fn new() -> Self {
        RawMode {
            yaw: FI32::from_num(0),
        }
    }
    pub fn update(&mut self, accel: Accel, gyro: Gyro) -> YawPitchRoll {
        // Accel is in range [-2G, 2G]
        // Gyro is in range [-2000 deg, 2000 deg]

        let accel_x = FI32::from_bits(accel.x as i32);
        let accel_y = FI32::from_bits(accel.y as i32);
        let accel_z = FI32::from_bits(accel.z as i32);

        let pitch = cordic::atan2(accel_x, accel_z);
        let roll = cordic::atan2(accel_y, cordic::sqrt(accel_x*accel_x + accel_z * accel_z));

        YawPitchRoll {
            yaw: FI32::from_num(0),
            pitch,
            roll
        }
    }
}
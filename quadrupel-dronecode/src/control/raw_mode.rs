use mpu6050_dmp::accel::Accel;
use mpu6050_dmp::gyro::Gyro;
use crate::filters::butterworth_2nd::ButterworthLowPass2nd;
use crate::library::fixed_point::{atan2_approx, FI32, FI64, sqrt_approx};
use crate::library::yaw_pitch_roll::YawPitchRoll;

pub struct RawMode {
    //TODO add filters
    yaw: FI64,
    yaw_filter: ButterworthLowPass2nd,
}

impl RawMode {
    pub fn new() -> Self {
        let a_yi = FI32::from_num(35.639);
        let a_yi_1 = FI32::from_num(52.512)/a_yi;
        let a_yi_2 = FI32::from_num(-20.873)/a_yi;
        let a_xi = FI32::from_num(1)/a_yi;
        let a_xi_1 = FI32::from_num(2)/a_yi;
        let a_xi_2 = FI32::from_num(1)/a_yi;

        RawMode {
            yaw: FI64::from_num(0),
            yaw_filter: ButterworthLowPass2nd::new(
                a_yi,
                a_yi_1,
                a_yi_2,
                a_xi,
                a_xi_1,
                a_xi_2,
            )
        }
    }

    pub fn update(&mut self, accel: Accel, gyro: Gyro, dt: u32) -> YawPitchRoll {
        // Accel is in range [-2G, 2G]
        // Gyro is in range [-2000 deg, 2000 deg]

        let accel_x: FI32 = FI32::from_bits(accel.x as i32);
        let accel_y: FI32 = FI32::from_bits(accel.y as i32);
        let accel_z: FI32 = FI32::from_bits(accel.z as i32);
        let gyro_z: FI32 = FI32::from_bits(gyro.z as i32);

        let pitch = atan2_approx(accel_x, accel_z);
        let roll = atan2_approx(accel_y, sqrt_approx(accel_x*accel_x + accel_z * accel_z));

        /*
        We're gonna do some trickery to convert the unit (2000 deg/second) to radians.
        dt is in 10^-6 seconds, so we get:
        dyaw * 2000 * (2*pi/360) * (dt/10^6)

        The number in radians will be too small to represent as a FI32
        Instead, we're gonna calculate the middle 32 bits of a FI64 with 48 decimal points (we really don't need the lower 16 bits, but fixed doesn't support 48 bit numbers), which has a value of 2^-16, then add those to the FI64
         */
        let mut d_yaw = gyro_z; //Change in 2000 deg/second

        // First to deg/second, then to rad/second
        d_yaw *= FI32::from_num(2000);
        d_yaw /= 360;
        d_yaw *= 2;
        d_yaw *= FI32::PI;
        d_yaw *= 2; // No clue why this one is needed

        // Then, we convert to radians. At the same time, we convert it to
        d_yaw *= FI32::from_num(dt);
        d_yaw *= FI32::from_num(0.065536); //(2^16 / 10^6);

        // Negative since yaw is the wrong way around in comparison with gyro
        self.yaw += FI64::from_bits((d_yaw.to_bits() as i64) << 16);
        if self.yaw > FI64::PI {
            self.yaw -= 2 * FI64::PI;
        }
        if self.yaw < -FI64::PI {
            self.yaw += 2 * FI64::PI;
        }

        let yaw = FI32::from_bits((self.yaw.to_bits() >> 32) as i32);
        let yaw: FI32 = self.yaw_filter.filter(yaw);

        YawPitchRoll {
            yaw,
            pitch,
            roll
        }
    }
}
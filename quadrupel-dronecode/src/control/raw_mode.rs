use mpu6050_dmp::accel::Accel;
use mpu6050_dmp::gyro::Gyro;
use crate::filters::butterworth_2nd::ButterworthLowPass2nd;
use crate::filters::kalman_filter::KalFilter;
use crate::library::fixed_point::{atan2_approx, FI32, FI64, sqrt_approx};
use crate::library::yaw_pitch_roll::YawPitchRoll;

pub struct RawMode {
    //TODO add filters
    yaw: FI64,
    yaw_filter: ButterworthLowPass2nd,
    roll_bw_filter: ButterworthLowPass2nd,
    roll_filter: KalFilter,
    pitch_bw_filter: ButterworthLowPass2nd,
    pitch_filter: KalFilter,
    accel_bw_filter: [ButterworthLowPass2nd; 2],
}

impl RawMode {
    pub fn new() -> Self {
        // 30 hz
        // TODO: Tune all filters (and possibly make them different across different filters)
        let a_yi = FI32::from_num(35.894);
        let a_yi_1 = FI32::from_num(52.961)/a_yi;
        let a_yi_2 = FI32::from_num(-21.667)/a_yi;
        let a_xi = FI32::from_num(1)/a_yi;
        let a_xi_1 = FI32::from_num(2)/a_yi;
        let a_xi_2 = FI32::from_num(1)/a_yi;

        // 15 hz
        // let a_yi = FI32::from_num(127.874);
        // let a_yi_1 = FI32::from_num(221.826)/a_yi;
        // let a_yi_2 = FI32::from_num(-97.952)/a_yi;
        // let a_xi = FI32::from_num(1)/a_yi;
        // let a_xi_1 = FI32::from_num(2)/a_yi;
        // let a_xi_2 = FI32::from_num(1)/a_yi;

        // 10 hz
        // let a_yi = FI32::from_num(276.115);
        // let a_yi_1 = FI32::from_num(503.273)/a_yi;
        // let a_yi_2 = FI32::from_num(-231.158)/a_yi;
        // let a_xi = FI32::from_num(1)/a_yi;
        // let a_xi_1 = FI32::from_num(2)/a_yi;
        // let a_xi_2 = FI32::from_num(1)/a_yi;

        // 3.32858877e-05 5.51620221e-03 6.48176954e-05
        let a_yi_a = FI32::from_num(4.841);
        let a_yi_1_a = FI32::from_num(1.789)/a_yi;
        let a_yi_2_a = FI32::from_num(-0.948)/a_yi;

        //3.32858877e-05 5.51620221e-03 6.48176954e-05
        let kal_q_angle = FI64::from_num(0.0000459274);
        let kal_q_bias = FI64::from_num(-0.00000013829);
        let kal_r_measure = FI64::from_num( 0.000015617);

        RawMode {
            yaw: FI64::from_num(0),
            yaw_filter: ButterworthLowPass2nd::new(
                a_yi,
                a_yi_1,
                a_yi_2,
                a_xi,
                a_xi_1,
                a_xi_2,
            ),
            roll_bw_filter: ButterworthLowPass2nd::new(
                a_yi,
                a_yi_1,
                a_yi_2,
                a_xi,
                a_xi_1,
                a_xi_2,
            ),
            // roll_filter: ComplFilter::new(
            //     FI32::from_num(22.8),
            //     FI32::from_num(12000),
            //     false,
            // ),
            roll_filter: KalFilter::new(
                kal_q_angle,
                kal_q_bias,
                kal_r_measure,
            ),
            pitch_bw_filter: ButterworthLowPass2nd::new(
                a_yi,
                a_yi_1,
                a_yi_2,
                a_xi,
                a_xi_1,
                a_xi_2,
            ),
            // pitch_filter: ComplFilter::new(
            //     FI32::from_num(22.8),
            //     FI32::from_num(12000),
            //     false,
            // ),
            pitch_filter: KalFilter::new(
                kal_q_angle,
                kal_q_bias,
                kal_r_measure,
            ),
            accel_bw_filter: [ButterworthLowPass2nd::new(
                a_yi_a,
                a_yi_1_a,
                a_yi_2_a,
                a_xi,
                a_xi_1,
                a_xi_2,
            ),
                ButterworthLowPass2nd::new(
                    a_yi_a,
                    a_yi_1_a,
                    a_yi_2_a,
                    a_xi,
                    a_xi_1,
                    a_xi_2,
                )]
        }
    }

    pub fn update(&mut self, accel: Accel, gyro: Gyro, dt: u32) -> (YawPitchRoll, YawPitchRoll, FI32, FI32) {
        // Accel is in range [-2G, 2G]
        // Gyro is in range [-2000 deg, 2000 deg]

        let accel_x: FI32 = FI32::from_bits(accel.x as i32);
        let accel_y: FI32 = FI32::from_bits(accel.y as i32);
        let accel_z: FI32 = FI32::from_bits(accel.z as i32);

        let gyro_pitch: FI32 = FI32::from_bits(gyro.x as i32);
        let gyro_roll: FI32 = FI32::from_bits(gyro.y as i32);
        let gyro_yaw: FI32 = FI32::from_bits(gyro.z as i32);

        let pitch = atan2_approx(accel_x, accel_z);
        let roll = atan2_approx(accel_y, sqrt_approx(accel_x*accel_x + accel_z * accel_z));

        let rp1 = pitch;
        let rp2 = gyro_pitch;

        let roll = self.accel_bw_filter[0].filter(roll);
        let pitch = self.accel_bw_filter[1].filter(pitch);
        let (roll_deriv, roll) = self.roll_filter.filter(gyro_roll, roll, dt);
        let (pitch_deriv, pitch) = self.pitch_filter.filter(gyro_pitch, pitch, dt);
        let roll = self.roll_bw_filter.filter(roll);
        let pitch = self.pitch_bw_filter.filter(pitch);

        /*
        We're gonna do some trickery to convert the unit (2000 deg/second) to radians.
        dt is in 10^-6 seconds, so we get:
        dyaw * 2000 * (2*pi/360) * (dt/10^6)

        The number in radians will be too small to represent as a FI32
        Instead, we're gonna calculate the middle 32 bits of a FI64 with 48 decimal points (we really don't need the lower 16 bits, but fixed doesn't support 48 bit numbers), which has a value of 2^-16, then add those to the FI64
         */
        let mut d_yaw = gyro_yaw    ; //Change in 2000 deg/second

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

        let ypr = YawPitchRoll {
            yaw,
            pitch,
            roll
        };
        let dypr = YawPitchRoll {
            yaw: FI32::ZERO,
            pitch: pitch_deriv,
            roll: roll_deriv,
        };

        (ypr, dypr, rp1, rp2)
    }
}
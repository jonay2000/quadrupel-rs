pub mod state;

use quadrupel_shared::MotorValue;
use crate::library::yaw_pitch_roll::YawPitchRoll;

pub struct SensorValues {
    mpu: YawPitchRoll,
}

pub fn calculate_motor_values(sensors: &SensorValues) -> [MotorValue; 4] {
    todo!()
}
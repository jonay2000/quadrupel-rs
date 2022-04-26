pub mod state;

use crate::library::yaw_pitch_roll::YawPitchRoll;
use quadrupel_shared::MotorValue;

pub struct SensorValues {
    mpu: YawPitchRoll,
}

pub fn calculate_motor_values(sensors: &SensorValues) -> [MotorValue; 4] {
    todo!()
}

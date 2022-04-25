use crate::library::yaw_pitch_roll::YawPitchRoll;

mod state;
mod message;

pub struct SensorValues {
    mpu: YawPitchRoll,
}

pub type MotorValue = u16;

pub fn calculate_motor_values(sensors: &SensorValues) -> [MotorValue; 4] {
    todo!()
}
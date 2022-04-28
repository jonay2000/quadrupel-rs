use quadrupel_shared::message::Motor;
use quadrupel_shared::{MotorValue, MotorValueDelta};
use quadrupel_shared::state::Mode;
use crate::Motors;

pub struct FlightState {
    pub mode: Mode,
    pub motor_values: [MotorValue; 4],
    pub last_heartbeat: u32,
}

impl FlightState {
    pub fn update_motor(&mut self, motor: Motor, delta: MotorValueDelta) {
        let current = self.motor_values[motor as usize] as i32;
        self.motor_values[motor as usize] = (current + delta).max(0) as MotorValue;
    }
}

impl Default for FlightState {
    fn default() -> Self {
        Self {
            mode: Mode::Safe,
            motor_values: [0; 4],
            last_heartbeat: Motors::get_time_us(),
        }
    }
}
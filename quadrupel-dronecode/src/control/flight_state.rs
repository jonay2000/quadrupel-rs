use quadrupel_shared::message::Motor;
use quadrupel_shared::{MotorValue, MotorValueDelta};
use quadrupel_shared::state::Mode;
use crate::motors::GlobalTime;

pub struct FlightState {
    pub mode: Mode,
    pub motor_values: [MotorValue; 4],
    pub last_heartbeat: u32,
}

impl Default for FlightState {
    fn default() -> Self {
        Self {
            mode: Mode::Safe,
            motor_values: [0; 4],
            last_heartbeat: GlobalTime().get_time_us(),
        }
    }
}
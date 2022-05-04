use quadrupel_shared::message::Motor;
use quadrupel_shared::{MotorValue, MotorValueDelta};
use quadrupel_shared::state::Mode;
use crate::library::yaw_pitch_roll::FI32;
use crate::motors::GlobalTime;

pub struct FlightState {
    pub mode: Mode,
    pub motor_values: [MotorValue; 4],
    pub last_heartbeat: u32,
    pub target_attitude: TargetAttitude,
}

pub struct TargetAttitude {
    /// -8 full left, 8 full right
    pub yaw: FI32,
    /// -8 full forward, 8 full backward
    pub pitch: FI32,
    /// -8 full left, 8 full right
    pub roll: FI32,
    /// 0 down, 16 up
    pub lift: FI32,
}

impl Default for FlightState {
    fn default() -> Self {
        Self {
            mode: Mode::Safe,
            motor_values: [0; 4],
            last_heartbeat: GlobalTime().get_time_us(),
            target_attitude: TargetAttitude {
                yaw: FI32::from_num(0),
                pitch: FI32::from_num(0),
                roll: FI32::from_num(0),
                lift: FI32::from_num(0),
            }
        }
    }
}
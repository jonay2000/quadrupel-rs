use crate::control::angle_mode::AngleMode;
use crate::library::pid::PID;
use crate::library::yaw_pitch_roll::FI32;
use crate::motors::GlobalTime;
use quadrupel_shared::state::Mode;
use quadrupel_shared::MotorValue;

pub struct FlightState {
    pub mode: Mode,
    pub motor_values: [Option<MotorValue>; 4],
    pub last_heartbeat: u32,
    pub current_attitude: TargetAttitude,
    pub target_attitude: TargetAttitude,
    pub angle_mode: AngleMode,
    pub count: u32,

    pub flash_record: bool,
    pub flash_send: bool,
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
            motor_values: [None; 4],
            last_heartbeat: GlobalTime().get_time_us(),
            current_attitude: TargetAttitude {
                yaw: FI32::from_num(0),
                pitch: FI32::from_num(0),
                roll: FI32::from_num(0),
                lift: FI32::from_num(0),
            },
            target_attitude: TargetAttitude {
                yaw: FI32::from_num(0),
                pitch: FI32::from_num(0),
                roll: FI32::from_num(0),
                lift: FI32::from_num(0),
            },
            angle_mode: AngleMode {
                yaw_pid: PID::new(
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0), //500
                    FI32::from_num(0),
                    true,
                ),
                pitch_pid: PID::new(
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0),
                    true,
                ),
                roll_pid: PID::new(
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0),
                    true,
                ),
            },
            count: 0,
            flash_record: false,
            flash_send: false,
        }
    }
}

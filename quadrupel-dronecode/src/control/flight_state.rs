use crate::control::angle_mode::AngleMode;
use crate::library::pid::PID;
use crate::library::fixed_point::FI32;
use quadrupel_shared::state::Mode;
use quadrupel_shared::MotorValue;
use crate::control::calibrate::Calibrate;
use crate::control::raw_mode::RawMode;
use crate::TIME;

pub struct FlightState {
    pub mode: Mode,
    pub motor_values: [Option<MotorValue>; 4],
    pub last_heartbeat: u32,
    pub current_attitude: CurrentAttitude,
    pub target_attitude: TargetAttitude,
    pub angle_mode: AngleMode,
    pub count: u32,
    pub raw_mode: RawMode,

    pub flash_record: bool,
    pub flash_send: bool,

    pub calibrate: Calibrate,

    pub height_mode_enable: bool,
    pub raw_mode_enable: bool,
    pub autoland_enable: bool,

    pub height_lock: Option<(FI32, FI32)>,
    pub pid_contributions: [FI32; 5],

    pub cs: [FI32; 2],
}

pub struct CurrentAttitude {
    pub yaw: FI32,
    pub pitch: FI32,
    pub roll: FI32,
    pub height: FI32,
    pub pitch_rate: Option<FI32>,
    pub roll_rate: Option<FI32>,
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
            last_heartbeat: TIME.as_mut_ref().get_time_us(),
            current_attitude: CurrentAttitude {
                yaw: FI32::from_num(0),
                pitch: FI32::from_num(0),
                roll: FI32::from_num(0),
                height: FI32::from_num(0),
                pitch_rate: None,
                roll_rate: None,
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
                height_pid: PID::new(
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0),
                    FI32::from_num(0),
                    false,
                ),
            },
            count: 0,
            raw_mode: RawMode::new(),
            flash_record: false,
            flash_send: false,
            calibrate: Calibrate::new(),
            height_lock: None,
            height_mode_enable: false,
            autoland_enable: false,
            raw_mode_enable: false,
            pid_contributions: [FI32::ZERO; 5],
            cs: [FI32::ZERO; 2]
        }
    }
}

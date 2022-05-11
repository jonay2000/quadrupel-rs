use crate::control::flight_state::{FlightState, TargetAttitude};
use crate::library::yaw_pitch_roll::FI32;
use crate::motors::GlobalTime;
use quadrupel_shared::message::MessageToDrone;
use quadrupel_shared::state::Mode;
use quadrupel_shared::MotorValue;

pub fn process_message(message: MessageToDrone, state: &mut FlightState) {
    // Always immediately handle panics
    if let MessageToDrone::ChangeState(Mode::Panic) = message {
        log::error!("Panic: Received change state to panic");
        state.mode = Mode::Panic;
        return;
    }
    state.last_heartbeat = GlobalTime().get_time_us();

    match message {
        MessageToDrone::ChangeState(new_mode) => {
            //Currently, allow all mode changes
            //TODO check if mode change is allowed
            state.mode = new_mode;
        }
        MessageToDrone::MotorValue { motor, value } => {
            state.motor_values[motor as usize] = value;
        }
        MessageToDrone::MotorValueRel { motor, value } => {
            let current = state.motor_values[motor as usize] as i32;
            state.motor_values[motor as usize] = (current + value).max(0) as MotorValue;
        }
        // inputs are [2^-19 to 2^19]
        MessageToDrone::TargetAttitude {
            yaw,
            pitch,
            roll,
            lift,
        } => {
            state.target_attitude = TargetAttitude {
                yaw: FI32::from_bits(yaw),
                pitch: FI32::from_bits(pitch),
                roll: FI32::from_bits(roll),
                lift: FI32::from_bits(lift),
            }
        }
        MessageToDrone::HeartBeat(_) => {}
        MessageToDrone::TunePID { yaw_P, yaw_I, yaw_D, yaw_CAP, pitch_P, pitch_I, pitch_D, pitch_CAP, roll_P, roll_I, roll_D, roll_CAP } => {
            state.angle_mode.yaw_pid.kp = FI32::from_bits(yaw_P as i32);
            state.angle_mode.yaw_pid.ki = FI32::from_bits(yaw_I as i32);
            state.angle_mode.yaw_pid.kd = FI32::from_bits(yaw_D as i32);
            state.angle_mode.yaw_pid.cap = FI32::from_bits(yaw_CAP as i32);

            state.angle_mode.pitch_pid.kp = FI32::from_bits(pitch_P as i32);
            state.angle_mode.pitch_pid.ki = FI32::from_bits(pitch_I as i32);
            state.angle_mode.pitch_pid.kd = FI32::from_bits(pitch_D as i32);
            state.angle_mode.pitch_pid.cap = FI32::from_bits(pitch_CAP as i32);

            state.angle_mode.roll_pid.kp = FI32::from_bits(roll_P as i32);
            state.angle_mode.roll_pid.ki = FI32::from_bits(roll_I as i32);
            state.angle_mode.roll_pid.kd = FI32::from_bits(roll_D as i32);
            state.angle_mode.roll_pid.cap = FI32::from_bits(roll_CAP as i32);
        }
    }
}

use crate::control::flight_state::{FlightState, TargetAttitude};
use crate::library::fixed_point::FI32;
use quadrupel_shared::message::MessageToDrone;
use quadrupel_shared::state::Mode;
use quadrupel_shared::MotorValue;
use crate::TIME;

pub fn process_message(message: MessageToDrone, state: &mut FlightState) {
    // Always immediately handle panics
    if let MessageToDrone::ChangeState(Mode::Panic) = message {
        log::error!("Panic: Received change state to panic");
        state.mode = Mode::Panic;
        return;
    }
    state.last_heartbeat = TIME.as_mut_ref().get_time_us();

    match message {
        MessageToDrone::ChangeState(new_mode) => {
            //Currently, allow all mode changes
            //TODO check if mode change is allowed
            state.mode = new_mode;
        }
        MessageToDrone::MotorValue { motor, value } => {
            state.motor_values[motor as usize] = Some(value);
        }
        MessageToDrone::MotorValueRel { motor, value } => {
            match &mut state.motor_values[motor as usize] {
                None => {}
                Some(v) => *v = (*v as i32 + value).max(0) as MotorValue,
            };
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
        MessageToDrone::TunePID {
            yaw_P,
            yaw_I,
            yaw_D,
            yaw_CAP,
            pitch_P,
            pitch_I,
            pitch_D,
            pitch_CAP,
            roll_P,
            roll_I,
            roll_D,
            roll_CAP, height_P, height_I, height_D, height_CAP, c1, c2,
        } => {
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

            state.angle_mode.height_pid.kp = FI32::from_bits(height_P as i32);
            state.angle_mode.height_pid.ki = FI32::from_bits(height_I as i32);
            state.angle_mode.height_pid.kd = FI32::from_bits(height_D as i32);
            state.angle_mode.height_pid.cap = FI32::from_bits(height_CAP as i32);

            state.cs = [FI32::from_bits(c1 as i32), FI32::from_bits(c2 as i32)];
        }
        MessageToDrone::FlashStartRecording => {
            log::info!("Start recording.");
            state.flash_record = true;
        }
        MessageToDrone::FlashStopRecording => {
            log::info!("Stop recording.");
            state.flash_record = false;
        }
        MessageToDrone::FlashRead => {
            state.flash_record = false;
            state.flash_send = true;
        },
        MessageToDrone::SetHeightMode(_) => {
            state.height_mode_enable = !state.height_mode_enable;
        }
        MessageToDrone::SetRawMode(_) => {
            state.raw_mode_enable = !state.raw_mode_enable;
        }
        MessageToDrone::AutoLand => {
            if state.height_mode_enable {
                state.autoland_enable = true;
            }
        }
    }
}

use quadrupel_shared::message::MessageToDrone;
use quadrupel_shared::MotorValue;
use quadrupel_shared::state::Mode;
use crate::control::flight_state::{FlightState, TargetAttitude};
use crate::control::modes::individual_motor_control::IndividualMotorControlMode;
use crate::control::modes::ModeTrait;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;
use crate::library::yaw_pitch_roll::FI32;
use crate::motors::GlobalTime;

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
        MessageToDrone::TargetAttitude { yaw, pitch, roll, lift } => {
            state.target_attitude = TargetAttitude {
                yaw: FI32::from_bits(yaw),
                pitch: FI32::from_bits(pitch),
                roll: FI32::from_bits(roll),
                lift: FI32::from_bits(lift),
            }
        }
        MessageToDrone::HeartBeat(_) => {}
        MessageToDrone::TunePID { .. } => {}
    }
}




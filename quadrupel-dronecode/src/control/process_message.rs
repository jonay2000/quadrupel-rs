use quadrupel_shared::message::MessageToDrone;
use quadrupel_shared::state::Mode;
use crate::{FlightState, Motors};
use crate::control::modes::individual_motor_control::IndividualMotorControlMode;
use crate::control::modes::ModeTrait;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;

pub fn process_message(message: MessageToDrone, state: &mut FlightState) {
    // Always immediately handle panics
    if let MessageToDrone::ChangeState(Mode::Panic) = message {
        state.mode = Mode::Panic;
        return;
    }
    state.last_heartbeat = Motors::get_time_us();

    match state.mode {
        Mode::Safe => SafeMode::handle_message(state, message),
        Mode::Calibration => {}
        Mode::Panic => PanicMode::handle_message(state, message),
        Mode::FullControl => {}
        Mode::IndividualMotorControl => IndividualMotorControlMode::handle_message(state, message),
    }
}




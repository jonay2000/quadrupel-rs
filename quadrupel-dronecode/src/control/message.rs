use quadrupel_shared::message::ReceiveMessage;
use quadrupel_shared::state::Mode;
use crate::{FlightState, QUart};

pub fn process_message(message: ReceiveMessage, state: &mut FlightState) {
    // Always immediately handle panics
    if let ReceiveMessage::ChangeState(Mode::Panic) = message {
        state.set_mode(Mode::Panic);
        return;
    }

    match state.get_mode() {
        Mode::Safe => match message {
            ReceiveMessage::ChangeState(Mode::IndividualMotorControl) => {
                state.set_mode(Mode::IndividualMotorControl);
            }
            _ => {
                // in safe mode we only react to requests to go to other states.
                // To do anything else, change the state out of safe mode
            }
        },
        Mode::Calibration => {}
        Mode::Panic => {
            // don't respond to any more messages until we are in safe mode
        }
        Mode::FullControl => {}
        Mode::IndividualMotorControl => match message {
            ReceiveMessage::MotorValue { motor, value } => {
                state.set_motor(motor, value)
            }
            ReceiveMessage::MotorValueRel { motor, value } => {
                state.update_motor(motor, value)
            }
            msg => {
                log::warn!("ignoring {:?} in", msg)
            }
        }
    }
}




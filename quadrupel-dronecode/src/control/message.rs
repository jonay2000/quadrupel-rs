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

#[derive(Debug)]
pub enum ReadMessageError {
    TooShort,
    Bincode(bincode::error::DecodeError),
}

// NEVER CALL CONCURRENTLY (FROM INTERRUPT)
pub unsafe fn read_message(uart: &QUart, bytes: usize) -> Result<ReceiveMessage, ReadMessageError> {
    // TODO: can also be the same buffer as used during serializing.
    static mut DESERIALIZE_BUFFER: [u8; 256] = [0u8; 256];

    for i in 0..bytes {
        DESERIALIZE_BUFFER[i] = uart.get_byte().ok_or(ReadMessageError::TooShort)?;
    }

    Ok(ReceiveMessage::decode(&DESERIALIZE_BUFFER[..bytes]).map_err(ReadMessageError::Bincode)?.0)
}


use quadrupel_shared::message::MessageToDrone;
use quadrupel_shared::state::Mode;
use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;

pub struct SafeMode;

impl ModeTrait for SafeMode {
    fn iteration(state: &mut FlightState) {
        state.motor_values = [0; 4];
    }

    fn handle_message(state: &mut FlightState, message: MessageToDrone) {
        match message {
            MessageToDrone::ChangeState(Mode::IndividualMotorControl) => {
                state.mode = Mode::IndividualMotorControl;
            }
            _ => {
                // in safe mode we only react to requests to go to other states.
                // To do anything else, change the state out of safe mode
            }
        }
    }
}
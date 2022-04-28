use quadrupel_shared::message::MessageToDrone;
use crate::control::modes::ModeTrait;
use crate::FlightState;

pub struct IndividualMotorControlMode;

impl ModeTrait for IndividualMotorControlMode {
    fn iteration(_state: &mut FlightState) {

    }

    fn handle_message(state: &mut FlightState, message: MessageToDrone) {
        match message {
            MessageToDrone::MotorValue { motor, value } => {
                state.motor_values[motor as usize] = value;
            }
            MessageToDrone::MotorValueRel { motor, value } => {
                state.update_motor(motor, value)
            }
            _ => {}
        }
    }
}
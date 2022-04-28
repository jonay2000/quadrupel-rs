use quadrupel_shared::message::MessageToDrone;
use crate::FlightState;

pub mod panic;
pub mod safe;
pub mod individual_motor_control;

pub trait ModeTrait {
    fn iteration(state: &mut FlightState);
    fn handle_message(state: &mut FlightState, message: MessageToDrone);
}
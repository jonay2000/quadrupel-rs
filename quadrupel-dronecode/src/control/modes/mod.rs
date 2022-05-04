use quadrupel_shared::message::MessageToDrone;
use crate::control::flight_state::FlightState;

pub mod panic;
pub mod safe;
pub mod individual_motor_control;
pub mod manual_control;

pub trait ModeTrait {
    fn iteration(state: &mut FlightState);
}
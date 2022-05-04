use quadrupel_shared::message::MessageToDrone;
use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;

pub struct IndividualMotorControlMode;

impl ModeTrait for IndividualMotorControlMode {
    fn iteration(_state: &mut FlightState, _dt: u32) {

    }
}
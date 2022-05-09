use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use quadrupel_shared::message::MessageToDrone;

pub struct IndividualMotorControlMode;

impl ModeTrait for IndividualMotorControlMode {
    fn iteration(_state: &mut FlightState) {}
}

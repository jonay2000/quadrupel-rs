use quadrupel_shared::message::MessageToDrone;
use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;

pub struct ManualControl;

impl ModeTrait for ManualControl {
    fn iteration(_state: &mut FlightState) {

    }
}
use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use quadrupel_shared::message::MessageToDrone;

pub struct ManualControl;

impl ModeTrait for ManualControl {
    fn iteration(_state: &mut FlightState) {}
}

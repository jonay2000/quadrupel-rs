use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;

pub struct SafeMode;

impl ModeTrait for SafeMode {
    fn iteration(state: &mut FlightState, _dt: u32) {
        state.motor_values = [None; 4];
    }
}

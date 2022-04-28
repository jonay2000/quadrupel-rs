use crate::FlightState;

/// Executed every event loop cycle when in safe mode
#[inline]
pub fn safe_mode(state: &mut FlightState) {
    // while in safe mode, force the motors off
    state.zero_motors();

}
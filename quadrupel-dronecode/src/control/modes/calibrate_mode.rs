use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;

pub struct CalibrateMode;

impl ModeTrait for CalibrateMode {
    fn iteration(state: &mut FlightState, _dt: u32) {
        state.calibrate.calibrate(state.current_attitude.yaw,state.current_attitude.pitch,state.current_attitude.roll)
    }
}

use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use crate::library::fixed_point::FI32;

pub struct CalibrateMode;

impl ModeTrait for CalibrateMode {
    fn iteration(state: &mut FlightState, _dt: u32) {
        state.calibrate.calibrate(state.current_attitude.yaw,state.current_attitude.pitch,state.current_attitude.roll,FI32::from_num(0))
    }
}

use crate::control::flight_state::FlightState;

pub mod full_control;
pub mod individual_motor_control;
pub mod manual_control;
pub mod panic;
pub mod safe;
pub mod calibrate_mode;

pub trait ModeTrait {
    fn iteration(state: &mut FlightState, dt: u32);
}

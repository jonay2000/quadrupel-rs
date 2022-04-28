use quadrupel_shared::MotorValue;
use crate::FlightState;

static MINIMAL_HOVERING_MOTOR_VALUE: MotorValue = 400;
static PANIC_MOTOR_REDUCTION_SPEED: u16 = 5;

/// Executed every event loop cycle when in panic mode
#[inline]
pub fn panic_mode(flight_state: &mut FlightState) {
    for i in flight_state.get_motors_mut() {
        // if the speed of a motor is larger than MINIMAL_HOVER_VALUE,
        // set it to that, and then reduce it by 5
        if *i > MINIMAL_HOVERING_MOTOR_VALUE {
            *i = MINIMAL_HOVERING_MOTOR_VALUE;
        } else if *i > PANIC_MOTOR_REDUCTION_SPEED {
            *i -= PANIC_MOTOR_REDUCTION_SPEED;
        } else {
            *i = 0;
        }
    }
}
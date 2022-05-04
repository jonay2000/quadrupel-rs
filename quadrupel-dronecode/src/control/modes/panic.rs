use quadrupel_shared::message::MessageToDrone;
use quadrupel_shared::MotorValue;
use quadrupel_shared::state::Mode;
use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;

static MINIMAL_HOVERING_MOTOR_VALUE: MotorValue = 400;
static PANIC_MOTOR_REDUCTION_SPEED: u16 = 5;

pub struct PanicMode;

impl ModeTrait for PanicMode {
    fn iteration(state: &mut FlightState, dt: u32) {
        // state.motor_values = [0; 4];
        for i in &mut state.motor_values {
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

        if state.motor_values.iter().all(|&i| i == 0) {
            state.mode = Mode::Safe;
        }
    }
}
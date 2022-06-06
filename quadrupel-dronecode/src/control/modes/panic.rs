use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use quadrupel_shared::state::Mode;
use quadrupel_shared::MotorValue;

static MINIMAL_HOVERING_MOTOR_VALUE: MotorValue = 400;
static PANIC_MOTOR_REDUCTION_SPEED: u16 = 1;

pub struct PanicMode;

impl ModeTrait for PanicMode {
    fn iteration(state: &mut FlightState, _dt: u32) {
        for i in &mut state.motor_values {
            // if the speed of a motor is larger than MINIMAL_HOVER_VALUE,
            // set it to that, and then reduce it by 5
            *i = match *i {
                None => None,
                Some(v) => {
                    if v > MINIMAL_HOVERING_MOTOR_VALUE {
                        Some(MINIMAL_HOVERING_MOTOR_VALUE)
                    } else if v > PANIC_MOTOR_REDUCTION_SPEED {
                        Some(v - PANIC_MOTOR_REDUCTION_SPEED)
                    } else {
                        None
                    }
                }
            };
        }

        if state.motor_values.iter().all(|&i| i.is_none()) {
            state.mode = Mode::Safe;
        }
    }
}

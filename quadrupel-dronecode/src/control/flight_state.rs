use core::sync::atomic::{AtomicBool, Ordering};
use quadrupel_shared::message::Motor;
use quadrupel_shared::{MotorValue, MotorValueDelta};
use quadrupel_shared::state::Mode;

static WANT_PANIC: AtomicBool = AtomicBool::new(false);
/// Set the flight state to panic, also works from interrupts or
/// places where there's no access to the flight state
pub fn go_panic() {
    WANT_PANIC.store(true, Ordering::SeqCst);
}

pub struct FlightState {
    mode: Mode,
    motor_values: [MotorValue; 4],
}

impl FlightState {
    /// Sets the current state to panic mode whenever something requested that (asynchrously)
    /// using [`go_panic`]. Always called at the start of the event loop.
    pub fn check_panic(&mut self) {
        if WANT_PANIC.load(Ordering::SeqCst) {
            self.set_mode(Mode::Panic);
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn zero_motors(&mut self) {
        self.motor_values = [0; 4];
    }
    pub fn get_motors_mut(&mut self) -> &mut [MotorValue; 4] {
        &mut self.motor_values
    }

    pub fn get_motors(&self) -> &[MotorValue; 4] {
        &self.motor_values
    }

    pub fn set_motor(&mut self, motor: Motor, value: MotorValue) {
        self.motor_values[motor as usize] = value;
    }

    pub fn update_motor(&mut self, motor: Motor, delta: MotorValueDelta) {
        let current = self.motor_values[motor as usize] as i32;
        self.motor_values[motor as usize] = (current + delta).max(0) as MotorValue;
    }
}

impl Default for FlightState {
    fn default() -> Self {
        Self {
            mode: Mode::Safe,
            motor_values: [0; 4],
        }
    }
}
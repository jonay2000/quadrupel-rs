use crate::library::cs_cell::CSCell;

const MOTOR_MIN: u16 = 0;
const MOTOR_MAX: u16 = 1000;

pub static MOTORS: CSCell<Motors> = CSCell::new(Motors::new());

#[derive(Copy, Clone)]
pub struct Motors {
    motor_values: [u16; 4]
}

impl Motors {
    pub const fn new() -> Motors {
        Motors{ motor_values: [0; 4] }
    }

    pub fn get_values(&self) -> [u16; 4] {
        self.motor_values
    }

    pub fn set_motor0(&mut self, val: u16) {
        self.motor_values[0] = val.clamp(MOTOR_MIN, MOTOR_MAX);
    }
    pub fn set_motor1(&mut self, val: u16) {
        self.motor_values[1] = val.clamp(MOTOR_MIN, MOTOR_MAX);
    }
    pub fn set_motor2(&mut self, val: u16) {
        self.motor_values[2] = val.clamp(MOTOR_MIN, MOTOR_MAX);
    }
    pub fn set_motor3(&mut self, val: u16) {
        self.motor_values[3] = val.clamp(MOTOR_MIN, MOTOR_MAX);
    }
}
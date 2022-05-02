use crate::library::yaw_pitch_roll::FI32;

pub struct PID {
    kp : FI32,
    ki : FI32,
    kd : FI32,
    last_state: FI32,
    buildup: FI32
}

impl PID {
    pub fn new(kp : FI32, ki : FI32, kd : FI32) -> Self {
        PID {
            kp,
            ki,
            kd,
            last_state: FI32::from_num(0),
            buildup: FI32::from_num(0),
        }
    }

    pub fn step(&mut self, dt : FI32, state: FI32, goal:FI32) -> FI32 {
        let p_term = (goal-state)*self.kp;
        let d_term = (self.last_state - state) * self.kd /dt;
        self.last_state = state;
        self.buildup += (goal-state)*dt;
        self.buildup = self.buildup.clamp(FI32::from_num(-10), FI32::from_num(10));
        let i_term = self.buildup*self.ki;
        p_term+d_term+i_term
    }
}
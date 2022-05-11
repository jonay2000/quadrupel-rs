use crate::library::yaw_pitch_roll::FI32;

pub struct PID {
    pub kp: FI32,
    pub ki: FI32,
    pub kd: FI32,
    pub cap: FI32,
    last_state: FI32,
    buildup: FI32,
    use_mod: bool,
}

impl PID {
    pub fn new(kp: FI32, ki: FI32, kd: FI32, cap: FI32, use_mod: bool) -> Self {
        PID {
            kp,
            ki,
            kd,
            cap,
            last_state: FI32::from_num(0),
            buildup: FI32::from_num(0),
            use_mod,
        }
    }

    pub fn round_dist(&mut self, state: FI32, goal: FI32) -> FI32 {
        if !self.use_mod {
            return goal - state;
        }
        let neutral = (goal - state).abs();
        let left = (goal - state + 2 * FI32::PI).abs();
        let right = (goal - state - 2 * FI32::PI).abs();

        return if neutral < left && neutral < right {
            goal - state
        } else if left < right {
            goal - state + 2 * FI32::PI
        } else {
            goal - state - 2 * FI32::PI
        };
    }

    pub fn step(&mut self, dt: FI32, state: FI32, goal: FI32) -> FI32 {
        let err = self.round_dist(state, goal);
        let rot_spd = self.round_dist(state, self.last_state);

        let p_term = err * self.kp;
        let d_term = rot_spd * self.kd / dt;
        self.last_state = state;
        self.buildup += err * dt;
        self.buildup = self.buildup.clamp(FI32::from_num(-10), FI32::from_num(10));
        let i_term = self.buildup * self.ki;
        p_term + d_term + i_term
    }
}

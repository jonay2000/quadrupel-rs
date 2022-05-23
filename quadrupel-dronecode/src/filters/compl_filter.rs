use crate::library::fixed_point::FI32;

pub struct ComplFilter {
    b: FI32,
    phi: FI32,
    c1: FI32,
    c2: FI32,
    use_mod: bool,
}

impl ComplFilter {
    pub fn new(c1: FI32, c2: FI32, use_mod: bool) -> Self {
        ComplFilter {
            b: FI32::from_num(0),
            phi: FI32::from_num(0),
            c1,
            c2,
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

    pub fn filter(&mut self, sp: FI32, sphi: FI32, dt: FI32) -> (FI32, FI32) {
        let p = sp - self.b;
        self.phi = self.phi + p * dt;
        let e = self.round_dist(sphi, self.phi);
        self.phi = self.phi - e / self.c1;
        self.phi = (self.phi + FI32::PI) % (2 * FI32::PI) - FI32::PI;
        self.b = self.b + (e / dt) / self.c2;
        (p, self.phi)
    }
}
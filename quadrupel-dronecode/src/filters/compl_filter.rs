use crate::FI64;
use crate::library::fixed_point::FI32;

pub struct ComplFilter {
    b: FI64,
    phi: FI64,
    c1: FI64,
    c2: FI64,
    use_mod: bool,
}

impl ComplFilter {
    pub fn new(c1: FI64, c2: FI64, use_mod: bool) -> Self {
        ComplFilter {
            b: FI64::from_num(0),
            phi: FI64::from_num(0),
            c1,
            c2,
            use_mod,
        }
    }

    pub fn round_dist(&mut self, state: FI64, goal: FI64) -> FI64 {
        if !self.use_mod {
            return goal - state;
        }
        let neutral = (goal - state).abs();
        let left = (goal - state + 2 * FI64::PI).abs();
        let right = (goal - state - 2 * FI64::PI).abs();

        return if neutral < left && neutral < right {
            goal - state
        } else if left < right {
            goal - state + 2 * FI64::PI
        } else {
            goal - state - 2 * FI64::PI
        };
    }

    pub fn filter(&mut self, sp: FI32, sphi: FI32, dt: u32) -> (FI32, FI32) {
        let dt = FI64::from_num(dt) / FI64::from_num(1000000);
        let sp = FI64::from_num(sp);
        let sphi = FI64::from_num(sphi);

        let p = sp - self.b;
        self.phi = self.phi + p * dt;
        let e = self.round_dist(sphi, self.phi);
        self.phi = self.phi - e / self.c1;
        self.phi = (self.phi + FI64::PI) % (2 * FI64::PI) - FI64::PI;
        self.b = self.b + (e / dt) / self.c2;
        (FI32::from_num(p), FI32::from_num(self.phi))
    }
}
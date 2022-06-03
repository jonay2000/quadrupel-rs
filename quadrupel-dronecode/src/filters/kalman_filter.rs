use crate::library::fixed_point::FI32;

pub struct KalFilter {
    r_measure : FI32,
    q_bias: FI32,
    q_angle : FI32,

    angle : FI32,
    bias : FI32,

    p : [[FI32; 2]; 2],
}

impl KalFilter {
    pub fn new(q_angle:FI32,q_bias:FI32,r_measure:FI32) -> Self {
        KalFilter {
            r_measure,
            q_bias,
            q_angle,
            angle: FI32::ZERO,
            bias: FI32::ZERO,
            p: [[FI32::ZERO;2];2],
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
        let rate = sp - self.bias;
        self.angle += dt*rate;

        self.p[0][0] += dt * (dt*self.p[1][1]-self.p[0][1]- self.p[1][0] + self.q_angle);
        self.p[0][1] -= dt*self.p[1][1];
        self.p[1][0] -= dt*self.p[1][1];
        self.p[1][1] += self.q_bias*dt;

        let s = self.p[0][0] +self.r_measure;
        let k = (self.p[0][0]/s,self.p[1][0]/s);

        let y = sphi - self.angle;

        self.angle += k[0]*y;
        self.bias += k[1]*y;

        let p00 = self.p[0][0];
        let p01 = self.p[0][1];

        self.p[0][0] -= k[0] * p00;
        self.p[0][1] -= k[0] * p01;
        self.p[1][0] -= k[1] * p00;
        self.p[1][1] -= k[1] * p01;

        return rate, self.angle
    }
}
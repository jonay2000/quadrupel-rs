use crate::library::fixed_point::FI32;
use crate::library::yaw_pitch_roll::YawPitchRoll;

pub struct Calibrate {
    yaw: FI32,
    pitch: FI32,
    roll: FI32,
    // yaw_rate: FI32
}

impl Calibrate {
    pub fn new() -> Self {
        Calibrate {
            yaw: FI32::from_num(0),
            pitch: FI32::from_num(0),
            roll: FI32::from_num(0),
            // yaw_rate: FI32::from_num(0),
        }
    }

    pub fn round_dist(&mut self, state: FI32, goal: FI32) -> FI32 {
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

    pub fn calibrate(&mut self, yaw: FI32, pitch: FI32, roll: FI32) {
        let yaw_err = self.round_dist(self.yaw,yaw);

        self.yaw = self.yaw + yaw_err * FI32::from_num(0.01);
        if self.yaw < -FI32::PI {
            self.yaw += FI32::PI;
        }
        if self.yaw > FI32::PI {
            self.yaw -= FI32::PI;
        }
        self.pitch = self.pitch * FI32::from_num(0.99) + pitch * FI32::from_num(0.01);
        self.roll = self.roll * FI32::from_num(0.99) + roll * FI32::from_num(0.01);
        // self.yaw_rate = self.yaw_rate * FI32::from_num(0.99) + yaw_rate * FI32::from_num(0.01);
    }

    pub fn fix_ypr(&mut self, ypr: YawPitchRoll) -> YawPitchRoll {
        YawPitchRoll {
            yaw: self.fix_yaw(ypr.yaw),
            pitch: self.fix_pitch(ypr.pitch),
            roll: self.fix_roll(ypr.roll),
        }
    }

    pub fn fix_yaw(&mut self, yaw: FI32) -> FI32 {
        yaw - self.yaw
    }

    pub fn fix_pitch(&mut self, pitch: FI32) -> FI32 {
        pitch - self.pitch
    }

    pub fn fix_roll(&mut self, roll: FI32) -> FI32 {
        roll - self.roll
    }

    // pub fn fix_yaw_rate(&mut self, yaw_rate: FI32) -> FI32 {
    //     yaw_rate - self.yaw_rate
    // }
}

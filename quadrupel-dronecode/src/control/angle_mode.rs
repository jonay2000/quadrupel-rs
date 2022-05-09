use crate::library::pid::PID;
use crate::library::yaw_pitch_roll::FI32;
use core::cmp::max;

const ENABLE_YAW: bool = true;
const ENABLE_PITCH: bool = false;
const ENABLE_ROLL: bool = false;

pub struct AngleMode {
    yaw_pid: PID,
    pitch_pid: PID,
    roll_pid: PID,
}

impl AngleMode {
    pub fn step(
        &mut self,
        dt: FI32,
        thrust: FI32,
        yaw_state: FI32,
        pitch_state: FI32,
        roll_state: FI32,
        yaw_goal: FI32,
        pitch_goal: FI32,
        roll_goal: FI32,
    ) -> [FI32; 4] {
        let yaw_off = if ENABLE_YAW && thrust > 0 {
            self.yaw_pid.step(dt, yaw_state, yaw_goal)
        } else {
            FI32::from_num(0)
        };

        let pitch_off = if ENABLE_PITCH && thrust > 0 {
            self.pitch_pid.step(dt, pitch_state, pitch_goal)
        } else {
            FI32::from_num(0)
        };

        let roll_off = if ENABLE_ROLL && thrust > 0 {
            self.roll_pid.step(dt, roll_state, roll_goal)
        } else {
            FI32::from_num(0)
        };

        return [
            (thrust + yaw_off + pitch_off).max(FI32::from_num(0)),
            (thrust - yaw_off + roll_off).max(FI32::from_num(0)),
            (thrust + yaw_off - pitch_off).max(FI32::from_num(0)),
            (thrust - yaw_off - roll_off).max(FI32::from_num(0)),
        ];
    }
}

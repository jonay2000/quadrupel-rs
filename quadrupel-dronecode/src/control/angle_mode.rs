use crate::library::pid::PID;
use crate::library::fixed_point::FI32;

const ENABLE_YAW: bool = true;
const ENABLE_PITCH: bool = true;
const ENABLE_ROLL: bool = true;

pub struct AngleMode {
    pub yaw_pid: PID,
    pub pitch_pid: PID,
    pub roll_pid: PID,
    pub height_pid: PID,
}

impl AngleMode {
    pub fn step(
        &mut self,
        dt: FI32,
        lift: FI32,
        yaw_state: FI32,
        pitch_state: FI32,
        roll_state: FI32,
        height_state: FI32,
        yaw_goal: FI32,
        pitch_goal: FI32,
        roll_goal: FI32,
        height_goal: FI32,
        enable_height: bool,
        yaw_control: bool,
    ) -> ([FI32; 4], [FI32; 5]) {
        let yaw_offset = if ENABLE_YAW && lift > 0 {
            self.yaw_pid.step(dt, yaw_state, yaw_goal)
        } else {
            FI32::from_num(0)
        };

        let pitch_offset = if ENABLE_PITCH && !yaw_control && lift > 0 {
            self.pitch_pid.step(dt, pitch_state, pitch_goal)
        } else {
            FI32::from_num(0)
        };

        let roll_offset = if ENABLE_ROLL && !yaw_control && lift > 0 {
            self.roll_pid.step(dt, roll_state, roll_goal)
        } else {
            FI32::from_num(0)
        };

        let height_offset = if enable_height && lift > 0 {
            self.height_pid.step(dt, height_state, height_goal)
        } else {
            FI32::from_num(0)
        };

        return ([
            (lift - height_offset - yaw_offset + pitch_offset).max(FI32::from_num(0)),
            (lift - height_offset + yaw_offset - roll_offset).max(FI32::from_num(0)),
            (lift - height_offset - yaw_offset - pitch_offset).max(FI32::from_num(0)),
            (lift - height_offset + yaw_offset + roll_offset).max(FI32::from_num(0)),
        ], [lift, height_offset, yaw_offset, pitch_offset, roll_offset]);
    }
}

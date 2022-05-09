use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use crate::library::yaw_pitch_roll::FI32;

pub struct FullControl;

impl ModeTrait for FullControl {
    fn iteration(state: &mut FlightState, dt: u32) {
        let dt = FI32::from_bits(dt as i32);

        let lift_goal_raw = state.target_attitude.lift;
        if lift_goal_raw <= 0.1 {
            state.motor_values = [0, 0, 0, 0];
            return;
        }
        let lift = lift_goal_raw * FI32::from_num(25);

        fn raw_to_10_deg(inp: FI32) -> FI32 {
            // * 2pi * (10/360) * (1/8) in optimal order
            inp * 2 * FI32::PI * FI32::from_num(10) / FI32::from_num(8) / FI32::from_num(360)
        }

        let yaw_goal = state.current_attitude.yaw + raw_to_10_deg(state.target_attitude.yaw);
        let pitch_goal = raw_to_10_deg(state.target_attitude.pitch);
        let roll_goal = raw_to_10_deg(state.target_attitude.roll);

        state.angle_mode.step(
            dt,
            lift,
            state.current_attitude.yaw,
            state.current_attitude.pitch,
            state.current_attitude.roll,
            yaw_goal,
            pitch_goal,
            roll_goal);

        //
        // let motors = [
        //     lift_goal_raw - yaw + pitch,
        //     lift_goal_raw + yaw + roll,
        //     lift_goal_raw - yaw - pitch,
        //     lift_goal_raw + yaw - roll,
        // ];
        //
        // state.motor_values = motors.map(|fi32| {
        //     fi32.clamp(FI32::from_num(0), FI32::from_num(500))
        //         .round()
        //         .to_num()
        // });
    }
}

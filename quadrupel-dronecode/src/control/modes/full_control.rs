use quadrupel_shared::state::Mode;
use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use crate::library::fixed_point::FI32;

pub struct FullControl;

impl ModeTrait for FullControl {
    fn iteration(state: &mut FlightState, dt: u32) {
        let dt = FI32::from_bits(dt as i32);

        let lift_goal_raw = state.target_attitude.lift;
        if lift_goal_raw <= 0.1 {
            state.motor_values = [None, None, None, None];
            return;
        }
        let lift_goal = lift_goal_raw * FI32::from_num(25);

        fn raw_to_10_deg(inp: FI32) -> FI32 {
            // * 2pi * (10/360) * (1/8) in optimal order
            inp * 2 * FI32::PI * FI32::from_num(10) / FI32::from_num(8) / FI32::from_num(360)
        }

        let yaw_goal = state.current_attitude.yaw + raw_to_10_deg(state.target_attitude.yaw);
        let pitch_goal = raw_to_10_deg(state.target_attitude.pitch);
        let roll_goal = -raw_to_10_deg(state.target_attitude.roll);

        // Height mode logic
        if state.height_mode_enable && state.height_lock.is_none() {
            state.height_lock = Some((state.target_attitude.lift, state.current_attitude.height));
            state.angle_mode.height_pid.buildup = FI32::ZERO;
        }
        if !state.height_mode_enable {
            state.height_lock = None;
        }
        let (prev_lift, height_goal) = state.height_lock.unwrap_or((state.target_attitude.lift, state.current_attitude.height));
        if state.height_mode_enable && (prev_lift.abs_diff(state.target_attitude.lift)) > FI32::from_num(0.1)  {
            state.height_mode_enable = false;
            state.height_lock = None;
        }

        //Autoland logic
        if state.autoland_enable && state.height_mode_enable {
            let (_, height_locked) = &mut state.height_lock.unwrap();
            *height_locked += dt >> 6;

            if state.angle_mode.height_pid.buildup == -state.angle_mode.height_pid.cap {
                log::info!("Landed.");
                state.autoland_enable = false;
                state.height_mode_enable = false;
                state.mode = Mode::Safe;
            }
        }

        let (motors, st) = state.angle_mode.step(
            dt,
            lift_goal,
            state.current_attitude.yaw,
            state.current_attitude.pitch,
            state.current_attitude.roll,
            state.current_attitude.height,
            yaw_goal,
            pitch_goal,
            roll_goal,
            height_goal,
            state.height_mode_enable,
            state.mode == Mode::YawControl,
        );

        state.motor_values = motors.map(|fi32| {
            Some(
                fi32.clamp(FI32::from_num(0), FI32::from_num(1000))
                    .round()
                    .to_num(),
            )
        });
        state.pid_contributions = st;
    }
}

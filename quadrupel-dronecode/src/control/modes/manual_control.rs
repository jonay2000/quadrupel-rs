use crate::control::flight_state::FlightState;
use crate::control::modes::ModeTrait;
use crate::library::yaw_pitch_roll::FI32;

pub struct ManualControl;

impl ModeTrait for ManualControl {
    fn iteration(state: &mut FlightState, _dt: u32) {
        let lift = FI32::from_num(25) * state.target_attitude.lift; // [0,16]
        if lift <= 0.1 {
            state.motor_values = [None, None, None, None];
            return;
        }

        let yaw = FI32::from_num(10) * state.target_attitude.yaw; // [-8,8]
        let pitch = FI32::from_num(10) * state.target_attitude.pitch; // [-8,8]
        let roll = FI32::from_num(10) * state.target_attitude.roll; // [-8,8]

        let motors = [
            lift - yaw + pitch,
            lift + yaw + roll,
            lift - yaw - pitch,
            lift + yaw - roll,
        ];

        state.motor_values = motors.map(|fi32| {
            Some(fi32.clamp(FI32::from_num(0), FI32::from_num(500))
                .round()
                .to_num())
        });
    }
}

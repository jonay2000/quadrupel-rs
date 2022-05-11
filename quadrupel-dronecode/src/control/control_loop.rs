use crate::control::flight_state::FlightState;
use crate::control::modes::full_control::FullControl;
use crate::control::modes::individual_motor_control::IndividualMotorControlMode;
use crate::control::modes::manual_control::ManualControl;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;
use crate::control::modes::ModeTrait;
use crate::control::process_message::process_message;
use crate::motors::GlobalTime;
use crate::*;
use embedded_hal::digital::v2::{OutputPin, PinState};
use quadrupel_shared::message::MessageToComputer;
use quadrupel_shared::state::Mode;

const HEARTBEAT_FREQ: u32 = 100000;
const HEARTBEAT_TIMEOUT_MULTIPLE: u32 = 2;

enum BlueLedStatus {
    OFF { at: u32 },
    ON { at: u32 },
}

pub fn start_loop() -> ! {
    let mut uart_protocol = UartProtocol::new();
    let mut state = FlightState::default();

    let start_time = GlobalTime().get_time_us();
    let mut last_time = GlobalTime().get_time_us();

    let mut blue_led_status = BlueLedStatus::OFF { at: start_time };
    let mut adc_warning = true;

    let mut time_since_last_print = 0;

    loop {
        let dt = GlobalTime().get_time_us() - last_time;
        last_time = GlobalTime().get_time_us();
        state.count += 1;

        //Process any incoming messages
        while let Some(msg) = uart_protocol.update() {
            process_message(msg, &mut state)
        }

        //Check heartbeat
        if state.mode != Mode::Safe
            && (GlobalTime().get_time_us() - state.last_heartbeat)
                > (HEARTBEAT_FREQ * HEARTBEAT_TIMEOUT_MULTIPLE)
        {
            log::error!("Panic: Heartbeat timeout");
            state.mode = Mode::Panic;
        }

        //Read hardware
        let ypr = MPU.as_mut_ref().block_read_mpu(I2C.as_mut_ref());
        let (_accel, _gyro) = MPU.as_mut_ref().read_accel_gyro(I2C.as_mut_ref());
        let (pres, _temp) = BARO.as_mut_ref().read_both(I2C.as_mut_ref());
        let motors = MOTORS.update_main(|motors| motors.get_motors());
        let adc = ADC.update_main(|adc| adc.read());

        //Check adc
        if adc > 650 && adc < 1050 {
            log::error!("Panic: Battery low {adc} 10^-2 V");
            state.mode = Mode::Panic;
        } else if adc != 0 && adc_warning && adc <= 650 {
            log::warn!(
                "Warning: Battery is < 6V ({adc}), continuing assuming that this is not a drone."
            );
            adc_warning = false;
        }

        //Update state
        state.current_attitude.yaw = ypr.yaw;
        state.current_attitude.pitch = ypr.pitch;
        state.current_attitude.roll = ypr.roll;

        // Do action corresponding to current mode
        match state.mode {
            Mode::Safe => SafeMode::iteration(&mut state, dt),
            Mode::Calibration => {}
            Mode::Panic => PanicMode::iteration(&mut state, dt),
            Mode::FullControl => FullControl::iteration(&mut state, dt),
            Mode::IndividualMotorControl => IndividualMotorControlMode::iteration(&mut state, dt),
            Mode::Manual => ManualControl::iteration(&mut state, dt),
        }

        //Update LEDS
        let leds = LEDS.as_mut_ref();
        blue_led_status = match blue_led_status {
            BlueLedStatus::OFF { at } if GlobalTime().get_time_us() - at > 1000000 => {
                leds.led_blue.set_low().unwrap();
                BlueLedStatus::ON { at: at + 1000000 }
            }
            BlueLedStatus::ON { at } if GlobalTime().get_time_us() - at > 1000000 => {
                leds.led_blue.set_high().unwrap();
                BlueLedStatus::OFF { at: at + 1000000 }
            }
            s => s,
        };
        //green yellow red
        let (g, y, r) = match state.mode {
            Mode::Safe => (true, false, false),
            Mode::Calibration => (false, false, false),
            Mode::Panic => (true, true, true),
            Mode::FullControl => (false, true, false),
            Mode::IndividualMotorControl => (false, true, false),
            Mode::Manual => (true, false, true),
        };
        leds.led_green.set_state(PinState::from(!g)).unwrap();
        leds.led_yellow.set_state(PinState::from(!y)).unwrap();
        leds.led_red.set_state(PinState::from(!r)).unwrap();

        // update peripherals according to current state
        MOTORS.update_main(|i| {
            let new_motor_values = state.motor_values.map(|m| if m==0 {0} else {(((m as u32)+70)*900).sqrt() as u16});
            i.set_motors(state.motor_values)
        });

        //Send state information
        time_since_last_print += dt;
        if time_since_last_print > 500000 {
            time_since_last_print = 0;
            // log::info!(
            //     "{:?} {} {} | {:?} | {} {} {} | {} {} {} {} | {} | {} | {}",
            //     state.mode,
            //     GlobalTime().get_time_us(),
            //     dt,
            //     motors,
            //     ypr.roll,
            //     ypr.pitch,
            //     ypr.yaw,
            //     state.target_attitude.roll,
            //     state.target_attitude.pitch,
            //     state.target_attitude.yaw,
            //     state.target_attitude.lift,
            //     adc,
            //     temp,
            //     pres
            // );

            let msg = MessageToComputer::StateInformation {
                state: state.mode,
                height: pres,
                battery: adc,
                dt,
                motors,
                sensor_ypr: [
                    ypr.yaw.to_bits(),
                    ypr.pitch.to_bits(),
                    ypr.roll.to_bits()
                ],
                input_typr: [
                    state.target_attitude.lift.to_bits(),
                    state.target_attitude.yaw.to_bits(),
                    state.target_attitude.pitch.to_bits(),
                    state.target_attitude.roll.to_bits(),
                ]
            };


            // let mut encoding_space: [u8; 256] = [0u8; 256];
            // let count = bincode::encode_into_slice(&msg, &mut encoding_space, standard()).unwrap();
            // log::info!("{} {:?}", count, &encoding_space[..count]);

            UART.as_mut_ref().send_message(msg);
        }
    }
}

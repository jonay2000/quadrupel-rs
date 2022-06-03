use crate::control::flash_protocol::FlashProtocol;
use crate::control::flight_state::FlightState;
use crate::control::modes::full_control::FullControl;
use crate::control::modes::individual_motor_control::IndividualMotorControlMode;
use crate::control::modes::manual_control::ManualControl;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;
use crate::control::modes::ModeTrait;
use crate::control::process_message::process_message;
use crate::filters::butterworth_2nd::ButterworthLowPass2nd;
use crate::*;
use embedded_hal::digital::v2::{OutputPin, PinState};
use quadrupel_shared::message::{MessageToComputer};
use quadrupel_shared::state::Mode;
use quadrupel_shared::state::Mode::{Calibration, Panic, Safe};
use crate::control::modes::calibrate_mode::CalibrateMode;
use crate::library::fixed_point::{FI32, rough_isqrt};

const HEARTBEAT_FREQ: u32 = 100000;
const HEARTBEAT_TIMEOUT_MULTIPLE: u32 = 2;

enum BlueLedStatus {
    OFF { at: u32 },
    ON { at: u32 },
}

pub fn start_loop() -> ! {
    let mut uart_protocol = UartProtocol::new();
    let mut flash_protocol = FlashProtocol::new();

    let mut state = FlightState::default();

    let start_time = TIME.as_mut_ref().get_time_us();
    let mut last_time = start_time;

    let mut blue_led_status = BlueLedStatus::OFF { at: start_time };
    let mut adc_warning = true;

    let mut time_since_last_print = 0;

    let mut adc_filtered = 1000;
    let mut dt_filtered = 1000;

    let a_yi = FI32::from_num(54818.728);
    let a_yi_1 = FI32::from_num(108973.229)/a_yi;
    let a_yi_2 = FI32::from_num(-54158.500)/a_yi;
    let a_xi = FI32::from_num(1)/a_yi;
    let a_xi_1 = FI32::from_num(2)/a_yi;
    let a_xi_2 = FI32::from_num(1)/a_yi;
    let mut height_filter_raw = ButterworthLowPass2nd::new(a_yi, a_yi_1, a_yi_2, a_xi, a_xi_1, a_xi_2);

    let a_yi = FI32::from_num(1291.029);
    let a_yi_1 = FI32::from_num(2478.450)/a_yi;
    let a_yi_2 = FI32::from_num(-1191.421)/a_yi;
    let a_xi = FI32::from_num(1)/a_yi;
    let a_xi_1 = FI32::from_num(2)/a_yi;
    let a_xi_2 = FI32::from_num(1)/a_yi;
    let mut height_filter_mpu = ButterworthLowPass2nd::new(a_yi, a_yi_1, a_yi_2, a_xi, a_xi_1, a_xi_2);

    loop {
        let cur_time = TIME.as_mut_ref().get_time_us();
        let dt = cur_time - last_time;
        if dt > dt_filtered {
            dt_filtered += 1;
        } else {
            dt_filtered -= 1;
        }
        last_time = cur_time;
        state.count += 1;


        //Process any incoming messages
        while let Some(msg) = uart_protocol.update() {
            process_message(msg, &mut state)
        }

        //Check heartbeat
        if state.mode != Mode::Safe
            && (TIME.as_mut_ref().get_time_us() - state.last_heartbeat)
                > (HEARTBEAT_FREQ * HEARTBEAT_TIMEOUT_MULTIPLE)
        {
            log::error!("Panic: Heartbeat timeout");
            state.mode = Mode::Panic;
        }

        //Switch between raw/non-raw mode
        if !state.raw_mode_enable && !MPU.as_mut_ref().is_mpu_enabled() {
            MPU.as_mut_ref().enable_mpu(I2C.as_mut_ref());
            log::info!("MPU Enabled.");
        }
        if state.raw_mode_enable && MPU.as_mut_ref().is_mpu_enabled() {
            MPU.as_mut_ref().disable_mpu(I2C.as_mut_ref());
            log::info!("MPU Disabled.");
        }

        //YPR
        let (accel, gyro) = MPU.as_mut_ref().read_accel_gyro(I2C.as_mut_ref());
        let ypr = if state.raw_mode_enable {
            state.raw_mode.update(accel, gyro, dt, state.flash_record, &mut flash_protocol)
        } else {
            MPU.as_mut_ref().block_read_mpu(I2C.as_mut_ref())
        };
        let (pres, _temp) = BARO.as_mut_ref().read_both(I2C.as_mut_ref());
        let mut pres = pres as i32;
        pres -= 100000;
        let mut pres = FI32::from_bits(pres << 16);

        if state.raw_mode_enable {
            pres = height_filter_raw.filter(pres);
        } else {
            pres = height_filter_mpu.filter(pres);
        }

        let ypr = state.calibrate.fix_ypr(ypr);
        state.current_attitude.yaw = ypr.yaw;
        state.current_attitude.pitch = ypr.pitch;
        state.current_attitude.roll = ypr.roll;
        state.current_attitude.height = pres;

        //Read other hardware
        let motors = MOTORS.update_main(|motors| motors.get_motors());


        //Check adc
        let adc = ADC.update_main(|adc| adc.read());
        adc_filtered = if adc > adc_filtered {
            adc_filtered.saturating_add(10)
        } else {
            adc_filtered.saturating_sub(10)
        };

        if adc_filtered > 650 && adc_filtered < 1000
        {
            log::error!("Panic: Battery low {adc_filtered} 10^-2 V");
            state.mode = Mode::Panic;
        } else if adc_filtered != 0 && adc_warning && adc_filtered <= 650 {
            log::warn!(
                "Warning: Battery is < 6V ({adc}), continuing assuming that this is not a drone."
            );
            adc_warning = false;
        }

        //Check max angle protection
        if state.mode != Panic && state.mode != Safe && state.mode != Calibration && (ypr.pitch.abs() > FI32::from_num(0.8) || ypr.roll.abs() > FI32::from_num(0.8)) {
            log::error!("Panic: Max angle protection activated.");
            state.mode = Mode::Panic;
        }

        // Do action corresponding to current mode
        match state.mode {
            Mode::Safe => SafeMode::iteration(&mut state, dt),
            Mode::Calibration => CalibrateMode::iteration(&mut state, dt),
            Mode::Panic => PanicMode::iteration(&mut state, dt),
            Mode::FullControl => FullControl::iteration(&mut state, dt),
            Mode::YawControl => FullControl::iteration(&mut state, dt),
            Mode::IndividualMotorControl => IndividualMotorControlMode::iteration(&mut state, dt),
            Mode::Manual => ManualControl::iteration(&mut state, dt),
        }

        //Update LEDS
        let leds = LEDS.as_mut_ref();
        blue_led_status = match blue_led_status {
            BlueLedStatus::OFF { at } if cur_time - at > 1000000 => {
                leds.led_blue.set_low().unwrap();
                BlueLedStatus::ON { at: at + 1000000 }
            }
            BlueLedStatus::ON { at } if cur_time - at > 1000000 => {
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
            Mode::YawControl => (false, true, false),
            Mode::FullControl => (false, true, false),
            Mode::IndividualMotorControl => (false, true, false),
            Mode::Manual => (true, false, true),
        };
        leds.led_green.set_state(PinState::from(!g)).unwrap();
        leds.led_yellow.set_state(PinState::from(!y)).unwrap();
        leds.led_red.set_state(PinState::from(!r)).unwrap();

        // update peripherals according to current state
        MOTORS.update_main(|i| {
            let new_motor_values = state.motor_values.map(|m| match m {
                None => 0,
                Some(m) => rough_isqrt(((m as u32) + 70) * 900) as u16,
            });
            i.set_motors(new_motor_values)
        });

        //Handle flash
        if state.flash_record && flash_protocol.is_done() {
            state.flash_record = false;
            state.flash_send = true;
            // TODO autosend? state.flash_send = true;
        }
        if state.flash_record {
            // flash_protocol.write(FlashPacket::Data(ypr.pitch.to_bits() as i16))
        }
        if state.flash_send {
            while UART.as_mut_ref().buffer_left_rx() >= 128
                && UART.as_mut_ref().buffer_left_tx() >= 128
            {
                if let Some(packet) = flash_protocol.read() {
                    UART.as_mut_ref()
                        .send_message(MessageToComputer::FlashPacket(packet));
                } else {
                    log::info!("Finished sending flash, resetting flash.");
                    state.flash_send = false;
                    flash_protocol.reset();
                    break;
                }
            }
        }

        //Send state information
        time_since_last_print += dt;
        if time_since_last_print > 100000 {
            time_since_last_print = 0;

            let msg = MessageToComputer::StateInformation {
                state: state.mode,
                height: pres.to_bits() >> 16,
                tgt_height: if let Some((_, i)) = state.height_lock {
                    i.to_bits()
                } else {
                    0
                },
                battery: adc_filtered,
                dt: dt_filtered,
                motors,
                sensor_ypr: [ypr.yaw.to_bits(), ypr.pitch.to_bits(), ypr.roll.to_bits()],
                input_typr: [
                    state.target_attitude.lift.to_bits(),
                    state.target_attitude.yaw.to_bits(),
                    state.target_attitude.pitch.to_bits(),
                    state.target_attitude.roll.to_bits(),
                ],
                i_buildup: [
                    state.angle_mode.yaw_pid.buildup.to_bits(),
                    state.angle_mode.pitch_pid.buildup.to_bits(),
                    state.angle_mode.roll_pid.buildup.to_bits(),
                    state.angle_mode.height_pid.buildup.to_bits(),
                ],
                accel: [accel.x, accel.y, accel.z],
                gyro: [gyro.x, gyro.y, gyro.z],
                height_mode: state.height_mode_enable,
                raw_mode: state.raw_mode_enable,
                autoland: state.autoland_enable,
                pid_contributions: state.pid_contributions.map(|f| f.to_bits()),
            };

            UART.as_mut_ref().send_message(msg);
        }

        //Wait to cap at 1000hz
        while TIME.as_mut_ref().get_time_us() - cur_time <= 995 {
        }
    }
}

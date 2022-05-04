use embedded_hal::digital::v2::{OutputPin, PinState};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use quadrupel_shared::message::MessageToComputer;
use quadrupel_shared::state::Mode;
use crate::*;
use crate::control::flight_state::FlightState;
use crate::control::modes::individual_motor_control::IndividualMotorControlMode;
use crate::control::modes::ModeTrait;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;
use crate::control::process_message::process_message;
use crate::motors::GlobalTime;

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
    let mut count = 0;

    let mut blue_led_status = BlueLedStatus::OFF { at: start_time };
    let mut adc_warning = true;

    loop {
        count += 1;

        //Process any incoming messages
        while let Some(msg) = uart_protocol.update() {
            process_message(msg, &mut state)
        }

        //Check heartbeat
        if state.mode != Mode::Safe && (GlobalTime().get_time_us() - state.last_heartbeat) > (HEARTBEAT_FREQ * HEARTBEAT_TIMEOUT_MULTIPLE) {
            log::error!("Panic: Heartbeat timeout");
            state.mode = Mode::Panic;
        }

        //Check adc
        let adc = ADC.update_main(|adc| adc.read());
        if adc > 600 && adc < 1050 {
            log::error!("Panic: Battery low {adc} 10^-2 V");
            state.mode = Mode::Panic;
        } else if adc_warning && adc <= 600 {
            log::warn!("Warning: Battery is < 6V, continuing assuming that this is not a drone.");
            adc_warning = false;
        }


        // Do action corresponding to current mode
        match state.mode {
            Mode::Safe => SafeMode::iteration(&mut state),
            Mode::Calibration => {}
            Mode::Panic => PanicMode::iteration(&mut state),
            Mode::FullControl => {}
            Mode::IndividualMotorControl => IndividualMotorControlMode::iteration(&mut state),
        }

        // Print all info
        let dt = (GlobalTime().get_time_us() - start_time) / count;
        let ypr = MPU.as_mut_ref().block_read_mpu(I2C.as_mut_ref());
        let (_accel, gyro) = MPU.as_mut_ref().read_accel_gyro(I2C.as_mut_ref());
        let (pres, temp) = BARO.as_mut_ref().read_both(I2C.as_mut_ref());
        let motors = MOTORS.update_main(|motors| motors.get_motors());
        if count % 100 == 0 {
            log::info!("{} {} | {:?} | {} {} {} | {} {} {} | {} | {} | {}",
                GlobalTime().get_time_us(), dt,
                motors,
                ypr.roll, ypr.pitch, ypr.yaw,
                gyro.x(), gyro.y(), gyro.z(),
                adc, temp, pres
            );
        }

        //Update LEDS
        let leds = LEDS.as_mut_ref();
        blue_led_status = match blue_led_status {
            BlueLedStatus::OFF { at } if GlobalTime().get_time_us() - at > 1000000 => {
                leds.led_blue.set_low().unwrap();
                BlueLedStatus::ON { at: at + 1000000 }
            },
            BlueLedStatus::ON { at } if GlobalTime().get_time_us() - at > 1000000 => {
                leds.led_blue.set_high().unwrap();
                BlueLedStatus::OFF { at: at + 1000000 }
            },
            s => s
        };
        //green yellow red
        let (g,y,r) = match state.mode {
            Mode::Safe => (true,false,false),
            Mode::Calibration => (false,false,false),
            Mode::Panic => (true,true,true),
            Mode::FullControl => (false,true,false),
            Mode::IndividualMotorControl => (false,true,false),
        };
        leds.led_green.set_state(PinState::from(!g)).unwrap();
        leds.led_yellow.set_state(PinState::from(!y)).unwrap();
        leds.led_red.set_state(PinState::from(!r)).unwrap();


        // update peripherals according to current state
        MOTORS.update_main(|i| {
            i.set_motors(state.motor_values)
        });

        //Send state information
        let msg = MessageToComputer::StateInformation {
            state: state.mode,
            height: pres,
            roll: ypr.roll.to_bits(),
            pitch: ypr.pitch.to_bits(),
            yaw: ypr.yaw.to_bits(),
            battery: adc,
            dt
        };
        // UART.as_mut_ref().send_message(msg);
        //TODO send msg
    }
}

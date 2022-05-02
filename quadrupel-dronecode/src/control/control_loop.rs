use embedded_hal::digital::v2::{OutputPin, PinState};
use log::info;
use quadrupel_shared::message::MessageToComputer;
use quadrupel_shared::state::Mode;
use crate::*;
use crate::control::flight_state::FlightState;
use crate::control::modes::individual_motor_control::IndividualMotorControlMode;
use crate::control::modes::ModeTrait;
use crate::control::modes::panic::PanicMode;
use crate::control::modes::safe::SafeMode;
use crate::control::process_message::process_message;

const HEARTBEAT_FREQ: u32 = 100000;
const HEARTBEAT_TIMEOUT_MULTIPLE: u32 = 2;

enum BlueLedStatus {
    OFF { at: u32 },
    ON { at: u32 },
}

pub fn start_loop() -> ! {
    let mut uart_protocol = UartProtocol::new();
    let mut state = FlightState::default();

    let start_time = Motors::get_time_us();
    let mut count = 0;

    let mut blue_led_status = BlueLedStatus::OFF { at: start_time };

    loop {
        count += 1;

        //Process any incoming messages
        while let Some(msg) = uart_protocol.update() {
            process_message(msg, &mut state)
        }

        //Check heartbeat
        if state.mode != Mode::Safe && (Motors::get_time_us() - state.last_heartbeat) > (HEARTBEAT_FREQ * HEARTBEAT_TIMEOUT_MULTIPLE) {
            log::error!("Panic: Heartbeat timeout");
            state.mode = Mode::Panic;
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
        let dt = (Motors::get_time_us() - start_time) / count;
        let ypr = MPU.as_mut_ref().block_read_mpu(I2C.as_mut_ref(), TIMER0.as_mut_ref());
        let (_accel, gyro) = MPU.as_mut_ref().read_accel_gyro(I2C.as_mut_ref());
        let adc = ADC.update_main(|adc| adc.read());
        let (pres, temp) = BARO.as_mut_ref().read_both(I2C.as_mut_ref());
        let motors = MOTORS.update_main(|motors| motors.get_motors());
        if count % 100 == 0 {
            log::info!("{} | {:?} | {} {} {} | {} {} {} | {} | {} | {}",
                dt,
                motors,
                ypr.roll, ypr.pitch, ypr.yaw,
                gyro.x(), gyro.y(), gyro.z(),
                adc, temp, pres
            );
        }

        //Update LEDS
        let leds = LEDS.as_mut_ref();
        blue_led_status = match blue_led_status {
            BlueLedStatus::OFF { at } if Motors::get_time_us() - at > 1000000 => {
                leds.led_blue.set_low().unwrap();
                BlueLedStatus::ON { at: at + 1000000 }
            },
            BlueLedStatus::ON { at } if Motors::get_time_us() - at > 1000000 => {
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
        //TODO send msg
    }
}

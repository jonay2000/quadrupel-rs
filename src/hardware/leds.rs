use nrf51_hal::gpio::{Disconnected, Output, PushPull};
use nrf51_hal::gpio::p0::{P0_22, P0_24, P0_28, P0_30};
use crate::Level;

pub struct QLeds {
    pub led_red: P0_22<Output<PushPull>>,
    pub led_yellow: P0_24<Output<PushPull>>,
    pub led_green: P0_28<Output<PushPull>>,
    pub led_blue: P0_30<Output<PushPull>>,
}

impl QLeds {
    pub fn new(led_red: P0_22<Disconnected>,
                led_yellow: P0_24<Disconnected>,
                led_green: P0_28<Disconnected>,
                led_blue: P0_30<Disconnected>,) -> Self {
        QLeds {
            led_red: led_red.into_push_pull_output(Level::High),
            led_yellow: led_yellow.into_push_pull_output(Level::High),
            led_green: led_green.into_push_pull_output(Level::High),
            led_blue: led_blue.into_push_pull_output(Level::High)
        }
    }
}
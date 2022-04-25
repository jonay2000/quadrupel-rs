use alloc::string::String;
use bincode::config::standard;
use bincode::{Encode, Decode};
use bincode::enc::write::Writer;
use bincode::error::EncodeError;
use crate::control::state::State;
use crate::control::MotorValue;

#[derive(Encode)]
pub enum SendMessage {
    Log(String),
    CurrentState(State),
    Sensors {
        height: u32,
        roll: u32,
        pitch: u32,
        yaw: u32,
    },
    MotorPidParams {
        /* TODO */
    },
    Battery(u16),
}

impl SendMessage {
    pub fn encode(&self, w: impl Writer) -> Result<(), EncodeError> {
        bincode::encode_into_writer(self, w, standard())
    }
}

#[derive(Encode, Decode)]
pub enum Motor {
    M0 = 0,
    M1 = 1 ,
    M2 = 2,
    M3 = 3,
}

#[derive(Decode)]
pub enum ReceiveMessage {
    ChangeState(State),
    MotorValue {
        motor: Motor,
        value: MotorValue,
    },
    MotorValueRel {
        motor: Motor,
        value: MotorValue,
    },
    TargetYaw(u32),
    TargetPitch(u32),
    TargetRoll(u32),
    TargetHeight(u32),
    TunePID {
        /* TODO */
    }
}
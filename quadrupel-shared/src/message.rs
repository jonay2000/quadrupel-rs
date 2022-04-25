use alloc::string::String;
use alloc::vec::Vec;
use bincode::config::standard;
use bincode::{Encode, Decode};
use bincode::enc::write::Writer;
use bincode::error::{DecodeError, EncodeError};
use crate::MotorValue;
use crate::state::State;

#[cfg(feature = "python")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode)]
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

    #[cfg(feature = "python")]
    pub fn decode(r: &[u8]) -> Result<(Self, usize), DecodeError> {
        bincode::decode_from_slice(r, standard())
    }
}


#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Encode, Decode)]
pub enum Motor {
    M0 = 0,
    M1 = 1 ,
    M2 = 2,
    M3 = 3,
}


#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode)]
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

impl ReceiveMessage {
    #[cfg(feature = "python")]
    pub fn encode_vec(&self) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(self, standard())
    }

    pub fn decode(r: &[u8]) -> Result<(Self, usize), DecodeError> {
        bincode::decode_from_slice(r, standard())
    }
}

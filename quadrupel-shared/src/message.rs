use crate::state::Mode;
use crate::{MotorValue, MotorValueDelta};
use alloc::string::String;
use bincode::config::standard;
use bincode::enc::write::Writer;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

#[cfg(feature = "python")]
use alloc::vec::Vec;

#[cfg(feature = "python")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode)]
pub enum MessageToComputer<'a> {
    Log(&'a [u8]),
    CurrentState(Mode),
    Sensors {
        height: u32,
        roll: u32,
        pitch: u32,
        yaw: u32,
    },
    MotorPidParams {/* TODO */},
    Battery(u16),
}

impl MessageToComputer<'_> {
    pub fn encode(&self, w: &mut impl Writer) -> Result<(), EncodeError> {
        let mut encoding_space: [u8; 256] = [0u8; 256];
        let bytes = bincode::encode_into_slice(self, &mut encoding_space, standard())?;
        assert!(bytes < 256);

        w.write(&[bytes as u8])?;
        w.write(&encoding_space)?;
        Ok(())
    }

    #[cfg(feature = "python")]
    pub fn decode(r: &[u8]) -> Result<(Self, usize), DecodeError> {
        bincode::decode_from_slice(r, standard())
    }
}

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Debug, Copy, Clone)]
pub enum Motor {
    M0 = 0,
    M1 = 1,
    M2 = 2,
    M3 = 3,
}

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, Debug)]
pub enum MessageToDrone {
    ChangeState(Mode),
    MotorValue { motor: Motor, value: MotorValue },
    MotorValueRel { motor: Motor, value: MotorValueDelta },
    TargetAttitude {
        yaw: u32,
        pitch: u32,
        roll: u32,
        lift: u32,
    },
    HeartBeat(u8),
    TunePID {/* TODO */},
}

impl MessageToDrone {
    #[cfg(feature = "python")]
    pub fn encode_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let mut res = bincode::encode_to_vec(self, standard())?;
        assert!(res.len() < 256);
        res.insert(0, res.len() as u8);

        Ok(res)
    }

    pub fn decode(r: &[u8]) -> Result<(Self, usize), DecodeError> {
        bincode::decode_from_slice(r, standard())
    }
}

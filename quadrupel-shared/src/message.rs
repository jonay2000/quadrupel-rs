use crate::state::Mode;
use crate::{MotorValue, MotorValueDelta};
use bincode::config::standard;
use bincode::enc::write::Writer;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

use alloc::vec::Vec;

#[cfg(feature = "python")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, Debug)]
pub enum MessageToComputer {
    Log(Vec<u8>),
    StateInformation {
        state: Mode,
        height: u32,
        battery: u16,
        dt: u32,
        motors: [u16; 4],
        input_typr: [i32; 4],
        sensor_ypr: [i32; 3],
        raw_ypr: [i32; 3],
        i_buildup: [i32; 3],
        accel: [i16; 3],
        gyro: [i16; 3],
    },
    FlashPacket(FlashPacket),
}

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, Debug)]
pub enum FlashPacket {
    Data(i16),
}

impl MessageToComputer {
    pub fn encode(&self, w: &mut impl Writer) -> Result<(), EncodeError> {
        let mut encoding_space: [u8; 256] = [0u8; 256];
        let count = bincode::encode_into_slice(self, &mut encoding_space[2..], standard())?;
        assert!(count < 256);
        encoding_space[1] = count as u8;
        encoding_space[0] = 0xab as u8;
        w.write(&encoding_space[..count+1])?;
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
    MotorValue {
        motor: Motor,
        value: MotorValue,
    },
    MotorValueRel {
        motor: Motor,
        value: MotorValueDelta,
    },
    TargetAttitude {
        yaw: i32,
        pitch: i32,
        roll: i32,
        lift: i32,
    },
    HeartBeat(u8),
    #[allow(non_snake_case)]
    TunePID {
        yaw_P: u32,
        yaw_I: u32,
        yaw_D: u32,
        yaw_CAP: u32,
        pitch_P: u32,
        pitch_I: u32,
        pitch_D: u32,
        pitch_CAP: u32,
        roll_P: u32,
        roll_I: u32,
        roll_D: u32,
        roll_CAP: u32,
    },
    FlashStartRecording,
    FlashStopRecording,
    FlashRead,
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

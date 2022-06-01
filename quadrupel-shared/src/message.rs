use crate::state::Mode;
use crate::{MotorValue, MotorValueDelta};
use bincode::config::standard;
use bincode::enc::write::Writer;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

use alloc::vec::Vec;
use crc::{Crc, CRC_16_IBM_SDLC};

#[cfg(feature = "python")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "python")]
use std::println;
#[cfg(feature = "python")]
extern crate std;

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, Debug)]
pub enum MessageToComputer {
    Log(Vec<u8>),
    StateInformation {
        state: Mode,
        height: i32,
        battery: u16,
        dt: u32,
        motors: [u16; 4],
        input_typr: [i32; 4],
        sensor_ypr: [i32; 3],
        i_buildup: [i32; 4],
        accel: [i16; 3],
        gyro: [i16; 3],
        height_mode: bool,
        raw_mode: bool,
        pid_contributions: [i32; 5],
    },
    FlashPacket(FlashPacket),
}

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, Debug)]
pub enum FlashPacket {
    Data(i32, i32, i32),
}

impl MessageToComputer {
    pub fn encode(&self, w: &mut impl Writer) -> Result<(), EncodeError> {
        let mut encoding_space: [u8; 256] = [0u8; 256];
        let count = bincode::encode_into_slice(self, &mut encoding_space[4..], standard())?;
        assert!(count < 250);
        encoding_space[1] = (count + 2) as u8;
        encoding_space[0] = 0xab as u8;

        let sum = X25.checksum(&encoding_space[4..count + 4]);
        encoding_space[2] = ((sum & 0xff00) >> 8) as u8;
        encoding_space[3] = (sum & 0x00ff) as u8;

        w.write(&encoding_space[..count+4])?;
        Ok(())
    }

    #[cfg(feature = "python")]
    pub fn decode(r: &[u8]) -> Result<(Self, usize), DecodeError> {
        let checksum = ((r[0] as u16) << 8) | (r[1] as u16);

        let expected_checksum = X25.checksum(&r[2..]);

        if checksum != expected_checksum {
            return Err(DecodeError::OtherString(alloc::string::String::from("checksum didn't match, dropping")))
        }

        bincode::decode_from_slice(&r[2..], standard())
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
    SetHeightMode(u8),
    SetRawMode(u8),
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
        height_P: u32,
        height_I: u32,
        height_D: u32,
        height_CAP: u32,
        c1: u32,
        c2: u32,
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
        let checksum = X25.checksum(&res);

        res.insert(0, (res.len() + 2) as u8);

        res.insert(1, ((checksum & 0xff00) >> 8) as u8);
        res.insert(2, (checksum & 0x00ff) as u8);

        Ok(res)
    }

    pub fn decode(r: &[u8]) -> Result<(Self, usize), DecodeError> {
        if r.len() < 2 {
            return Err(DecodeError::UnexpectedEnd);
        }

        let checksum = ((r[0] as u16) << 8) | (r[1] as u16);
        let expected_checksum = X25.checksum(&r[2..]);

        if checksum != expected_checksum {
            return Err(DecodeError::OtherString(alloc::format!("checksum didn't match, dropping {checksum:04x} {expected_checksum:04x}")))
        }

        bincode::decode_from_slice(&r[2..], standard())
    }
}
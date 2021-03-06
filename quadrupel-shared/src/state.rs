use bincode::{Decode, Encode};

#[cfg(feature = "python")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Safe,
    Calibration,
    Panic,
    FullControl,
    IndividualMotorControl,
    Manual,
    YawControl,
}

use bincode::{Decode, Encode};

#[cfg(feature = "python")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Encode, Decode)]
pub enum State {
    Safe,
    Calibration,
    Panic,
    FullControl,
}

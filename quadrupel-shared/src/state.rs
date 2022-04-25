use bincode::{Encode, Decode};

#[cfg(feature = "python")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "python", derive(Serialize, Deserialize))]
#[derive(Encode, Decode)]
pub enum State {
    Safe,
    Calibration,
    Panic,
    FullControl,
}

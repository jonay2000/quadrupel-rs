use crate::library::cs_cell::CSCell;
use bincode::{Encode, Decode};

#[derive(Encode, Decode)]
pub enum State {
    Safe,
    Calibration,
    Panic,
    FullControl,
}

static GLOBAL_STATE: CSCell<State> = CSCell::new(State::Safe);

pub fn go_panic() {
    GLOBAL_STATE.update(|s| {
        *s = State::Panic;
    });
}
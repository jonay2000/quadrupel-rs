use crate::library::cs_cell::CSCell;
use quadrupel_shared::state::State;

static GLOBAL_STATE: CSCell<State> = CSCell::new(State::Safe);

pub fn go_panic() {
    GLOBAL_STATE.update(|s| {
        *s = State::Panic;
    });
}

use crate::utilities::timestamp::{self, Timestamp};
use state::InitCell;
use std::sync::RwLock;

static STATE: InitCell<RwLock<NetworkEmulatorStat>> = InitCell::new();

pub struct NetworkEmulatorStat {
    pub loss_rate: u64,
    pub total_message: u64,
    pub total_drop: u64,
}

pub struct NetworkEmulator {}

impl NetworkEmulator {
    pub fn init() {
        let state = NetworkEmulatorStat {
            loss_rate: 5,
            total_message: 0,
            total_drop: 0,
        };
        STATE.set(RwLock::new(state));
    }

    pub fn is_lost() -> bool {
        let mut state = STATE.get().write().unwrap();
        state.total_message = state.total_message + 1;

        let lost_rate = state.total_drop * 100 / state.total_message;
        if lost_rate < state.loss_rate {
            state.total_drop = state.total_drop + 1;
        }
        lost_rate < state.loss_rate
    }
}

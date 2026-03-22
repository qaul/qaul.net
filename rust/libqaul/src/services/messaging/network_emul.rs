use std::sync::RwLock;

pub struct NetworkEmulatorStat {
    pub loss_rate: u64,
    pub total_message: u64,
    pub total_drop: u64,
}

pub struct NetworkEmulator {}

impl NetworkEmulator {
    pub fn init(state: &crate::QaulState) {
        let mut emul = state.services.messaging.network_emul.write().unwrap();
        emul.loss_rate = 5;
        emul.total_message = 0;
        emul.total_drop = 0;
    }

    pub fn is_lost(state: &crate::QaulState) -> bool {
        let mut emul = state.services.messaging.network_emul.write().unwrap();
        emul.total_message = emul.total_message + 1;

        let lost_rate = emul.total_drop * 100 / emul.total_message;
        if lost_rate < emul.loss_rate {
            emul.total_drop = emul.total_drop + 1;
        }
        lost_rate < emul.loss_rate
    }
}

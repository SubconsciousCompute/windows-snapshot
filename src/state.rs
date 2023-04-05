use crate::operating_system::processes;
use serde::{Deserialize, Serialize};
use tokio::join;

/// Our main struct
///
/// Holds the state/snapshot of Windows
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Windows {
    pub processes: processes::Processes,
    pub threads: processes::Threads,
}

impl Windows {
    pub fn update(&mut self) {
        self.processes.update();
        self.threads.update();
    }

    pub async fn async_update(&mut self) {
        join!(self.threads.async_update(), self.processes.async_update());
    }
}

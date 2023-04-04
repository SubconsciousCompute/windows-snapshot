use crate::operating_system::processes;
use serde::{Deserialize, Serialize};
use tokio::join;
use wmi::{COMLibrary, WMIConnection};

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
        let com_con = unsafe { COMLibrary::assume_initialized() };

        self.processes.update();
        self.threads.update();
    }

    pub async fn async_update(&mut self) {
        let com_con = unsafe { COMLibrary::assume_initialized() };

        let wmi_con = WMIConnection::new(com_con).unwrap();

        join!(self.threads.async_update(), self.processes.async_update());
    }
}

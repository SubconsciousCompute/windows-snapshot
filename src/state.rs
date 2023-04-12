//! Stores the main state of Windows machine

use crate::operating_system::{desktop, drivers, processes, registry, services, users};
use serde::{Deserialize, Serialize};
use tokio::join;

/// Our main struct
///
/// Holds the state/snapshot of Windows
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Windows {
    /// State of Windows Processes
    pub processes: processes::Processes,
    /// State of Windows Threads
    pub threads: processes::Threads,
    /// State of Windows Drivers
    pub drivers: drivers::Drivers,
    /// State of Windows Registry
    pub registry: registry::Registry,
    /// State of Windows Services
    pub services: services::Services,
    /// State of Windows Desktops
    pub desktops: desktop::Desktops,
    /// State of Windows Environments
    pub environment: desktop::Environments,
    /// State of Windows TimeZones
    pub timezones: desktop::TimeZones,
    /// State of Windows User Accounts
    pub user_accounts: users::UserAccounts,
    /// Relation of user account and desktop settings that are specific to it
    pub user_desktops: desktop::UserDesktops,
}

impl Windows {
    /// Synchronously update all the fields
    pub fn update(&mut self) {
        self.processes.update();
        self.threads.update();
        self.drivers.update();
        self.registry.update();
        self.services.update();
        self.desktops.update();
        self.environment.update();
        self.timezones.update();
        self.user_accounts.update();
        // self.user_desktops.update();
    }

    /// Asynchronously update all the fields
    pub async fn async_update(&mut self) {
        join!(
            self.threads.async_update(),
            self.processes.async_update(),
            self.drivers.async_update(),
            self.registry.async_update(),
            self.services.async_update(),
            self.desktops.async_update(),
            self.environment.async_update(),
            self.timezones.async_update(),
            self.user_accounts.async_update(),
            // self.user_desktops.async_update(),
        );
    }
}

//! Stores the main state of Windows machine

use crate::operating_system::{
    desktop, drivers, file_system, processes, registry, services, users,
};
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
    /* This fails for now as a string is returned on object
    /// Relation of user account and desktop settings that are specific to it
    pub user_desktops: desktop::UserDesktops,
    */
    /* Abstract class, don't need it
    /// State of Windows user accounts and group accounts
    pub accounts: users::Accounts,
    */
    /// State of Windows Groups
    pub groups: users::Groups,
    /// State of Windows Logon Sessions
    pub logon_sessions: users::LogonSessions,
    /// State of Windows Network Logins
    pub network_login_profiles: users::NetworkLoginProfiles,
    /// State of Windows System Accounts
    pub system_accounts: users::SystemAccounts,
    /// State of windows Directory
    pub directories: file_system::Directories,
    /// State of windows Directory Specifications
    pub directories_specifications: file_system::DirectorySpecifications,
    /// State of windows Directory Disk Partitions
    pub disk_partition: file_system::DiskPartitions,
    /// State of windows Logical Disks
    pub logical_disks: file_system::LogicalDisks,
    /// State of windows Mapped Logical Disks
    pub mapped_logical_disks: file_system::MappedLogicalDisks,
    /// State of windows Quota Settings
    pub quota_settings: file_system::QuotaSettings,
    /// State of windows Shortcut Files
    pub shortcut_files: file_system::ShortcutFiles,
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
        // self.accounts.update();
        self.groups.update();
        self.logon_sessions.update();
        self.network_login_profiles.update();
        self.system_accounts.update();
        self.directories.update();
        self.directories_specifications.update();
        self.disk_partition.update();
        self.logical_disks.update();
        self.mapped_logical_disks.update();
        self.quota_settings.update();
        self.shortcut_files.update();
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
            // self.accounts.async_update(),
            self.groups.async_update(),
            self.logon_sessions.async_update(),
            self.network_login_profiles.async_update(),
            self.system_accounts.async_update(),
            self.directories.async_update(),
            self.directories_specifications.async_update(),
            self.disk_partition.async_update(),
            self.logical_disks.async_update(),
            self.mapped_logical_disks.async_update(),
            self.quota_settings.async_update(),
            self.shortcut_files.async_update(),
        );
    }
}

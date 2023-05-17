//! Stores the main state of Windows machine

use crate::operating_system::{
    desktop, drivers, file_system, processes, registry, services, users, event_log, memory_and_pagefiles, scheduler_jobs, product_activation, software_license_provider, shares
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
    /// State of windows Volumes
    pub volumes: file_system::Volumes,
    /// State of windows NTEventLogFiles
    pub nt_event_log_files: event_log::NTEventlogFiles,
    /// State of windows NTLogEvents
    pub nt_log_events: event_log::NTLogEvents,
    /// State of windows PageFiles
    pub pagefiles: memory_and_pagefiles::PageFiles,
    /// State of windows PageFileSettings
    pub pagefile_settings: memory_and_pagefiles::PageFileSettings,
    /// State of windows PageFileUsages
    pub pagefile_usages: memory_and_pagefiles::PageFileUsages,
    /// State of windows ScheduledJobs
    pub scheduled_jobs: scheduler_jobs::ScheduledJobs,
    /// State of windows LocalTimes
    pub local_times: scheduler_jobs::LocalTimes,
    /// State of windows UTCTimes
    pub utc_times: scheduler_jobs::UTCTimes,
    /// State of windows Proxys
    pub proxys: product_activation::Proxys,
    /// State of windows WindowsProductActivations
    pub windows_product_activations: product_activation::WindowsProductActivations,
    /// State of windows SoftwareLicensingProducts
    pub software_licensing_products: software_license_provider::SoftwareLicensingProducts,
    /// State of windows SoftwareLicensingServices
    pub software_licensing_services: software_license_provider::SoftwareLicensingServices,
    /// State of windows SoftwareLicensingTokenActivationLicenses
    pub software_licensing_token_activation_licenses: software_license_provider::SoftwareLicensingTokenActivationLicenses,
    /// State of windows ServerConnections
    pub server_connections: shares::ServerConnections,
    /// State of windows ServerSessions
    pub server_sessions: shares::ServerSessions,
    /// State of windows Shares
    pub shares: shares::Shares,
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
        self.volumes.update();
        self.nt_event_log_files.update();
        self.nt_log_events.update();
        self.pagefiles.update();
        self.pagefile_settings.update();
        self.pagefile_usages.update();
        self.scheduled_jobs.update();
        self.local_times.update();
        self.utc_times.update();
        self.software_licensing_products.update();
        self.software_licensing_services.update();
        self.software_licensing_token_activation_licenses.update();
        self.server_connections.update();
        self.server_sessions.update();
        self.shares.update();
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
            self.volumes.async_update(),
            self.nt_event_log_files.async_update(),
            self.nt_log_events.async_update(),
            self.pagefiles.async_update(),
            self.pagefile_settings.async_update(),
            self.pagefile_usages.async_update(),
            self.scheduled_jobs.async_update(),
            self.local_times.async_update(),
            self.utc_times.async_update(),
            self.software_licensing_products.async_update(),
            self.software_licensing_services.async_update(),
            self.software_licensing_token_activation_licenses.async_update(),
            self.server_connections.async_update(),
            self.server_sessions.async_update(),
            self.shares.async_update(),
        );
    }
}

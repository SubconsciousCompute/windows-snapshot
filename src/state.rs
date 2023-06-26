//! Stores the main state of Windows machine

use crate::operating_system::{
    desktop, drivers, file_system, processes, registry, services, users, event_log, memory_and_pagefiles, scheduler_jobs, product_activation, software_license_provider, shares, multimedia_audio_visual, storage, security, start_menu, networking, job_objects, operating_system_settings
};
use crate::hardware::{
    cooling_device, input_device, mass_storage, networking_device, telephony, power, video_monitor
};
use serde::{Deserialize, Serialize};
use tokio::join;

/// Our main struct
///
/// Holds the state/snapshot of Windows
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
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
    /// State of Windows CodecFiles
    pub codec_files: multimedia_audio_visual::CodecFiles,
    /// State of Windows ShadowCopys
    pub shadow_copys: storage::ShadowCopys,
    /// State of Windows ShadowContexts
    pub shadow_contexts: storage::ShadowContexts,
    /// State of Windows ShadowProviders
    pub shadow_providers: storage::ShadowProviders,
    /// State of Windows LogicalFileSecuritySettings
    pub logical_file_security_settings: security::LogicalFileSecuritySettings,
    /// State of Windows LogicalShareSecuritySettings
    pub logical_share_security_settings: security::LogicalShareSecuritySettings,
    /// State of Windows PrivilegesStatuses
    pub privileges_statuses: security::PrivilegesStatuses,
    // /// State of Windows Trustees
    // pub trustees: security::Trustees,
    // /// State of Windows ACEs
    // pub aces: security::ACEs,
    // /// State of Windows SecurityDescriptors
    // pub security_descriptors: security::SecurityDescriptors,
    // /// State of Windows SecuritySettings
    // pub security_settings: security::SecuritySettings,
    /// State of Windows LogicalProgramGroups
    pub logical_program_groups: start_menu::LogicalProgramGroups,
    /// State of Windows LogicalProgramGroupItems
    pub logical_program_group_items: start_menu::LogicalProgramGroupItems,
    // /// State of Windows ProgramGroupOrItems
    // pub program_group_or_items: start_menu::ProgramGroupOrItems,
    /// State of Windows IP4PersistedRouteTables
    pub ip4_persisted_route_tables: networking::IP4PersistedRouteTables,
    /// State of Windows IP4RouteTables
    pub ip4_route_tables: networking::IP4RouteTables,
    /// State of Windows NetworkClients
    pub nework_clients: networking::NetworkClients,
    /// State of Windows NetworkConnections
    pub nework_connections: networking::NetworkConnections,
    /// State of Windows NetworkProtocols
    pub nework_protocols: networking::NetworkProtocols,
    /// State of Windows NTDomains
    pub nt_domains: networking::NTDomains,
    /// State of Windows IP4RouteTableEvents
    pub ip4_route_table_events: networking::IP4RouteTableEvents,
    // /// State of Windows LUIDs
    // pub luids: job_objects::LUIDs,
    // /// State of Windows LUIDandAttributes
    // pub luid_and_attributes: job_objects::LUIDandAttributes,
    /// State of Windows NamedJobObjects
    pub named_job_objects: job_objects::NamedJobObjects,
    /// State of Windows NamedJobObjectActgInfos
    pub named_job_object_actg_infos: job_objects::NamedJobObjectActgInfos,
    /// State of Windows NamedJobObjectLimitSettings
    pub named_job_object_limit_settings: job_objects::NamedJobObjectLimitSettings,
    /// State of Windows BootConfigurations
    pub boot_configurations: operating_system_settings::BootConfigurations,
    /// State of Windows ComputerSystems
    pub computer_systems: operating_system_settings::ComputerSystems,
    /// State of Windows ComputerSystemProducts
    pub computer_system_products: operating_system_settings::ComputerSystemProducts,
    /// State of Windows LoadOrderGroups
    pub load_order_groups: operating_system_settings::LoadOrderGroups,
    /// State of Windows OperatingSystems
    pub operating_systems: operating_system_settings::OperatingSystems,
    /// State of Windows OSRecoveryConfigurations
    pub os_recovery_configurations: operating_system_settings::OSRecoveryConfigurations,
    /// State of Windows QuickFixEngineerings
    pub quick_fix_engineerings: operating_system_settings::QuickFixEngineerings,
    /// State of Windows StartupCommands
    pub startup_commands: operating_system_settings::StartupCommands,
    /// State of Windows Fans
    pub fans: cooling_device::Fans,
    /// State of Windows HeatPipes
    pub heat_pipes: cooling_device::HeatPipes,
    /// State of Windows Refrigerations
    pub refrigerations: cooling_device::Refrigerations,
    /// State of Windows TemperatureProbes
    pub temperature_probes: cooling_device::TemperatureProbes,
    /// State of Windows Keyboards
    pub keyboards: input_device::Keyboards,
    /// State of Windows PointingDevices
    pub pointing_devices: input_device::PointingDevices,
    /// State of Windows AutochkSettings
    pub autochk_settings: mass_storage::AutochkSettings,
    /// State of Windows CDROMDrives
    pub cd_rom_drives: mass_storage::CDROMDrives,
    /// State of Windows DiskDrives
    pub disk_drives: mass_storage::DiskDrives,
    /// State of Windows PhysicalMedias
    pub physical_medias: mass_storage::PhysicalMedias,
    /// State of Windows TapeDrives
    pub tape_drives: mass_storage::TapeDrives,
    /// State of Windows NetworkAdapters
    pub network_adapters: networking_device::NetworkAdapters,
    /// State of Windows NetworkAdapterConfigurations
    pub network_adapter_configurations: networking_device::NetworkAdapterConfigurations,
    /// State of Windows POTSModems
    pub pot_modems: telephony::POTSModems,
    /// State of Windows Batteries
    pub batteries: power::Batteries,
    /// State of Windows CurrentProbes
    pub current_probes: power::CurrentProbes,
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
        self.codec_files.update();
        self.shadow_copys.update();
        self.shadow_contexts.update();
        self.shadow_providers.update();
        self.logical_file_security_settings.update();
        self.logical_share_security_settings.update();
        self.privileges_statuses.update();
        self.logical_program_groups.update();
        self.logical_program_group_items.update();
        self.ip4_persisted_route_tables.update();
        self.ip4_route_tables.update();
        self.nework_clients.update();
        self.nework_connections.update();
        self.nework_protocols.update();
        self.nt_domains.update();
        self.ip4_route_table_events.update();
        self.named_job_objects.update();
        self.named_job_object_actg_infos.update();
        self.named_job_object_limit_settings.update();
        self.boot_configurations.update();
        self.computer_systems.update();
        self.computer_system_products.update();
        self.load_order_groups.update();
        self.operating_systems.update();
        self.os_recovery_configurations.update();
        self.quick_fix_engineerings.update();
        self.startup_commands.update();
        self.fans.update();
        self.heat_pipes.update();
        self.refrigerations.update();
        self.temperature_probes.update();
        self.keyboards.update();
        self.pointing_devices.update();
        self.autochk_settings.update();
        self.cd_rom_drives.update();
        self.disk_drives.update();
        self.physical_medias.update();
        self.tape_drives.update();
        self.network_adapters.update();
        self.network_adapter_configurations.update();
        self.pot_modems.update();
        self.batteries.update();
        self.current_probes.update();
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
            self.codec_files.async_update(),
            self.shadow_copys.async_update(),
            self.shadow_contexts.async_update(),
            self.shadow_providers.async_update(),
            self.logical_file_security_settings.async_update(),
            self.logical_share_security_settings.async_update(),
            self.privileges_statuses.async_update(),
            self.logical_program_groups.async_update(),
            self.logical_program_group_items.async_update(),
            self.ip4_persisted_route_tables.async_update(),
            self.ip4_route_tables.async_update(),
            self.nework_clients.async_update(),
            self.nework_connections.async_update(),
            self.nework_protocols.async_update(),
            self.nt_domains.async_update(),
            self.ip4_route_table_events.async_update(),
            self.named_job_objects.async_update(),
            self.named_job_object_actg_infos.async_update(),
            self.named_job_object_limit_settings.async_update(),
            self.boot_configurations.async_update(),
            self.computer_systems.async_update(),
            self.computer_system_products.async_update(),
            self.load_order_groups.async_update(),
            self.operating_systems.async_update(),
            self.os_recovery_configurations.async_update(),
            self.quick_fix_engineerings.async_update(),
            self.startup_commands.async_update(),
            self.fans.async_update(),
            self.heat_pipes.async_update(),
            self.refrigerations.async_update(),
            self.temperature_probes.async_update(),
            self.keyboards.async_update(),
            self.pointing_devices.async_update(),
            self.autochk_settings.async_update(),
            self.cd_rom_drives.async_update(),
            self.disk_drives.async_update(),
            self.physical_medias.async_update(),
            self.tape_drives.async_update(),
            self.network_adapters.async_update(),
            self.network_adapter_configurations.async_update(),
            self.pot_modems.async_update(),
            self.batteries.async_update(),
            self.current_probes.async_update(),
        );
    }
}

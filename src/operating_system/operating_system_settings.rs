//! The Operating System Settings subcategory groups classes that represent the Operating System and its settings.
//! 
//! | Class                                                                                       | Description                                                                                                                                                                        |
//! |---------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_BootConfiguration**](win32-bootconfiguration)                                 | Instance class<br/> Represents the boot configuration of a computer system running Windows.<br/>                                                                       |
//! | [**Win32\_ComputerSystem**](win32-computersystem)                                       | Instance class<br/> Represents a computer system operating in a Windows environment.<br/>                                                                              |
//! | [**Win32\_ComputerSystemProcessor**](win32-computersystemprocessor)                     | Association class<br/> Relates a computer system and a processor running on that system.<br/>                                                                          |
//! | [**Win32\_ComputerSystemProduct**](win32-computersystemproduct)                         | Instance class<br/> Represents a product.<br/>                                                                                                                         |
//! | [**Win32\_DependentService**](win32-dependentservice)                                   | Association class<br/> Relates two interdependent base services.<br/>                                                                                                  |
//! | [**Win32\_LoadOrderGroup**](win32-loadordergroup)                                       | Instance class<br/> Represents a group of system services that define execution dependencies.<br/>                                                                     |
//! | [**Win32\_LoadOrderGroupServiceDependencies**](win32-loadordergroupservicedependencies) | Instance class<br/> Represents an association between a base service and a load order group that the service depends on to start running.<br/>                         |
//! | [**Win32\_LoadOrderGroupServiceMembers**](win32-loadordergroupservicemembers)           | Association class<br/> Relates a load order group and a base service.<br/>                                                                                             |
//! | [**Win32\_OperatingSystem**](win32-operatingsystem)                                     | Instance class<br/> Represents an operating system installed on a computer system running Windows.<br/>                                                                |
//! | [**Win32\_OperatingSystemQFE**](win32-operatingsystemqfe)                               | Association class<br/> Relates an operating system and product updates applied as represented in [**Win32\_QuickFixEngineering**](win32-quickfixengineering.md).<br/> |
//! | [**Win32\_OSRecoveryConfiguration**](win32-osrecoveryconfiguration)                     | Instance class<br/> Represents the types of information that will be gathered from memory when the operating system fails.<br/>                                        |
//! | [**Win32\_QuickFixEngineering**](win32-quickfixengineering)                             | Instance class<br/> Represents system-wide Quick Fix Engineering (QFE) or updates that have been applied to the current operating system.<br/>                         |
//! | [**Win32\_StartupCommand**](win32-startupcommand)                                       | Instance class<br/> Represents a command that runs automatically when a user logs onto the computer system.<br/>                                                       |
//! | [**Win32\_SystemBootConfiguration**](win32-systembootconfiguration)                     | Association class<br/> Relates a computer system and its boot configuration.<br/>                                                                                      |
//! | [**Win32\_SystemDesktop**](win32-systemdesktop)                                         | Association class<br/> Relates a computer system and its desktop configuration.<br/>                                                                                   |
//! | [**Win32\_SystemDevices**](win32-systemdevices)                                         | Association class<br/> Relates a computer system and a logical device installed on that system.<br/>                                                                   |
//! | [**Win32\_SystemLoadOrderGroups**](win32-systemloadordergroups)                         | Association class<br/> Relates a computer system and a load order group.<br/>                                                                                          |
//! | [**Win32\_SystemNetworkConnections**](win32-systemnetworkconnections)                   | Association class<br/> Relates a network connection and the computer system on which it resides.<br/>                                                                  |
//! | [**Win32\_SystemOperatingSystem**](win32-systemoperatingsystem)                         | Association class<br/> Relates a computer system and its operating system.<br/>                                                                                        |
//! | [**Win32\_SystemProcesses**](win32-systemprocesses)                                     | Association class <br/> Relates a computer system and a process running on that system.<br/>                                                                           |
//! | [**Win32\_SystemProgramGroups**](win32-systemprogramgroups)                             | Association class<br/> Relates a computer system and a logical program group.<br/>                                                                                     |
//! | [**Win32\_SystemResources**](win32-systemresources)                                     | Association class<br/> Relates a system resource and the computer system it resides on.<br/>                                                                           |
//! | [**Win32\_SystemServices**](win32-systemservices)                                       | Association class<br/> Relates a computer system and a service program that exists on the system.<br/>                                                                 |
//! | [**Win32\_SystemSetting**](win32-systemsetting)                                         | Association class<br/> Relates a computer system and a general setting on that system.<br/>                                                                            |
//! | [**Win32\_SystemSystemDriver**](win32-systemsystemdriver)                               | Association class<br/> Relates a computer system and a system driver running on that computer system.<br/>                                                             |
//! | [**Win32\_SystemTimeZone**](win32-systemtimezone)                                       | Association class<br/> Relates a computer system and a time zone.<br/>                                                                                                 |
//! | [**Win32\_SystemUsers**](win32-systemusers)                                             | Association class<br/> Relates a computer system and a user account on that system.<br/>                                                                               |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection};

/// Represents the state of Windows BootConfigurations
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BootConfigurations {
    /// Represents sequence of Windows `BootConfigurations`
    pub boot_configurations: Vec<Win32_BootConfiguration>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(BootConfigurations, boot_configurations);

/// The Win32_BootConfiguration WMI class represents the boot configuration of a computer system running Windows.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-bootconfiguration>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_BootConfiguration {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Path to the system files required for booting the system.
    /// 
    /// Example: "C:\Windows"
    pub BootDirectory: Option<String>,
    /// Path to the configuration files. This value may be similar to the value in the BootDirectory property.
    pub ConfigurationPath: Option<String>,
    /// Last drive letter to which a physical drive is assigned.
    /// 
    /// Example: "E:"
    pub LastDrive: Option<String>,
    /// Name of the boot configuration. It is an identifier for the boot configuration.
    pub Name: Option<String>,
    /// Directory where temporary files can reside during boot time.
    pub ScratchDirectory: Option<String>,
    /// Directory where temporary files are stored.
    /// 
    /// Example: "C:\TEMP"
    pub TempDirectory: Option<String>,
}
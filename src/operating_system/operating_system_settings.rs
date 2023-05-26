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
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows BootConfigurations
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BootConfigurations {
    /// Represents sequence of Windows `BootConfigurations`
    pub boot_configurations: Vec<Win32_BootConfiguration>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(BootConfigurations, boot_configurations);

/// Represents the state of Windows ComputerSystems
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ComputerSystems {
    /// Represents sequence of Windows `ComputerSystems`
    pub computer_systems: Vec<Win32_ComputerSystem>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(ComputerSystems, computer_systems);

/// Represents the state of Windows ComputerSystemProducts
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ComputerSystemProducts {
    /// Represents sequence of Windows `ComputerSystemProducts`
    pub computer_system_products: Vec<Win32_ComputerSystemProduct>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(ComputerSystemProducts, computer_system_products);

/// Represents the state of Windows LoadOrderGroups
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoadOrderGroups {
    /// Represents sequence of Windows `LoadOrderGroups`
    pub load_order_groups: Vec<Win32_LoadOrderGroup>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LoadOrderGroups, load_order_groups);

/// Represents the state of Windows OperatingSystems
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OperatingSystems {
    /// Represents sequence of Windows `OperatingSystems`
    pub operating_systems: Vec<Win32_OperatingSystem>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(OperatingSystems, operating_systems);

/// Represents the state of Windows OSRecoveryConfigurations
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OSRecoveryConfigurations {
    /// Represents sequence of Windows `OSRecoveryConfigurations`
    pub os_recovery_configurations: Vec<Win32_OSRecoveryConfiguration>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(OSRecoveryConfigurations, os_recovery_configurations);

/// Represents the state of Windows QuickFixEngineerings
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuickFixEngineerings {
    /// Represents sequence of Windows `QuickFixEngineerings`
    pub quick_fix_engineerings: Vec<Win32_QuickFixEngineering>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(QuickFixEngineerings, quick_fix_engineerings);

/// Represents the state of Windows StartupCommands
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StartupCommands {
    /// Represents sequence of Windows `StartupCommands`
    pub startup_commands: Vec<Win32_StartupCommand>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(StartupCommands, startup_commands);

/// The `Win32_BootConfiguration` WMI class represents the boot configuration of a computer system running Windows.
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

/// The `Win32_ComputerSystem` WMI class represents a computer system running Windows.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystem>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ComputerSystem {
    /// System hardware security settings for administrator password status.
    /// 
    /// - `Disabled` (0)
    /// - `Enabled` (1)
    /// - `Not Implemented` (2)
    /// - `Unknown` (3)
    pub AdminPasswordStatus: Option<u16>,
    /// If `True`, the system manages the page file.
    pub AutomaticManagedPagefile: Option<bool>,
    /// If `True`, the automatic reset boot option is enabled.
    pub AutomaticResetBootOption: Option<bool>,
    /// If `True`, the automatic reset is enabled.
    pub AutomaticResetCapability: Option<bool>,
    /// Boot option limit is ON. Identifies the system action when the  ResetLimit` value is reached.
    /// 
    /// - `Reserved` (0)
    /// - `Operating system` (1)
    /// - `System utilities` (2)
    /// - `Do not reboot` (3)
    pub BootOptionOnLimit: Option<u16>,
    /// Type of reboot action after the time on the watchdog timer is elapsed.
    /// 
    /// - `Reserved` (0)
    /// - `Operating system` (1)
    /// - `System utilities` (2)
    /// - `Do not reboot` (3)
    pub BootOptionOnWatchDog: Option<u16>,
    /// If `True`, indicates whether a boot ROM is supported.
    pub BootROMSupported: Option<bool>,
    /// System is started. Fail-safe boot bypasses the user startup files also called SafeBoot.
    /// 
    /// The following list contains the required values:
    /// - "Normal boot"
    /// - "Fail-safe boot"
    /// - "Fail-safe with network boot"
    /// 
    /// `Normal boot` ("Normal boot")
    /// 
    /// `Fail-safe boot` ("Fail-safe boot")
    /// 
    /// `Fail-safe with network boot` ("Fail-safe with network boot")
    pub BootupState: Option<String>,
    /// Status and Additional Data fields that identify the boot status.
    /// 
    /// This value comes from the `Boot Status` member of the `System Boot Information` structure in the SMBIOS 
    /// information.
    /// 
    /// `Windows Server 2012 R2`, `Windows 8.1`, `Windows Server 2012`, `Windows 8`, `Windows Server 2008 R2`, 
    /// `Windows 7`, `Windows Server 2008` and `Windows Vista`: This property is not supported before Windows 10 
    /// and Windows Server 2016.
    pub BootStatus: Option<Vec<u16>>,
    /// Short description of the object a one-line string.
    pub Caption: Option<String>,
    /// Boot up state of the chassis.
    /// 
    /// This value comes from the `Boot-up State` member of the `System Enclosure or Chassis` structure in the SMBIOS 
    /// information.
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Safe` (3)
    /// - `Warning` (4)
    /// - `Critical` (5)
    /// - `Non-recoverable` (6)
    pub ChassisBootupState: Option<u16>,
    /// The chassis or enclosure SKU number as a string.
    /// 
    /// This value comes from the `SKU Number` member of the `System Enclosure or Chassis` structure in the SMBIOS 
    /// information.
    /// 
    /// `Windows Server 2012 R2`, `Windows 8.1`, `Windows Server 2012`, `Windows 8`, `Windows Server 2008 R2`, 
    /// `Windows 7`, `Windows Server 2008` and `Windows Vista`: This property is not supported before Windows 10 and 
    /// Windows Server 2016.
    pub ChassisSKUNumber: Option<String>,
    /// Name of the first concrete class in the inheritance chain of an instance. You can use this property with other 
    /// properties of the class to identify all instances of the class and its subclasses.
    pub CreationClassName: Option<String>,
    /// Amount of time the unitary computer system is offset from Coordinated Universal Time (UTC).
    pub CurrentTimeZone: Option<i16>,
    /// If `True`, the daylight savings mode is ON.
    pub DaylightInEffect: Option<bool>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Name of local computer according to the domain name server (DNS).
    pub DNSHostName: Option<String>,
    /// Name of the domain to which a computer belongs.
    /// 
    /// Note: If the computer is not part of a domain, then the name of the workgroup is returned.
    pub Domain: Option<String>,
    /// Role of a computer in an assigned domain workgroup. A domain workgroup is a collection of computers on the 
    /// same network. For example, a `DomainRole` property may show that a computer is a member workstation.
    /// 
    /// - `Standalone Workstation` (0)
    /// - `Member Workstation` (1)
    /// - `Standalone Server` (2)
    /// - `Member Server` (3)
    /// - `Backup Domain Controller` (4)
    /// - `Primary Domain Controller` (5)
    pub DomainRole: Option<u16>,
    /// Enables daylight savings time (DST) on a computer. A value of `True` indicates that the system time changes to 
    /// an hour ahead or behind when DST starts or ends. A value of `False` indicates that the system time does not 
    /// change to an hour ahead or behind when DST starts or ends. A value of `NULL` indicates that the DST status is 
    /// unknown on a system.
    pub EnableDaylightSavingsTime: Option<bool>,
    /// The following table lists the hardware security settings for the reset button on a computer.
    ///
    /// - `Disabled` (0)
    /// - `Enabled` (1)
    /// - `Not Implemented` (2)
    /// - `Unknown` (3)
    pub FrontPanelResetStatus: Option<u16>,
    /// If `True`, a hypervisor is present.
    /// 
    /// `Windows Server 2008 R2`, `Windows 7`, `Windows Server 2008` and `Windows Vista`: This property is not 
    /// supported before Windows 8 and Windows Server 2012.
    pub HypervisorPresent: Option<bool>,
    /// If `True`, an infrared (IR) port exists on a computer system.
    pub InfraredSupported: Option<bool>,
    /// Data required to find the initial load device or boot service to request that the operating system start up.
    /// 
    /// `Windows Server 2008 R2`: This property is available, but empty.
    pub InitialLoadInfo: Option<Vec<String>>,
    /// Object is installed. An object does not need a value to indicate that it is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// System hardware security settings for Keyboard Password Status.
    /// 
    /// - `Disabled` (0)
    /// - `Enabled` (1)
    /// - `Not Implemented` (2)
    /// - `Unknown` (3)
    pub KeyboardPasswordStatus: Option<u16>,
    /// Array entry of the `InitialLoadInfo` property that contains the data to start the loaded operating system.
    pub LastLoadInfo: Option<String>,
    /// Name of a computer manufacturer.
    /// 
    /// Example: Adventure Works
    pub Manufacturer: Option<String>,
    /// Product name that a manufacturer gives to a computer. This property must have a value.
    pub Model: Option<String>,
    /// Key of a `CIM_System` instance in an enterprise environment.
    pub Name: Option<String>,
    /// Computer system `Name` value that is generated automatically. The `CIM_ComputerSystem` object and its derivatives 
    /// are top-level objects of the Common Information Model (CIM). They provide the scope for several components. 
    /// Unique `CIM_System` keys are required, but you can define a heuristic to create the `CIM_ComputerSystem` name that 
    /// generates the same name, and is independent from the discovery protocol. This prevents inventory and management 
    /// problems when the same asset or entity is discovered multiple times, but cannot be resolved to one object. Using 
    /// a heuristic is recommended, but not required.
    /// 
    /// The heuristic is outlined in the CIM V2 Common Model specification, and assumes that the documented rules are 
    /// used to determine and assign a name. The `NameFormat` values list defines the order to assign a computer system 
    /// name. Several rules map to the same value.
    /// 
    /// The `CIM_ComputerSystem Name` value that is calculated using the heuristic is the key value of the system. 
    /// However, use aliases to assign a different name for `CIM_ComputerSystem`, which can be more unique to your company.
    /// 
    /// Values include the following:
    /// - `IP` ("IP")
    /// - `Dial` ("Dial")
    /// - `HID` ("HID")
    /// - `NWA` ("NWA")
    /// - `HWA` ("HWA")
    /// - `X25` ("X25")
    /// - `ISDN` ("ISDN")
    /// - `IPX` ("IPX")
    /// - `DCC` ("DCC")
    /// - `ICD` ("ICD")
    /// - `E.164` ("E.164")
    /// - `SNA` ("SNA")
    /// - `OID/OSI` ("OID/OSI")
    /// - `Other` ("Other")
    pub NameFormat: Option<String>,
    /// If `True`, the network Server Mode is enabled.
    pub NetworkServerModeEnabled: Option<bool>,
    /// Number of logical processors available on the computer.
    /// 
    /// You can use `NumberOfLogicalProcessors` and `NumberOfProcessors` to determine if the computer is hyperthreading. 
    pub NumberOfLogicalProcessors: Option<u32>,
    /// Number of physical processors currently available on a system. This is the number of enabled processors for a 
    /// system, which does not include the disabled processors. If a computer system has two physical processors each 
    /// containing two logical processors, then the value of `NumberOfProcessors` is 2 and `NumberOfLogicalProcessors` is 4. 
    /// The processors may be multicore or they may be hyperthreading processors.
    pub NumberOfProcessors: Option<u32>,
    /// List of data for a bitmap that the original equipment manufacturer (OEM) creates.
    pub OEMLogoBitmap: Option<Vec<u8>>,
    /// List of free-form strings that an OEM defines. For example, an OEM defines the part numbers for system reference 
    /// documents, manufacturer contact information, and so on.
    pub OEMStringArray: Option<Vec<String>>,
    /// If `True`, the computer is part of a domain. If the value is `NULL`, the computer is not in a domain or the status is 
    /// unknown. If you remove the computer from a domain, the value becomes `false`.
    pub PartOfDomain: Option<bool>,
    /// Time delay before a reboot is initiated in milliseconds. It is used after a system power cycle, local or remote 
    /// system reset, and automatic system reset. A value of 1 (minus one) indicates that the pause value is unknown.
    /// 
    /// `Windows Vista`: This property may return an unknown number.
    pub PauseAfterReset: Option<i64>,
    /// Type of the computer in use, such as laptop, desktop, or Tablet.
    /// 
    /// - `Unspecified` (0)
    /// - `Desktop` (1)
    /// - `Mobile` (2)
    /// - `Workstation` (3)
    /// - `Enterprise Server` (4)
    /// - `SOHO Server` (5) : Small Office and Home Office (SOHO) Server
    /// - `Appliance PC` (6)
    /// - `Performance Server` (7)
    /// - `Maximum` (8)
    pub PCSystemType: Option<u16>,
    /// Type of the computer in use, such as laptop, desktop, or Tablet.
    /// 
    /// `Windows Server 2012`, `Windows 8`, `Windows Server 2008 R2`, `Windows 7`, `Windows Server 2008` and `Windows Vista`: 
    /// This property is not supported before Windows 8.1 and Windows Server 2012 R2.
    /// 
    /// - `Unspecified` (0)
    /// - `Desktop` (1)
    /// - `Mobile` (2)
    /// - `Workstation` (3)
    /// - `Enterprise Server` (4)
    /// - `SOHO Server` (5)
    /// - `Appliance PC` (6)
    /// - `Performance Server` (7)
    /// - `Slate` (8)
    /// - `Maximum` (9)
    pub PCSystemTypeEx: Option<u16>,
    /// Array of the specific power-related capabilities of a logical device.
    /// 
    // - `Unknown` (0)
    // - `Not Supported` (1)
    // - `Disabled` (2)
    // - `Enabled` (3): The power management features are currently enabled, but the exact feature set is unknown or the information is unavailable.
    // - `Power Saving Modes Entered Automatically` (4): The device can change its power state based on usage or other criteria.
    // - `Power State Settable` (5): The SetPowerState method is supported. This method is found on the parent CIM_LogicalDevice class and can be implemented. 
    // - `Power Cycling Supported` (6): The SetPowerState method can be invoked with the PowerState parameter set to 5 (Power Cycle).
    // - `Timed Power On Supported` (7): Timed Power-On Supported [The SetPowerState method can be invoked with the PowerState parameter set to 5 (Power Cycle) and Time set to a specific date and time, or interval, for power-on]
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /// If `True`, device can be power-managed, for example, a device can be put into suspend mode, and so on. 
    /// This property does not indicate that power management features are enabled currently, but it does indicate 
    /// that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// System hardware security settings for Power-On Password Status.
    /// 
    /// - `Disabled` (0)
    /// - `Enabled` (1)
    /// - `Not Implemented` (2)
    /// - `Unknown` (3)
    pub PowerOnPasswordStatus: Option<u16>,
    /// Current power state of a computer and its associated operating system. The power saving states have the following 
    /// values: Value 4 (Unknown) indicates that the system is known to be in a power save mode, but its exact status in 
    /// this mode is unknown; 2 (Low Power Mode) indicates that the system is in a power save state, but still functioning 
    /// and may exhibit degraded performance; 3 (Standby) indicates that the system is not functioning, but could be 
    /// brought to full power quickly; and 7 (Warning) indicates that the computer system is in a warning state and a 
    /// power save mode.
    /// 
    /// - `Unknown` (0)
    /// - `Full Power` (1)
    /// - `Power Save` - Low Power Mode (2)
    /// - `Power Save` - Standby (3)
    /// - `Power Save` - Unknown (4)
    /// - `Power Cycle` (5)
    /// - `Power Off` (6)
    /// - `Power Save` - Warning (7)
    /// - `Power Save` - Hibernate (8): Power save hibernate.
    /// - `Power Save` - Soft Off (9): Power save soft off.
    pub PowerState: Option<u16>,
    /// State of the power supply or supplies when last booted.
    /// 
    /// This value comes from the `Power Supply State` member of the `System Enclosure or Chassis` structure in the 
    /// SMBIOS information.
    /// 
    /// The following list identifies the values for this property.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Safe` (3)
    /// - `Warning` (4)
    /// - `Critical` (5)
    /// - `Non-recoverable` (6): Nonrecoverable
    pub PowerSupplyState: Option<u16>,
    /// Contact information for the primary system owner, for example, phone number, email address, and so on.
    pub PrimaryOwnerContact: Option<String>,
    /// Name of the primary system owner.
    pub PrimaryOwnerName: Option<String>,
    /// If enabled, the value is 4 and the unitary computer system can be reset using the power and reset buttons. 
    /// If disabled, the value is 3, and a reset is not allowed.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Disabled` (3)
    /// - `Enabled` (4)
    /// - `Not Implemented` (5): Nonrecoverable
    pub ResetCapability: Option<u16>,
    /// Number of automatic resets since the last reset. A value of 1 (minus one) indicates that the count is unknown.
    pub ResetCount: Option<i16>,
    /// Number of consecutive times a system reset is attempted. A value of 1 (minus one) indicates that the limit is 
    /// unknown.
    pub ResetLimit: Option<i16>,
    /// List that specifies the roles of a system in the information technology environment.
    pub Roles: Option<Vec<String>>,
    /// Current status of an object.
    /// 
    /// For Win32_ComputerSystem, the Status is always “OK”.
    pub Status: Option<String>,
    /// List of the support contact information for the Windows operating system.
    pub SupportContactDescription: Option<Vec<String>>,
    /// The family to which a particular computer belongs. A family refers to a set of computers that are similar but 
    /// not identical from a hardware or software point of view.
    /// 
    /// This value comes from the `Family` member of the `System Information` structure in the SMBIOS information.
    /// 
    /// `Windows Server 2012 R2`, `Windows 8.1`, `Windows Server 2012`, `Windows 8`, `Windows Server 2008 R2`, `Windows 7`, 
    /// `Windows Server 2008` and `Windows Vista`: This property is not supported before Windows 10 and Windows Server 2016.
    pub SystemFamily: Option<String>,
    /// Identifies a particular computer configuration for sale. It is sometimes also called a product ID or purchase 
    /// order number.
    /// 
    /// This value comes from the SKU Number member of the System Information structure in the SMBIOS information.
    /// 
    /// `Windows Server 2012 R2`, `Windows 8.1`, `Windows Server 2012`, `Windows 8`, `Windows Server 2008 R2`, `Windows 7`, 
    /// `Windows Server 2008` and `Windows Vista`: This property is not supported before Windows 10 and Windows Server 2016.
    pub SystemSKUNumber: Option<String>,
    /// `SystemStartupDelay` is no longer available for use because Boot.ini is not used to configure system startup. 
    /// Instead, use the BCD classes supplied by the Boot Configuration Data (BCD) WMI provider or the `Bcdedit` command.
    pub SystemStartupDelay: Option<u16>,
    /// `SystemStartupOptions` is no longer available for use because Boot.ini is not used to configure system startup. 
    /// Instead, use the BCD classes supplied by the Boot Configuration Data (BCD) WMI provider or the `Bcdedit` command.
    pub SystemStartupOptions: Option<Vec<String>>,
    /// `SystemStartupSetting` is no longer available for use because Boot.ini is not used to configure system startup. 
    /// Instead, use the BCD classes supplied by the Boot Configuration Data (BCD) WMI provider or the `Bcdedit` command.
    pub SystemStartupSetting: Option<u8>,
    /// System running on the Windows-based computer. This property must have a value.
    /// 
    /// The following list identifies some of the possible values for this property.
    /// - "x64-based PC"
    /// - "X86-based PC"
    /// - "MIPS-based PC"
    /// - "Alpha-based PC"
    /// - "Power PC"
    /// - "SH-x PC"
    /// - "StrongARM PC"
    /// - "64-bit Intel PC"
    /// - "64-bit Alpha PC"
    /// - "Unknown"
    /// - "X86-Nec98 PC"
    /// 
    /// X86-based PC ("X86-based PC")
    /// 
    /// MIPS-based PC ("MIPS-based PC")
    /// 
    /// Alpha-based PC ("Alpha-based PC")
    /// 
    /// Power PC ("Power PC")
    /// 
    /// SH-x PC ("SH-x PC")
    /// 
    /// StrongARM PC ("StrongARM PC")
    /// 
    /// 64-bit Intel PC ("64-bit Intel PC")
    /// 
    /// x64-based PC ("x64-based PC")
    /// 
    /// Unknown ("Unknown")
    /// 
    /// X86-Nec98 PC ("X86-Nec98 PC")
    pub SystemType: Option<String>,
    /// Thermal state of the system when last booted.
    /// 
    /// This value comes from the `Thermal State` member of the `System Enclosure or Chassis` structure in the SMBIOS 
    /// information.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Safe` (3)
    /// - `Warning` (4)
    /// - `Critical` (5)
    /// - `Non-recoverable` (6)
    pub ThermalState: Option<u16>,
    /// Total size of physical memory. Be aware that, under some circumstances, this property may not return an 
    /// accurate value for the physical memory. For example, it is not accurate if the BIOS is using some of the 
    /// physical memory. For an accurate value, use the `Capacity` property in `Win32_PhysicalMemory` instead.
    /// 
    /// Example: 67108864
    pub TotalPhysicalMemory: Option<u64>,
    /// Name of a user that is logged on currently. This property must have a value. In a terminal services session, 
    /// `UserName` returns the name of the user that is logged on to the console not the user logged on during the 
    /// terminal service session.
    /// 
    /// Example: jeffsmith
    pub UserName: Option<String>,
    /// Event that causes the system to power up.
    /// 
    /// This value comes from the `Wake-up Type` member of the `System Information` structure in the SMBIOS information.
    /// 
    /// - `Reserved` (0)
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `APM Timer` (3)
    /// - `Modem Ring` (4)
    /// - `LAN Remote` (5)
    /// - `Power Switch` (6)
    /// - `PCI PME#` (7)
    /// - `AC Power Restored` (8)
    pub WakeUpType: Option<u16>,
    /// Name of the workgroup for this computer. If the value of the `PartOfDomain` property is `False`, then the name of 
    /// the workgroup is returned.
    pub Workgroup: Option<String>,
}

/// The `Win32_ComputerSystemProduct` WMI class represents a product. This includes software and hardware used on this 
/// computer system.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystemproduct>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ComputerSystemProduct {
    /// Short textual description for the product.
    pub Caption: Option<String>,
    /// Textual description of the product.
    pub Description: Option<String>,
    /// Product identification, such as a serial number on software, a die number on a hardware chip, or 
    /// (for noncommercial products) a project number.
    pub IdentifyingNumber: Option<String>,
    /// Commonly used product name.
    pub Name: Option<String>,
    /// Product's stock-keeping unit (SKU) information.
    pub SKUNumber: Option<String>,
    /// Name of the product's supplier, or the entity selling the product (the manufacturer, reseller, OEM, and so on).
    pub Vendor: Option<String>,
    /// Product version information.
    pub Version: Option<String>,
    /// Universally unique identifier (UUID) for this product. A UUID is a 128-bit identifier that is guaranteed to be 
    /// different from other generated UUIDs. If a UUID is not available, a UUID of all zeros is used.
    /// 
    /// This value comes from the `UUID` member of the `System Information` structure in the SMBIOS information.
    pub UUID: Option<String>,
}

/// The `Win32_LoadOrderGroup` WMI class represents a group of system services that define execution dependencies. 
/// The services must be initiated in the order specified by the Load Order Group, as the services are dependent on 
/// each other. These dependent services require the presence of the antecedent services to function correctly. The 
/// data in this class is derived by the provider from the registry key: 
/// `HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\ServiceGroupOrder`
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-loadordergroup>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LoadOrderGroup {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object. Operational and non-operational status can be defined. 
    /// Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" indicates that an element is 
    /// functioning properly, but is predicting a failure (for example, a SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can apply during 
    /// disk mirror-resilvering, reloading a user permissions list, or other administrative work. Not all such work is 
    /// online, but the managed element is neither "OK" nor in one of the other states.
    /// 
    /// Values include the following:
    /// - `OK` ("OK")
    /// - `Error` ("Error")
    /// - `Degraded` ("Degraded")
    /// - `Unknown` ("Unknown")
    /// - `Pred Fail` ("Pred Fail")
    /// - `Starting` ("Starting")
    /// - `Stopping` ("Stopping")
    /// - `Service` ("Service")
    /// - `Stressed` ("Stressed")
    /// - `NonRecover` ("NonRecover")
    /// - `No Contact` ("No Contact")
    /// - `Lost Comm` ("Lost Comm")
    pub Status: Option<String>,
    /// Indicates whether this load order group can include drivers along with system services.
    pub DriverEnabled: Option<bool>,
    /// Sequence in which this group of services is loaded onto the operating system.
    /// 
    /// Example: 2
    pub GroupOrder: Option<u32>,
    /// Name of the load order group.
    /// 
    /// Example: "Primary disk"
    pub Name: Option<String>,
}

/// The `Win32_OperatingSystem` WMI class represents a Windows-based operating system installed on a computer.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_OperatingSystem {
    /// Name of the disk drive from which the Windows operating system starts.
    /// 
    /// Example: "\\Device\Harddisk0"
    pub BootDevice: Option<String>,
    /// Build number of an operating system. It can be used for more precise version information than product release 
    /// version numbers.
    /// 
    /// Example: "1381"
    pub BuildNumber: Option<String>,
    /// Type of build used for an operating system.
    /// 
    /// Examples: ""retail build"", ""checked build""
    pub BuildType: Option<String>,
    /// Short description of the object—a one-line string. The string includes the operating system version. For 
    /// example, "Microsoft Windows 7 Enterprise ". This property can be localized.
    /// 
    /// `Windows Vista and Windows 7`: This property may contain trailing characters. For example, the string 
    /// "Microsoft Windows 7 Enterprise " (trailing space included) may be necessary to retrieve information using 
    /// this property.
    pub Caption: Option<String>,
    /// Code page value an operating system uses. A code page contains a character table that an operating system uses 
    /// to translate strings for different languages. The American National Standards Institute (ANSI) lists values 
    /// that represent defined code pages. If an operating system does not use an ANSI code page, this member is set 
    /// to 0 (zero). The `CodeSet` string can use a maximum of six characters to define the code page value.
    /// 
    /// Example: "1255"
    pub CodeSet: Option<String>,
    /// Code for the country/region that an operating system uses. Values are based on international phone dialing 
    /// prefixes—also referred to as IBM country/region codes. This property can use a maximum of six characters to 
    /// define the country/region code value.
    /// 
    /// Example: "1" (United States)
    pub CountryCode: Option<String>,
    /// Name of the first concrete class that appears in the inheritance chain used in the creation of an instance. 
    /// When used with other key properties of the class, this property allows all instances of this class and its 
    /// subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Creation class name of the scoping computer system.
    pub CSCreationClassName: Option<String>,
    /// `NULL`-terminated string that indicates the latest service pack installed on a computer. If no service pack is 
    /// installed, the string is `NULL`.
    /// 
    /// Example: "Service Pack 3"
    pub CSDVersion: Option<String>,
    /// Name of the scoping computer system.
    pub CSName: Option<String>,
    /// Number, in minutes, an operating system is offset from Greenwich mean time (GMT). The number is positive, 
    /// negative, or zero.
    pub CurrentTimeZone: Option<i16>,
    /// Data execution prevention is a hardware feature to prevent buffer overrun attacks by stopping the execution of 
    /// code on data-type memory pages. If True, then this feature is available. On 64-bit computers, the data 
    /// execution prevention feature is configured in the BCD store and the properties in `Win32_OperatingSystem` are 
    /// set accordingly.
    pub DataExecutionPrevention_Available: Option<bool>,
    /// When the data execution prevention hardware feature is available, this property indicates that the feature is 
    /// set to work for 32-bit applications if `True`. On 64-bit computers, the data execution prevention feature is 
    /// configured in the Boot Configuration Data (BCD) store and the properties in `Win32_OperatingSystem` are set 
    /// accordingly.
    pub DataExecutionPrevention_32BitApplications: Option<bool>,
    /// When the data execution prevention hardware feature is available, this property indicates that the feature is 
    /// set to work for drivers if `True`. On 64-bit computers, the data execution prevention feature is configured in 
    /// the BCD store and the properties in `Win32_OperatingSystem` are set accordingly.
    pub DataExecutionPrevention_Drivers: Option<bool>,
    /// Indicates which Data Execution Prevention (DEP) setting is applied. The DEP setting specifies the extent to 
    /// which DEP applies to 32-bit applications on the system. DEP is always applied to the Windows kernel.
    /// - `Always Off` (0): DEP is turned off for all 32-bit applications on the computer with no exceptions. This setting is not available for the user interface.
    /// - `Always On` (1): DEP is enabled for all 32-bit applications on the computer. This setting is not available for the user interface.
    /// - `Opt In` (2): DEP is enabled for a limited number of binaries, the kernel, and all Windows-based services. However, it is off by default for all 32-bit applications. A user or administrator must explicitly choose either the Always On or the Opt Out setting before DEP can be applied to 32-bit applications.
    /// - `Opt Out` (3): DEP is enabled by default for all 32-bit applications. A user or administrator can explicitly remove support for a 32-bit application by adding the application to an exceptions list.
    pub DataExecutionPrevention_SupportPolicy: Option<u8>,
    /// Operating system is a checked (debug) build. If `True`, the debugging version is installed. Checked builds 
    /// provide error checking, argument verification, and system debugging code. Additional code in a checked binary 
    /// generates a kernel debugger error message and breaks into the debugger. This helps immediately determine the 
    /// cause and location of the error. Performance may be affected in a checked build due to the additional code that
    /// is executed.
    pub Debug: Option<bool>,
    /// Description of the Windows operating system. Some user interfaces for example, those that allow editing of this 
    /// description, limit its length to 48 characters.
    pub Description: Option<String>,
    /// If `True`, the operating system is distributed across several computer system nodes. If so, these nodes should be 
    /// grouped as a cluster.
    pub Distributed: Option<bool>,
    /// Encryption level for secure transactions: 40-bit, 128-bit, or n-bit.
    /// 
    /// - `40-bit` (0)
    /// - `128-bit` (1)
    /// - `n-bit` (2)
    pub EncryptionLevel: Option<u32>,
    /// Increase in priority is given to the foreground application. Application boost is implemented by giving an application more execution time slices (quantum lengths).
    /// - `None` (0): The system boosts the quantum length by 6.
    /// - `Minimum` (1): The system boosts the quantum length by 12.
    /// - `Maximum` (2): The system boosts the quantum length by 18.
    pub ForegroundApplicationBoost: Option<u8>,
    /// Number, in kilobytes, of physical memory currently unused and available.
    pub FreePhysicalMemory: Option<u64>,
    /// Number, in kilobytes, that can be mapped into the operating system paging files without causing any other 
    /// pages to be swapped out.
    pub FreeSpaceInPagingFiles: Option<u64>,
    /// Number, in kilobytes, of virtual memory currently unused and available.
    pub FreeVirtualMemory: Option<u64>,
    /// Date object was installed. This property does not require a value to indicate that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// This property is obsolete and not supported.
    /// - `Optimize for Applications` (0): Optimize memory for applications.
    /// - `Optimize for System Performance` (1): Optimize memory for system performance.
    pub LargeSystemCache: Option<u32>,
    /// Date and time the operating system was last restarted.
    pub LastBootUpTime: Option<WMIDateTime>,
    /// Operating system version of the local date and time-of-day.
    pub LocalDateTime: Option<WMIDateTime>,
    /// Language identifier used by the operating system. A language identifier is a standard international numeric 
    /// abbreviation for a country/region. Each language has a unique language identifier (LANGID), a 16-bit value 
    /// that consists of a primary language identifier and a secondary language identifier.
    pub Locale: Option<String>,
    /// Name of the operating system manufacturer. For Windows-based systems, this value is "Microsoft Corporation".
    pub Manufacturer: Option<String>,
    /// Maximum number of process contexts the operating system can support. The default value set by the provider is 
    /// 4294967295 (0xFFFFFFFF). If there is no fixed maximum, the value should be 0 (zero). On systems that have a 
    /// fixed maximum, this object can help diagnose failures that occur when the maximum is reached—if unknown, enter 
    /// 4294967295 (0xFFFFFFFF).
    pub MaxNumberOfProcesses: Option<u32>,
    /// Maximum number, in kilobytes, of memory that can be allocated to a process. For operating systems with no 
    /// virtual memory, typically this value is equal to the total amount of physical memory minus the memory used 
    /// by the BIOS and the operating system. For some operating systems, this value may be infinity, in which case 
    /// 0 (zero) should be entered. In other cases, this value could be a constant, for example, 2G or 4G.
    pub MaxProcessMemorySize: Option<u64>,
    /// Multilingual User Interface Pack (MUI Pack ) languages installed on the computer. For example, "en-us". MUI 
    /// Pack languages are resource files that can be installed on the English version of the operating system. When 
    /// an MUI Pack is installed, you can can change the user interface language to one of 33 supported languages.
    pub MUILanguages: Option<Vec<String>>,
    /// Operating system instance within a computer system.
    pub Name: Option<String>,
    /// Number of user licenses for the operating system. If unlimited, enter 0 (zero). If unknown, enter -1.
    pub NumberOfLicensedUsers: Option<u32>,
    /// Number of process contexts currently loaded or running on the operating system.
    pub NumberOfProcesses: Option<u32>,
    /// Number of user sessions for which the operating system is storing state information currently.
    pub NumberOfUsers: Option<u32>,
    /// Stock Keeping Unit (SKU) number for the operating system. These values are the same as the `PRODUCT_*` constants 
    /// defined in WinNT.h that are used with the  `GetProductInfo` function.
    /// 
    /// The following list lists possible SKU values.
    /// 
    /// - `PRODUCT_UNDEFINED` (0): Undefined
    /// - `PRODUCT_ULTIMATE` (1): Ultimate Edition, e.g. Windows Vista Ultimate.
    /// - `PRODUCT_HOME_BASIC` (2): Home Basic Edition
    /// - `PRODUCT_HOME_PREMIUM` (3): Home Premium Edition
    /// - `PRODUCT_ENTERPRISE` (4): Enterprise Edition
    /// - `PRODUCT_BUSINESS` (6): Business Edition
    /// - `PRODUCT_STANDARD_SERVER` (7): Windows Server Standard Edition (Desktop Experience installation)
    /// - `PRODUCT_DATACENTER_SERVER` (8): Windows Server Datacenter Edition (Desktop Experience installation)
    /// - `PRODUCT_SMALLBUSINESS_SERVER` (9): Small Business Server Edition
    /// - `PRODUCT_ENTERPRISE_SERVER` (10): Enterprise Server Edition
    /// - `PRODUCT_STARTER` (11): Starter Edition
    /// - `PRODUCT_DATACENTER_SERVER_CORE` (12): Datacenter Server Core Edition
    /// - `PRODUCT_STANDARD_SERVER_CORE` (13): Standard Server Core Edition
    /// - `PRODUCT_ENTERPRISE_SERVER_CORE` (14): Enterprise Server Core Edition
    /// - `PRODUCT_WEB_SERVER` (17): Web Server Edition
    /// - `PRODUCT_HOME_SERVER` (19): Home Server Edition
    /// - `PRODUCT_STORAGE_EXPRESS_SERVER` (20): Storage Express Server Edition
    /// - `PRODUCT_STORAGE_STANDARD_SERVER` (21): Windows Storage Server Standard Edition (Desktop Experience installation)
    /// - `PRODUCT_STORAGE_WORKGROUP_SERVER` (22): Windows Storage Server Workgroup Edition (Desktop Experience installation)
    /// - `PRODUCT_STORAGE_ENTERPRISE_SERVER` (23): Storage Enterprise Server Edition
    /// - `PRODUCT_SERVER_FOR_SMALLBUSINESS` (24): Server For Small Business Edition
    /// - `PRODUCT_SMALLBUSINESS_SERVER_PREMIUM` (25): Small Business Server Premium Edition
    /// - `PRODUCT_ENTERPRISE_N` (27): Windows Enterprise Edition
    /// - `PRODUCT_ULTIMATE_N` (28): Windows Ultimate Edition
    /// - `PRODUCT_WEB_SERVER_CORE` (29): Windows Server Web Server Edition (Server Core installation)
    /// - `PRODUCT_STANDARD_SERVER_V` (36): Windows Server Standard Edition without Hyper-V
    /// - `PRODUCT_DATACENTER_SERVER_V` (37): Windows Server Datacenter Edition without Hyper-V (full installation)
    /// - `PRODUCT_ENTERPRISE_SERVER_V` (38): Windows Server Enterprise Edition without Hyper-V (full installation)
    /// - `PRODUCT_DATACENTER_SERVER_CORE_V` (39): Windows Server Datacenter Edition without Hyper-V (Server Core installation)
    /// - `PRODUCT_STANDARD_SERVER_CORE_V` (40): Windows Server Standard Edition without Hyper-V (Server Core installation)
    /// - `PRODUCT_ENTERPRISE_SERVER_CORE_V` (41): Windows Server Enterprise Edition without Hyper-V (Server Core installation)
    /// - `PRODUCT_HYPERV` (42): Microsoft Hyper-V Server
    /// - `PRODUCT_STORAGE_EXPRESS_SERVER_CORE` (43): Storage Server Express Edition (Server Core installation)
    /// - `PRODUCT_STORAGE_STANDARD_SERVER_CORE` (44): Storage Server Standard Edition (Server Core installation)
    /// - `PRODUCT_STORAGE_WORKGROUP_SERVER_CORE` (45): Storage Server Workgroup Edition (Server Core installation)
    /// - `PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE` (46): Storage Server Enterprise Edition (Server Core installation)
    /// - `PRODUCT_PROFESSIONAL` (48): Windows Professional
    /// - `PRODUCT_SB_SOLUTION_SERVER` (50): Windows Server Essentials (Desktop Experience installation)
    /// - `PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE` (63): Small Business Server Premium (Server Core installation)
    /// - `PRODUCT_CLUSTER_SERVER_V` (64): Windows Compute Cluster Server without Hyper-V
    /// - `PRODUCT_CORE_ARM` (97): Windows RT
    /// - `PRODUCT_CORE` (101): Windows Home
    /// - `PRODUCT_PROFESSIONAL_WMC` (103): Windows Professional with Media Center
    /// - `PRODUCT_MOBILE_CORE` (104): Windows Mobile
    /// - `PRODUCT_IOTUAP` (123): Windows IoT (Internet of Things) Core
    /// - `PRODUCT_DATACENTER_NANO_SERVER` (143): Windows Server Datacenter Edition (Nano Server installation)
    /// - `PRODUCT_STANDARD_NANO_SERVER` (144): Windows Server Standard Edition (Nano Server installation)
    /// - `PRODUCT_DATACENTER_WS_SERVER_CORE` (147): Windows Server Datacenter Edition (Server Core installation)
    /// - `PRODUCT_STANDARD_WS_SERVER_CORE` (148): Windows Server Standard Edition (Server Core
    /// - `PRODUCT_ENTERPRISE_FOR_VIRTUAL_DESKTOPS` (175): Windows Enterprise for Virtual Desktops (Azure Virtual Desktop)
    /// - `PRODUCT_DATACENTER_SERVER_AZURE_EDITION` (407): Windows Server Datacenter: Azure Edition
    pub OperatingSystemSKU: Option<u32>,
    /// Company name for the registered user of the operating system.
    /// 
    /// Example: "Microsoft Corporation"
    pub Organization: Option<String>,
    /// Architecture of the operating system, as opposed to the processor. This property can be localized.
    /// 
    /// Example: 32-bit
    pub OSArchitecture: Option<String>,
    /// Language version of the operating system installed. The following list lists the possible values. 
    /// 
    /// Example: 0x0807 (German, Switzerland).
    /// 
    /// - 1 (0x1): Arabic
    /// - 4 (0x4): Chinese (Simplified)– China
    /// - 9 (0x9): English
    /// - 1025 (0x401): Arabic – Saudi Arabia
    /// - 1026 (0x402): Bulgarian
    /// - 1027 (0x403): Catalan
    /// - 1028 (0x404): Chinese (Traditional) – Taiwan
    /// - 1029 (0x405): Czech
    /// - 1030 (0x406): Danish
    /// - 1031 (0x407): German – Germany
    /// - 1032 (0x408): Greek
    /// - 1033 (0x409): English – United States
    /// - 1034 (0x40A): Spanish – Traditional Sort
    /// - 1035 (0x40B): Finnish
    /// - 1036 (0x40C): French – France
    /// - 1037 (0x40D): Hebrew
    /// - 1038 (0x40E): Hungarian
    /// - 1039 (0x40F): Icelandic
    /// - 1040 (0x410): Italian – Italy
    /// - 1041 (0x411): Japanese
    /// - 1042 (0x412): Korean
    /// - 1043 (0x413): Dutch – Netherlands
    /// - 1044 (0x414): Norwegian – Bokmal
    /// - 1045 (0x415): Polish
    /// - 1046 (0x416): Portuguese – Brazil
    /// - 1047 (0x417): Rhaeto-Romanic
    /// - 1048 (0x418): Romanian
    /// - 1049 (0x419): Russian
    /// - 1050 (0x41A): Croatian
    /// - 1051 (0x41B): Slovak
    /// - 1052 (0x41C): Albanian
    /// - 1053 (0x41D): Swedish
    /// - 1054 (0x41E): Thai
    /// - 1055 (0x41F): Turkish
    /// - 1056 (0x420): Urdu
    /// - 1057 (0x421): Indonesian
    /// - 1058 (0x422): Ukrainian
    /// - 1059 (0x423): Belarusian
    /// - 1060 (0x424): Slovenian
    /// - 1061 (0x425): Estonian
    /// - 1062 (0x426): Latvian
    /// - 1063 (0x427): Lithuanian
    /// - 1065 (0x429): Persian
    /// - 1066 (0x42A): Vietnamese
    /// - 1069 (0x42D): Basque (Basque)
    /// - 1070 (0x42E): Serbian
    /// - 1071 (0x42F): Macedonian (North Macedonia)
    /// - 1072 (0x430): Sutu
    /// - 1073 (0x431): Tsonga
    /// - 1074 (0x432): Tswana
    /// - 1076 (0x434): Xhosa
    /// - 1077 (0x435): Zulu
    /// - 1078 (0x436): Afrikaans
    /// - 1080 (0x438): Faeroese
    /// - 1081 (0x439): Hindi
    /// - 1082 (0x43A): Maltese
    /// - 1084 (0x43C): Scottish Gaelic (United Kingdom)
    /// - 1085 (0x43D): Yiddish
    /// - 1086 (0x43E): Malay – Malaysia
    /// - 2049 (0x801): Arabic – Iraq
    /// - 2052 (0x804): Chinese (Simplified) – PRC
    /// - 2055 (0x807): German – Switzerland
    /// - 2057 (0x809): English – United Kingdom
    /// - 2058 (0x80A): Spanish – Mexico
    /// - 2060 (0x80C): French – Belgium
    /// - 2064 (0x810): Italian – Switzerland
    /// - 2067 (0x813): Dutch – Belgium
    /// - 2068 (0x814): Norwegian – Nynorsk
    /// - 2070 (0x816): Portuguese – Portugal
    /// - 2072 (0x818): Romanian – Moldova
    /// - 2073 (0x819): Russian – Moldova
    /// - 2074 (0x81A): Serbian – Latin
    /// - 2077 (0x81D): Swedish – Finland
    /// - 3073 (0xC01): Arabic – Egypt
    /// - 3076 (0xC04): Chinese (Traditional) – Hong Kong SAR
    /// - 3079 (0xC07): German – Austria
    /// - 3081 (0xC09): English – Australia
    /// - 3082 (0xC0A): Spanish – International Sort
    /// - 3084 (0xC0C): French – Canada
    /// - 3098 (0xC1A): Serbian – Cyrillic
    /// - 4097 (0x1001): Arabic – Libya
    /// - 4100 (0x1004): Chinese (Simplified) – Singapore
    /// - 4103 (0x1007): German – Luxembourg
    /// - 4105 (0x1009): English – Canada
    /// - 4106 (0x100A): Spanish – Guatemala
    /// - 4108 (0x100C): French – Switzerland
    /// - 5121 (0x1401): Arabic – Algeria
    /// - 5127 (0x1407): German – Liechtenstein
    /// - 5129 (0x1409): English – New Zealand
    /// - 5130 (0x140A): Spanish – Costa Rica
    /// - 5132 (0x140C): French – Luxembourg
    /// - 6145 (0x1801): Arabic – Morocco
    /// - 6153 (0x1809): English – Ireland
    /// - 6154 (0x180A): Spanish – Panama
    /// - 7169 (0x1C01): Arabic – Tunisia
    /// - 7177 (0x1C09): English – South Africa
    /// - 7178 (0x1C0A): Spanish – Dominican Republic
    /// - 8193 (0x2001): Arabic – Oman
    /// - 8201 (0x2009): English – Jamaica
    /// - 8202 (0x200A): Spanish – Venezuela
    /// - 9217 (0x2401): Arabic – Yemen
    /// - 9226 (0x240A): Spanish – Colombia
    /// - 10241 (0x2801): Arabic – Syria
    /// - 10249 (0x2809): English – Belize
    /// - 10250 (0x280A): Spanish – Peru
    /// - 11265 (0x2C01): Arabic – Jordan
    /// - 11273 (0x2C09): English – Trinidad
    /// - 11274 (0x2C0A): Spanish – Argentina
    /// - 12289 (0x3001): Arabic – Lebanon
    /// - 12298 (0x300A): Spanish – Ecuador
    /// - 13313 (0x3401): Arabic – Kuwait
    /// - 13322 (0x340A): Spanish – Chile
    /// - 14337 (0x3801): Arabic – U.A.E.
    /// - 14346 (0x380A): Spanish – Uruguay
    /// - 15361 (0x3C01): Arabic – Bahrain
    /// - 15370 (0x3C0A): Spanish – Paraguay
    /// - 16385 (0x4001): Arabic – Qatar
    /// - 16394 (0x400A): Spanish – Bolivia
    /// - 17418 (0x440A): Spanish – El Salvador
    /// - 18442 (0x480A): Spanish – Honduras
    /// - 19466 (0x4C0A): Spanish – Nicaragua
    /// - 20490 (0x500A): Spanish – Puerto Rico
    pub OSLanguage: Option<u32>,
    /// Installed and licensed system product additions to the operating system. For example, the value of 146 (0x92) 
    /// for `OSProductSuite` indicates Enterprise, Terminal Services, and Data Center (bits one, four, and seven set). 
    /// The following list lists possible values.
    /// 
    /// - 1 (0x1): Microsoft Small Business Server was once installed, but may have been upgraded to another version of Windows.
    /// - 2 (0x2): Windows Server 2008 Enterprise is installed.
    /// - 4 (0x4): Windows BackOffice components are installed.
    /// - 8 (0x8): Communication Server is installed.
    /// - 16 (0x10): Terminal Services is installed.
    /// - 32 (0x20): Microsoft Small Business Server is installed with the restrictive client license.
    /// - 64 (0x40): Windows Embedded is installed.
    /// - 128 (0x80): A Datacenter edition is installed.
    /// - 256 (0x100): Terminal Services is installed, but only one interactive session is supported.
    /// - 512 (0x200): Windows Home Edition is installed.
    /// - 1024 (0x400): Web Server Edition is installed.
    /// - 8192 (0x2000): Storage Server Edition is installed.
    /// - 16384 (0x4000): Compute Cluster Edition is installed.
    pub OSProductSuite: Option<u32>,
    /// Type of operating system. The following list identifies the possible values.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `MACOS` (2): MACROS
    /// - `ATTUNIX` (3)
    /// - `DGUX` (4)
    /// - `DECNT` (5)
    /// - `Digital Unix` (6)
    /// - `OpenVMS` (7)
    /// - `HPUX` (8)
    /// - `AIX` (9)
    /// - `MVS` (10)
    /// - `OS400` (11)
    /// - `OS/2` (12)
    /// - `JavaVM` (13)
    /// - `MSDOS` (14)
    /// - `WIN3x` (15)
    /// - `WIN95` (16)
    /// - `WIN98` (17)
    /// - `WINNT` (18)
    /// - `WINCE` (19)
    /// - `NCR3000` (20)
    /// - `NetWare` (21)
    /// - `OSF` (22)
    /// - `DC/OS` (23)
    /// - `Reliant UNIX` (24)
    /// - `SCO UnixWare` (25)
    /// - `SCO OpenServer` (26)
    /// - `Sequent` (27)
    /// - `IRIX` (28)
    /// - `Solaris` (29): Solaris
    /// - `SunOS` (30)
    /// - `U6000` (31)
    /// - `ASERIES` (32)
    /// - `TandemNSK` (33)
    /// - `TandemNT` (34)
    /// - `BS2000` (35)
    /// - `LINUX` (36)
    /// - `Lynx` (37)
    /// - `XENIX` (38)
    /// - `VM/ESA` (39)
    /// - `Interactive UNIX` (40)
    /// - `BSDUNIX` (41)
    /// - `FreeBSD` (42)
    /// - `NetBSD` (43)
    /// - `GNU Hurd` (44)
    /// - `OS9` (45)
    /// - `MACH Kernel` (46)
    /// - `Inferno` (47)
    /// - `QNX` (48)
    /// - `EPOC` (49)
    /// - `IxWorks` (50)
    /// - `VxWorks` (51)
    /// - `MiNT` (52)
    /// - `BeOS` (53)
    /// - `HP MPE` (54)
    /// - `NextStep` (55)
    /// - `PalmPilot` (56)
    /// - `Rhapsody` (57)
    /// - `Windows 2000` (58)
    /// - `Dedicated` (59)
    /// - `OS/390` (60)
    /// - `VSE` (61)
    /// - `TPF` (62)
    pub OSType: Option<u16>,
    /// Additional description for the current operating system version.
    pub OtherTypeDescription: Option<String>,
    /// If `True`, the physical address extensions (PAE) are enabled by the operating system running on Intel processors. 
    /// PAE allows applications to address more than 4 GB of physical memory. When PAE is enabled, the operating system 
    /// uses three-level linear address translation rather than two-level. Providing more physical memory to an 
    /// application reduces the need to swap memory to the page file and increases performance. To enable, PAE, use the 
    /// "/PAE" switch in the Boot.ini file. For more information about the Physical Address Extension feature, see 
    /// Physical Address Extension.
    pub PAEEnabled: Option<bool>,
    /// Not supported.
    pub PlusProductID: Option<String>,
    /// Not supported.
    pub PlusVersionNumber: Option<String>,
    /// Specifies whether the operating system booted from an external USB device. If true, the operating system has 
    /// detected it is booting on a supported locally connected storage device.
    /// 
    /// `Windows Server 2008 R2`, `Windows 7`, `Windows Server 2008` and `Windows Vista`: This property is not supported 
    /// before Windows 8 and Windows Server 2012.
    pub PortableOperatingSystem: Option<bool>,
    /// Specifies whether this is the primary operating system.
    pub Primary: Option<bool>,
    /// Additional system information.
    /// 
    /// - `Work Station` (1)
    /// - `Domain Controller` (2)
    /// - `Server` (3)
    pub ProductType: Option<u32>,
    /// Name of the registered user of the operating system.
    /// 
    /// Example: "Ben Smith"
    pub RegisteredUser: Option<String>,
    /// Operating system product serial identification number.
    /// 
    /// Example: "10497-OEM-0031416-71674"
    pub SerialNumber: Option<String>,
    /// Major version number of the service pack installed on the computer system. If no service pack has been 
    /// installed, the value is 0 (zero).
    pub ServicePackMajorVersion: Option<u16>,
    /// Minor version number of the service pack installed on the computer system. If no service pack has been 
    /// installed, the value is 0 (zero).
    pub ServicePackMinorVersion: Option<u16>,
    /// Total number of kilobytes that can be stored in the operating system paging files—0 (zero) indicates that 
    /// there are no paging files. Be aware that this number does not represent the actual physical size of the 
    /// paging file on disk.
    pub SizeStoredInPagingFiles: Option<u64>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. Operational 
    /// statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a SMART-enabled hard disk drive may 
    /// function properly, but predicts a failure in the near future). Nonoperational statuses include: "Error", 
    /// "Starting", "Stopping", and "Service". The Service status applies to administrative work, such as 
    /// mirror-resilvering of a disk, reload of a user permissions list, or other administrative work. Not all such 
    /// work is online, but the managed element is neither "OK" nor in one of the other states.
    /// 
    /// - `OK` ("OK")
    /// - `Error` ("Error")
    /// - `Degraded` ("Degraded")
    /// - `Unknown` ("Unknown")
    /// - `Pred Fail` ("Pred Fail")
    /// - `Starting` ("Starting")
    /// - `Stopping` ("Stopping")
    /// - `Service` ("Service")
    /// - `Stressed` ("Stressed")
    /// - `NonRecover` ("NonRecover")
    /// - `No Contact` ("No Contact")
    /// - `Lost Comm` ("Lost Comm")
    pub Status: Option<String>,
    /// Bit flags that identify the product suites available on the system.
    /// 
    /// For example, to specify both Personal and BackOffice, set `SuiteMask` to `4 | 512` or `516`.
    /// 
    /// - 1: Small Business
    /// - 2: Enterprise
    /// - 4: BackOffice
    /// - 8: Communications
    /// - 16: Terminal Services
    /// - 32: Small Business Restricted
    /// - 64: Embedded Edition
    /// - 128: Datacenter Edition
    /// - 256: Single User
    /// - 512: Home Edition
    /// - 1024: Web Server Edition
    pub SuiteMask: Option<u32>,
    /// Physical disk partition on which the operating system is installed.
    pub SystemDevice: Option<String>,
    /// System directory of the operating system.
    /// 
    /// Example: "C:\WINDOWS\SYSTEM32"
    pub SystemDirectory: Option<String>,
    /// Letter of the disk drive on which the operating system resides. Example: "C:"
    pub SystemDrive: Option<String>,
    /// Total swap space in kilobytes. This value may be `NULL` (unspecified) if the swap space is not distinguished 
    /// from page files. However, some operating systems distinguish these concepts. For example, in UNIX, whole 
    /// processes can be swapped out when the free page list falls and remains below a specified amount.
    pub TotalSwapSpaceSize: Option<u64>,
    /// Number, in kilobytes, of virtual memory. For example, this may be calculated by adding the amount of total 
    /// RAM to the amount of paging space, that is, adding the amount of memory in or aggregated by the computer 
    /// system to the property, `SizeStoredInPagingFiles`.
    pub TotalVirtualMemorySize: Option<u64>,
    /// Total amount, in kilobytes, of physical memory available to the operating system. This value does not 
    /// necessarily indicate the true amount of physical memory, but what is reported to the operating system as 
    /// available to it.
    pub TotalVisibleMemorySize: Option<u64>,
    /// Version number of the operating system.
    /// 
    /// Example: "4.0"
    pub Version: Option<String>,
    /// Windows directory of the operating system.
    /// 
    /// Example: "C:\WINDOWS"
    pub WindowsDirectory: Option<String>,
    /* 
    /// Not supported
    /// 
    /// **Windows Server 2008 and Windows Vista:  **
    /// 
    /// The `QuantumLength` property defines the number of clock ticks per quantum. A quantum is a unit of execution time 
    /// that the scheduler is allowed to give to an application before switching to other applications. When a thread 
    /// runs one quantum, the kernel preempts it and moves it to the end of a queue for applications with equal priorities. 
    /// The actual length of a thread's quantum varies across different Windows platforms. For Windows NT/Windows 2000 
    /// only.
    /// 
    /// The possible values are.
    // - `Unknown` (0)
    // - `One tick` (1)
    // - `Two ticks` (2)
    pub QuantumLength: Option<u8>,
    /// Not supported
    /// 
    /// **Windows Server 2008 and Windows Vista:  **
    /// 
    /// The QuantumType property specifies either fixed or variable length quantums. Windows defaults to variable length 
    /// quantums where the foreground application has a longer quantum than the background applications. Windows Server 
    /// defaults to fixed-length quantums. A quantum is a unit of execution time that the scheduler is allowed to give to 
    /// an application before switching to another application. When a thread runs one quantum, the kernel preempts it 
    /// and moves it to the end of a queue for applications with equal priorities. The actual length of a thread's 
    /// quantum varies across different Windows platforms.
    /// 
    /// The possible values are.
    /// - `Unknown` (0)
    /// - `Fixed` (1)
    /// - `Variable` (2)
    pub QuantumType: Option<u8>,
    */
}

/// The `Win32_OSRecoveryConfiguration` WMI class represents the types of information that will 
/// be gathered from memory when the operating system fails. This includes boot failures and 
/// system crashes.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-osrecoveryconfiguration>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_OSRecoveryConfiguration {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// System will automatically reboot during a recovery operation.
    pub AutoReboot: Option<bool>,
    /// Full path to the debug file. A debug file is created with the memory state of the 
    /// computer after a computer failure.
    /// 
    /// Example: "C:\Windows\Memory.dmp"
    pub DebugFilePath: Option<String>,
    /// Type of debugging information written to the log file.
    /// 
    /// - `None` (0)
    /// - `Complete memory dump` (1)
    /// - `Kernel memory dump` (2)
    /// - `Small memory dump` (3)
    pub DebugInfoType: Option<u32>,
    /// Expanded version of the `DebugFilePath` property.
    /// 
    /// Example: "C:\Windows\Memory.dmp"
    pub ExpandedDebugFilePath: Option<String>,
    /// Expanded version of the `MiniDumpDirectory` property.
    /// 
    /// Example: "C:\Windows\MiniDump"
    pub ExpandedMiniDumpDirectory: Option<String>,
    /// Only kernel debug information will be written to the debug log file. If `TRUE`, then only 
    /// the state of the kernel is written to a file in the event of a system failure. If `FALSE`, 
    /// the system will try to log the state of the memory, and any devices that can provide 
    /// information about the system when it failed.
    pub KernelDumpOnly: Option<bool>,
    /// Directory where small memory dump files will be recorded and accumulated.
    /// 
    /// Example: "%systemRoot%\MiniDump"
    pub MiniDumpDirectory: Option<String>,
    /// Identifying name for this instance of the `Win32_OSRecoveryConfiguration` class.
    pub Name: Option<String>,
    /// New debug file will overwrite an existing one.
    pub OverwriteExistingDebugFile: Option<bool>,
    /// Alert message will be sent to the system administrator in the event of an operating system 
    /// failure.
    pub SendAdminAlert: Option<bool>,
    /// Debugging information is to be written to a log file.
    pub WriteDebugInfo: Option<bool>,
    /// Events will be written to a system log.
    pub WriteToSystemLog: Option<bool>,
}

/// The `Win32_QuickFixEngineering` WMI class represents a small system-wide update, commonly referred to as a 
/// quick-fix engineering (QFE) update, applied to the current operating system. This class returns only the updates 
/// supplied by Component Based Servicing (CBS). These updates are not listed in the registry. Updates supplied by 
/// Microsoft Windows Installer (MSI) or the Windows update site (https://update.microsoft.com) are not returned by 
/// `Win32_QuickFixEngineering`.
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_QuickFixEngineering {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a key property.
    pub Name: Option<String>,
    /// String that indicates the current status of the object. Operational and non-operational status can be 
    /// defined. Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" indicates that an 
    /// element is functioning properly, but is predicting a failure (for example, a SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can apply 
    /// during disk mirror-resilvering, reloading a user permissions list, or other administrative work. Not all such 
    /// work is online, but the managed element is neither "OK" nor in one of the other states.
    /// 
    /// Values include the following:
    /// 
    /// - `OK` ("OK")
    /// - `Error` ("Error")
    /// - `Degraded` ("Degraded")
    /// - `Unknown` ("Unknown")
    /// - `Pred Fail` ("Pred Fail")
    /// - `Starting` ("Starting")
    /// - `Stopping` ("Stopping")
    /// - `Service` ("Service")
    /// - `Stressed` ("Stressed")
    /// - `NonRecover` ("NonRecover")
    /// - `No Contact` ("No Contact")
    /// - `Lost Comm` ("Lost Comm")
    pub Status: Option<String>,
    /// Local name of the computer system. 
    pub CSName: Option<String>,
    /// Additional comments that relate to the update.
    pub FixComments: Option<String>,
    /// Unique identifier associated with a particular update.
    pub HotFixID: Option<String>,
    /// Person who installed the update. If this value is unknown, the property is empty.
    pub InstalledBy: Option<String>,
    /// Date that the update was installed. If this value is unknown, the property is empty.
    /// 
    /// Note: This property may use different formats, depending on when the QuickFix was installed. Most systems 
    /// use a standard date format, such as "23-10-2013". However, some systems may return a 64-bit hexidecimal 
    /// value in the Win32 `FILETIME` format.
    pub InstalledOn: Option<String>,
    /// Service pack in effect when the update was applied. If no service pack has been applied, the property takes 
    /// on the value SP0. If it cannot be determined what service pack was in effect, this property is `NULL`.
    pub ServicePackInEffect: Option<String>,
}

/// The `Win32_StartupCommand` WMI class represents a command that runs automatically when a user logs onto the 
/// computer system.
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_StartupCommand {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Command run by the startup command.
    /// 
    /// WMI obtains this data from the registry key
    /// 
    /// `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`
    /// 
    /// Example: "C:\Windows\notepad.exe myfile.txt"
    pub Command: Option<String>,
    /// Path where the startup command resides on the disk file system.
    /// 
    /// For example: HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
    /// 
    /// `Startup` ("Startup")
    /// 
    /// `Common Startup` ("Common Startup")
    /// 
    /// `HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run` ("HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")
    /// 
    /// `HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunServices` ("HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunServices")
    pub Location: Option<String>,
    /// File name of the startup command.
    /// 
    /// Example: "FindFast"
    pub Name: Option<String>,
    /// User name for whom this startup command will run.
    /// 
    /// Example: "mydomain\myname"
    pub User: Option<String>,
    /// The UserSID property indicates the SID of the user for whom this startup command will run. That User property 
    /// may be empty but UserSID still has a value if the user name can't be resolved (like in the case of a deleted 
    /// user). The property is helpful to distinguish between commands associated w/ two different users with unresolved 
    /// names. It may be NULL when the command is associated with items not actually identifying a user like All Users.
    /// 
    /// Example:S-1-5-21-1579938362-1064596589-3161144252-1006
    pub UserSID: Option<String>,
}

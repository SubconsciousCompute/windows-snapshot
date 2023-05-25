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

/// The Win32_ComputerSystem WMI class represents a computer system running Windows.
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

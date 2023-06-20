//! Classes in the Mass Storage subcategory represent storage devices such as hard disk drives, CD-ROM drives, and tape drives.
//! 
//! | Class                                                     | Description                                                                                  |
//! |-----------------------------------------------------------|----------------------------------------------------------------------------------------------|
//! | [**Win32\_AutochkSetting**](win32-autochksetting)        | Represents the settings for the autocheck operation of a disk.                               |
//! | [**Win32\_CDROMDrive**](win32-cdromdrive)                | Represents a CD-ROM drive on a computer system running Windows.                              |
//! | [**Win32\_DiskDrive**](win32-diskdrive)                  | Represents a physical disk drive as seen by a computer running the Windows operating system. |
//! | [**Win32\_FloppyDrive**](win32-floppydrive)              | Manages the capabilities of a floppy disk drive.                                             |
//! | [**Win32\_PhysicalMedia**](/previous-versions/windows/desktop/cimwin32a/win32-physicalmedia) | Represents any type of documentation or storage medium.                                      |
//! | [**Win32\_TapeDrive**](win32-tapedrive)                  | Represents a tape drive on a computer system running Windows.                                |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's AutochkSettings
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct AutochkSettings {
    /// Sequence of windows AutochkSettings states
    pub autochk_settings: Vec<Win32_AutochkSetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(AutochkSettings, autochk_settings);

/// Represents the state of Windows user's CDROMDrives
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct CDROMDrives {
    /// Sequence of windows CDROMDrives states
    pub cd_rom_drives: Vec<Win32_CDROMDrive>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(CDROMDrives, cd_rom_drives);

/// Represents the state of Windows user's DiskDrives
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct DiskDrives {
    /// Sequence of windows DiskDrives states
    pub disk_drives: Vec<Win32_DiskDrive>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(DiskDrives, disk_drives);

/// The `Win32_AutochkSetting` WMI class represents the settings for the autocheck operation of 
/// a disk.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-autochksetting> 
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_AutochkSetting {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// An ID that is used as part of a key for the current object.
    pub SettingID: Option<String>,
    /// The user input delay for autocheck.
    pub UserInputDelay: Option<u32>,
}

/// The Win32_CDROMDrive WMI class represents a CD-ROM drive on a computer system running Windows.
/// 
/// Note: Be aware that the name of the drive does not correspond to the logical drive letter 
/// assigned to the device.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-cdromdrive>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_CDROMDrive {
    /// Availability and status of the device.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Running` / `Full Power` (3): Running or Full Power
    /// - `Warning` (4)
    /// - `In Test` (5)
    /// - `Not Applicable` (6)
    /// - `Power Off` (7)
    /// - `Off Line` (8)
    /// - `Off Duty` (9)
    /// - `Degraded` (10)
    /// - `Not Installed` (11)
    /// - `Install Error` (12)
    /// - `Power Save - Unknown` (13): The device is known to be in a power save mode, but its exact status is unknown.
    /// - `Power Save - Low Power Mode` (14): The device is in a power save state but still functioning, and may exhibit degraded performance.
    /// - `Power Save - Standby` (15): The device is not functioning, but could be brought to full power quickly.
    /// - `Power Cycle` (16)
    /// - `Power Save - Warning` (17): The device is in a warning state, though also in a power save mode.
    /// - `Paused` (18): The device is paused.
    /// - `Not Ready` (19): The device is not ready.
    /// - `Not Configured` (20): The device is not configured.
    /// - `Quiesced` (21): The device is quiet. 
    pub Availability: Option<u16>,
    /// Array of capabilities of the media access device. For example, the device may support 
    /// random access (3), removable media (7), and automatic cleaning (9).
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `Sequential Access` (2)
    /// - `Random Access` (3)
    /// - `Supports Writing` (4)
    /// - `Encryption` (5)
    /// - `Compression` (6)
    /// - `Supports Removeable Media` (7): Supports Removable Media
    /// - `Manual Cleaning` (8)
    /// - `Automatic Cleaning` (9)
    /// - `SMART Notification` (10)
    /// - `Supports Dual Sided Media` (11): Supports Dual-Sided Media
    /// - `Predismount Eject Not Required` (12)
    pub Capabilities: Option<Vec<u16>>,
    /// Array of more detailed explanations for any of the access device features indicated in 
    /// the `Capabilities` array. Each entry of this array is related to the entry in the 
    /// `Capabilities` array that is located at the same index.
    pub CapabilityDescriptions: Option<Vec<String>>,
    /// Short description of the object a one-line string.
    pub Caption: Option<String>,
    /// Algorithm or tool used by the device to support compression. If it is not possible or 
    /// not desired to describe the compression scheme (perhaps because it is not known), use 
    /// the following words: "Unknown" to represent that it is not known whether the device 
    /// supports compression capabilities; "Compressed" to represent that the device supports 
    /// compression capabilities but either its compression scheme is not known or not 
    /// disclosed; and "Not Compressed" to represent that the device does not support 
    /// compression capabilities.
    /// 
    /// - ("Unknown"): The compression scheme is unknown or not described.
    /// - ("Compressed"): The logical file is compressed, but the compression scheme is unknown or not described
    /// - ("Not Compressed"): If the logical file is not compressed
    pub CompressionMethod: Option<String>,
    /// Windows Configuration Manager error code.
    /// 
    /// - `This device is working properly.` (0): Device is working properly.
    /// - `This device is not configured correctly.` (1): Device is not configured correctly.
    /// - `Windows cannot load the driver for this device.` (2)
    /// - `The driver for this device might be corrupted, or your system may be running low on memory or other resources.` (3): Driver for this device might be corrupted, or the system may be low on memory or other resources.
    /// - `This device is not working properly. One of its drivers or your registry might be corrupted.` (4): Device is not working properly. One of its drivers or the registry might be corrupted.
    /// - `The driver for this device needs a resource that Windows cannot manage.` (5): Driver for the device requires a resource that Windows cannot manage.
    /// - `The boot configuration for this device conflicts with other devices.` (6): Boot configuration for the device conflicts with other devices.
    /// - `Cannot filter. (7)
    /// - `The driver loader for the device is missing.` (8): Driver loader for the device is missing.
    /// - `This device is not working properly because the controlling firmware is reporting the resources for the device incorrectly.` (9): Device is not working properly. The controlling firmware is incorrectly reporting the resources for the device.
    /// - `This device cannot start.` (10): Device cannot start.
    /// - `This device failed.` (11): Device failed.
    /// - `This device cannot find enough free resources that it can use.` (12): Device cannot find enough free resources to use.
    /// - `Windows cannot verify this device's resources.` (13): Windows cannot verify the device's resources.
    /// - `This device cannot work properly until you restart your computer.` (14): Device cannot work properly until the computer is restarted.
    /// - `This device is not working properly because there is probably a re-enumeration problem.` (15): Device is not working properly due to a possible re-enumeration problem.
    /// - `Windows cannot identify all the resources this device uses.` (16): Windows cannot identify all of the resources that the device uses.
    /// - `This device is asking for an unknown resource type.` (17): Device is requesting an unknown resource type.
    /// - `Reinstall the drivers for this device.` (18): Device drivers must be reinstalled.
    /// - `Failure using the VxD loader.` (19)
    /// - `Your registry might be corrupted.` (20): Registry might be corrupted.
    /// - `System failure: Try changing the driver for this device. If that does not work, see your hardware documentation. Windows is removing this device.` (21): System failure. If changing the device driver is ineffective, see the hardware documentation. Windows is removing the device.
    /// - `This device is disabled.` (22): Device is disabled.
    /// - `System failure: Try changing the driver for this device. If that doesn't work, see your hardware documentation.` (23): System failure. If changing the device driver is ineffective, see the hardware documentation.
    /// - `This device is not present, is not working properly, or does not have all its drivers installed.` (24): Device is not present, not working properly, or does not have all of its drivers installed.
    /// - `Windows is still setting up this device.` (25): Windows is still setting up the device.
    /// - `Windows is still setting up this device.` (26): Windows is still setting up the device.
    /// - `This device does not have valid log configuration.` (27): Device does not have valid log configuration.
    /// - `The drivers for this device are not installed.` (28): Device drivers are not installed.
    /// - `This device is disabled because the firmware of the device did not give it the required resources.` (29): Device is disabled. The device firmware did not provide the required resources.
    /// - `This device is using an Interrupt Request (IRQ) resource that another device is using.` (30): Device is using an IRQ resource that another device is using.
    /// - `This device is not working properly because Windows cannot load the drivers required for this device.` (31): Device is not working properly. Windows cannot load the required device drivers.
    pub ConfigManagerErrorCode: Option<u32>,
    /// If `True`, the device is using a user-defined configuration.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Name of the first concrete class that appears in the inheritance chain used in the 
    /// creation of an instance. When used with the other key properties of the class, the 
    /// property allows all instances of this class and its subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Default block size, in bytes, for this device.
    pub DefaultBlockSize: Option<u64>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier for a CD-ROM drive.
    pub DeviceID: Option<String>,
    /// Drive letter of the CD-ROM drive.
    /// 
    /// Example: "d:\"
    pub Drive: Option<String>,
    /// If `True`, files can be accurately read from the CD device. This is achieved by reading a 
    /// block of data twice and comparing the data against itself.
    pub DriveIntegrity: Option<bool>,
    /// If `True`, the error reported in LastErrorCode is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information about 
    /// corrective actions that can be taken.
    pub ErrorDescription: Option<String>,
    /// Type of error detection and correction supported by this device.
    pub ErrorMethodology: Option<String>,
    /*
    /// This property is obsolete. In place of this property, use `FileSystemFlagsEx`.
    pub FileSystemFlags: Option<u16>,
    */
    /// File system flags associated with the Windows CD-ROM drive. This parameter can be any 
    /// combination of flags, but `FS_FILE_COMPRESSION` and `FS_VOL_IS_COMPRESSED` are mutually 
    /// exclusive.
    /// 
    /// - `Case Sensitive Search` (1): The file system supports case-sensitive file names.
    /// - `Case Preserved Names` (2): The file system preserves the case of file names when it places a name on a disk.
    /// - `Unicode On Disk` (4): The file system supports Unicode in file names as they appear on the disk.
    /// - `Persistent ACLs` (8): The file system preserves and enforces access control lists (ACLs). For example, the NTFS file system preserves and enforces ACLs and the FAT file system does not.
    /// - `File Compression` (16): The file system supports file-based compression.
    /// - `Volume Quotas` (32): The file system supports disk quotas.
    /// - `Supports Sparse Files` (64): The file system supports spare files.
    /// - `Supports Reparse Points` (128): The file system supports reparse points.
    /// - `Supports Remote Storage` (256): The file system supports the remote storage of files.
    /// - `Supports Long Names` (16384): The file system supports file names that are longer than eight characters.
    /// - `Volume Is Compressed` (32768): The specified disk volume is a compressed volume, for example, a DoubleSpace volume.
    /// - `Read Only Volume` (524289): The specified volume is read-only.
    /// - `Supports Object IDS` (65536): The file system supports object identifiers.
    /// - `Supports Encryption` (131072): The file system supports the Encrypted File System (EFS).
    /// - `Supports Named Streams` (262144): The file system supports named streams.
    /// 
    /// Example: 0
    pub FileSystemFlagsEx: Option<u32>,
    /// Drive letter that uniquely identifies this CD-ROM drive.
    /// 
    /// Example: "d:\"
    pub Id: Option<String>,
    /// Date and time the object is installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Manufacturer of the Windows CD-ROM drive.
    /// 
    /// Example: "PLEXTOR"
    pub Manufacturer: Option<String>,
    /// Maximum block size, in bytes, for media accessed by this device.
    pub MaxBlockSize: Option<u64>,
    /// Maximum length of a filename component supported by the Windows CD-ROM drive. A file 
    /// name component the portion of a file name between backslashes. The value can be used 
    /// to indicate that long names are supported by the specified file system. For example, 
    /// for a FAT file system supporting long names, the function stores the value 255, rather 
    /// than the previous 8.3 indicator. Long names can also be supported on systems that use 
    /// the NTFS file system.
    /// 
    /// Example: 255
    pub MaximumComponentLength: Option<u32>,
    /// Maximum size, in kilobytes, of media supported by this device.
    pub MaxMediaSize: Option<u64>,
    /// If `True`, a CD-ROM is in the drive.
    pub MediaLoaded: Option<bool>,
    /// Type of media that can be used or accessed by this device. Possible values are:
    /// 
    /// - `Random Access` ("Random Access")
    /// - `Supports Writing` ("Supports Writing")
    /// - `Removable Media` ("Removable Media")
    /// - `CD-ROM` ("CD-ROM")
    pub MediaType: Option<String>,
    /// Firmware revision level that is assigned by the manufacturer.
    pub MfrAssignedRevisionLevel: Option<String>,
    /// Minimum block size, in bytes, for media accessed by this device.
    pub MinBlockSize: Option<u64>,
    /// Label for the object. When subclassed, the property can be overridden to be a key property.
    pub Name: Option<String>,
    /// If `True`, the media access device needs cleaning. Whether manual or automatic cleaning 
    /// is possible is indicated in the `Capabilities` property.
    pub NeedsCleaning: Option<bool>,
    /// Maximum number of media that can be supported or inserted, when the media access device 
    /// supports multiple individual media.
    pub NumberOfMediaSupported: Option<u32>,
    /// Windows Plug and Play device identifier of the logical device.
    /// 
    /// Example: "*PNP030b"
    pub PNPDeviceID: Option<String>,
    /// Array of the specific power-related capabilities of a logical device.
    /// 
    /// - `Unknown` (0)
    /// - `Not Supported` (1): Power-related capacities are not supported for this device.
    /// - `Disabled` (2)
    /// - `Enabled` (3): The power management features are currently enabled but the exact feature set is unknown or the information is unavailable.
    /// - `Power Saving Modes Entered Automatically` (4): The device can change its power state based on usage or other criteria.
    /// - `Power State Settable` (5): The `SetPowerState` method is supported. This method is found on the parent CIM_LogicalDevice class and can be implemented. For more information, see Designing Managed Object Format (MOF) Classes.
    /// - `Power Cycling Supported` (6): The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle).
    /// - `Timed Power On Supported` (7): Timed Power-On Supported. The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle) and Time set to a specific date and time, or interval, for power-on.
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /// If `True`, the device can be power-managed, which means that it can be put into suspend 
    /// mode, and so on. The property does not indicate that power management features are 
    /// currently enabled, only that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// Firmware revision level of the Windows CD-ROM drive.
    pub RevisionLevel: Option<String>,
    /// SCSI bus number for the disk drive.
    /// 
    /// Example: 0
    pub SCSIBus: Option<u32>,
    /// SCSI logical unit number (LUN) of the disk drive. The LUN is used to designate which 
    /// SCSI controller is being accessed in a system with more than one controller being used. 
    /// The SCSI device identifier is similar, but is the designation for multiple devices on 
    /// one controller.
    /// 
    /// Example: 0
    pub SCSILogicalUnit: Option<u16>,
    /// SCSI port number of the disk drive.
    /// 
    /// Example: 1
    pub SCSIPort: Option<u16>,
    /// SCSI identifier number of the Windows CD-ROM drive.
    /// 
    /// Example: 0
    pub SCSITargetId: Option<u16>,
    /// Number supplied by the manufacturer that identifies the physical media. 
    /// 
    /// Example: WD-WM3493798728.
    pub SerialNumber: Option<String>,
    /// Size of the disk drive.
    pub Size: Option<u64>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. 
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a 
    /// SMART-enabled hard disk drive, may be functioning properly but predicting a failure in the 
    /// near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service". 
    /// The latter, "Service", could apply during mirror-resilvering of a disk, reload of a user 
    /// permissions list, or other administrative work. Not all such work is online, yet the managed 
    /// element is neither "OK" nor in one of the other states.
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
    /// State of the logical device. If this property does not apply to the logical device, the 
    /// value 5 (Not Applicable) should be used.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Enabled` (3)
    /// - `Disabled` (4)
    /// - `Not Applicable` (5)
    pub StatusInfo: Option<u16>,
    /// Value of the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// Transfer rate of the CD-ROM drive. A value of -1 indicates that the rate cannot be 
    /// determined. When this happens, the CD is not in the drive.
    pub TransferRate: Option<i64>,
    /// Volume name of the Windows CD-ROM drive.
    pub VolumeName: Option<String>,
    /// Volume serial number of the media in the CD-ROM drive.
    /// 
    /// Example: A8C3-D032
    pub VolumeSerialNumber: Option<String>,
}

/// The `Win32_DiskDrive` WMI class represents a physical disk drive as seen by a computer 
/// running the Windows operating system.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskdrive>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_DiskDrive {
    /// Availability and status of the device.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Running` / `Full Power` (3): Running or Full Power
    /// - `Warning` (4)
    /// - `In Test` (5)
    /// - `Not Applicable` (6)
    /// - `Power Off` (7)
    /// - `Off Line` (8)
    /// - `Off Duty` (9)
    /// - `Degraded` (10)
    /// - `Not Installed` (11)
    /// - `Install Error` (12)
    /// - `Power Save - Unknown` (13): The device is known to be in a power save mode, but its exact status is unknown.
    /// - `Power Save - Low Power Mode` (14): The device is in a power save state but still functioning, and may exhibit degraded performance.
    /// - `Power Save - Standby` (15): The device is not functioning, but could be brought to full power quickly.
    /// - `Power Cycle` (16)
    /// - `Power Save - Warning` (17): The device is in a warning state, though also in a power save mode.
    /// - `Paused` (18): The device is paused.
    /// - `Not Ready` (19): The device is not ready.
    /// - `Not Configured` (20): The device is not configured.
    /// - `Quiesced` (21): The device is quiet. 
    pub Availability: Option<u16>,
    /// Number of bytes in each sector for the physical disk drive.
    /// 
    /// Example: 512
    pub BytesPerSector: Option<u32>,
    /// Array of capabilities of the media access device. For example, the device may support 
    /// random access (3), removable media (7), and automatic cleaning (9).
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `Sequential Access` (2)
    /// - `Random Access` (3)
    /// - `Supports Writing` (4)
    /// - `Encryption` (5)
    /// - `Compression` (6)
    /// - `Supports Removeable Media` (7): Supports Removable Media
    /// - `Manual Cleaning` (8)
    /// - `Automatic Cleaning` (9)
    /// - `SMART Notification` (10)
    /// - `Supports Dual Sided Media` (11): Supports Dual-Sided Media
    /// - `Predismount Eject Not Required` (12): Ejection Prior to Drive Dismount Not Required
    pub Capabilities: Option<Vec<u16>>,
    /// List of more detailed explanations for any of the access device features indicated in 
    /// the Capabilities array. Note, each entry of this array is related to the entry in the 
    /// Capabilities array that is located at the same index.
    pub CapabilityDescriptions: Option<Vec<String>>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// The name of the compression algorithm or one of the following values.
    /// 
    /// - ("Unknown"): Whether the device supports compression capabilities or not is not known.
    /// - ("Compressed"): The device supports compression capabilities but its compression scheme is not known or not disclosed.
    /// - ("Not Compressed"): The device does not support compression.
    pub CompressionMethod: Option<String>,
    /// Windows Configuration Manager error code.
    /// 
    /// - `This device is working properly.` (0): Device is working properly.
    /// - `This device is not configured correctly.` (1): Device is not configured correctly.
    /// - `Windows cannot load the driver for this device.` (2)
    /// - `The driver for this device might be corrupted, or your system may be running low on memory or other resources.` (3): Driver for this device might be corrupted, or the system may be low on memory or other resources.
    /// - `This device is not working properly. One of its drivers or your registry might be corrupted.` (4): Device is not working properly. One of its drivers or the registry might be corrupted.
    /// - `The driver for this device needs a resource that Windows cannot manage.` (5): Driver for the device requires a resource that Windows cannot manage.
    /// - `The boot configuration for this device conflicts with other devices.` (6): Boot configuration for the device conflicts with other devices.
    /// - `Cannot filter. (7)
    /// - `The driver loader for the device is missing.` (8): Driver loader for the device is missing.
    /// - `This device is not working properly because the controlling firmware is reporting the resources for the device incorrectly.` (9): Device is not working properly. The controlling firmware is incorrectly reporting the resources for the device.
    /// - `This device cannot start.` (10): Device cannot start.
    /// - `This device failed.` (11): Device failed.
    /// - `This device cannot find enough free resources that it can use.` (12): Device cannot find enough free resources to use.
    /// - `Windows cannot verify this device's resources.` (13): Windows cannot verify the device's resources.
    /// - `This device cannot work properly until you restart your computer.` (14): Device cannot work properly until the computer is restarted.
    /// - `This device is not working properly because there is probably a re-enumeration problem.` (15): Device is not working properly due to a possible re-enumeration problem.
    /// - `Windows cannot identify all the resources this device uses.` (16): Windows cannot identify all of the resources that the device uses.
    /// - `This device is asking for an unknown resource type.` (17): Device is requesting an unknown resource type.
    /// - `Reinstall the drivers for this device.` (18): Device drivers must be reinstalled.
    /// - `Failure using the VxD loader.` (19)
    /// - `Your registry might be corrupted.` (20): Registry might be corrupted.
    /// - `System failure: Try changing the driver for this device. If that does not work, see your hardware documentation. Windows is removing this device.` (21): System failure. If changing the device driver is ineffective, see the hardware documentation. Windows is removing the device.
    /// - `This device is disabled.` (22): Device is disabled.
    /// - `System failure: Try changing the driver for this device. If that doesn't work, see your hardware documentation.` (23): System failure. If changing the device driver is ineffective, see the hardware documentation.
    /// - `This device is not present, is not working properly, or does not have all its drivers installed.` (24): Device is not present, not working properly, or does not have all of its drivers installed.
    /// - `Windows is still setting up this device.` (25): Windows is still setting up the device.
    /// - `Windows is still setting up this device.` (26): Windows is still setting up the device.
    /// - `This device does not have valid log configuration.` (27): Device does not have valid log configuration.
    /// - `The drivers for this device are not installed.` (28): Device drivers are not installed.
    /// - `This device is disabled because the firmware of the device did not give it the required resources.` (29): Device is disabled. The device firmware did not provide the required resources.
    /// - `This device is using an Interrupt Request (IRQ) resource that another device is using.` (30): Device is using an IRQ resource that another device is using.
    /// - `This device is not working properly because Windows cannot load the drivers required for this device.` (31): Device is not working properly. Windows cannot load the required device drivers.
    pub ConfigManagerErrorCode: Option<u32>,
    /// If `True`, the device is using a user-defined configuration.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation 
    /// of an instance. When used with the other key properties of the class, the property 
    /// allows all instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Default block size, in bytes, for this device.
    pub DefaultBlockSize: Option<u64>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the disk drive with other devices on the system.
    pub DeviceID: Option<String>,
    /// If `True`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information on any 
    /// corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Type of error detection and correction supported by this device.
    pub ErrorMethodology: Option<String>,
    /// Revision for the disk drive firmware that is assigned by the manufacturer.
    pub FirmwareRevision: Option<String>,
    /// Physical drive number of the given drive. This property is filled by the `STORAGE_DEVICE_NUMBER` 
    /// structure returned from the `IOCTL_STORAGE_GET_DEVICE_NUMBER` control code. A value of 
    /// 0xffffffff indicates that the given drive does not map to a physical drive.
    /// 
    /// Example: 1
    pub Index: Option<u32>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Interface type of physical disk drive.
    /// 
    /// The values are:
    /// - SCSI
    /// - HDC
    /// - IDE
    /// - USB
    /// - 1394
    pub InterfaceType: Option<String>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Name of the disk drive manufacturer.
    /// 
    /// Example: "Seagate"
    pub Manufacturer: Option<String>,
    /// Maximum block size, in bytes, for media accessed by this device.
    pub MaxBlockSize: Option<u64>,
    /// Maximum media size, in kilobytes, of media supported by this device.
    pub MaxMediaSize: Option<u64>,
    /// If `True`, the media for a disk drive is loaded, which means that the device has a 
    /// readable file system and is accessible. For fixed disk drives, this property will 
    /// always be `TRUE`.
    pub MediaLoaded: Option<bool>,
    /// Type of media used or accessed by this device.
    /// 
    /// Possible values are:
    /// - `External hard disk media`
    /// - `Removable media` ("Removable media other than floppy")
    /// - `Fixed hard disk` ("Fixed hard disk media")
    /// - `Unknown` ("Format is unknown")
    pub MediaType: Option<String>,
    /// Minimum block size, in bytes, for media accessed by this device.
    pub MinBlockSize: Option<u64>,
    /// Manufacturer's model number of the disk drive.
    /// 
    /// Example: "ST32171W"
    pub Model: Option<String>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
    pub Name: Option<String>,
    /// If True, the media access device needs cleaning. Whether manual or automatic cleaning 
    /// is possible is indicated in the Capabilities property.
    pub NeedsCleaning: Option<bool>,
    /// Maximum number of media which can be supported or inserted (when the media access 
    /// device supports multiple individual media).
    pub NumberOfMediaSupported: Option<u32>,
    /// Number of partitions on this physical disk drive that are recognized by the operating 
    /// system.
    /// 
    /// Example: 2
    pub Partitions: Option<u32>,
    /// Windows Plug and Play device identifier of the logical device.
    /// 
    /// Example: "*PNP030b"
    pub PNPDeviceID: Option<String>,
    /// Array of the specific power-related capabilities of a logical device.
    /// 
    /// - `Unknown` (0)
    /// - `Not Supported` (1): Power-related capacities are not supported for this device.
    /// - `Disabled` (2)
    /// - `Enabled` (3): The power management features are currently enabled but the exact feature set is unknown or the information is unavailable.
    /// - `Power Saving Modes Entered Automatically` (4): The device can change its power state based on usage or other criteria.
    /// - `Power State Settable` (5): The `SetPowerState` method is supported. This method is found on the parent CIM_LogicalDevice class and can be implemented. For more information, see Designing Managed Object Format (MOF) Classes.
    /// - `Power Cycling Supported` (6): The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle).
    /// - `Timed Power On Supported` (7): Timed Power-On Supported. The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle) and Time set to a specific date and time, or interval, for power-on.
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /// If `True`, the device can be power-managed (can be put into suspend mode, and so on). 
    /// The property does not indicate that power management features are currently enabled, 
    /// only that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// SCSI bus number of the disk drive.
    /// 
    /// Example: 0
    pub SCSIBus: Option<u32>,
    /// SCSI logical unit number (LUN) of the disk drive.
    /// 
    /// Example: 0
    pub SCSILogicalUnit: Option<u16>,
    /// SCSI port number of the disk drive.
    /// 
    /// Example: 0
    pub SCSIPort: Option<u16>,
    /// SCSI identifier number of the disk drive.
    /// 
    /// Example: 0
    pub SCSITargetId: Option<u16>,
    /// Number of sectors in each track for this physical disk drive.
    /// 
    /// Example: 63
    pub SectorsPerTrack: Option<u32>,
    /// Number allocated by the manufacturer to identify the physical media.
    /// 
    /// Example: WD-WM3493798728
    pub SerialNumber: Option<String>,
    /// Disk identification. This property can be used to identify a shared resource.
    pub Signature: Option<u32>,
    /// Size of the disk drive. It is calculated by multiplying the total number of cylinders, 
    /// tracks in each cylinder, sectors in each track, and bytes in each sector.
    pub Size: Option<u64>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. 
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a 
    /// SMART-enabled hard disk drive, may be functioning properly but predicting a failure in the 
    /// near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service". 
    /// The latter, "Service", could apply during mirror-resilvering of a disk, reload of a user 
    /// permissions list, or other administrative work. Not all such work is online, yet the managed 
    /// element is neither "OK" nor in one of the other states.
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
    /// State of the logical device. If this property does not apply to the logical device, the 
    /// value 5 (Not Applicable) should be used.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Enabled` (3)
    /// - `Disabled` (4)
    /// - `Not Applicable` (5)
    pub StatusInfo: Option<u16>,
    /// Value of the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// Total number of cylinders on the physical disk drive. Note: the value for this property 
    /// is obtained through extended functions of BIOS interrupt 13h. The value may be inaccurate 
    /// if the drive uses a translation scheme to support high-capacity disk sizes. Consult the 
    /// manufacturer for accurate drive specifications.
    /// 
    /// Example: 657
    pub TotalCylinders: Option<u64>,
    /// Total number of heads on the disk drive. Note: the value for this property is obtained 
    /// through extended functions of BIOS interrupt 13h. The value may be inaccurate if the 
    /// drive uses a translation scheme to support high-capacity disk sizes. Consult the 
    /// manufacturer for accurate drive specifications.
    pub TotalHeads: Option<u32>,
    /// Total number of sectors on the physical disk drive. Note: the value for this property 
    /// is obtained through extended functions of BIOS interrupt 13h. The value may be 
    /// inaccurate if the drive uses a translation scheme to support high-capacity disk sizes. 
    /// Consult the manufacturer for accurate drive specifications.
    /// 
    /// Example: 2649024
    pub TotalSectors: Option<u64>,
    /// Total number of tracks on the physical disk drive. Note: the value for this property 
    /// is obtained through extended functions of BIOS interrupt 13h. The value may be 
    /// inaccurate if the drive uses a translation scheme to support high-capacity disk sizes. 
    /// Consult the manufacturer for accurate drive specifications.
    /// 
    /// Example: 42048
    pub TotalTracks: Option<u64>,
    /// Number of tracks in each cylinder on the physical disk drive. Note: the value for this 
    /// property is obtained through extended functions of BIOS interrupt 13h. The value may be 
    /// inaccurate if the drive uses a translation scheme to support high-capacity disk sizes. 
    /// Consult the manufacturer for accurate drive specifications.
    /// 
    /// Example: 64
    pub TracksPerCylinder: Option<u32>,
}
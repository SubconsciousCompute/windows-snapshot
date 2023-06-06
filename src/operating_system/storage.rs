//! The Users subcategory groups classes that represent storage information.
//! 
//! | Class                                                                   | Description                                                                                                                                  |
//! |-------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_ShadowBy**](/previous-versions/windows/desktop/vsswmi/win32-shadowby)                               | Association class<br/> Represents the association between a shadow copy and the provider that creates the shadow copy.<br/>      |
//! | [**Win32\_ShadowContext**](/previous-versions/windows/desktop/vsswmi/win32-shadowcontext)                     | Association class<br/> Specifies how a shadow copy is to be created, queried, or deleted.<br/>                                   |
//! | [**Win32\_ShadowCopy**](/previous-versions/windows/desktop/legacy/aa394428(v=vs.85))                           | Instance class<br/> Represents a duplicate copy of the original volume at a previous time.<br/>                                  |
//! | [**Win32\_ShadowDiffVolumeSupport**](/previous-versions/windows/desktop/vsswmi/win32-shadowdiffvolumesupport) | Association class<br/> Represents an association between a shadow copy provider and a storage volume.<br/>                       |
//! | [**Win32\_ShadowFor**](/previous-versions/windows/desktop/vsswmi/win32-shadowfor)                             | Association class<br/> Represents an association between a shadow copy and the volume for which the shadow copy is created.<br/> |
//! | [**Win32\_ShadowOn**](/previous-versions/windows/desktop/vsswmi/win32-shadowon)                               | Association class<br/> Represents an association between a shadow copy and where the differential data is written.<br/>          |
//! | [**Win32\_ShadowProvider**](/previous-versions/windows/desktop/vsswmi/win32-shadowprovider)                   | Association class<br/> Represents a component that creates and represents volume shadow copies.<br/>                             |
//! | [**Win32\_ShadowStorage**](/previous-versions/windows/desktop/legacy/aa394433(v=vs.85))                     | Association class<br/> Represents an association between a shadow copy and where the differential data is written.<br/>          |
//! | [**Win32\_ShadowVolumeSupport**](/previous-versions/windows/desktop/vsswmi/win32-shadowvolumesupport)         | Association class<br/> Represents an association between a shadow copy provider with a supported volume.<br/>                    |
//! | [**Win32\_Volume**](/previous-versions/windows/desktop/legacy/aa394515(v=vs.85))                                   | Instance class<br/> Represents an area of storage on a hard disk.<br/>                                                           |
//! | [**Win32\_VolumeUserQuota**](/previous-versions/windows/desktop/vdswmi/win32-volumeuserquota)                 | Association class<br/> Represents a volume to the per volume quota settings.<br/>                                                |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows `ShadowCopys`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShadowCopys {
    /// Represents sequence of `ShadowCopys`
    pub shadow_copys: Vec<Win32_ShadowCopy>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(ShadowCopys, shadow_copys);

/// Represents the state of Windows `Volumes`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Volumes {
    /// Represents sequence of `Volumes`
    pub volumes: Vec<Win32_Volume>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Volumes, volumes);

/// Represents the state of Windows `ShadowContexts`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShadowContexts {
    /// Represents sequence of `ShadowContexts`
    pub shadow_contexts: Vec<Win32_ShadowContext>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(ShadowContexts, shadow_contexts);

/// Represents the state of Windows `ShadowProviders`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShadowProviders {
    /// Represents sequence of `ShadowProviders`
    pub shadow_providers: Vec<Win32_ShadowProvider>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(ShadowProviders, shadow_providers);

/// The `Win32_ShadowCopy` class is a storage extent that represents a duplicate copy of the 
/// original volume at a previous time.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394428(v=vs.85)>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ShadowCopy {
    /// Short textual description of the object. 
    pub Caption: Option<String>,
    /// Textual description of the object. 
    pub Description: Option<String>,
    /// Unique identifier for a shadow copy on the system.
    pub ID: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate that 
    /// the object is installed. 
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a 
    /// key property. 
    pub Name: Option<String>,
    /// Unique identifier for a shadow copy set to which the shadow belongs.
    pub SetID: Option<String>,
    /// Unique identifier for a shadow provider that creates a shadow.
    pub ProviderID: Option<String>,
    /// Current status of the object. 
    /// 
    /// Values include the following:
    /// - "OK"
    /// - "Error"
    /// - "Degraded"
    /// - "Unknown"
    /// - "Pred Fail"
    /// - "Starting"
    /// - "Stopping"
    /// - "Service"
    /// - "Stressed"
    /// - "NonRecover"
    /// - "No Contact"
    /// - "Lost Comm"
    pub Status: Option<String>,
    /// Number of shadow copies in a shadow copy set to which a shadow copy belongs.
    pub Count: Option<u32>,
    /// Windows object manager name for an underlying storage device that supports the 
    /// original volume.
    pub DeviceObject: Option<String>,
    /// Name of the original volume for which a shadow copy is made.
    pub VolumeName: Option<String>,
    /// Name of the computer that hosts the original volume.
    pub OriginatingMachine: Option<String>,
    /// Name of the computer that services the shadow copy.
    pub ServiceMachine: Option<String>,
    /// File system name of a shadow copy when it is exposed. The `ExposedName` property might 
    /// contain a drive letter or mount point. The `ExposedName` property is `NULL` when a shadow 
    /// copy is hidden or otherwise not exposed.
    pub ExposedName: Option<String>,
    /// Current state of a shadow copy.
    /// 
    /// Value: Meaning
    /// - 0: Unknown
    /// - 1: Preparing
    /// - 2: ProcessingPrepare
    /// - 3: Prepared
    /// - 4: ProcessingPrecommit
    /// - 5: Precommitted
    /// - 6: ProcessingCommit
    /// - 7: Committed
    /// - 8: ProcessingPostcommit
    /// - 9: Created
    /// - 10: Aborted
    /// - 11: Deleted
    /// - 12: Count
    pub State: Option<u32>,
    /// If `true`, the shadow copy is persistent across reboots.
    pub Persistent: Option<bool>,
    /// If `true`, the shadow copy is created by the Windows Previous Versions component.
    pub ClientAccessible: Option<bool>,
    /// If `true`, the shadow copy is retained after the requestor process ends. If `false`, the shadow 
    /// copy is automatically deleted when the requestor process ends.
    pub NoAutoRelease: Option<bool>,
    /// If `true`, the shadow copy is created with the involvement of the shadow copy writer components.
    pub NoWriters: Option<bool>,
    /// If `true`, the shadow copy can be surfaced on another computer. If `false`, and the volumes are 
    /// surfaced locally, it may not be possible to surface them later on a different computer.
    pub Transportable: Option<bool>,
    /// If `true`, the shadow copy is not currently in the device namespace of a local computer.
    pub NotSurfaced: Option<bool>,
    /// If `true`, the shadow copy is created by a hardware shadow copy provider.
    pub HardwareAssisted: Option<bool>,
    /// If `true`, the shadow copy is created by a differential shadow copy provider. The provider 
    /// can be implemented in hardware or software.
    pub Differential: Option<bool>,
    /// If `true`, the shadow copy is created by a split mirror shadow copy provider. The provider 
    /// can be implemented in hardware or software.
    pub Plex: Option<bool>,
    /// If `true`, the shadow copy is imported to a computer by using the `Import` method and is not 
    /// created by using the `Create` method.
    pub Imported: Option<bool>,
    /// If `true`, the shadow copy is exposed on a remote computer with a network share. If `ExposedRemotely` 
    /// and `ExposedLocally` are not set, the shadow copy is hidden.
    pub ExposedRemotely: Option<bool>,
    /// If `true`, the shadow copy is exposed on the local computer with a drive letter or mount point. 
    /// If `ExposedLocally` and `ExposedRemotely` are not set, the shadow copy is hidden.
    pub ExposedLocally: Option<bool>,
}

/// The `Win32_Volume` class represents an area of storage on a hard disk. The class returns local volumes 
/// that are formatted, unformatted, mounted, or offline. A volume is formatted by using a file system, 
/// such as FAT or NTFS, and might have a drive letter assigned to it. One hard disk can have multiple 
/// volumes, and volumes can span multiple physical disks. The `Win32_Volume` class does not support disk 
/// drive management.
///
/// Windows XP and earlier: This class is not available.
/// 
/// Note: This class has been repeated in File System as well. 
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394515(v=vs.85)>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Volume {
    /// Describes whether the media is readable.
    /// 
    /// This can be one of the following values.
    /// 
    /// Value: Meaning
    /// - 0 (0x0): Unknown media.
    /// - 1 (0x1): The media is readable.
    /// - 2 (0x2): The media is writable.
    /// - 3 (0x3): The media is readable and writable.
    /// - 4 (0x4): "Write once" media.
    pub Access: Option<u16>,
    /// If true, the volume is mounted to the file system automatically when the first I/O is issued. 
    /// If false, the volume is not mounted until explicitly mounted by using the `Mount` method, or 
    /// by adding a drive letter or mount point.
    pub Automount: Option<bool>,
    /// Describes the availability and status of the device. Inherited from CIM_LogicalDevice. 
    /// 
    /// This can be one of the following values.
    /// 
    /// Value: Meaning
    /// - 1 (0x1): Other
    /// - 2 (0x2): Unknown
    /// - 3 (0x3): Running or Full Power
    /// - 4 (0x4): Warning
    /// - 5 (0x5): In Test
    /// - 6 (0x6): Not Applicable
    /// - 7 (0x7): Power Off
    /// - 8 (0x8): Offline
    /// - 9 (0x9): Off Duty
    /// - 10 (0xA): Degraded
    /// - 11 (0xB): Not Installed
    /// - 12 (0xC): Install Error
    /// - 13 (0xD): Power Save - Unknown [The device is known to be in a power save mode, but its exact status is unknown]
    /// - 14 (0xE): Power Save - Low Power Mode [The device is in a power save state, but still functioning, and may exhibit degraded performance]
    /// - 15 (0xF):	Power Save - Standby [The device is not functioning, but could be brought to full power quickly]
    /// - 16 (0x10): Power Cycle
    /// - 17 (0x11): Power Save - Warning [The device is in a warning state, but also in a power save mode]
    /// - 18 (0x12): Paused
    /// - 19 (0x13): Not Ready
    /// - 20 (0x14): Not Configured
    /// - 21 (0x15): Quiesced
    pub Availability: Option<u16>,
    /// Size in bytes of the blocks in this storage extent. If there is a variable block size, then 
    /// the maximum block size in bytes is specified. If the block size is unknown or if a block 
    /// concept is not valid (for example, for Aggregate Extents, Memory, or LogicalDisks), the value 
    /// is 1 (one). 
    pub BlockSize: Option<u64>,
    /// Size of the volume in bytes.
    pub Capacity: Option<u64>,
    /// A short description of the area of storage. 
    pub Caption: Option<String>,
    /// If `true`, the volume exists as one compressed entity, such as a DoubleSpace volume. If file-based 
    /// compression is supported, such as the NTFS file system, this property is `false`.
    pub Compressed: Option<bool>,
    /// Indicates the Win32 Configuration Manager error code. This can be one of the following values.
    /// 
    /// Value: Meaning
    /// - 0 (0x0): This device is working properly.
    /// - 1 (0x1): This device is not configured correctly.
    /// - 2 (0x2): Windows cannot load the driver for this device.
    /// - 3 (0x3): The driver for this device might be corrupted, or your system may be running low on memory or other resources.
    /// - 4 (0x4): This device is not working properly. One of its drivers or your registry might be corrupted.
    /// - 5 (0x5): The driver for this device needs a resource that Windows cannot manage.
    /// - 6 (0x6): The boot configuration for this device conflicts with other devices.
    /// - 7 (0x7): Cannot filter.
    /// - 8 (0x8): The driver loader for the device is missing.
    /// - 9 (0x9): This device is not working properly because the controlling firmware is reporting the resources for the device incorrectly.
    /// - 10 (0xA): This device cannot start.
    /// - 11 (0xB): This device failed.
    /// - 12 (0xC): This device cannot find enough free resources that it can use.
    /// - 13 (0xD): Windows cannot verify this device's resources.
    /// - 14 (0xE): This device cannot work properly until you restart your computer.
    /// - 15 (0xF): This device is not working properly because there is probably a re-enumeration problem.
    /// - 16 (0x10): Windows cannot identify all the resources this device uses.
    /// - 17 (0x11): This device is asking for an unknown resource type.
    /// - 18 (0x12): Reinstall the drivers for this device.
    /// - 19 (0x13): Failure using the VxD loader.
    /// - 20 (0x14): Your registry might be corrupted.
    /// - 21 (0x15): System failure. Try changing the driver for this device. If that does not work, see your hardware documentation. Windows is removing this device.
    /// - 22 (0x16): This device is disabled.
    /// - 23 (0x17): System failure. Try changing the driver for this device. If that does not work, see your hardware documentation.
    /// - 24 (0x18): This device is not present, is not working properly, or does not have all of its drivers installed.
    /// - 25 (0x19): Windows is still setting up this device.
    /// - 26 (0x1A): Windows is still setting up this device.
    /// - 27 (0x1B): This device does not have a valid log configuration.
    /// - 28 (0x1C): The drivers for this device are not installed.
    /// - 29 (0x1D): This device is disabled because the firmware of the device did not give it the required resources.
    /// - 30 (0x1E): This device is using an Interrupt Request resource that another device is using.
    /// - 31 (0x1F): This device is not working properly because Windows cannot load the drivers required for this device.
    pub ConfigManagerErrorCode: Option<u32>,
    /// If `True`, the device is using a user-defined configuration. Otherwise, `False`.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Indicates the name of the class or the subclass used in the creation of an instance of this class. 
    /// When used with the other key properties of this class, this property allows all instances of this 
    /// class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// A description of the object. 
    pub Description: Option<String>,
    /// Unique identifier for the volume on this system.
    pub DeviceID: Option<String>,
    /// If `true`, the Chkdsk method is automatically run by the system at the next restart.
    pub DirtyBitSet: Option<bool>,
    /// Drive letter assigned to a volume. This property is `NULL` for volumes without drive letters.
    pub DriveLetter: Option<String>,
    /// Numeric value that corresponds to the type of disk drive that this logical disk represents.
    /// 
    /// The values are:
    /// 
    /// Value: Meaning
    /// - 0 (0x0): Unknown
    /// - 1 (0x1): No Root Directory
    /// - 2 (0x2): Removable Disk
    /// - 3 (0x3): Local Disk
    /// - 4 (0x4): Network Drive
    /// - 5 (0x5): Compact Disk
    /// - 6 (0x6): RAM Disk
    pub DriveType: Option<u32>,
    /// If `True`, the error reported in `LastErrorCode` is now cleared. 
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information on any corrective 
    /// actions that may be taken. 
    pub ErrorDescription: Option<String>,
    /// Type of error detection and correction supported by this storage extent. 
    pub ErrorMethodology: Option<String>,
    /// File system on the logical disk.
    /// 
    /// Example: NTFS
    pub FileSystem: Option<String>,
    /// Space, in bytes, available on the logical disk. 
    pub FreeSpace: Option<u64>,
    /// If `true`, context indexing is enabled.
    pub IndexingEnabled: Option<bool>,
    /// Date and time the object was installed. This property does not require a value to indicate that 
    /// the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Volume name of the logical disk. This property is `null` for volumes without a label. For FAT 
    /// and FAT32 systems, the maximum length is 11 characters. For NTFS file systems, the maximum 
    /// length is 32 characters.
    pub Label: Option<String>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Maximum length, in characters, of a filename component supported by a Windows drive. A filename 
    /// component is the portion of a filename between backslashes. This value can be used to indicate 
    /// that long names are supported by the file system. For example, for a FAT file system that supports 
    /// long names, the property stores the value 255—not the previous 8.3 indicator. Long names can be 
    /// supported on systems that use the NTFS file system.
    /// 
    /// Example: 255
    pub MaximumFileNameLength: Option<u32>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a key 
    /// property.
    pub Name: Option<String>,
    /// Total number of consecutive blocks, each block the size of the value contained in the `BlockSize` 
    /// property, which form this storage extent. Total size of the storage extent can be calculated by 
    /// multiplying the value of the `BlockSize` property by the value of this property. If the value of 
    /// `BlockSize` is 1, this property is the total size of the storage extent.
    pub NumberOfBlocks: Option<u64>,
    /// Indicates the Win32 Plug and Play device ID of the logical device. Example: *PNP030b.
    pub PNPDeviceID: Option<String>,
    /// Indicates the specific power-related capabilities of the logical device. This can be one of the 
    /// following values.
    /// 
    /// Value: Meaning
    /// - 0 (0x0): Unknown
    /// - 1 (0x1): Not Supported
    /// - 2 (0x2): Disabled
    /// - 3 (0x3): Enabled [The power management features are currently enabled but the exact feature set is unknown or the information is unavailable]
    /// - 4 (0x4): Power Saving Modes Entered Automatically [The device can change its power state based on usage or other criteria]
    /// - 5 (0x5): Power State Settable [The SetPowerState method is supported. This method is found on the parent CIM_LogicalDevice class and can be implemented]
    /// - 6 (0x6): Power Cycling Supported [The SetPowerState method can be invoked with the PowerState parameter set to 5 (Power Cycle)]
    /// - 7 (0x7): Timed Power-On Supported [The SetPowerState method can be invoked with the PowerState parameter set to 5 (Power Cycle) and Time set to a specific date and time, or interval, for power-on]
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /// `True`, if the device can be power managed (put into a power save state), otherwise `False`. This 
    /// boolean does not indicate that power management features are currently enabled, or if enabled, 
    /// what features are supported. Refer to the PowerManagementCapabilities array for this information. 
    /// If this boolean is false, the integer value 1, for the string, "Not Supported", should be the only 
    /// entry in the PowerManagementCapabilities array. 
    pub PowerManagementSupported: Option<bool>,
    /// Describes the media and its use.
    pub Purpose: Option<String>,
    /// If `true`, quota management is enabled for this volume.
    pub QuotasEnabled: Option<bool>,
    /// If `true`, quota management was used but is disabled. Incomplete refers to the information left in 
    /// the file system after quota management is disabled.
    pub QuotasIncomplete: Option<bool>,
    /// If `true`, the file system is in the process of compiling information and setting the disk up for 
    /// quota management.
    pub QuotasRebuilding: Option<bool>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. 
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a SMART-enabled 
    /// hard disk drive, may be functioning properly but predicting a failure in the near future). 
    /// Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service". The latter, "Service", 
    /// could apply during mirror-resilvering of a disk, reload of a user permissions list, or other 
    /// administrative work. Not all such work is online, yet the managed element is neither "OK" nor in 
    /// one of the other states. 
    /// 
    /// The values are:
    /// - "OK"
    /// - "Error"
    /// - "Degraded"
    /// - "Unknown"
    /// - "Pred Fail"
    /// - "Starting"
    /// - "Stopping"
    /// - "Service"
    /// - "Stressed"
    /// - "NonRecover"
    /// - "No Contact"
    /// - "Lost Comm"
    pub Status: Option<String>,
    /// Indicates the state of the logical device. This can be one of the following values.
    /// 
    /// Value	Meaning
    /// - 1 (0x1): Other
    /// - 2 (0x2): Unknown
    /// - 3 (0x3): Enabled
    /// - 4 (0x4): Disabled
    /// - 5 (0x5): Not Applicable
    pub StatusInfo: Option<u16>,
    /// Indicates the system's creation class name.
    pub SystemCreationClassName: Option<String>,
    /// Indicates the system's name.
    pub SystemName: Option<String>,
    /// Serial number of the volume.
    /// 
    /// Example: A8C3D032
    pub SerialNumber: Option<u32>,
    /// If `true`, the volume supports disk quotas.
    pub SupportsDiskQuotas: Option<bool>,
    /// If `True`, the logical disk partition supports file-based compression, such as is the case 
    /// with the NTFS file system. This property is `False` when the `Compressed` property is `True`.
    pub SupportsFileBasedCompression: Option<bool>,
}

/// The `Win32_ShadowContext` class specifies how a shadow copy is to be created, queried, or deleted, 
/// and the degree of writer involvement.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/vsswmi/win32-shadowcontext>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ShadowContext {
    /// Name of the context.
    pub Name: Option<String>,
    /// If `true`, the shadow copy persists across restarts.
    pub Persistent: Option<bool>,
    /// If `true`, the shadow copy is created by the Windows Previous Versions component.
    pub ClientAccessible: Option<bool>,
    /// If `true`, the shadow copy is retained after the requestor process ends. If `false`, the shadow 
    /// copy is automatically deleted when the shadow copy requestor process ends.
    pub NoAutoRelease: Option<bool>,
    /// If `true`, the shadow copy is created without involvement of shadow copy writer components.
    pub NoWriters: Option<bool>,
    /// If `true`, the shadow copy can be surfaced on another computer. If `false`, and the volumes are 
    /// surfaced locally, it may not be possible to surface them later on a different computer.
    pub Transportable: Option<bool>,
    /// If `true`, the shadow copy is not currently in the device namespace of the local computer.
    pub NotSurfaced: Option<bool>,
    /// If `true`, the shadow copy is created by a hardware shadow copy provider.
    pub HardwareAssisted: Option<bool>,
    /// If `true`, the shadow copy is created by a differential shadow copy provider. The provider can be 
    /// implemented in hardware or software.
    pub Differential: Option<bool>,
    /// If `true`, the shadow copy is created by a split-mirror shadow copy provider.
    pub Plex: Option<bool>,
    /// If `true`, the shadow copy is imported to a computer by using the `Import` method and is not created 
    /// by using the `Create` method.
    pub Imported: Option<bool>,
    /// If `true`, the shadow copy is exposed on a remote computer with a network share. If both 
    /// `ExposedRemotely` and `ExposedLocally` are `false`, the shadow copy is hidden.
    pub ExposedRemotely: Option<bool>,
    /// If `true`, the shadow copy is exposed on a remote computer with a network share. If both 
    /// `ExposedLocally` and `ExposedRemotely` are `false`, the shadow copy is hidden.
    pub ExposedLocally: Option<bool>,
}

/// Typically, the `Win32_ShadowProvider` class represents a component that is a combination of user-mode 
/// and kernel or firmware implementation, that creates and represents volume shadow copies.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/vsswmi/win32-shadowprovider>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ShadowProvider {
    /// Uniquely identifies the shadow provider on a system.
    pub ID: Option<String>,
    /// Descriptive name of a provider.
    pub Name: Option<String>,
    /// Common Object Model (COM) class ID registered for a shadow provider.
    pub CLSID: Option<String>,
    /// Specifies the class to which a shadow provider belongs.
    /// 
    /// Possible values:
    /// 
    /// Value: Meaning
    /// - 0: Unknown
    /// - 1: System
    /// - 2: Software
    /// - 3: Hardware
    pub Type: Option<u32>,
    /// Text representation of a shadow provider version.
    pub Version: Option<String>,
    /// Numeric representation of a shadow provider version.
    pub VersionID: Option<String>,
}
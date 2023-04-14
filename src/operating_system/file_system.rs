//! ## File System
//!
//! The File System subcategory groups classes that represent the way a hard disk is logically
//! arranged. This includes the type of file system used, the directory structure, and way the disk
//! is partitioned.
//!
//! | Class                                                                               | Description                                                                                                                                                                          |
//! |-------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_CIMLogicalDeviceCIMDataFile**](win32-cimlogicaldevicecimdatafile)     | Association class<br/> Relates logical devices and data files, indicating the driver files used by the device.<br/>                                                      |
//! | [**Win32\_Directory**](win32-directory)                                         | Instance class<br/> Represents a directory entry on a computer system running Windows.<br/>                                                                              |
//! | [**Win32\_DirectorySpecification**](/previous-versions/windows/desktop/msiprov/win32-directoryspecification)               | Instance class<br/> Represents the directory layout for the product.<br/>                                                                                                |
//! | [**Win32\_DiskDriveToDiskPartition**](win32-diskdrivetodiskpartition)           | Association class<br/> Relates a disk drive and a partition existing on it.<br/>                                                                                         |
//! | [**Win32\_DiskPartition**](win32-diskpartition)                                 | Instance class<br/> Represents the capabilities and management capacity of a partitioned area of a physical disk on a computer system running Windows.<br/>              |
//! | [**Win32\_DiskQuota**](/previous-versions/windows/desktop/wmipdskq/win32-diskquota)                                    | Association class<br/> Tracks disk space usage for NTFS file system volumes.<br/>                                                                                        |
//! | [**Win32\_LogicalDisk**](win32-logicaldisk)                                     | Represents a data source that resolves to an actual local storage device on a computer system running Windows.<br/>                                                            |
//! | [**Win32\_LogicalDiskRootDirectory**](win32-logicaldiskrootdirectory)           | Association class<br/> Relates a logical disk and its directory structure.<br/>                                                                                          |
//! | [**Win32\_LogicalDiskToPartition**](win32-logicaldisktopartition)               | Association class<br/> Relates a logical disk drive and the disk partition it resides on.<br/>                                                                           |
//! | [**Win32\_MappedLogicalDisk**](win32-mappedlogicaldisk)                         | Represents network storage devices that are mapped as logical disks on the computer system running Windows.<br/>                                                               |
//! | [**Win32\_OperatingSystemAutochkSetting**](/previous-versions//aa394240(v=vs.85)) | Association class<br/> Represents the association between a [**CIM\_ManagedSystemElement**](cim-managedsystemelement) instance and the settings defined for it.<br/> |
//! | [**Win32\_QuotaSetting**](/previous-versions/windows/desktop/wmipdskq/win32-quotasetting)                              | Instance class<br/> Contains setting information for disk quotas on a volume.<br/>                                                                                       |
//! | [**Win32\_ShortcutFile**](win32-shortcutfile)                                   | Instance class<br/> Represents files that are shortcuts to other files, directories, and commands.<br/>                                                                  |
//! | [**Win32\_SubDirectory**](win32-subdirectory)                                   | Association class<br/> Relates a directory (folder) and one of its subdirectories (subfolders).<br/>                                                                     |
//! | [**Win32\_SystemPartitions**](win32-systempartitions)                           | Association class<br/> Relates a computer system and a disk partition on that system.<br/>                                                                               |
//! | [**Win32\_Volume**](/previous-versions/windows/desktop/legacy/aa394515(v=vs.85))                                               | Instance class<br/> Represents an area of storage on a hard disk.<br/>                                                                                                   |
//! | [**Win32\_VolumeQuota**](/previous-versions/windows/desktop/vdswmi/win32-volumequota)                                     | Association class<br/> Relates a volume to the per volume quota settings.<br/>                                                                                           |
//! | [**Win32\_VolumeQuotaSetting**](/previous-versions/windows/desktop/wmipdskq/win32-volumequotasetting)                  | Association class<br/> Relates disk quota settings with a specific disk volume.<br/>                                                                                     |
//! | [**Win32\_VolumeUserQuota**](/previous-versions/windows/desktop/vdswmi/win32-volumeuserquota)                             | Association class<br/> Relates per user quotas to quota-enabled volumes.<br/>

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows Directories
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Directories {
    /// Sequence of windows directories
    pub directories: Vec<Win32_Directory>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Directories, directories);

/// Represents the state of Windows Directory Specification
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DirectorySpecifications {
    /// Sequence of windows directories specifications
    pub directory_specifications: Vec<Win32_DirectorySpecification>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(DirectorySpecifications, directory_specifications);

/// Represents the state of Windows Disk Partitions
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiskPartitions {
    /// Sequence of windows disk partitions
    pub disk_partitions: Vec<Win32_DiskPartition>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

//update!(DiskPartitions, disk_partitions);
impl DiskPartitions {
    ///   Update fields synchronously
    pub fn update(&mut self) {
        let com_con = unsafe { COMLibrary::assume_initialized() };

        let wmi_con = WMIConnection::new(com_con).unwrap();

        self.last_updated = SystemTime::now();
        self.disk_partitions = wmi_con.query().unwrap();
    }

    ///   Update fields asynchronously
    pub async fn async_update(&mut self) {
        let com_con = unsafe { COMLibrary::assume_initialized() };

        let wmi_con = WMIConnection::new(com_con).unwrap();

        self.last_updated = SystemTime::now();
        self.disk_partitions = wmi_con.async_query().await.unwrap();
    }
}
impl Default for DiskPartitions {
    ///   `last_updated`  defaults to the the current  `SystemTime`
    fn default() -> Self {
        DiskPartitions {
            disk_partitions: Default::default(),
            last_updated: SystemTime::now(),
        }
    }
}

/// The `Win32_Directory` WMI class represents a directory entry on a computer system running Windows.
/// A directory is a type of file that logically groups data files and provides path information for
/// the grouped files. Example: C:\TEMP. Win32_Directory does not include directories of network
/// drives.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-directory>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Directory {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object
    /// is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// The Name property is a string representing the inherited name that serves as a key of a
    /// logical file instance within a file system. Full path names should be provided. Example:
    /// C:\Windows\system\win.ini
    pub Name: Option<String>,
    /// String that indicates the current status of the object.
    ///
    /// Values include the following:
    ///
    /// - OK ("OK")
    /// - Error ("Error")
    /// - Degraded ("Degraded")
    /// - Unknown ("Unknown")
    /// - Pred Fail ("Pred Fail")
    /// - Starting ("Starting")
    /// - Stopping ("Stopping")
    /// - Service ("Service")
    /// - Stressed ("Stressed")
    /// - NonRecover ("NonRecover")
    /// - No Contact ("No Contact")
    /// - Lost Comm ("Lost Comm")
    pub Status: Option<String>,
    /// Bitmask that represents the access rights required to access or perform specific operations
    /// on the directory. For bit values, see File and Directory Access Rights Constants.
    ///
    /// Note: On FAT volumes, the FULL_ACCESS value is returned instead, which indicates no security
    /// has been set on the object.
    ///
    /// - FILE_READ_DATA (file) or FILE_LIST_DIRECTORY (directory) (1): Grants the right to read data from the file. For a directory, this value grants the right to list the contents of the directory.
    /// - FILE_WRITE_DATA (file) or FILE_ADD_FILE (directory) (2): Grants the right to write data to the file. For a directory, this value grants the right to create a file in the directory.
    /// - FILE_APPEND_DATA (file) or FILE_ADD_SUBDIRECTORY (4): Grants the right to append data to the file. For a directory, this value grants the right to create a subdirectory.
    /// - FILE_READ_EA (8): Grants the right to read extended attributes.
    /// - FILE_WRITE_EA (16): Grants the right to write extended attributes.
    /// - FILE_EXECUTE (file) or FILE_TRAVERSE (directory) (32): Grants the right to execute a file. For a directory, the directory can be traversed.
    /// - FILE_DELETE_CHILD (directory) (64): Grants the right to delete a directory and all of the files it contains (its children), even if the files are read-only.
    /// - FILE_READ_ATTRIBUTES (128): Grants the right to read file attributes.
    /// - FILE_WRITE_ATTRIBUTES (256): Grants the right to change file attributes.
    /// - DELETE (65536): Grants delete access.
    /// - READ_CONTROL (131072): Grants read access to the security descriptor and owner.
    /// - WRITE_DAC (262144): Grants write access to the discretionary ACL.
    /// - WRITE_OWNER (524288): Assigns the write owner.
    /// - SYNCHRONIZE (1048576): Synchronizes access and allows a process to wait for an object to enter the signaled state.
    /// - ACCESS_SYSTEM_SECURITY (18809343): Controls the ability to get or set the SACL in an object's security descriptor.
    pub AccessMask: Option<u32>,
    /// Indicates whether the archive bit on the folder has been set. The archive bit is used by
    /// backup programs to identify files that should be backed up. If True, the file should be
    /// archived.
    pub Archive: Option<bool>,
    /// Indicates whether or not the folder has been compressed. WMI recognizes folders compressed
    /// using WMI itself or using the graphical user interface; it does not, however, recognize
    /// .ZIP files as being compressed. If True, the file is compressed.
    pub Compressed: Option<bool>,
    /// Algorithm or tool (usually a method) used to compress the logical file. If it is not
    /// possible (or not desired) to describe the compression scheme (perhaps because it is not
    /// known), use the following words: "Unknown" to represent that it is not known whether the
    /// logical file is compressed; "Compressed" to represent that the file is compressed, but
    /// either its compression scheme is not known or not disclosed; and "Not Compressed" to
    /// represent that the logical file is not compressed.
    pub CompressionMethod: Option<String>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of
    /// an instance. When used with the other key properties of the class, this property allows all
    /// instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Date that the file system object was created.
    pub CreationDate: Option<WMIDateTime>,
    /// Creation class name of the scoping computer system.
    pub CSCreationClassName: Option<String>,
    /// Name of the computer where the file system object is stored.
    pub CSName: Option<String>,
    /// Drive letter of the drive (including colon) where the file system object is stored.
    ///
    /// Example: "c:"
    pub Drive: Option<String>,
    /// MS-DOS -compatible name for the folder.
    ///
    /// Example: "c:\progra~1"
    pub EightDotThreeFileName: Option<String>,
    /// Indicates whether or not the folder has been encrypted. If True, the folder is encrypted.
    pub Encrypted: Option<bool>,
    /// Algorithm or tool used to encrypt the logical file. If it is not possible (or not desired)
    /// to describe the encryption scheme (perhaps for security reasons), use the following words:
    /// "Unknown" to represent that it is not known whether the logical file is encrypted;
    /// "Encrypted" to represent that the file is encrypted, but either its encryption scheme is
    /// not known or not disclosed; and "Not Encrypted" to represent that the logical file is not
    /// encrypted.
    pub EncryptionMethod: Option<String>,
    /// File name extension for the file system object, not including the dot (.) that separates
    /// the extension from the file name.
    ///
    /// Examples: "txt", "mof", "mdb"
    pub Extension: Option<String>,
    /// File name (without the dot or extension) of the file.
    ///
    /// Example: "autoexec"
    pub FileName: Option<String>,
    /// Size of the file system object, in bytes. Although folders possess a FileSize property,
    /// the value 0 is always returned. To determine the size of a folder, use the FileSystemObject
    /// or add up the size of all the files stored in the folder.
    pub FileSize: Option<u64>,
    /// For example, an .mdb file is likely to have the file type Microsoft Access Application. An
    /// .asp file likely has the file type HTML Document. Folders are typically reported simply as
    /// Folder.
    pub FileType: Option<String>,
    /// Class of the file system.
    pub FSCreationClassName: Option<String>,
    /// Type of file system (NTFS, FAT, FAT32) installed on the drive where the file or folder is
    /// located.
    pub FSName: Option<String>,
    /// Indicates whether the file system object is hidden. If True, the file is hidden.
    pub Hidden: Option<bool>,
    /// Number of "file opens" that are currently active against the file.
    pub InUseCount: Option<u64>,
    /// Date the file was last accessed.
    pub LastAccessed: Option<WMIDateTime>,
    /// Date the file was last modified.
    pub LastModified: Option<WMIDateTime>,
    /// Path for the file. The path includes the leading and trailing backslashes, but not the drive
    /// letter or the folder name.
    ///
    /// For the folder c:\windows\system32\wbem, the path is \windows\system32\. For the folder
    /// c:\scripts, the path is \.
    pub Path: Option<String>,
    /// Indicates whether you can read items in the folder. If True, the file can be read.
    pub Readable: Option<bool>,
    /// Indicates whether the object is a system file. If True, the file is a system file
    pub System: Option<bool>,
    /// If True, the file can be written.
    pub Writeable: Option<bool>,
}

/// The `Win32_DirectorySpecification` class represents the directory layout for the product.
/// Each instance of the class represents a directory in both the source image and the destination image.
///
/// Directory resolution is performed as follows:
///
/// - Root destination directories:
/// The root directory entries are those with a null Directory_Parent value or a Directory_Parent value identical to the Directory value.
/// The value in the Directory property is interpreted as the name of a property
/// defining the location of the destination directory.
/// If the property is defined, the destination directory is resolved to the property's value.
/// If the property is undefined, the ROOTDRIVE property is used instead to resolve the path.
/// - Root source directories:
/// The value of the DefaultDir column for root entries is interpreted as the name of a property
/// defining the source location of this directory.
/// This property must be defined or an error will occur.
/// - Nonroot destination directories:
/// The Directory value for a nonroot directory is also interpreted as the name of a property
/// defining the location of the destination.
/// If the property is defined, the destination directory is resolved to the property's value.
/// If the property is not defined,
/// the destination directory is resolved to a subdirectory beneath the resolved destination directory for the Directory_Parent entry.
/// The DefaultDir value defines the name of the subdirectory.
/// - Nonroot source directories:
/// The source directory for a nonroot directory is resolved to a subdirectory of the resolved source directory for the Directory_Parent entry.
/// Again, the DefaultDir value defines the name of the subdirectory.
///
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/msiprov/win32-directoryspecification>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_DirectorySpecification {
    /// Short description of the object.
    pub Caption: Option<String>,
    /// Identifier used in conjunction with other keys to uniquely identify the check.
    pub CheckID: Option<String>,
    /// Condition is expected to exist or not exist in the environment.
    /// When TRUE,
    /// the condition is expected to exist (a file is expected to be on a system)
    /// so the Invoke() method is expected to return TRUE.
    pub CheckMode: Option<bool>,
    pub DefaultDir: Option<String>,
    /// Description of the objects.
    pub Description: Option<String>,
    pub Directory: Option<String>,
    /// Name of a directory.
    /// The value supplied by an application provider is actually a default or recommended path name
    /// and can be changed for a particular environment.
    pub DirectoryPath: Option<String>,
    /// Type of directory being described.
    ///
    /// Value: Meaning
    ///
    /// - 1: Product log directory
    /// - 2: Shared base directory
    /// - 3: Shared executable directory
    /// - 4: Shared library directory
    /// - 5: Shared include directory
    /// - 6: System base directory
    /// - 7: System executable directory
    /// - 8: System library directory
    /// - 9: System configuration directory
    /// - 10: System include directory
    /// - 11: System log directory
    /// - 12: Other
    pub DirectoryType: Option<u16>,
    /// Name used to identify this software element.
    pub Name: Option<String>,
    /// Identifier for this software element.
    pub SoftwareElementID: Option<String>,
    /// State of a software element.
    ///
    /// Value: Meaning
    ///
    /// - 1: Disabled
    /// - 2: Installable
    /// - 3: Executable
    /// - 4: Running
    pub SoftwareElementState: Option<u16>,
    /// Target operating system of the owning software element. The possible values for this
    /// property are as follows.
    ///
    /// Value: Meaning
    ///
    /// - 0: Unknown
    /// - 1: Other
    /// - 2: MACOS
    /// - 3: ATTUNIX
    /// - 4: DGUX
    /// - 5: DECNT
    /// - 6: Digital UNIX
    /// - 7: OpenVMS
    /// - 8: HPUX
    /// - 9: AIX
    /// - 10: MVS
    /// - 11: OS400
    /// - 12: OS/2
    /// - 13: JavaVM
    /// - 14: MSDOS
    /// - 15: WIN3x
    /// - 16: WIN95
    /// - 17: WIN98
    /// - 18: WINNT
    /// - 19: WINCE
    /// - 20: NCR3000
    /// - 21: NetWare
    /// - 22: OSF
    /// - 23: DC/OS
    /// - 24: Reliant UNIX
    /// - 25: SCO UnixWare
    /// - 26: SCO OpenServer
    /// - 27: Sequent
    /// - 28: IRIX
    /// - 29: Solaris
    /// - 30: SunOS
    /// - 31: U6000
    /// - 32: ASERIES
    /// - 33: TandemNSK
    /// - 34: TandemNT
    /// - 35: BS2000
    /// - 36: LINUX
    /// - 37: Lynx
    /// - 38: XENIX
    /// - 39: VM/ESA
    /// - 40: Interactive UNIX
    /// - 41: BSDUNIX
    /// - 42: FreeBSD
    /// - 43: NetBSD
    /// - 44: GNU Hurd
    /// - 45: OS9
    /// - 46: MACH Kernel
    /// - 47: Inferno
    /// - 48: QNX
    /// - 49: EPOC
    /// - 50: IxWorks
    /// - 51: VxWorks
    /// - 52: MiNT
    /// - 53: BeOS
    /// - 54: HP MPE
    /// - 55: NextStep
    /// - 56: PalmPilot
    /// - 57: Rhapsody
    pub TargetOperatingSystem: Option<u16>,
    /// Version of the software element. Version should be in the form <Major>.<Minor>.<Revision> or
    /// <Major>.<Minor><letter><revision>.
    pub Version: Option<String>,
}

/// The `Win32_DiskPartition` WMI class represents the capabilities and management capacity of a
/// partitioned area of a physical disk on a computer system running Windows.
/// Example: Disk #0, Partition #1.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskpartition>
// Some struct fields no longer exist
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_DiskPartition {
    /*
    /// Additional availability and status of the Device,
    /// beyond that specified in the Availability property.
    /// The Availability property denotes the primary status and availability of the Device.
    /// In some cases, this will not be sufficient to denote the complete status of the Device.
    /// In those cases, the AdditionalAvailability property can be used to provide further information.
    /// For example, a Device's primary Availability may be Off line (value=8),
    /// but it may also be in a low power state (AdditonalAvailability value=14),
    /// or the Device could be running Diagnostics (AdditionalAvailability value=5, In Test)."
    ///
    /// - Other (1)
    /// - Unknown (2)
    /// - Running/Full Power (3)
    /// - Warning (4)
    /// - In Test (5)
    /// - Not Applicable (6)
    /// - Power Off (7)
    /// - Off Line (8)
    /// - Off Duty (9)
    /// - Degraded (10)
    /// - Not Installed (11)
    /// - Install Error (12)
    /// - Power Save - Unknown (13)
    /// - Power Save - Low Power Mode (14)
    /// - Power Save - Standby (15)
    /// - Power Cycle (16)
    /// - Power Save - Warning (17)
    /// - Paused (18)
    /// - Not Ready (19)
    /// - Not Configured (20)
    /// - Quiesce (21)
    pub AdditionalAvailability: Option<u16>,
    */
    /// Availability and status of the device.
    ///
    /// - Other (1)
    /// - Unknown (2)
    /// - Running/Full Power (3)
    /// - Warning (4)
    /// - In Test (5)
    /// - Not Applicable (6)
    /// - Power Off (7)
    /// - Off Line (8)
    /// - Off Duty (9)
    /// - Degraded (10)
    /// - Not Installed (11)
    /// - Install Error (12)
    /// - Power Save: Unknown (13): The device is known to be in a power save mode, but its exact status is unknown.
    /// - Power Save: Low Power Mode (14): he device is in a power save state but still functioning, and may exhibit degraded performance.
    /// - Power Save: Standby (15): he device is not functioning but could be brought to full power quickly.
    /// - Power Cycle (16)
    /// - Power Save: Warning (17): The device is in a warning state, though also in a power save mode.
    /// - Paused (18): he device is paused.
    /// - Not Ready (19): The device is not ready.
    /// - Not Configured (20): The device is not configured.
    /// - Quiesced (21): he device is quiet.
    pub Availability: Option<u16>,
    /// Indicates the specific power-related capabilities of the logical device.
    /// The array values, 0="Unknown", 1="Not Supported" and 2="Disabled" are self-explanatory.
    /// The value, 3="Enabled"
    /// indicates that the power management features are currently enabled
    /// but the exact feature set is unknown or the information is unavailable.
    /// "Power Saving Modes Entered Automatically"
    /// (4) describes that a device can change its power state based on usage or other criteria.
    /// "Power State Settable" (5) indicates that the SetPowerState method is supported.
    /// "Power Cycling Supported"
    /// (6)
    /// indicates that the SetPowerState method can be invoked with the PowerState input variable set to 5
    /// ("Power Cycle").
    /// "Timed Power On Supported"
    /// (7)
    /// indicates that the SetPowerState method can be invoked with the PowerState input variable set to 5
    /// ("Power Cycle") and the Time parameter set to a specific date and time,
    /// or interval, for power-on.
    ///
    /// - Unknown (0)
    /// - Not Supported (1)
    /// - Disabled (2)
    /// - Enabled (3)
    /// - Power Saving Modes Entered Automatically (4)
    /// - Power State Settable (5)
    /// - Power Cycling Supported (6)
    /// - Timed Power On Supported (7)
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /*
    /// An array of free-form strings
    /// providing explanations and details behind the entries in the OtherIdentifyingInfo array.
    /// Note,
    /// each entry of this array is related to the entry in OtherIdentifyingInfo
    /// that is located at the same index.
    pub IdentifyingDescriptions: Option<Vec<Option<String>>>,
    /// Maximum time in milliseconds, that a Device can run in a Quiesced state.
    /// A Device's state is defined in its Availability and AdditionalAvailability properties,
    /// where Quiesced is conveyed by the value 21. What occurs at the end of the time limit is device-specific.
    /// The Device may unquiesce, may offline or take other action.
    /// A value of 0 indicates that a Device can remain quiesced indefinitely.
    ///
    /// Note: "The MaxQuiesceTime property has been deprecated.
    /// When evaluating the use of Quiesce,
    /// it was determine
    /// that this single property is not adequate
    /// for describing when a device will automatically exit a quiescent state.
    /// In fact,
    /// the most likely scenario for a device to exit a quiescent state was determined
    /// to be based on the number of outstanding requests queued rather than on a maximum time.
    /// This will be re-evaluated and repositioned later.
    pub MaxQuiesceTime: Option<u64>,
    /// Array that captures additional data, beyond DeviceID information,
    /// that could be used to identify a LogicalDevice.
    /// One example would be
    /// to hold the Operating System's user friendly name for the Device in this property.
    /// Maximum length is 256.
    pub OtherIdentifyingInfo: Option<u64>,
    /// State of the logical device.
    /// If this property does not apply to the logical device,
    /// the value 5 ("Not Applicable") should be used.
    ///
    /// - Other (1)
    /// - Unknown (2)
    /// - Enabled (3)
    /// - Disabled (4)
    /// - Not Applicable (5)
    pub StatusInfo: Option<u16>,
    /// The number of consecutive hours that this Device has been powered, since its last power cycle.
    pub PowerOnHours: Option<u64>,
    /// The total number of hours that this device has been powered.
    //pub TotalPowerOnHours: Option<u64>,
    */
    /// Media access available.
    ///
    /// - Unknown (0)
    /// - Readable (1)
    /// - Writeable (2)
    /// - Read/Write Supported (3)
    /// - Write Once (4)
    pub Access: Option<u16>,
    /// Size in bytes of the blocks which form this storage extent.
    /// If unknown or if a block concept is not valid
    /// (for example, for aggregate extents, memory or logical disks),
    /// enter a 1.
    pub BlockSize: Option<u64>,
    /// Indicates whether the computer can be booted from this partition.
    pub Bootable: Option<bool>,
    /// Partition is the active partition.
    /// The operating system uses the active partition when booting from a hard disk.
    pub BootPartition: Option<bool>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// Windows Configuration Manager error code.
    ///
    /// - This device is working properly. (0): Device is working properly.
    /// - This device is not configured correctly. (1): Device is not configured correctly.
    /// - Windows cannot load the driver for this device. (2)
    /// - The driver for this device might be corrupted, or your system may be running low on memory or other resources. (3)
    /// - This device is not working properly. One of its drivers or your registry might be corrupted. (4)
    /// - The driver for this device needs a resource that Windows cannot manage. (5)
    /// - The boot configuration for this device conflicts with other devices. (6)
    /// - Cannot filter. (7)
    /// - The driver loader for the device is missing. (8)
    /// - This device is not working properly because the controlling firmware is reporting the resources for the device incorrectly. (9)
    /// - This device cannot start. (10)
    /// - This device failed. (11)
    /// - This device cannot find enough free resources that it can use. (12)
    /// - Windows cannot verify this device's resources. (13)
    /// - This device cannot work properly until you restart your computer. (14)
    /// - This device is not working properly because there is probably a re-enumeration problem. (15)
    /// - Windows cannot identify all the resources this device uses. (16)
    /// - This device is asking for an unknown resource type. (17)
    /// - Reinstall the drivers for this device. (18)
    /// - Failure using the VxD loader. (19)
    /// - Your registry might be corrupted. (20)
    /// - System failure: Try changing the driver for this device. If that does not work, see your hardware documentation. Windows is removing this device. (21)
    /// - This device is disabled. (22)
    /// - System failure: Try changing the driver for this device. If that doesn't work, see your hardware documentation. (23)
    /// - This device is not present, is not working properly, or does not have all its drivers installed. (24)
    /// - Windows is still setting up this device. (25)
    /// - Windows is still setting up this device. (26)
    /// - This device does not have valid log configuration. (27)
    /// - The drivers for this device are not installed. (28)
    /// - This device is disabled because the firmware of the device did not give it the required resources. (29)
    /// - This device is using an Interrupt Request (IRQ) resource that another device is using. (30)
    /// - This device is not working properly because Windows cannot load the drivers required for this device. (31)
    pub ConfigManagerErrorCode: Option<u32>,
    /// If True, the device is using a user-defined configuration.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Name of the first concrete class to appear in the inheritance chain
    /// used in the creation of an instance.
    /// When used with the other key properties of the class,
    /// the property allows all instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the disk drive and partition, from the rest of the system.
    pub DeviceID: Option<String>,
    /// Index number of the disk containing this partition.
    ///
    /// Example: 0
    pub DiskIndex: Option<u32>,
    /// If True, the error reported in LastErrorCode is now cleared.
    pub ErrorCleared: Option<bool>,
    /// Information about the error recorded in LastErrorCode,
    /// and information on any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Type of error detection and correction supported by this storage extent.
    pub ErrorMethodology: Option<String>,
    /// Number of hidden sectors in the partition.
    ///
    /// Example: 63
    pub HiddenSectors: Option<u32>,
    /// Index number of the partition.
    ///
    /// Example: 1
    pub Index: Option<u32>,
    /// Date the object was installed.
    /// This property does not need a value to indicate that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Label by which the object is known.
    /// When subclassed, the property can be overridden to be a key property.
    pub Name: Option<String>,
    /// Total number of consecutive blocks,
    /// each block the size of the value contained in the BlockSize property,
    /// which form this storage extent.
    /// Total size of the storage extent can be calculated by multiplying the value of the BlockSize property by the value of this property.
    /// If the value of BlockSize is 1, this property is the total size of the storage extent.
    pub NumberOfBlocks: Option<u64>,
    /// Windows Plug and Play device identifier of the logical device.
    ///
    /// Example: "*PNP030b"
    pub PNPDeviceID: Option<String>,
    /// If True, the device can be power-managed (can be put into suspend mode, and so on).
    /// The property does not indicate that power management features are currently enabled,
    /// only that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// If True, this is the primary partition.
    pub PrimaryPartition: Option<bool>,
    /// Description of the media and its use.
    pub Purpose: Option<String>,
    /// If True, the partition information has changed.
    /// When you change a partition (with IOCTL_DISK_SET_DRIVE_LAYOUT),
    /// the system uses this property
    /// to determine which partitions have changed and need their information rewritten.
    /// If TRUE, the partition must be rewritten.
    pub RewritePartition: Option<bool>,
    /// Total size of the partition.
    ///
    /// Example: 1059045376
    pub Size: Option<u64>,
    /// Starting offset (in bytes) of the partition.
    ///
    /// Example: 32256
    pub StartingOffset: Option<u64>,
    /// Current status of the object.
    /// Various operational and nonoperational statuses can be defined.
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail"
    /// (an element, such as a SMART-enabled hard disk drive,
    /// may be functioning properly but predicting a failure in the near future).
    /// Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service".
    /// The latter, "Service", could apply during mirror-re-silvering of a disk,
    /// reload of a user permissions list, or other administrative work.
    /// Not all such work is online,
    /// yet the managed element is neither "OK" nor in one of the other states.
    ///
    /// The values are:
    ///
    /// - OK ("OK")
    /// - Error ("Error")
    /// - Degraded ("Degraded")
    /// - Unknown ("Unknown")
    /// - Pred Fail ("Pred Fail")
    /// - Starting ("Starting")
    /// - Stopping ("Stopping")
    /// - Service ("Service")
    /// - Stressed ("Stressed")
    /// - NonRecover ("NonRecover")
    /// - No Contact ("No Contact")
    /// - Lost Comm ("Lost Comm")
    pub Status: Option<String>,
    /// Creation class name of the scoping system.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// Type of the partition.
    ///
    /// The values are:
    ///
    /// - Unused ("Unused")
    /// - 12-bit FAT ("12-bit FAT")
    /// - Xenix Type 1 ("Xenix Type 1")
    /// - Xenix Type 2 ("Xenix Type 2")
    /// - 16-bit FAT ("16-bit FAT")
    /// - Extended Partition ("Extended Partition")
    /// - MS-DOS V4 Huge ("MS-DOS V4 Huge")
    /// - Installable File System ("Installable File System")
    /// - PowerPC Reference Platform ("PowerPC Reference Platform")
    /// - UNIX ("UNIX")
    /// - NTFS ("NTFS")
    /// - Win95 w/Extended Int 13 ("Win95 w/Extended Int 13")
    /// - Extended w/Extended Int 13 ("Extended w/Extended Int 13")
    /// - Logical Disk Manager ("Logical Disk Manager")
    /// - Unknown ("Unknown")
    pub Type: Option<String>,
}

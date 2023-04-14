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

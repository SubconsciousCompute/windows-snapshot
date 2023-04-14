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

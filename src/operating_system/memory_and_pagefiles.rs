//! The Memory and Page files subcategory groups classes that represent page file configuration settings.
//! 
//! | Class                                                                 | Description                                                                                                                                   |
//! |-----------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_PageFile**](win32-pagefile)                                | Instance class<br/> Represents the file used for handling virtual memory file swapping on a Windows system.<br/>                  |
//! | [**Win32\_PageFileElementSetting**](win32-pagefileelementsetting)    | Association class<br/> Relates the initial settings of a page file and the state of those settings during normal use.<br/>        |
//! | [**Win32\_PageFileSetting**](win32-pagefilesetting)                  | Instance class<br/> Represents the settings of a page file.<br/>                                                                  |
//! | [**Win32\_PageFileUsage**](win32-pagefileusage)                      | Instance class<br/> Represents the file used for handling virtual memory file swapping on a computer system running Windows.<br/> |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows `PageFiles`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageFiles {
    /// Represents sequence of Windows `PageFiles`
    pub pagefiles: Vec<Win32_PageFile>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(PageFiles, pagefiles);

/// Represents the state of Windows `PageFileSettings`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageFileSettings {
    /// Represents the Windows `PageFileSettings`
    pub pagefile_settings: Vec<Win32_PageFileSetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(PageFileSettings, pagefile_settings);

/// Represents the state of Windows `PageFileUsages`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageFileUsages {
    /// Represents the Windows `PageFileUsages` details
    pub pagefile_usage: Vec<Win32_PageFileUsage>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(PageFileUsages, pagefile_usage);

/// The `Win32_PageFile` WMI class represents the file used for handling virtual memory file swapping 
/// on a Win32 system. This class has been deprecated.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pagefile>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PageFile {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is 
    /// not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// If `True`, the file should be archived.
    pub Archive: Option<bool>,
    /// If `True`, the file is compressed.
    pub Compressed: Option<bool>,
    /// Free-form string that indicates the algorithm or tool used to compress the logical file. If the 
    /// compression scheme is unknown or not described, use "Unknown". If the logical file is compressed, 
    /// but the compression scheme is unknown or not described, use "Compressed". If the logical file is 
    /// not compressed, use "Not Compressed".
    pub CompressionMethod: Option<String>,
    /// Name of the class.
    pub CreationClassName: Option<String>,
    /// Date and time of the file's creation.
    pub CreationDate: Option<WMIDateTime>,
    /// Class of the computer system.
    pub CSCreationClassName: Option<String>,
    /// Name of the computer system.
    pub CSName: Option<String>,
    /// Drive letter (including the colon that follows the drive letter) of the file. 
    /// 
    /// Example: "c:"
    pub Drive: Option<String>,
    /// DOS-compatible file name.
    /// 
    /// Example: "c:\progra~1"
    pub EightDotThreeFileName: Option<String>,
    /// If `True`, the file is encrypted.
    pub Encrypted: Option<bool>,
    /// Free-form string that identifies the algorithm or tool used to encrypt a logical file. If the encryption 
    /// scheme is not indulged (for security reasons, for example), use "Unknown". If the file is encrypted, but 
    /// either its encryption scheme is unknown or not disclosed, use "Encrypted". If the logical file is not 
    /// encrypted, use "Not Encrypted".
    pub EncryptionMethod: Option<String>,
    /// File name extension without the preceding period (dot).
    /// 
    /// Example: "txt", "mof", "mdb"
    pub Extension: Option<String>,
    /// File name without the file name extension. Example: "MyDataFile"
    pub FileName: Option<String>,
    /// Size of the file, in bytes.
    pub FileSize: Option<u64>,
    /// Descriptor that represents the file type indicated by the `Extension` property.
    pub FileType: Option<String>,
    /// Class of the file system.
    pub FSCreationClassName: Option<String>,
    /// Name of the file system.
    pub FSName: Option<String>,
    /// If `True`, the file is hidden.
    pub Hidden: Option<bool>,
    /// Number of "file opens" that are currently active against the file.
    pub InUseCount: Option<u64>,
    /// Date and time the file was last accessed.
    pub LastAccessed: Option<WMIDateTime>,
    /// Date and time the file was last modified.
    pub LastModified: Option<WMIDateTime>,
    /// Path of the file including the leading and trailing backslashes.
    /// 
    /// Example: "\windows\system\"
    pub Path: Option<String>,
    /// If `True`, the file can be read.
    pub Readable: Option<bool>,
    /// If `True`, the file is a system file.
    pub System: Option<bool>,
    /// If `True`, the file can be written.
    pub Writeable: Option<bool>,
    /// Bitmask that represents the access rights required to access or perform specific operations 
    /// on the file. For values, see `File and Directory Access Rights Constants`.
    /// 
    /// - `FILE_READ_DATA` (file) or `FILE_LIST_DIRECTORY` (directory) (1)
    /// - `FILE_WRITE_DATA` (file) or `FILE_ADD_FILE` (directory) (2)
    /// - `FILE_APPEND_DATA` (file) or `FILE_ADD_SUBDIRECTORY` (directory) (4)
    /// - `FILE_READ_EA` (8)
    /// - `FILE_WRITE_EA` (16)
    /// - `FILE_EXECUTE` (file) or `FILE_TRAVERSE` (directory) (32)
    /// - `FILE_DELETE_CHILD` (directory) (64)
    /// - `FILE_READ_ATTRIBUTES` (128)
    /// - `FILE_WRITE_ATTRIBUTES` (256)
    /// - `DELETE` (65536)
    /// - `READ_CONTROL` (131072)
    /// - `WRITE_DAC` (262144)
    /// - `WRITE_OWNER` (524288)
    /// - `SYNCHRONIZE` (1048576)
    pub AccessMask: Option<u32>,
    /// Manufacturer string from the version resource (if one is present).
    pub Manufacturer: Option<String>,
    /// String that indicates the current status of the object.
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
    /// Version string from the version resource (if one is present).
    pub Version: Option<String>,
    /// Space available in the paging file. The operating system can enlarge the paging file as necessary, 
    /// up to the limit imposed by the user. This property shows the difference between the size of current 
    /// committed memory and the current size of the paging file; it does not show the largest possible 
    /// size of the paging file.
    pub FreeSpace: Option<u32>,
    /// Initial size of the page file.
    pub InitialSize: Option<u32>,
    /// Maximum size of the page file as set by the user. The operating system will not allow the page file 
    /// to exceed this limit.
    pub MaximumSize: Option<u32>,
    /// Name of the page file.
    /// 
    /// Example: "C:\PAGEFILE.SYS"
    pub Name: Option<String>,
}

/// The `Win32_PageFileSetting` WMI class represents the settings of a page file. Information contained within 
/// objects instantiated from this class specify the page file parameters used when the file is created at 
/// system startup. The properties in this class can be modified and deferred until startup. These settings 
/// are different from the run-time state of a page file expressed through the associated class `Win32_PageFileUsage`.
/// 
/// To create an instance of this class, enable the `SeCreatePagefilePrivilege` privilege. 
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PageFileSetting {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Initial size of the page file.
    /// 
    /// Example: 139
    pub InitialSize: Option<u32>,
    /// Maximum size of the page file.
    /// 
    /// Example: 178
    pub MaximumSize: Option<u32>,
    /// Path and file name of the page file.
    /// 
    /// Example: "C:\PAGEFILE.SYS"
    pub Name: Option<String>,
}

/// The `Win32_PageFileUsage` WMI class represents the file used for handling virtual memory file swapping on 
/// a Win32 system. Information contained within objects instantiated from this class specify the run-time state 
/// of the page file.
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PageFileUsage {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is not installed..
    pub InstallDate: Option<String>,
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
    /// Actual amount of disk space allocated for use with this page file. This value corresponds to the range 
    /// established in `Win32_PageFileSetting` under the `InitialSize` and `MaximumSize` properties, set at 
    /// system startup.
    /// 
    /// Example: 178
    pub AllocatedBaseSize: Option<u32>,
    /// Amount of disk space currently used by the page file.
    pub CurrentUsage: Option<u32>,
    /// Name of the page file.
    ///
    /// Example: "C:\PAGEFILE.SYS"
    pub Name: Option<String>,
    /// Highest use page file.
    pub PeakUsage: Option<u32>,
    /// If true, a temporary page file has been created, usually because there is no permanent page file on the 
    /// current computer system.
    pub TempPageFile: Option<bool>,
}
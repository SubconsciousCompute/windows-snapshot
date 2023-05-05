//! The Windows Event Log subcategory groups classes that represent events, event log entries, event log configuration settings, and so on.
//!
//! | Class                                                         | Description                                                                                                                                                                   |
//! |---------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_NTEventlogFile**](/previous-versions/windows/desktop/legacy/aa394225(v=vs.85))         | Instance class<br/> Represents data stored in a Windows Event log file.<br/>                                                                                      |
//! | [**Win32\_NTLogEvent**](win32-ntlogevent)                 | Instance class<br/> Represents Windows events.<br/>                                                                                                               |
//! | [**Win32\_NTLogEventComputer**](win32-ntlogeventcomputer) | Association class<br/> Relates instances of [**Win32\_NTLogEvent**](/previous-versions/windows/desktop/eventlogprov/win32-ntlogevent) and [**Win32\_ComputerSystem**](win32-computersystem.md).<br/>         |
//! | [**Win32\_NTLogEventLog**](/previous-versions/windows/desktop/eventlogprov/win32-ntlogeventlog)           | Association class<br/> Relates instances of [**Win32\_NTLogEvent**](/previous-versions/windows/desktop/eventlogprov/win32-ntlogevent) and [**Win32\_NTEventlogFile**](/previous-versions/windows/desktop/legacy/aa394225(v=vs.85)) classes.<br/> |
//! | [**Win32\_NTLogEventUser**](/previous-versions/windows/desktop/eventlogprov/win32-ntlogeventuser)         | Association class<br/> Relates instances of [**Win32\_NTLogEvent**](/previous-versions/windows/desktop/eventlogprov/win32-ntlogevent) and [**Win32\_UserAccount**](win32-useraccount.md).<br/>               |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows NTEventlogFiles
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NTEventlogFiles {
    /// Represents data stored in a Windows Event log file.
    pub nt_event_log_files: Vec<Win32_NTEventlogFile>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(NTEventlogFiles, nt_event_log_files);

/// The `Win32_NTEventlogFile` WMI class represents a logical file or directory of operating system
/// events. The file is also known as the event log.
///
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394225(v=vs.85)>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NTEventlogFile {
    /// Bitmask that represents the access rights required to access or perform specific operations
    /// on the event log file. For bit values, see File and Directory Access Rights Constants.
    ///
    /// Note: On FAT volumes, the `FULL_ACCESS` value is returned instead, which indicates no security
    /// has been set on the object.
    pub AccessMask: Option<u32>,
    /// If `True`, a file that contains Windows events should be archived.
    pub Archive: Option<bool>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// If `True`, a file that contains Windows events is compressed.
    pub Compressed: Option<bool>,
    /// Algorithm or tool used to compress the logical file that contains Windows events.
    pub CompressionMethod: Option<String>,
    /// Qualifiers: Key, Dynamic, MaxLen (256) , Dynamic
    /// 
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of an 
    /// instance. When used with the other key properties of the class, this property allows all 
    /// instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Date that the file that contains Windows events was created.
    pub CreationDate: Option<WMIDateTime>,
    /// Class of the computer system.
    pub CSCreationClassName: Option<String>,
    /// Name of the computer system.
    pub CSName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Drive letter (including colon) of the file that contains Windows events.
    /// 
    /// Example: "c:"
    pub Drive: Option<String>,
    /// DOS-compatible file name for the file that contains Windows events.
    /// 
    /// Example: "c:\progra~1"
    pub EightDotThreeFileName: Option<String>,
    /// File that contains Windows events is encrypted.
    pub Encrypted: Option<bool>,
    /// Algorithm or tool used to encrypt the logical file.
    pub EncryptionMethod: Option<String>,
    /// File name extension (without the dot) of the file that contains Windows events.
    /// 
    /// Example: "txt", "mof", "mdb"
    pub Extension: Option<String>,
    /// File name (without extension) of the file that contains Windows events.
    /// 
    /// Example: "autoexec"
    pub FileName: Option<String>,
    /// Size of the file that contains Windows events (in bytes).
    /// 
    /// For more information about using `uint64` values in scripts, see Scripting in WMI.
    pub FileSize: Option<u64>,
    /// File type (indicated by the `Extension` property).
    pub FileType: Option<String>,
    /// Class of the file system.
    pub FSCreationClassName: Option<String>,
    /// Name of the file system.
    pub FSName: Option<String>,
    /// If `True`, a file that contains Windows events is hidden.
    pub Hidden: Option<bool>,
    /// Object is installed. This property does not need a value to indicate that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Number of "file opens" that are currently active against the file that contains Windows events.
    /// 
    /// For more information about using `uint64` values in scripts, see Scripting in WMI.
    pub InUseCount: Option<u64>,
    /// Date and time that the file that contains Windows events was last accessed.
    pub LastAccessed: Option<WMIDateTime>,
    /// Date and time that the file that contains Windows events was last modified.
    pub LastModified: Option<WMIDateTime>,
    /// Name of the file that contains Windows events. Standard log file names include: Application, 
    /// System, and Security.
    /// 
    /// To return the actual path and file name of the event log 
    /// (for example, C:\Windows\System32\Config\Sysevent.evt), use the Name property instead.
    pub LogfileName: Option<String>,
    /// Manufacturer from version resource, if one is present.
    pub Manufacturer: Option<String>,
    /// Maximum size (in bytes) permitted for the file that contains Windows events. If the file 
    /// exceeds its maximum size, its contents are moved to another file and the primary file is 
    /// emptied. A value of zero indicates no size limit. WMI retrieves the `Maxsize` value from the 
    /// Event Log Service registry values.
    /// 
    /// Although event logs can be sized as large as 4 gigabytes, in practice they should be limited 
    /// to no more than 300 megabytes. Event logs larger than that can be difficult to analyze because 
    /// of the number of events contained within the log and because event logs are not optimized for 
    /// data retrieval.
    pub MaxFileSize: Option<u32>,
    /// Qualifiers: Key, Dynamic
    /// 
    /// Inherited name that serves as a key of a logical file instance that contains Windows events 
    /// within a file system. Full path names should be provided.
    /// 
    /// Example: "c:\winnt\system\win.ini"
    pub Name: Option<String>,
    /// Number of records in the file that contains Windows events. This value is determined by calling 
    /// the Windows function `GetNumberOfEventLogRecords`.
    pub NumberOfRecords: Option<u32>,
    /// Qualifiers: Units (Days) , Dynamic
    /// 
    /// Number of days after which an event can be overwritten.
    /// 
    /// Possible values for `OverwriteOutDated` include the following.
    /// 
    /// Value: Meaning
    /// 
    /// - 0 (0x0): Any record can be overwritten if necessary. If necessary, all existing events in the event log can be overwritten to make room for new events.
    /// - 1...365: `Windows Server 2003 and Windows XP`:  Possible values for `OverwriteOutDated` include the following. Events older than the specified number of days can be overwritten as needed. If the event log does not contain any records older than the value specified, no new events will be recorded until the log has been cleared.
    /// - 4294967295 (0xFFFFFFFF): No records can be overwritten. If the log reaches its maximum size, no new events will be recorded until the log has been cleared.
    pub OverwriteOutDated: Option<u32>,
    /// Current overwrite policy the Event Log service employs for this log file. Data can be never
    /// overwritten, or can be overwritten when necessary or when outdated. When data is outdated
    /// depends on the OverwriteOutDated value.
    ///
    /// Value: Meaning
    ///
    /// - WhenNeeded: The value of `OverwriteOutDated` equals 0 (zero). Any record can be overwritten to make room for new records.
    /// - OutDated: The value of `OverwriteOutDated` ranges from 1 to 365. Records older than a specified number of days can be overwritten to make room for new records.
    /// - Never: The value of `OverwriteOutDated` equals 4294967295. Old records are never overwritten.
    pub OverWritePolicy: Option<String>,
    /// Path of the file that contains Windows event. This includes leading and trailing backslashes.
    /// 
    /// Example: "\windows\system\"
    pub Path: Option<String>,
    /// If `True`, a file that contains Windows events can be read.
    pub Readable: Option<bool>,
    /// List of applications that are registered to log into this log file.
    pub Sources: Option<Vec<String>>,
    /// Current status of the object.
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
    /// If `True`, a file that contains Windows event is a system file.
    pub System: Option<bool>,
    /// Version string from version resource if one is present.
    pub Version: Option<String>,
    /// If `True`, a file that contains Windows events can be written.
    pub Writeable: Option<bool>,
}


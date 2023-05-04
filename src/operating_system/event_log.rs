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
    pub Archive: Option<bool>,
    pub Caption: Option<String>,
    pub Compressed: Option<bool>,
    pub CompressionMethod: Option<String>,
    pub CreationClassName: Option<String>,
    pub CreationDate: Option<WMIDateTime>,
    pub CSCreationClassName: Option<String>,
    pub CSName: Option<String>,
    pub Description: Option<String>,
    pub Drive: Option<String>,
    pub EightDotThreeFileName: Option<String>,
    pub Encrypted: Option<bool>,
    pub EncryptionMethod: Option<String>,
    pub Extension: Option<String>,
    pub FileName: Option<String>,
    pub FileSize: Option<u64>,
    pub FileType: Option<String>,
    pub FSCreationClassName: Option<String>,
    pub FSName: Option<String>,
    pub Hidden: Option<bool>,
    pub InstallDate: Option<WMIDateTime>,
    pub InUseCount: Option<u64>,
    pub LastAccessed: Option<WMIDateTime>,
    pub LastModified: Option<WMIDateTime>,
    pub LogfileName: Option<String>,
    pub Manufacturer: Option<String>,
    pub MaxFileSize: Option<u32>,
    pub Name: Option<String>,
    pub NumberOfRecords: Option<u32>,
    pub OverwriteOutDated: Option<u32>,
    /// Current overwrite policy the Event Log service employs for this log file. Data can be never
    /// overwritten, or can be overwritten when necessary or when outdated. When data is outdated
    /// depends on the OverwriteOutDated value.
    ///
    /// Value: Meaning
    ///
    /// - WhenNeeded: The value of OverwriteOutDated equals 0 (zero). Any record can be overwritten to make room for new records.
    /// - OutDated: The value of OverwriteOutDated ranges from 1 to 365. Records older than a specified number of days can be overwritten to make room for new records.
    /// - Never: The value of OverwriteOutDated equals 4294967295. Old records are never overwritten.
    pub OverWritePolicy: Option<String>,
    pub Path: Option<String>,
    pub Readable: Option<bool>,
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
    pub System: Option<bool>,
    pub Version: Option<String>,
    pub Writeable: Option<bool>,
}


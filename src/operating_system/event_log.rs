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
    /// Represents data stored in a Windows Event log file
    pub nt_event_log_files: Vec<Win32_NTEventlogFile>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(NTEventlogFiles, nt_event_log_files);

/// Represents sequence of Windows NTLogEvents
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NTLogEvents {
    /// Represents Windows events
    pub nt_log_events: Vec<Win32_NTLogEvent>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(NTLogEvents, nt_log_events);

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
    /// Inherited name that serves as a key of a logical file instance that contains Windows events 
    /// within a file system. Full path names should be provided.
    /// 
    /// Example: "c:\winnt\system\win.ini"
    pub Name: Option<String>,
    /// Number of records in the file that contains Windows events. This value is determined by calling 
    /// the Windows function `GetNumberOfEventLogRecords`.
    pub NumberOfRecords: Option<u32>,
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

/// The `Win32_NTLogEvent` WMI class is used to translate instances from the Windows event log. 
/// An application must have `SeSecurityPrivilege` to receive events from the security event log, 
/// otherwise "Access Denied" is returned to the application.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/eventlogprov/win32-ntlogevent>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NTLogEvent {
    /// Classification of the event as determined by the source. This subcategory is source-specific.
    /// 
    /// Although primarily used when recording Security events, this property is available in other 
    /// event logs as well. Common Security categories include Logon/Logoff, Account Management, and 
    /// System Event.
    pub Category: Option<u16>,
    /// Translation of the subcategory. The translation is source-specific.
    pub CategoryString: Option<String>,
    /// Name of the computer that generated this event.
    pub ComputerName: Option<String>,
    /// List of the binary data that accompanied the report of the Windows event.
    pub Data: Option<Vec<u8>>,
    /// Value of the lower 16-bits of the `EventIdentifier` property. It is present to match the value 
    /// displayed in the Windows Event Viewer.
    /// 
    /// Note: Two events from the same source may have the same value for this property but may have 
    /// different severity and EventIdentifier values. For example, a successful logoff is recorded in 
    /// the Security log with the Event ID 538. However, Event IDs are not necessarily unique. 
    /// It is possible that, when retrieving Event ID 538, you can get other kinds of events with ID 538. 
    /// If this happens, you might need to filter by the source as well as ID.
    pub EventCode: Option<u16>,
    /// Identifier of the event. This is specific to the source that generated the event log entry and 
    /// is used, together with `SourceName`, to uniquely identify a Windows event type.
    pub EventIdentifier: Option<u32>,
    /// Type of event.
    /// 
    /// Value: Meaning
    /// 
    /// - 1: Error
    /// - 2: Warning
    /// - 3: Information
    /// - 4: Security Audit Success
    /// - 5: Security Audit Failure
    pub EventType: Option<u8>,
    /// List of the insertion strings that accompanied the report of the Windows event.
    pub InsertionStrings: Option<Vec<String>>,
    /// Name of Windows event log file. Together with `RecordNumber`, this is used to uniquely identify 
    /// an instance of this class.
    pub Logfile: Option<String>,
    /// Event message as it appears in the Windows event log. This is a standard message with zero or more 
    /// insertion strings supplied by the source of the Windows event. The insertion strings are inserted 
    /// into the standard message in a predefined format. If there are no insertion strings or there is a 
    /// problem inserting the insertion strings, only the standard message will be present in this field.
    pub Message: Option<String>,
    /// Identifies the event within the Windows event log file. This is specific to the log file and is 
    /// used together with the log file name to uniquely identify an instance of this class.
    /// 
    /// Record numbers are always unique; they are not reset to 1 when an event log is cleared. As a result, 
    /// the highest record number also indicates the number of records that have been written to the event log 
    /// since the operating system was installed
    pub RecordNumber: Option<u32>,
    /// Name of the source (application, service, driver, or subsystem) that generated the entry. It is used, 
    /// together with `EventIdentifier` to uniquely identify a Windows event type.
    pub SourceName: Option<String>,
    /// The time when the event is generated.
    pub TimeGenerated: Option<WMIDateTime>,
    /// The time when the event is written to the log file.
    pub TimeWritten: Option<WMIDateTime>,
    /// Type of event. This is an enumerated string. It is preferable to use the `EventType` property rather than 
    /// the `Type` property.
    /// 
    /// Value: Meaning
    /// 
    /// - 1: Error
    /// - 2: Warning
    /// - 4: Information
    /// - 8: Security Audit Success
    /// - 16: Security Audit Failure
    pub Type: Option<String>,
    /// User name of the logged-on user when the event occurred. If the user name cannot be determined, 
    /// this will be `NULL`.
    pub User: Option<String>,
}
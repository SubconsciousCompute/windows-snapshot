//! The Job Objects subcategory groups classes that represent classes that provide the means of instrumenting named job objects. An unnamed job object cannot be instrumented.
//! 
//! | Class                                                                               | Description                                                                                                                                                                                |
//! |-------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_CollectionStatistics**](/previous-versions/windows/desktop/cimwin32a/win32-collectionstatistics)                   | Association class<br/> Relates a managed system element collection and the class representing statistical information about the collection.<br/>                               |
//! | [**Win32\_LUID**](/previous-versions/windows/desktop/wmipjobobjprov/win32-luid)                                                   | Instance class<br/> Represents a locally unique identifier (LUID)<br/>                                                                                                         |
//! | [**Win32\_LUIDandAttributes**](/previous-versions/windows/desktop/wmipjobobjprov/win32-luidandattributes)                         | Instance class<br/> Represents a LUID and its attributes.<br/>                                                                                                                 |
//! | [**Win32\_NamedJobObject**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobject)                               | Instance class<br/> Represents a kernel object that is used to group processes for the sake of controlling the life and resources of the processes within the job object.<br/> |
//! | [**Win32\_NamedJobObjectActgInfo**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectactginfo)               | Instance class<br/> Represents the I/O accounting information for a job object.<br/>                                                                                           |
//! | [**Win32\_NamedJobObjectLimit**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectlimit)                     | Instance class<br/> Represents an association between a job object and the job object limit settings.<br/>                                                                     |
//! | [**Win32\_NamedJobObjectLimitSetting**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectlimitsetting)       | Instance class<br/> Represents the limit settings for a job object.<br/>                                                                                                       |
//! | [**Win32\_NamedJobObjectProcess**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectprocess)                 | Instance class<br/> Relates a job object and the process contained in the job object.<br/>                                                                                     |
//! | [**Win32\_NamedJobObjectSecLimit**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectseclimit)               | Instance class<br/> Relates a job object and the job object security limit settings.<br/>                                                                                      |
//! | [**Win32\_NamedJobObjectSecLimitSetting**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectseclimitsetting) | Instance class<br/> Represents the security limit settings for a job object.<br/>                                                                                              |
//! | [**Win32\_NamedJobObjectStatistics**](/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectstatistics)           | Instance class<br/> Represents an association between a job object and the job object I/O accounting information class.<br/>                                                   |
//! | [**Win32\_SIDandAttributes**](/previous-versions/windows/desktop/wmipjobobjprov/win32-sidandattributes)                           | Instance class<br/> Represents a security identifier (SID) and its attributes.<br/>                                                                                            |
//! | [**Win32\_TokenGroups**](/previous-versions/windows/desktop/wmipjobobjprov/win32-tokengroups)                                     | Event class<br/> Represents information about the group SIDs in an access token.<br/>                                                                                          |
//! | [**Win32\_TokenPrivileges**](/previous-versions/windows/desktop/wmipjobobjprov/win32-tokenprivileges)                             | Event class<br/> Represents information about a set of privileges for an access token.<br/>                                                                                    |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection};

/// Represents the state of Windows `LUIDs`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LUIDs {
    /// Represents sequence of Windows `LUIDs`
    pub luids: Vec<Win32_LUID>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(LUIDs, luids);

/// Represents the state of Windows `LUIDandAttributes`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LUIDandAttributes {
    /// Represents sequence of Windows `LUIDandAttributes`
    pub luid_and_attributes: Vec<Win32_LUIDandAttributes>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(LUIDandAttributes, luid_and_attributes);

/// Represents the state of Windows `NamedJobObjects`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NamedJobObjects {
    /// Represents sequence of Windows `NamedJobObjects`
    pub named_job_objects: Vec<Win32_NamedJobObject>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(NamedJobObjects, named_job_objects);

/// Represents the state of Windows `NamedJobObjectActgInfos`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NamedJobObjectActgInfos {
    /// Represents sequence of Windows `NamedJobObjectActgInfos`
    pub named_job_object_actg_infos: Vec<Win32_NamedJobObjectActgInfo>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(NamedJobObjectActgInfos, named_job_object_actg_infos);

/// Represents the state of Windows `NamedJobObjectLimitSettings`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NamedJobObjectLimitSettings {
    /// Represents sequence of Windows `NamedJobObjectLimitSettings`
    pub named_job_object_limit_settings: Vec<Win32_NamedJobObjectLimitSetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(NamedJobObjectLimitSettings, named_job_object_limit_settings);

/// The `Win32_LUID` abstract WMI class represents a locally unique identifier (LUID), an identifier unique on the 
/// local computer that is used in security tokens.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipjobobjprov/win32-luid>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LUID {
    /// Most significant bits of the LUID.
    pub HighPart: Option<u32>,
    /// Least significant bits of the LUID.
    pub LowPart: Option<u32>,
}

/// The `Win32_LUIDandAttribute` abstract WMI class represents a locally unique identifier (LUID) and its attributes. 
/// Each LUID and attributes structure defines the availability of a security privilege.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipjobobjprov/win32-luidandattributes>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LUIDandAttributes {
    /// Instance containing the attributes of the LUID. This value holds two 32-bit flags. Its meaning is dependent 
    /// on the definition and use of the LUID.
    pub Attributes: Option<u32>,
    /// Representing a LUID value.
    pub LUID: Option<Win32_LUID>,
}

/// The `Win32_NamedJobObject` WMI class represents a kernel object that is used to group processes for controlling 
/// the life cycle and resources of the processes within the job object. Only the job objects that are named are 
/// instrumented.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobject>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NamedJobObject {
    /// Short textual description of the object.
    pub Caption: Option<String>,
    /// Textual description of the object.
    pub Description: Option<String>,
    /// Restrictions on a job regarding the user interface.
    /// - 1 (0x1): Desktop
    /// - 2 (0x2): Display Settings
    /// - 4 (0x4): Exit Windows
    /// - 8 (0x8): Global Atoms
    /// - 16 (0x10): Handles
    /// - 32 (0x20): Read Clipboard
    /// - 64 (0x40): System Parameters
    /// - 128 (0x80): Write Clipboard
    pub BasicUIRestrictions: Option<u32>,
    /// Number that identifies a job object. Because they are kernel objects, job object names are case sensitive. 
    /// However, Windows Management Instrumentation (WMI) keys are case insensitive and must be decorated to 
    /// distinguish case. To indicate a capital letter, precede the letter by using a backslash. 
    /// 
    /// For example, "A" and "a" are lowercase and "\A" and "\a" are uppercase. 
    pub CollectionID: Option<String>,
}

/// The `Win32_NamedJobObjectActgInfo` WMI class class represents the I/O accounting information for a job object.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectactginfo>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NamedJobObjectActgInfo {
    /// Short textual description for the statistic or metric.
    pub Caption: Option<String>,
    /// Textual description of the statistic or metric.
    pub Description: Option<String>,
    /// Total number of processes associated with a job. When a process is associated with a job, but the association 
    /// fails because of a limit violation, the value is temporarily incremented. When the terminated process exits 
    /// and all references to the process are released, the value is decremented.
    pub ActiveProcesses: Option<u32>,
    /// Label by which the statistic or metric is known. When subclassed, this property can be overridden to be a key 
    /// property.
    pub Name: Option<String>,
    /// Number of I/O operations performed, other than read and write operations, by all of the processes that have been 
    /// associated with the job including all of the processes currently associated with the job.
    pub OtherOperationCount: Option<u64>,
    /// Number of bytes transferred during operations, other than read and write, by all of the processes that have been 
    /// associated with the job including all of the processes currently associated with the job.
    pub OtherTransferCount: Option<u64>,
    /// Peak memory in kilobyte usage of all of the processes associated with the job.
    pub PeakJobMemoryUsed: Option<u32>,
    /// The most process memory in kilobytes used by any process ever associated with the job.
    pub PeakProcessMemoryUsed: Option<u32>,
    /// Number of read operations performed by all of the processes that have ever been associated with the job, in 
    /// addition to all of the processes currently associated with the job.
    pub ReadOperationCount: Option<u64>,
    /// Number of bytes read by all of the processes that have been associated with the job including all of the 
    /// processes currently associated with the job.
    pub ReadTransferCount: Option<u64>,
    /// Total amount of kernel-mode execution time, in 100 nanosecond units, for all active processes associated with 
    /// the job (and all terminated processes no longer associated with the job) after the last call that set a 
    /// per-job kernel-mode time limit. This property is set to 0 (zero) when a job is created, and each time a 
    /// per-job kernel-mode time limit is established.
    pub ThisPeriodTotalKernelTime: Option<u64>,
    /// Total amount of user-mode execution time, in 100 nanosecond units, for all active processes associated with 
    /// the job (and all terminated processes no longer associated with the job) since the last call that set a 
    /// per-job user-mode time limit. This property is set to 0 (zero) when a job is created, and each time a 
    /// per-job user-mode time limit is established.
    pub ThisPeriodTotalUserTime: Option<u64>,
    /// Total amount of kernel-mode execution time, in 100 nanosecond units, for all active processes associated with 
    /// the job, as well as all terminated processes no longer associated with the job.
    pub TotalKernelTime: Option<u64>,
    /// Total number of page faults encountered by all of the active processes associated with the job, as well as all 
    /// of the terminated processes no longer associated with the job.
    pub TotalPageFaultCount: Option<u32>,
    /// Total number of processes associated with the job during its lifetime, including those that are terminated. 
    /// For example, when a process is associated with a job, but the association fails because of a limit violation, 
    /// the value is incremented.
    pub TotalProcesses: Option<u32>,
    /// Total number of processes terminated because of a limit violation.
    pub TotalTerminatedProcesses: Option<u32>,
    /// Total amount of user-mode execution time, in 100 nanosecond units, for all active processes associated with a 
    /// job, and all of the terminated processes not associated with a job.
    pub TotalUserTime: Option<u64>,
    /// Number of write operations performed by all processes that have been associated with a job, and all of the 
    /// processes currently associated with the job.
    pub WriteOperationCount: Option<u64>,
    /// Number of bytes written by all of the processes that have been associated with a job, and all processes 
    /// currently associated with a job.
    pub WriteTransferCount: Option<u64>,
}

/// The `Win32_NamedJobObjectLimitSetting` WMI class represents the limit settings for a job object.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipjobobjprov/win32-namedjobobjectlimitsetting>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NamedJobObjectLimitSetting {
    /// Short textual description of the `CIM_Setting` object.
    pub Caption: Option<String>,
    /// Textual description of the `CIM_Setting` object.
    pub Description: Option<String>,
    /// Active process limit for a job. If associating a process with a job causes the active process count to exceed 
    /// the limit, the process is terminated and the association fails. This property is ignored unless the LimitFlags 
    /// property specifies the Active Process Limit value.
    pub ActiveProcessLimit: Option<u32>,
    /// Processor affinity for all of the processes associated with a job. The affinity of each thread is set to this 
    /// value, but threads are free to set the affinity subsequently, when it is a subset of the specified affinity 
    /// mask. Processes cannot set an affinity mask. This property is ignored unless `LimitFlags` specifies the Limit 
    /// Affinity value.
    pub Affinity: Option<u32>,
    /// Per-job memory limit in kilobytes. This property is ignored unless `LimitFlags` specifies the Limit Job Memory 
    /// value.
    pub JobMemoryLimit: Option<u32>,
    /// Bitmap that represents the job limits. One or more of the limits can be in effect at the same time.
    /// 
    /// - 1 (0x1): Limit Working Set [Causes all of the processes associated with the job to use the same minimum and maximum working set sizes]
    /// - 2 (0x2): Limit Process Time [Establishes a user-mode execution time limit for each currently active process and all of the future processes associated with a job]
    /// - 4 (0x4): Limit Job Time [Establishes a user-mode execution time limit for the job. This flag cannot be used with Limit Preserve Job Time]
    /// - 8 (0x8): Active Process Limit [Establishes a maximum number of simultaneously active processes associated with the job]
    /// - 16 (0x10): Limit Affinity [Causes all of the processes associated with the job to use the same processor affinity]
    /// - 32 (0x20): Limit Priority Class [Causes all of the processes associated with a job to use the same priority class]
    /// - 64 (0x40): Limit Preserve Job Time [Preserves any job time limits you set previously. When this flag is set, you can establish a per-job time limit one time, then alter other limits in subsequent calls. This flag cannot be used with Limit Job Time]
    /// - 128 (0x80): Limit Scheduling Class [Causes all of the processes in a job to use the same scheduling class]
    /// - 256 (0x100): Limit Process Memory [Causes all of the processes associated with a job to limit their committed memory. When a process attempts to commit memory that exceeds the perprocess limit, it fails. If the job object is associated with a completion port, a `JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT` message is sent to the completion port]
    /// - 512 (0x200): Limit Job Memory [Causes all of the processes associated with a job to limit the job-wide sum of the committed memory. When a process attempts to commit memory that exceeds the job-wide limit, it fails. If the job object is associated with a completion port, a `JOB_OBJECT_MSG_JOB_MEMORY_LIMIT` message is sent to the completion port]
    /// - 1024 (0x400): Limit Die On Unhandled Exception [Forces a call to the `SetErrorMode` function with the `SEM_NOGPFAULTERRORBOX` flag for each process associated with a job]
    /// - 2048 (0x800): Limit Breakaway OK [If any of the process associated with a job creates a child process using the `CREATE_BREAKAWAY_FROM_JOB` flag while this limit is in effect, the child process is not associated with the job]
    /// - 4096 (0x1000): Silent Breakaway OK [Allows any process associated with a job to create child processes that are not associated with the job]
    pub LimitFlags: Option<u32>,
    /// Maximum working set size for all of the processes associated with a job. This property is ignored unless 
    /// `LimitFlags` specifies the Limit Working Set value.
    pub MaximumWorkingSetSize: Option<u32>,
    /// Minimum working set size for all of the processes associated with a job. This property is ignored unless 
    /// `LimitFlags` specifies the Limit Working Set value.
    pub MinimumWorkingSetSize: Option<u32>,
    /// Per-job user-mode execution time limit, in 100 nanosecond units. The system adds the current time of the 
    /// processes associated with the job to this limit. For example, if you set this limit to 1 minute, and the 
    /// job has a process that has accumulated 5 minutes of user mode time, the limit actually enforced is 6 minutes.
    pub PerJobUserTimeLimit: Option<u64>,
    /// Per-process user-mode execution time limit, in 100-nanosecond units. This property is ignored unless `LimitFlags` 
    /// specifies Limit Process Time. The system periodically checks to determine whether or not each process 
    /// associated with the job has accumulated more user-mode time than the set limit. If it has, the process is 
    /// terminated.
    pub PerProcessUserTimeLimit: Option<u64>,
    /// Priority class for all of the processes associated with the job. Processes and threads cannot modify their 
    /// priority class. This property is ignored unless `LimitFlags` specifies the Limit Priority value.
    pub PriorityClass: Option<u32>,
    /// Per-process memory limit in kilobytes. This property is ignored unless `LimitFlags` specifies the Limit Process 
    /// Memory value.
    pub ProcessMemoryLimit: Option<u32>,
    /// Scheduling class for all of the processes associated with the job. The valid values are 0 (zero) to 9 (nine). 
    /// Use 0 (zero) for the least favorable scheduling class relative to other threads, and 9 (nine) for the most 
    /// favorable scheduling class relative to other threads. This property is ignored unless `LimitFlags` specifies the 
    /// Limit Scheduling Class value.
    pub SchedulingClass: Option<u32>,
    /// Job object limit setting instance. Because they are kernel objects, job object names are case-sensitive. 
    /// However, Windows Management Instrumentation (WMI) keys are case-insensitive and must be decorated to 
    /// distinguish case. To indicate a capital letter, precede the letter by a backslash. For example, "A" and "a" 
    /// are lowercase and "\A" and "\a" are uppercase.
    pub SettingID: Option<String>,
}

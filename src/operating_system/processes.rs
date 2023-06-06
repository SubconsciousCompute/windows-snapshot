//! The Processes subcategory groups classes that represent system processes and threads.
//!
//! | Class                                                 | Description                                                                                                     |
//! |-------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Process**](Win32_Process)               | Instance class<br/> Represents a sequence of events on a computer system running Windows.<br/>      |
//! | [**Win32\_Thread**](Win32_Thread)                 | Instance class<br/> Represents a thread of execution.<br/>                                          |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows Processes
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Processes {
    /// Sequence of Process based on when they were launched in chronological order
    pub processes: Vec<Win32_Process>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Processes, processes);

/// Represents the state of Windows threads
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Threads {
    /// Sequence of Threads based on when they were launched in chronological order
    pub threads: Vec<Win32_Thread>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Threads, threads);

/// The `Win32_Process` WMI class represents a process on an operating system.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Process {
    /// Name of the class or subclass used in the creation of an instance. When used with other key
    /// properties of the class, this property allows all instances of the class and its subclasses
    /// to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Short description of an object—a one-line string.
    pub Caption: Option<String>,
    /// Command line used to start a specific process, if applicable.
    pub CommandLine: Option<String>,
    /// Date the process begins executing.
    pub CreationDate: Option<WMIDateTime>,
    /// Creation class name of the scoping computer system.
    pub CSCreationClassName: Option<String>,
    /// Name of the scoping computer system.
    pub CSName: Option<String>,
    /// Description of an object.
    pub Description: Option<String>,
    /// Path to the executable file of the process.
    ///
    /// Example: "C:\Windows\System\Explorer.Exe"
    pub ExecutablePath: Option<String>,
    /// Current operating condition of the process.
    ///
    /// - Unknown (0)
    /// - Other (1)
    /// - Ready (2)
    /// - Running (3)
    /// - Blocked (4)
    /// - Suspended Blocked (5)
    /// - Suspended Ready (6)
    /// - Terminated (7)
    /// - Stopped (8)
    /// - Growing (9)
    pub ExecutionState: Option<u16>,
    /// Process identifier.
    pub Handle: Option<String>,
    /// Total number of open handles owned by the process. HandleCount is the sum of the handles
    /// currently open by each thread in this process. A handle is used to examine or modify the
    /// system resources. Each handle has an entry in a table that is maintained internally. Entries
    /// contain the addresses of the resources and data to identify the resource type.
    pub HandleCount: Option<u32>,
    /// Date an object is installed. The object may be installed without a value being written to
    /// this property.
    pub InstallDate: Option<WMIDateTime>,
    /// Time in kernel mode, in milliseconds. If this information is not available, use a value of 0
    /// (zero).
    pub KernelModeTime: Option<u64>,
    /// Maximum working set size of the process. The working set of a process is the set of memory
    /// pages visible to the process in physical RAM. These pages are resident, and available for an
    /// application to use without triggering a page fault.
    ///
    /// Example: 1413120
    pub MaximumWorkingSetSize: Option<u32>,
    /// Minimum working set size of the process. The working set of a process is the set of memory
    /// pages visible to the process in physical RAM. These pages are resident and available for an
    /// application to use without triggering a page fault.
    ///
    /// Example: 20480
    pub MinimumWorkingSetSize: Option<u32>,
    /// Name of the executable file responsible for the process, equivalent to the Image Name
    /// property in Task Manager.
    ///
    /// When inherited by a subclass, the property can be overridden to be a key property. The name
    /// is hard-coded into the application itself and is not affected by changing the file name.
    /// For example, even if you rename Calc.exe, the name Calc.exe will still appear in Task
    /// Manager and in any WMI scripts that retrieve the process name.
    pub Name: Option<String>,
    /// Creation class name of the scoping operating system.
    pub OSCreationClassName: Option<String>,
    /// Name of the scoping operating system.
    pub OSName: Option<String>,
    /// Number of I/O operations performed that are not read or write operations.
    pub OtherOperationCount: Option<u64>,
    /// Amount of data transferred during operations that are not read or write operations.
    pub OtherTransferCount: Option<u64>,
    /// Number of page faults that a process generates.
    ///
    /// Example: 10
    pub PageFaults: Option<u32>,
    /// Amount of page file space that a process is using currently. This value is consistent with
    /// the VMSize value in TaskMgr.exe.
    ///
    /// Example: 102435
    pub PageFileUsage: Option<u32>,
    /// Unique identifier of the process that creates a process. Process identifier numbers are
    /// reused, so they only identify a process for the lifetime of that process. It is possible
    /// that the process identified by ParentProcessId is terminated, so `ParentProcessId` may not
    /// refer to a running process. It is also possible that ParentProcessId incorrectly refers to
    /// a process that reuses a process identifier. You can use the `CreationDate` property to
    /// determine whether the specified parent was created after the process represented by this
    /// `Win32_Process` instance was created.
    pub ParentProcessId: Option<u32>,
    /// Maximum amount of page file space used during the life of a process.
    ///
    /// Example: 102367
    pub PeakPageFileUsage: Option<u32>,
    /// Maximum virtual address space a process uses at any one time. Using virtual address space
    /// does not necessarily imply corresponding use of either disk or main memory pages. However,
    /// virtual space is finite, and by using too much the process might not be able to load
    /// libraries.
    pub PeakVirtualSize: Option<u64>,
    /// Peak working set size of a process.
    ///
    /// Example: 1413120
    pub PeakWorkingSetSize: Option<u32>,
    /// Scheduling priority of a process within an operating system. The higher the value, the
    /// higher priority a process receives. Priority values can range from 0 (zero), which is the
    /// lowest priority to 31, which is highest priority.
    ///
    /// Example: 7
    pub Priority: Option<u32>,
    /// Current number of pages allocated that are only accessible to the process represented by
    /// this `Win32_Process` instance.
    pub PrivatePageCount: Option<u64>,
    /// Numeric identifier used to distinguish one process from another. ProcessIDs are valid from
    /// process creation time to process termination. Upon termination, that same numeric identifier
    /// can be applied to a new process.
    ///
    /// This means that you cannot use ProcessID alone to monitor a particular process. For example,
    /// an application could have a ProcessID of 7, and then fail. When a new process is started,
    /// the new process could be assigned ProcessID 7. A script that checked only for a specified
    /// `ProcessID` could thus be "fooled" into thinking that the original application was still
    /// running.
    pub ProcessId: Option<u32>,
    /// Quota amount of nonpaged pool usage for a process.
    ///
    /// Example: 15
    pub QuotaNonPagedPoolUsage: Option<u32>,
    /// Quota amount of paged pool usage for a process.
    ///
    /// Example: 22
    pub QuotaPagedPoolUsage: Option<u32>,
    /// Peak quota amount of nonpaged pool usage for a process.
    ///
    /// Example: 31
    pub QuotaPeakNonPagedPoolUsage: Option<u32>,
    /// Peak quota amount of paged pool usage for a process.
    ///
    /// Example: 31
    pub QuotaPeakPagedPoolUsage: Option<u32>,
    /// Number of read operations performed.
    pub ReadOperationCount: Option<u64>,
    /// Amount of data read.
    pub ReadTransferCount: Option<u64>,
    /// Unique identifier that an operating system generates when a session is created. A session
    /// spans a period of time from logon until logoff from a specific system.
    pub SessionId: Option<u32>,
    /// This property is not implemented and does not get populated for any instance of this class.
    /// It is always NULL.
    pub Status: Option<String>,
    /// Process was stopped or terminated.
    /// To get the termination time, a handle to the process must be held open.
    /// Otherwise, this property returns NULL.
    pub TerminationDate: Option<WMIDateTime>,
    /// Number of active threads in a process.
    /// An instruction is the basic unit of execution in a processor,
    /// and a thread is the object that executes an instruction.
    /// Each running process has at least one thread.
    pub ThreadCount: Option<u32>,
    /// Time in user mode, in 100 nanosecond units. If this information is not available, use a
    /// value of 0 (zero).
    pub UserModeTime: Option<u64>,
    /// Current size of the virtual address space that a process is using,
    /// not the physical or virtual memory actually used by the process.
    /// Using virtual address space does not necessarily imply corresponding use of either disk or main memory pages.
    /// Virtual space is finite, and by using too much, the process might not be able to load libraries.
    /// This value is consistent with what you see in Perfmon.exe.
    pub VirtualSize: Option<u64>,
    /// Version of Windows in which the process is running.
    ///
    /// Example: 4.0
    pub WindowsVersion: Option<String>,
    /// Amount of memory in bytes
    /// that a process needs
    /// to execute efficiently—for an operating system that uses page-based memory management.
    /// If the system does not have enough memory (less than the working set size), thrashing occurs.
    /// If the size of the working set is not known, use NULL or 0 (zero).
    /// If working set data is provided,
    /// you can monitor the information to understand the changing memory requirements of a process.
    pub WorkingSetSize: Option<u64>,
    /// Number of write operations performed.
    pub WriteOperationCount: Option<u64>,
    /// Amount of data written.
    pub WriteTransferCount: Option<u64>,
}

/// The `Win32_Thread` WMI class represents a thread of execution. While a process must have one
/// thread of execution, the process can create other threads to execute tasks in parallel. Threads
/// share the process environment, thus multiple threads under the same process use less memory than
/// the same number of processes.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-thread>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Thread {
    /// Short description of the object.
    pub Caption: Option<String>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of
    /// an instance. When used with the other key properties of the class, this property allows all
    /// instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Creation class name of the scoping computer system.
    pub CSCreationClassName: Option<String>,
    /// Name of the scoping computer system.
    pub CSName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Total execution time, in milliseconds, given to this thread since its creation.
    pub ElapsedTime: Option<u64>,
    /// Current operating condition of the thread.
    ///
    /// - Unknown (0)
    /// - Other (1)
    /// - Ready (2)
    /// - Running (3)
    /// - Blocked (4)
    /// - Suspended Blocked (5)
    /// - Suspended Ready (6)
    pub ExecutionState: Option<u16>,
    /// Handle to a thread. The handle has full access rights by default. With the correct security
    /// access, the handle can be used in any function that accepts a thread handle. Depending on
    /// the inheritance flag specified when it is created, this handle can be inherited by child
    /// processes.
    pub Handle: Option<String>,
    /// Object was installed. This property does not need a value to indicate that the object is
    /// installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Time in kernel mode, in 100 nanosecond units. If this information is not available, a value
    /// of 0 (zero) should be used.
    pub KernelModeTime: Option<u64>,
    /// Label by which the object is known. When subclassed, the property can be overridden to be a
    /// key property.
    pub Name: Option<String>,
    /// Creation class name of the scoping operating system.
    pub OSCreationClassName: Option<String>,
    /// Name of the scoping operating system.
    pub OSName: Option<String>,
    /// Dynamic priority of the thread. Each thread has a dynamic priority that the scheduler uses
    /// to determine which thread to execute. Initially, a thread's dynamic priority is the same as
    /// its base priority. The system can raise and lower the dynamic priority, to ensure that it is
    /// responsive (guaranteeing that no threads are starved for processor time). The system does
    /// not boost the priority of threads with a base priority level between 16 and 31. Only threads
    /// with a base priority between 0 and 15 receive dynamic priority boosts. Higher numbers
    /// indicate higher priorities.
    pub Priority: Option<u32>,
    /// Current base priority of a thread. The operating system may raise the thread's dynamic
    /// priority above the base priority if the thread is handling user input, or lower it toward
    /// the base priority if the thread becomes compute-bound. The PriorityBase property can have a
    /// value between 0 and 31.
    pub PriorityBase: Option<u32>,
    /// Value of the scoping process `CreationClassName` property.
    pub ProcessCreationClassName: Option<String>,
    /// Process that created the thread. The contents of this property can be used by Windows
    /// application programming interface (API) elements.
    pub ProcessHandle: Option<String>,
    /// Starting address of the thread. Because any application with appropriate access to the
    /// thread can change the thread's context, this value may only be an approximation of the
    /// thread's starting address.
    pub StartAddress: Option<u32>,
    /// Current status of the object. Various operational and nonoperational statuses can be
    /// defined. Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such
    /// as a SMART-enabled hard disk drive, may be functioning properly but predicting a failure in
    /// the near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and
    /// "Service". The latter, "Service", could apply during mirror-resilvering of a disk, reload of
    /// a user permissions list, or other administrative work. Not all such work is online, yet the
    /// managed element is neither "OK" nor in one of the other states.
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
    /// Current execution state for the thread.
    ///
    /// - Initialized (0) — It is recognized by the microkernel.
    /// - Ready (1) — It is prepared to run on the next available processor.
    /// - Running (2) — It is executing.
    /// - Standby (3) — It is about to run, only one thread may be in this state at a time.
    /// - Terminated (4) — It is finished executing.
    /// - Waiting (5) — It is not ready for the processor, when ready, it will be rescheduled.
    /// - Transition (6) — The thread is waiting for resources other than the processor,
    /// - Unknown (7) — The thread state is unknown.
    pub ThreadState: Option<u32>,
    /// Reason why the thread is waiting. This value is only valid if the `ThreadState` member is set
    /// to Transition (6). Event pairs allow communication with protected subsystems.
    ///
    /// - Executive (0)
    /// - FreePage (1)
    /// - PageIn (2)
    /// - PoolAllocation (3)
    /// - ExecutionDelay (4)
    /// - FreePage (5)
    /// - PageIn (6)
    /// - Executive (7)
    /// - FreePage (8)
    /// - PageIn (9)
    /// - PoolAllocation (10)
    /// - ExecutionDelay (11)
    /// - FreePage (12)
    /// - PageIn (13)
    /// - EventPairHigh (14)
    /// - EventPairLow (15)
    /// - LPCReceive (16)
    /// - LPCReply (17)
    /// - VirtualMemory (18)
    /// - PageOut (19)
    /// - Unknown (20)
    pub ThreadWaitReason: Option<u32>,
    /// Time in user mode, in 100 nanoseconds units. If this information is not available, a value
    /// of 0 (zero) should be used.
    pub UserModeTime: Option<u64>,
}

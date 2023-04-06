//! The Drivers subcategory groups classes that represent virtual device drivers and system drivers for base services.
//!
//! | Class                                             | Description                                                                           |
//! |---------------------------------------------------|---------------------------------------------------------------------------------------|
//! | [**Win32\_SystemDriver**](Win32_SystemDriver) | Instance class<br/> Represents the system driver for a base service.<br/> |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows Drivers
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Drivers {
    /// Sequence of Drivers based on when they were loaded in chronological order
    pub drivers: Vec<Win32_SystemDriver>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Drivers, drivers);

/// The `Win32_SystemDriver` WMI class represents a process on an operating system.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemdriver>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_SystemDriver {
    /// Service can be paused.
    AcceptPause: Option<bool>,
    /// Service can be stopped.
    AcceptStop: Option<bool>,
    /// Short description of the object.
    Caption: Option<String>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of
    /// an instance. When used with the other key properties of the class, this property allows all
    /// instances of this class and its subclasses to be uniquely identified.
    CreationClassName: Option<String>,
    /// Description of the object.
    Description: Option<String>,
    /// This service can create or communicate with windows on the desktop.
    DesktopInteract: Option<bool>,
    /// Display name of the service. This string has a maximum length of 256 characters. The name is
    /// case-preserved in the Service Control Manager. DisplayName comparisons are always
    /// case-insensitive.
    ///
    /// Constraints: Accepts the same value as the Name property.
    ///
    /// Example: "Atdisk"
    DisplayName: Option<String>,
    /// Severity of the error if this service fails to start during startup. This value indicates
    /// the action taken by the startup program if failure occurs. All errors are logged by the
    /// computer system.
    ///
    /// - Ignore ("Ignore"): User is not notified.
    /// - Normal ("Normal"): User is notified.
    /// - Severe ("Severe"): System is restarted with the last-known-good configuration.
    /// - Critical ("Critical"): System attempts to restart with a good configuration.
    /// - Unknown ("Unknown"): Cause of the failure is unknown.
    ErrorControl: Option<String>,
    /// Windows error code defining any problems encountered in starting or stopping the service.
    /// This property is set to `ERROR_SERVICE_SPECIFIC_ERROR (1066)` when the error is unique to the
    /// service represented by this class, and information about the error is available in the
    /// `ServiceSpecificExitCode` property. The service sets this value to `NO_ERROR` when running, and
    /// again upon normal termination.
    ExitCode: Option<u32>,
    /// Object was installed. This property does not need a value to indicate that the object is
    /// installed.
    InstallDate: Option<WMIDateTime>,
    /// Unique identifier for the service which provides an indication of the functionality that is
    /// managed. This functionality is described in more detail in the object Description property.
    Name: Option<String>,
    /// Fully qualified path to the service binary file that implements the service.
    ///
    /// Example: "\SystemRoot\System32\drivers\afd.sys"
    PathName: Option<String>,
    /// Service-specific error code for errors that occur while the service is either starting or
    /// stopping. The exit codes are defined by the service represented by this class. This value is
    /// only set when the ExitCode property value is `ERROR_SERVICE_SPECIFIC_ERROR (1066)`.
    ServiceSpecificExitCode: Option<u32>,
    /// Type of service provided to calling processes.
    ///
    /// The values are:
    ///
    /// - Kernel Driver ("Kernel Driver")
    /// - File System Driver ("File System Driver")
    /// - Adapter ("Adapter")
    /// - Recognizer Driver ("Recognizer Driver")
    /// - Own Process ("Own Process")
    /// - Share Process ("Share Process")
    /// - Interactive Process ("Interactive Process")
    ServiceType: Option<String>,
    /// Service has been started.
    Started: Option<bool>,
    /// Start mode of the system driver.
    ///
    /// - Boot ("Boot"): Device driver started by the operating system loader (valid only for driver services).
    /// - System ("System"): Device driver started by the operating system initialization process. This value is valid only for driver services.
    /// - Auto ("Auto"): Service to be started automatically by the service control manager during system start up.
    /// - Manual ("Manual"): Service to be started by the service control manager when a process calls the StartService method.
    /// - Disabled ("Disabled"): Service that can no longer be started.
    StartMode: Option<String>,
    /// Account name under which the service runs. Depending on the service type, the account name
    /// may be in the form of DomainName\Username. The service process will be logged using one of
    /// these two forms when it runs. If the account belongs to the built-in domain, .\Username can
    /// be specified. If NULL is specified, the service will be logged on as the LocalSystem
    /// account. For kernel or system-level drivers, StartName contains the driver object name
    /// (that is, \FileSystem\Rdr or \Driver\Xns) which the input and output (I/O) system uses to
    /// load the device driver. Additionally, if NULL is specified, the driver runs with a default
    /// object name created by the I/O system based on the service name.
    ///
    /// Example: "DWDOM\Admin"
    StartName: Option<String>,
    /// Current state of the base service.
    ///
    /// The values are:
    ///
    /// - Stopped ("Stopped")
    /// - Start Pending ("Start Pending")
    /// - Stop Pending ("Stop Pending")
    /// - Running ("Running")
    /// - Continue Pending ("Continue Pending")
    /// - Pause Pending ("Pause Pending")
    /// - Paused ("Paused")
    /// - Unknown ("Unknown")
    State: Option<String>,
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
    Status: Option<String>,
    /// Type name of the system that hosts this service.
    SystemCreationClassName: Option<String>,
    /// Name of the system that hosts this service.
    SystemName: Option<String>,
    /// Unique tag value for this service in the group. A value of 0 (zero) indicates that the
    /// service has not been assigned a tag. A tag can be used for ordering service startup within
    /// a load order group by specifying a tag order vector in the registry located at:
    ///
    /// `HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\GroupOrderList.`
    ///
    /// Tags are only evaluated for Kernel Driver and File System Driver start-type services that
    /// have Boot or System start modes.
    TagId: Option<u32>,
}

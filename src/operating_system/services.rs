//! The Services subcategory groups classes that represent services and base services.
//!
//! | Class                                           | Description                                                                                                                                             |
//! |-------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**`Win32\_Servic`e**](win32-service)         | Instance class<br/> Represents a service on a computer system running Windows.<br/>                                                         |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows Drivers
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Services {
    /// Sequence of Drivers based on when they were loaded in chronological order
    pub services: Vec<Win32_Service>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Services, services);

/// The `Win32_Service` WMI class represents a process on an operating system.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-service>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Service {
    /// Indicates whether the service can be paused.
    pub AcceptPause: Option<bool>,
    /// Indicates whether the service can be stopped.
    pub AcceptStop: Option<bool>,
    /// Short description of the service — a one-line string.
    pub Caption: Option<String>,
    /// Value that the service increments periodically to report its progress during a long start,
    /// stop, pause, or continue operation. For example, the service increments this value as it
    /// completes each step of its initialization when it is starting up. The user interface program
    /// that invokes the operation on the service uses this value to track the progress of the
    /// service during a lengthy operation. This value is not valid and should be zero when the
    /// service does not have a start, stop, pause, or continue operation pending.
    pub CheckPoint: Option<u32>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of
    /// an instance. When used with the other key properties of the class, this property allows all
    /// instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// If True, the service is started after other auto-start services are started plus a short
    /// delay.
    ///
    /// Windows Server 2012 R2, Windows 8.1, Windows Server 2012, Windows 8, Windows Server 2008 R2,
    /// Windows 7, Windows Server 2008 and Windows Vista: This property is not supported before
    /// Windows Server 2016 and Windows 10.
    pub DelayedAutoStart: Option<bool>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Indicates whether the service can create or communicate with windows on the desktop, and
    /// thus interact in some way with a user. Interactive services must run under the Local System
    /// account. Most services are not interactive; that is, they do not communicate with the user
    /// in any way.
    pub DesktopInteract: Option<bool>,
    /// Name of the service as viewed in the Services snap-in. This string has a maximum length of
    /// 256 characters. Note that the display name and the service name (which is stored in the
    /// registry) are not always the same. For example, the DHCP Client service has the service
    /// name Dhcp but the display name DHCP Client. The name is case-preserved in the Service
    /// Control Manager. However, DisplayName comparisons are always case-insensitive.
    ///
    /// Constraint: Accepts the same value as the Name property.
    ///
    /// Example: "Atdisk"
    pub DisplayName: Option<String>,
    /// Severity of the error if this service fails to start during startup. The value indicates
    /// the action taken by the startup program if failure occurs. All errors are logged by the
    /// computer system.
    ///
    /// - Ignore ("Ignore"): User is not notified.
    /// - Normal ("Normal"): User is notified. Usually this will be a message box display notifying the user of the problem.
    /// - Severe ("Severe"): System is restarted with the last-known-good configuration.
    /// - Critical ("Critical"): System attempts to restart with a good configuration. If the service fails to start a second time, startup fails.
    /// - Unknown ("Unknown"): Severity of the error is unknown.
    pub ErrorControl: Option<String>,
    /// Windows error code that defines errors encountered in starting or stopping the service.
    /// This property is set to ERROR_SERVICE_SPECIFIC_ERROR (1066) when the error is unique to the
    /// service represented by this class, and information about the error is available in the
    /// ServiceSpecificExitCode property. The service sets this value to NO_ERROR when running, and
    /// again upon normal termination.
    pub ExitCode: Option<u32>,
    /// Date object is installed. This property does not require a value to indicate that the object
    /// is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Unique identifier of the service that provides an indication of the functionality that is
    /// managed. This functionality is described in the Description property of the object.
    pub Name: Option<String>,
    /// Fully qualified path to the service binary file that implements the service.
    ///
    /// Example: "\SystemRoot\System32\drivers\afd.sys"
    pub PathName: Option<String>,
    /// Process identifier of the service.
    ///
    /// Example: 324
    pub ProcessId: Option<u32>,
    /// Service-specific error code for errors that occur while the service is either starting or
    /// stopping. The exit codes are defined by the service represented by this class. This value
    /// is only set when the ExitCode property value is ERROR_SERVICE_SPECIFIC_ERROR (1066).
    pub ServiceSpecificExitCode: Option<u32>,
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
    pub ServiceType: Option<String>,
    /// Indicates whether or not the service is started.
    pub Started: Option<bool>,
    /// Start mode of the Windows base service.
    ///
    /// - Boot ("Boot"): Device driver started by the operating system loader (valid only for driver services).
    /// - System ("System"): Device driver started by the operating system initialization process. This value is valid only for driver services.
    /// - Auto ("Auto"): Service to be started automatically by the service control manager during system startup. Auto services are started even if a user does not log on.
    /// - Manual ("Manual"): Service to be started by the Service Control Manager when a process calls the StartService method. These services do not start unless a user logs on and starts them.
    /// - Disabled ("Disabled"): Service that cannot be started until its StartMode is changed to either Auto or Manual.
    pub StartMode: Option<String>,
    /// Account name under which a service runs. Depending on the service type, the account name
    /// may be in the form of "DomainName\Username" or UPN format ("Username@DomainName"). The
    /// service process is logged by using one of these two forms when it runs. If the account
    /// belongs to the built-in domain, then ".\Username" can be specified. For kernel or
    /// system-level drivers, StartName contains the driver object name (that is, "\FileSystem\Rdr"
    /// or "\Driver\Xns") which the I/O system uses to load the device driver. Additionally, if NULL
    /// is specified, the driver runs with a default object name created by the I/O system based on
    /// the service name.
    ///
    /// Example: "DWDOM\Admin"
    pub StartName: Option<String>,
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
    pub State: Option<String>,
    /// Current status of the object. Various operational and nonoperational statuses can be
    /// defined. Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element,
    /// such as a SMART-enabled hard disk drive, may be functioning properly but predicting a
    /// failure in the near future). Nonoperational statuses include: "Error", "Starting",
    /// "Stopping", and "Service". The latter, "Service", could apply during mirror-resilvering
    /// of a disk, reload of a user permissions list, or other administrative work. Not all such
    /// work is online, yet the managed element is neither "OK" nor in one of the other states.
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
    /// Type name of the system that hosts this service.
    pub SystemCreationClassName: Option<String>,
    /// Name of the system that hosts this service.
    pub SystemName: Option<String>,
    /// Unique tag value for this service in the group. A value of 0 (zero) indicates that the
    /// service does not have a tag. A tag can be used to order service startup within a load order
    /// group by specifying a tag order vector in the registry located at:
    ///
    /// `HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\    GroupOrderList`
    ///
    /// Tags are only evaluated for Kernel Driver and File System Driver start type services that
    /// have Boot or System start modes.
    pub TagId: Option<u32>,
    /// Estimated time required, in milliseconds, for a pending start, stop, pause, or continue
    /// operation. After the specified time has elapsed, the service makes its next call to the
    /// SetServiceStatus method with either an incremented CheckPoint value or a change in
    /// CurrentState. If the amount of time specified by WaitHint passes, and CheckPoint has not
    /// been incremented, or CurrentState has not changed, the service control manager or service
    /// control program assumes that an error has occurred.
    pub WaitHint: Option<u32>,
}

//! The class in the Registry subcategory represents the contents of the Windows registry.
//!
//! | Class                                     | Description                                                                                               |
//! |-------------------------------------------|-----------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Registry**](Win32_Registry) | Instance class<br/> Represents the system registry on a computer system running Windows.<br/> |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows Registry
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Registry {
    /// Sequence of Registry
    pub registries: Vec<Win32_Registry>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Registry, registries);

/// The `Win32_Registry` WMI class represents a process on an operating system.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-registry>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Registry {
    /// A short textual description of the object.
    Caption: Option<String>,
    /// A textual description of the object.
    Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object
    /// is not installed.
    InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object. Operational and non-operational
    /// status can be defined. Operational status can include "OK", "Degraded", and "Pred Fail".
    /// "Pred Fail" indicates that an element is functioning properly, but is predicting a failure
    /// (for example, a SMART-enabled hard disk drive).
    ///
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service"
    /// can apply during disk mirror-resilvering, reloading a user permissions list, or other
    /// administrative work. Not all such work is online, but the managed element is neither "OK"
    /// nor in one of the other states.
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
    Status: Option<String>,
    /// Current physical size of the Windows registry.
    ///
    /// Example: 10
    CurrentSize: Option<u32>,
    /// Maximum size of the Windows registry. If the system is successful in using the ProposedSize
    /// property, MaximumSize should contain the same value.
    MaximumSize: Option<u32>,
    /// Name of the Windows registry. The maximum length is 256 characters.
    Name: Option<String>,
    /// Proposed size of the Windows registry. It is the only registry setting that can be modified,
    /// and its proposal is attempted the next time the system boots.
    ProposedSize: Option<u32>,
}

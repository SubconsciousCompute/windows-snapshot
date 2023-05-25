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

/// Represents the state of Windows LUIDs
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LUIDs {
    /// Represents sequence of Windows `LUIDs`
    pub luids: Vec<Win32_LUID>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LUIDs, luids);

/// Represents the state of Windows LUIDandAttributes
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LUIDandAttributes {
    /// Represents sequence of Windows `LUIDs`
    pub luid_and_attributes: Vec<Win32_LUIDandAttributes>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LUIDandAttributes, luid_and_attributes);

/// Represents the state of Windows NamedJobObjects
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NamedJobObjects {
    /// Represents sequence of Windows `NamedJobObjects`
    pub named_job_objects: Vec<Win32_NamedJobObject>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(NamedJobObjects, named_job_objects);

/// The `Win32_LUID` abstract WMI class represents a locally unique identifier (LUID), an identifier unique on the 
/// local computer that is used in security tokens.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipjobobjprov/win32-luid>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
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
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
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
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
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

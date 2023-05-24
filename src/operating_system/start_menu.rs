//! The Start Menu subcategory groups classes that represent program groups.
//! 
//! | Class                                                                                   | Description                                                                                                                                                              |
//! |-----------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_LogicalProgramGroup**](win32-logicalprogramgroup.md)                         | Instance class<br/> Represents a program group in a computer system running Windows.<br/>                                                                    |
//! | [**Win32\_LogicalProgramGroupDirectory**](win32-logicalprogramgroupdirectory.md)       | Association class<br/> Relates logical program groups (groupings in the Start menu), and the file directories in which they are stored.<br/>                 |
//! | [**Win32\_LogicalProgramGroupItem**](win32-logicalprogramgroupitem.md)                 | Instance class<br/> Represents an element contained by a **Win32\_ProgramGroup** instance, that is not itself another **Win32\_ProgramGroup** instance.<br/> |
//! | [**Win32\_LogicalProgramGroupItemDataFile**](win32-logicalprogramgroupitemdatafile.md) | Association class<br/> Relates the program group items of the Start menu, and the files in which they are stored.<br/>                                       |
//! | [**Win32\_ProgramGroupContents**](win32-programgroupcontents.md)                       | Association class<br/> Relates a program group order and an individual program group or item contained in it.<br/>                                           |
//! | [**Win32\_ProgramGroupOrItem**](win32-programgrouporitem.md)                           | Instance class<br/> Represents a logical grouping of programs on the user's **Start**\|**Programs** menu.<br/>                                               |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows LogicalProgramGroups
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogicalProgramGroups {
    /// Represents sequence of Windows `LogicalProgramGroups`
    pub logical_program_groups: Vec<Win32_LogicalProgramGroup>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LogicalProgramGroups, logical_program_groups);

/// Represents the state of Windows LogicalProgramGroupItems
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogicalProgramGroupItems {
    /// Represents sequence of Windows `LogicalProgramGroupItems`
    pub logical_program_group_items: Vec<Win32_LogicalProgramGroupItem>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LogicalProgramGroupItems, logical_program_group_items);

/// Represents the state of Windows ProgramGroupOrItems
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProgramGroupOrItems {
    /// Represents sequence of Windows `LogicalProgramGroupItems`
    pub program_group_or_items: Vec<Win32_ProgramGroupOrItem>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(ProgramGroupOrItems, program_group_or_items);

/// The `Win32_LogicalProgramGroup` WMI class represents a program group in a computer system running 
/// Windows. For example, Accessories or Startup.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-logicalprogramgroup>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LogicalProgramGroup {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is 
    /// not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object. Operational and non-operational status 
    /// can be defined. Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" 
    /// indicates that an element is functioning properly, but is predicting a failure (for example, a 
    /// SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can 
    /// apply during disk mirror-resilvering, reloading a user permissions list, or other administrative 
    /// work. Not all such work is online, but the managed element is neither "OK" nor in one of the other 
    /// states.
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
    /// Name of the Windows program group. Program groups are implemented as file folders in Win32.
    /// 
    /// Example: "Accessories\System Tools"
    pub GroupName: Option<String>,
    /// User-assigned name followed by the group name. Program groups are implemented as file folders 
    /// in Win32.
    /// 
    /// Example: "All Users:Accessories\System Tools"
    pub Name: Option<String>,
    /// Users who can access the Windows program group. Program groups are implemented as file folders 
    /// in Win32.
    /// 
    /// Example: "All Users"
    pub UserName: Option<String>,
}

/// The `Win32_LogicalProgramGroupItem` WMI class represents an element contained by a `Win32_LogicalProgramGroup` 
/// that is not also another `Win32_LogicalProgramGroup` instance.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-logicalprogramgroupitem>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LogicalProgramGroupItem {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is 
    /// not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object. Operational and non-operational status 
    /// can be defined. Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" 
    /// indicates that an element is functioning properly, but is predicting a failure (for example, a 
    /// SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" 
    /// can apply during disk mirror-resilvering, reloading a user permissions list, or other 
    /// administrative work. Not all such work is online, but the managed element is neither "OK" nor in 
    /// one of the other states.
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
    /// Instance within a computer system. Program groups are implemented as file folders in Win32. 
    /// Full path names should be provided.
    /// 
    /// Example: "C:\Users\someone\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Accessories\NotePad.Lnk"
    pub Name: Option<String>,
}

/// The `Win32_ProgramGroupOrItem` abstract WMI class represents a logical grouping of programs on the 
/// user's `Start\Programs` menu. It contains program groups and program group items.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-programgrouporitem>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ProgramGroupOrItem {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object 
    /// is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be 
    /// a key property.
    pub Name: Option<String>,
    /// String that indicates the current status of the object. Operational and non-operational status 
    /// can be defined. Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" 
    /// indicates that an element is functioning properly, but is predicting a failure (for example, a 
    /// SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can 
    /// apply during disk mirror-resilvering, reloading a user permissions list, or other administrative 
    /// work. Not all such work is online, but the managed element is neither "OK" nor in one of the other 
    /// states.
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
}
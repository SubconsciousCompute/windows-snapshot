//! The Security subcategory groups classes that represent system security settings.
//! 
//! | Class                                                                               | Description                                                                                                                                                  |
//! |-------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_AccountSID**](/previous-versions/windows/desktop/secrcw32prov/win32-accountsid)                                       | Association class<br/> Relates a security account instance with a security descriptor instance.<br/>                                             |
//! | [**Win32\_ACE**](/previous-versions/windows/desktop/secrcw32prov/win32-ace)                                                     | Instance class<br/> Represents an access control entry (ACE).<br/>                                                                               |
//! | [**Win32\_LogicalFileAccess**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalfileaccess)                         | Association class<br/> Relates the security settings of a file or directory and one member of its discretionary access control list (DACL).<br/> |
//! | [**Win32\_LogicalFileAuditing**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalfileauditing)                     | Association class<br/> Relates the security settings of a file or directory one member of its system access control list (SACL).<br/>            |
//! | [**Win32\_LogicalFileGroup**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalfilegroup)                           | Association class<br/> Relates the security settings of a file or directory and its group.<br/>                                                  |
//! | [**Win32\_LogicalFileOwner**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalfileowner)                           | Association class<br/> Relates the security settings of a file or directory and its owner.<br/>                                                  |
//! | [**Win32\_LogicalFileSecuritySetting**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalfilesecuritysetting)       | Instance class<br/> Represents security settings for a logical file.<br/>                                                                        |
//! | [**Win32\_LogicalShareAccess**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalshareaccess)                       | Association class<br/> Relates the security settings of a share and one member of its DACL.<br/>                                                 |
//! | [**Win32\_LogicalShareAuditing**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalshareauditing)                   | Association class<br/> Relates the security settings of a share and one member of its SACL.<br/>                                                 |
//! | [**Win32\_LogicalShareSecuritySetting**](/previous-versions/windows/desktop/secrcw32prov/win32-logicalsharesecuritysetting)     | Instance class<br/> Represents security settings for a logical file.<br/>                                                                        |
//! | [**Win32\_PrivilegesStatus**](win32-privilegesstatus.md)                           | Instance class<br/> Represents information about the privileges required to complete an operation.<br/>                                          |
//! | [**Win32\_SecurityDescriptor**](/previous-versions/windows/desktop/secrcw32prov/win32-securitydescriptor)                       | Instance class<br/> Represents a structural representation of a [**SECURITY\_DESCRIPTOR**](/windows/desktop/api/winnt/ns-winnt-security_descriptor).<br/>                   |
//! | [**Win32\_SecuritySetting**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysetting)                             | Instance class<br/> Represents security settings for a managed element.<br/>                                                                     |
//! | [**Win32\_SecuritySettingAccess**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettingaccess)                 | Instance class<br/> Represents the rights granted and denied to a trustee for a given object.<br/>                                               |
//! | [**Win32\_SecuritySettingAuditing**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettingauditing)             | Instance class<br/> Represents the auditing for a given trustee on a given object.<br/>                                                          |
//! | [**Win32\_SecuritySettingGroup**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettinggroup)                   | Association class<br/> Relates the security of an object and its group.<br/>                                                                     |
//! | [**Win32\_SecuritySettingOfLogicalFile**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettingoflogicalfile)   | Instance class<br/> Represents security settings of a file or directory object.<br/>                                                             |
//! | [**Win32\_SecuritySettingOfLogicalShare**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettingoflogicalshare) | Instance class<br/> Represents security settings of a shared object.<br/>                                                                        |
//! | [**Win32\_SecuritySettingOfObject**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettingofobject)             | Association class<br/> Relates an object to its security settings.<br/>                                                                          |
//! | [**Win32\_SecuritySettingOwner**](/previous-versions/windows/desktop/secrcw32prov/win32-securitysettingowner)                   | Association class<br/> Relates the security settings of an object and its owner.<br/>                                                            |
//! | [**Win32\_SID**](/previous-versions/windows/desktop/secrcw32prov/win32-sid)                                                     | Instance class<br/> Represents an arbitrary SID.<br/>                                                                                            |
//! | [**Win32\_Trustee**](/previous-versions/windows/desktop/secrcw32prov/win32-trustee)                                             | Instance class<br/> Represents a trustee.<br/>                                                                                                   |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection};

/// Represents the state of Windows LogicalFileSecuritySettings
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogicalFileSecuritySettings {
    /// Represents sequence of Windows `LogicalFileSecuritySettings`
    pub logical_file_security_settings: Vec<Win32_LogicalFileSecuritySetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LogicalFileSecuritySettings, logical_file_security_settings);

/// Represents the state of Windows ACEs
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ACEs {
    /// Represents sequence of Windows `ACEs`
    pub aces: Vec<Win32_ACE>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(ACEs, aces);

/// Represents the state of Windows LogicalShareSecuritySettings
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogicalShareSecuritySettings {
    /// Represents sequence of Windows `LogicalFileSecuritySettings`
    pub logical_share_security_settings: Vec<Win32_LogicalShareSecuritySetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LogicalShareSecuritySettings, logical_share_security_settings);

/// Represents the state of Windows PrivilegesStatuses
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PrivilegesStatuses {
    /// Represents sequence of Windows `PrivilegesStatuses`
    pub privileges_statuses: Vec<Win32_PrivilegesStatus>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(PrivilegesStatuses, privileges_statuses);

/// Represents the state of Windows Trustees
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Trustees {
    /// Represents sequence of Windows `Trustees`
    pub trustees: Vec<Win32_Trustee>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Trustees, trustees);

/// Represents the state of Windows SecurityDescriptors
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SecurityDescriptors {
    /// Represents sequence of Windows `SecurityDescriptors`
    pub security_descriptors: Vec<Win32_SecurityDescriptor>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(SecurityDescriptors, security_descriptors);

/// The `Win32_ACE` abstract WMI class specifies an access control entry (ACE). An ACE grants permission 
/// to execute a restricted operation, such as writing to a file or formatting a disk. An ACE that 
/// is specific to WMI allows logon, remote access, method execution, and writing to the WMI repository.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/secrcw32prov/win32-ace>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ACE {
    /// The time, in the CIM_DATETIME format, when the security descriptor was created.
    pub TIME_CREATED: Option<u64>,
    /// Bit flags that indicate rights granted or denied to the trustee. 
    /// 
    /// - `FILE_READ_DATA` (file) or FILE_LIST_DIRECTORY (directory) (1 (0x1)): Grants the right to read data from the file. For a directory, this value grants the right to list the contents of the directory.
    /// - `FILE_WRITE_DATA` (file) or FILE_ADD_FILE (directory) (2 (0x2)): Grants the right to write data to the file. For a directory, this value grants the right to create a file in the directory.
    /// - `FILE_APPEND_DATA` (file) or FILE_ADD_SUBDIRECTORY (directory) (4 (0x4)): Grants the right to append data to the file. For a directory, this value grants the right to create a subdirectory.
    /// - `FILE_READ_EA` (8 (0x8)): Grants the right to read extended attributes.
    /// - `FILE_WRITE_EA` (16 (0x10)): Grants the right to write extended attributes.
    /// - `FILE_EXECUTE` (file) or FILE_TRAVERSE (directory) (32 (0x20)): Grants the right to execute a file. For a directory, the directory can be traversed.
    /// - `FILE_DELETE_CHILD` (64 (0x40)): Grants the right to delete a directory and all the files it contains (its children), even if the files are read-only.
    /// - `FILE_READ_ATTRIBUTES` (128 (0x80)): Grants the right to read file attributes.
    /// - `FILE_WRITE_ATTRIBUTES` (256 (0x100)): Grants the right to change file attributes.
    /// - `DELETE` (65536 (0x10000)): Grants delete access.
    /// - `READ_CONTROL` (131072 (0x20000)): Grants read access to the security descriptor and owner.
    /// - `WRITE_DAC` (262144 (0x40000)): Grants write access to the discretionary access control list (ACL).
    /// - `WRITE_OWNER` (524288 (0x80000)): Assigns the write owner.
    /// - `SYNCHRONIZE` (1048576 (0x100000)): Synchronizes access and allows a process to wait for an object to enter the signaled state.
    pub AccessMask: Option<u32>,
    /// Bit flags that specify inheritance of the ACE. The the relevant permission values for `AceFlags` are 
    /// listed below.
    /// 
    /// - `OBJECT_INHERIT_ACE` (1 (0x1)): Noncontainer child objects inherit the ACE as an effective ACE. For child objects that are containers, the ACE is inherited as an inherit-only ACE unless the NO_PROPAGATE_INHERIT_ACE bit flag is also set.
    /// - `CONTAINER_INHERIT_ACE` (2 (0x2)): Child objects that are containers, such as directories, inherit the ACE as an effective ACE. The inherited ACE is inheritable unless the NO_PROPAGATE_INHERIT_ACE bit flag is also set.
    /// - `NO_PROPAGATE_INHERIT_ACE` (4 (0x4)): If the ACE is inherited by a child object, the system clears the OBJECT_INHERIT_ACE and CONTAINER_INHERIT_ACE flags in the inherited ACE. This prevents the ACE from being inherited by subsequent generations of objects.
    /// - `INHERIT_ONLY_ACE` (8 (0x8)): Indicates an inherit-only ACE which does not control access to the object to which it is attached. If this flag is not set, the ACE is an effective ACE which controls access to the object to which it is attached. Both effective and inherit-only ACEs can be inherited depending on the state of the other inheritance flags.
    /// - `INHERITED_ACE` (16 (0x10)): The system sets this bit when it propagates an inherited ACE to a child object. The two possible values for AceFlags that pertain only to an ACE contained within a system access control list (SACL) are listed below.
    /// - `SUCCESSFUL_ACCESS_ACE_FLAG` (64 (0x40)): Used with system-audit ACEs in an SACL to generate audit messages for successful access attempts.
    /// - `FAILED_ACCESS_ACE_FLAG` (128 (0x80)): Used with system-audit ACEs in an SACL to generate audit messages for failed access attempts.
    pub AceFlags: Option<u32>,
    /// Type of ACE.
    /// - `Access Allowed` (0)
    /// - `Access Denied` (1)
    /// - `Audit` (2)
    pub AceType: Option<u32>,
    /// Globally unique identifier (GUID) associated with the parent of the object to which these rights apply.
    pub GuidInheritedObjectType: Option<String>,
    /// GUID associated with the type of object to which these rights apply.
    pub GuidObjectType: Option<String>,
    /// Object representing the user account, group account, or logon session to which an ACE applies.
    pub Trustee: Option<Win32_Trustee>,
}

/// The `Win32_LogicalFileSecuritySetting` WMI class represents security settings for a logical file. 
/// You cannot enumerate instances of this class.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/secrcw32prov/win32-logicalfilesecuritysetting>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LogicalFileSecuritySetting {
    /// Short textual description of the `CIM_Setting` object.
    pub Caption: Option<String>,
    /// Textual description of the `CIM_Setting` object.
    pub Description: Option<String>,
    /// Identifier by which the `CIM_Setting` object is known.
    pub SettingID: Option<String>,
    /// Control bits that qualify the meaning of an SD or its individual members. For 
    /// 
    /// The following list lists the flags in `ControlFlags`. 
    /// - `SE_OWNER_DEFAULTED` (1 (0x1)): Indicates an SD with a default owner security identifier (SID). You can use this bit to find all of the objects that have default owner permissions set.
    /// - `SE_GROUP_DEFAULTED` (2 (0x2)): Indicates an SD with a default group SID. You can use this bit to find all of the objects that have default group permissions set.
    /// - `SE_DACL_PRESENT` (4 (0x4)): Indicates an SD that has a discretionary access control list (DACL). If this flag is not set, or if this flag is set and the DACL is `NULL`, the SD allows full access to everyone.
    /// - `SE_DACL_DEFAULTED` (8 (0x8)): Indicates an SD with a default DACL. For example, if an object creator does not specify a DACL, the object receives the default DACL from the access token of the creator. This flag can affect how the system treats the DACL, with respect to access control entry (ACE) inheritance. The system ignores this flag if the `SE_DACL_PRESENT` flag is not set.
    /// - `SE_SACL_PRESENT` (16 (0x10)): Indicates an SD that has a system access control list (SACL).
    /// - `SE_SACL_DEFAULTED` (32 (0x20)): Indicates an SD with a default SACL. For example, if an object creator does not specify an SACL, the object receives the default SACL from the access token of the creator. This flag can affect how the system treats the SACL, with respect to ACE inheritance. The system ignores this flag if the `SE_SACL_PRESENT` flag is not set.
    /// - `SE_DACL_AUTO_INHERIT_REQ` (256 (0x100)): Requests that the provider for the object protected by the SD automatically propagate the DACL to existing child objects. If the provider supports automatic inheritance, it propagates the DACL to any existing child objects, and sets the `SE_DACL_AUTO_INHERITED` bit in the security descriptors of the object and its child objects.
    /// - `SE_SACL_AUTO_INHERIT_REQ` (512 (0x200)): Requests that the provider for the object protected by the SD automatically propagate the SACL to existing child objects. If the provider supports automatic inheritance, it propagates the SACL to any existing child objects, and sets the `SE_SACL_AUTO_INHERITED` bit in the SDs of the object and its child objects.
    /// - `SE_DACL_AUTO_INHERITED` (1024 (0x400)): Windows 2000 only. Indicates an SD in which the DACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - `SE_SACL_AUTO_INHERITED` (2048 (0x800)): Windows 2000: Indicates an SD in which the SACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - `SE_DACL_PROTECTED` (4096 (0x1000)): Windows 2000: Prevents the DACL of the SD from being modified by inheritable ACEs.
    /// - `SE_SACL_PROTECTED` (8192 (0x2000)): Windows 2000: Prevents the SACL of the SD from being modified by inheritable ACEs.
    /// - `SE_SELF_RELATIVE` (32768 (0x8000)): Indicates an SD in self-relative format with all of the security information in a contiguous block of memory. If this flag is not set, the SD is in absolute format. 
    pub ControlFlags: Option<u32>,
    /// Owner permissions to the object.
    pub OwnerPermissions: Option<bool>,
    /// Full path of the file or directory.
    pub Path: Option<String>,
}

/// The `Win32_LogicalShareSecuritySetting` WMI class represents security settings for a logical file.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/secrcw32prov/win32-logicalsharesecuritysetting>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LogicalShareSecuritySetting {
    /// Short textual description of the `CIM_Setting` object.
    pub Caption: Option<String>,
    /// Textual description of the `CIM_Setting` object.
    pub Description: Option<String>,
    /// Identifier by which the `CIM_Setting` object is known.
    pub SettingID: Option<String>,
    /// Control bits that qualify the meaning of an SD or its individual members.
    /// 
    /// The following list lists the flags in `ControlFlags`.
    /// 
    /// - `SE_OWNER_DEFAULTED` (1 (0x1)): Indicates an SD with a default owner security identifier (SID). You can use this bit to find all of the objects that have default owner permissions set.
    /// - `SE_GROUP_DEFAULTED` (2 (0x2)): Indicates an SD with a default group SID. You can use this bit to find all of the objects that have default group permissions set.
    /// - `SE_DACL_PRESENT` (4 (0x4)): Indicates an SD that has a discretionary access control list (DACL). If this flag is not set, or if this flag is set and the DACL is `NULL, the SD allows full access to everyone.
    /// - `SE_DACL_DEFAULTED` (8 (0x8)): Indicates an SD with a default DACL. For example, if an object creator does not specify a DACL, the object receives the default DACL from the access token of the creator. This flag can affect how the system treats the DACL, with respect to access control entry (ACE) inheritance. The system ignores this flag if the `SE_DACL_PRESENT` flag is not set.
    /// - `SE_SACL_PRESENT` (16 (0x10)): Indicates an SD that has a system access control list (SACL).
    /// - `SE_SACL_DEFAULTED` (32 (0x20)): Indicates an SD with a default SACL. For example, if an object creator does not specify an SACL, the object receives the default SACL from the access token of the creator. This flag can affect how the system treats the SACL, with respect to ACE inheritance. The system ignores this flag if the `SE_SACL_PRESENT` flag is not set.
    /// - `SE_DACL_AUTO_INHERIT_REQ` (256 (0x100)): Requests that the provider for the object protected by the SD automatically propagate the DACL to existing child objects. If the provider supports automatic inheritance, it propagates the DACL to any existing child objects, and sets the `SE_DACL_AUTO_INHERITED` bit in the security descriptors of the object and its child objects.
    /// - `SE_SACL_AUTO_INHERIT_REQ` (512 (0x200)): Requests that the provider for the object protected by the SD automatically propagate the SACL to existing child objects. If the provider supports automatic inheritance, it propagates the SACL to any existing child objects, and sets the `SE_SACL_AUTO_INHERITED` bit in the SDs of the object and its child objects.
    /// - `SE_DACL_AUTO_INHERITED` (1024 (0x400)): Windows 2000 only. Indicates an SD in which the DACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - `SE_SACL_AUTO_INHERITED` (2048 (0x800)): Windows 2000: Indicates an SD in which the SACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - `SE_DACL_PROTECTED` (4096 (0x1000)): Windows 2000: Prevents the DACL of the SD from being modified by inheritable ACEs.
    /// - `SE_SACL_PROTECTED` (8192 (0x2000)): Windows 2000: Prevents the SACL of the SD from being modified by inheritable ACEs.
    /// - `SE_SELF_RELATIVE` (32768 (0x8000)): Indicates an SD in self-relative format with all of the security information in a contiguous block of memory. If this flag is not set, the SD is in absolute format.
    pub ControlFlags: Option<u32>,
    /// Name of the share.
    pub Name: Option<String>,
}

/// The `Win32_PrivilegesStatus` WMI class reports information about privileges required to complete 
/// an operation. It may be returned when an operation failed or when a partially populated instance 
/// has been returned.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-privilegesstatus>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PrivilegesStatus {
    /// Any user-defined string that describes an error or operational status.
    pub Description: Option<String>,
    /// Operation that takes place at the time of a failure or anomaly. Typically, Windows Management 
    /// Instrumentation (WMI) sets this property to the name of a COM API for WMI method such as the 
    /// following: `IWbemServices::CreateInstanceEnum`.
    pub Operation: Option<String>,
    /// Parameters involved in an error or status change. For example, if an application attempts to 
    /// retrieve a class that does not exist, this property is set to the offending class name.
    pub ParameterInfo: Option<String>,
    /// Identifies the provider that causes or reports an error or status change. If a provider is not 
    /// involved, this string is set to "Windows Management".
    pub ProviderName: Option<String>,
    /// Contains an error or information code for an operation. This can be any value defined by the 
    /// provider, but the value 0 (zero) is usually reserved to indicate success.
    pub StatusCode: Option<u32>,
    /// Listing required access privileges missing to complete an operation. The types of access 
    /// privileges can be found under the Windows Privileges.
    /// 
    /// Example: "SE_SHUTDOWN_NAME"
    pub PrivilegesNotHeld: Option<Vec<String>>,
    /// Listing of all of the privileges required to perform an operation. This includes values from 
    /// the PrivilegesNotHeld property.
    /// 
    /// Example: "SE_SHUTDOWN_NAME"
    pub PrivilegesRequired: Option<Vec<String>>,
}

/// The `Win32_SecurityDescriptor` abstract WMI class represents a `SECURITY_DESCRIPTOR` structure. 
/// A security descriptor contains the security information for a securable object. The `Owner` and `Group` 
/// properties identify the owner and primary group for the object. It can also contain a discretionary 
/// access control list (DACL) that controls access to the object and a system access control list (SACL) 
/// that controls the logging of attempts to access the object.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/secrcw32prov/win32-securitydescriptor>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_SecurityDescriptor {
    /// Time in the CIM_DATETIME format when the security descriptor was created.
    pub TIME_CREATED: Option<u64>,
    /// Control bits that qualify the meaning of a security descriptor (SD) or its individual members.
    /// 
    /// The following list lists the flags in `ControlFlags`.
    /// 
    /// - `SE_OWNER_DEFAULTED` (1 (0x1)): Indicates an SD with a default owner security identifier (SID). Use this bit to find all of the objects that have default owner permissions set.
    /// - `SE_GROUP_DEFAULTED` (2 (0x2)): Indicates an SD with a default group SID. Use this bit to find all of the objects that have default group permissions set.
    /// - `SE_DACL_PRESENT` (4 (0x4)): Indicates an SD that has a DACL. If this flag is not set, or if this flag is set and the DACL is `NULL`, the SD allows full access to everyone.
    /// - `SE_DACL_DEFAULTED` (8 (0x8)): Indicates an SD with a default DACL. For example, if an object creator does not specify a DACL, the object receives the default DACL from the access token of the creator. This flag can affect how the system treats the DACL, with respect to access control entry (ACE) inheritance. The system ignores this flag if the `SE_DACL_PRESENT` flag is not set.
    /// - `SE_SACL_PRESENT` (16 (0x10)): Indicates an SD that has a system access control list (SACL).
    /// - `SE_SACL_DEFAULTED` (32 (0x20)): Indicates an SD with a default SACL. For example, if an object creator does not specify an SACL, the object receives the default SACL from the access token of the creator. This flag can affect how the system treats the SACL, with respect to ACE inheritance. The system ignores this flag if the `SE_SACL_PRESENT` flag is not set.
    /// - `SE_DACL_AUTO_INHERIT_REQ` (256 (0x100)): Requests that the provider for the object protected by the SD automatically propagate the DACL to existing child objects. If the provider supports automatic inheritance, the DACL is propagated to any existing child objects, and the `SE_DACL_AUTO_INHERITED` bit in the SD of the parent and child objects is set.
    /// - `SE_SACL_AUTO_INHERIT_REQ` (512 (0x200)): Requests that the provider for the object protected by the SD automatically propagate the SACL to existing child objects. If the provider supports automatic inheritance, the SACL is propagated to any existing child objects, and the `SE_SACL_AUTO_INHERITED` bit in the SDs of the parent object and child objects is set.
    /// - `SE_DACL_AUTO_INHERITED` (1024 (0x400)): Indicates an SD in which the DACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects.
    /// - `SE_SACL_AUTO_INHERITED` (2048 (0x800)): Indicates an SD in which the SACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects.
    /// - `SE_DACL_PROTECTED` (4096 (0x1000)): Prevents the DACL of an SD from being modified by inheritable ACEs.
    /// - `SE_SACL_PROTECTED` (8192 (0x2000)): Prevents the SACL of an SD from being modified by inheritable ACEs.
    /// - `SE_SELF_RELATIVE` (32768 (0x8000)): Indicates an SD in self-relative format with all the security information in a contiguous block of memory. If this flag is not set, the SD is in absolute format. 
    pub ControlFlags: Option<u32>,
    /// Each array entry defines the type of object access that the system grants to a specific user or group.
    pub DACL: Option<Vec<Win32_ACE>>,
    /// Group that owns this object.
    pub Group: Option<Win32_Trustee>,
    /// Owner of an object.
    pub Owner: Option<Win32_Trustee>,
    /// Each array entry defines the type of access attempts that generate audit records for a specific 
    /// user or group.
    pub SACL: Option<Vec<Win32_ACE>>,
}

/// The `Win32_Trustee` abstract WMI class specifies a trustee that can be a name or a security 
/// identifier (SID) byte array.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/secrcw32prov/win32-trustee>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Trustee {
    /// Time in the CIM_DATETIME format when the security descriptor was created.
    pub TIME_CREATED: Option<u64>,
    /// Domain to which a trustee belongs.
    pub Domain: Option<String>,
    /// A trustee can be a user account, group account, or logon session.
    pub Name: Option<String>,
    /// SID that uniquely identifies a user or group.
    pub SID: Option<Vec<u8>>,
    /// Length of a SID in bytes.
    pub SidLength: Option<u32>,
    /// SID of a trustee in string format. The format for a string value is the following:
    /// 
    /// 1. The "S" character identifies the series of digits as a SID.
    /// 2. The revision level.
    /// 3. Identifier authority value.
    /// 4. One or more subauthority values.
    /// 
    /// Example: "S-1-1-0"
    pub SIDString: Option<String>,
}

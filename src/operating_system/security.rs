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

/// Represents the state of Windows LogicalShareSecuritySettings
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogicalShareSecuritySettings {
    /// Represents sequence of Windows `LogicalFileSecuritySettings`
    pub logical_share_security_settings: Vec<Win32_LogicalShareSecuritySetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(LogicalShareSecuritySettings, logical_share_security_settings);

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
    /// - SE_OWNER_DEFAULTED (1 (0x1)): Indicates an SD with a default owner security identifier (SID). You can use this bit to find all of the objects that have default owner permissions set.
    /// - SE_GROUP_DEFAULTED (2 (0x2)): Indicates an SD with a default group SID. You can use this bit to find all of the objects that have default group permissions set.
    /// - SE_DACL_PRESENT (4 (0x4)): Indicates an SD that has a discretionary access control list (DACL). If this flag is not set, or if this flag is set and the DACL is `NULL`, the SD allows full access to everyone.
    /// - SE_DACL_DEFAULTED (8 (0x8)): Indicates an SD with a default DACL. For example, if an object creator does not specify a DACL, the object receives the default DACL from the access token of the creator. This flag can affect how the system treats the DACL, with respect to access control entry (ACE) inheritance. The system ignores this flag if the `SE_DACL_PRESENT` flag is not set.
    /// - SE_SACL_PRESENT (16 (0x10)): Indicates an SD that has a system access control list (SACL).
    /// - SE_SACL_DEFAULTED (32 (0x20)): Indicates an SD with a default SACL. For example, if an object creator does not specify an SACL, the object receives the default SACL from the access token of the creator. This flag can affect how the system treats the SACL, with respect to ACE inheritance. The system ignores this flag if the `SE_SACL_PRESENT` flag is not set.
    /// - SE_DACL_AUTO_INHERIT_REQ (256 (0x100)): Requests that the provider for the object protected by the SD automatically propagate the DACL to existing child objects. If the provider supports automatic inheritance, it propagates the DACL to any existing child objects, and sets the `SE_DACL_AUTO_INHERITED` bit in the security descriptors of the object and its child objects.
    /// - SE_SACL_AUTO_INHERIT_REQ (512 (0x200)): Requests that the provider for the object protected by the SD automatically propagate the SACL to existing child objects. If the provider supports automatic inheritance, it propagates the SACL to any existing child objects, and sets the `SE_SACL_AUTO_INHERITED` bit in the SDs of the object and its child objects.
    /// - SE_DACL_AUTO_INHERITED (1024 (0x400)): Windows 2000 only. Indicates an SD in which the DACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - SE_SACL_AUTO_INHERITED (2048 (0x800)): Windows 2000: Indicates an SD in which the SACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - SE_DACL_PROTECTED (4096 (0x1000)): Windows 2000: Prevents the DACL of the SD from being modified by inheritable ACEs.
    /// - SE_SACL_PROTECTED (8192 (0x2000)): Windows 2000: Prevents the SACL of the SD from being modified by inheritable ACEs.
    /// - SE_SELF_RELATIVE (32768 (0x8000)): Indicates an SD in self-relative format with all of the security information in a contiguous block of memory. If this flag is not set, the SD is in absolute format. 
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
    /// - SE_OWNER_DEFAULTED (1 (0x1)): Indicates an SD with a default owner security identifier (SID). You can use this bit to find all of the objects that have default owner permissions set.
    /// - SE_GROUP_DEFAULTED (2 (0x2)): Indicates an SD with a default group SID. You can use this bit to find all of the objects that have default group permissions set.
    /// - SE_DACL_PRESENT (4 (0x4)): Indicates an SD that has a discretionary access control list (DACL). If this flag is not set, or if this flag is set and the DACL is `NULL, the SD allows full access to everyone.
    /// - SE_DACL_DEFAULTED (8 (0x8)): Indicates an SD with a default DACL. For example, if an object creator does not specify a DACL, the object receives the default DACL from the access token of the creator. This flag can affect how the system treats the DACL, with respect to access control entry (ACE) inheritance. The system ignores this flag if the `SE_DACL_PRESENT` flag is not set.
    /// - SE_SACL_PRESENT (16 (0x10)): Indicates an SD that has a system access control list (SACL).
    /// - SE_SACL_DEFAULTED (32 (0x20)): Indicates an SD with a default SACL. For example, if an object creator does not specify an SACL, the object receives the default SACL from the access token of the creator. This flag can affect how the system treats the SACL, with respect to ACE inheritance. The system ignores this flag if the `SE_SACL_PRESENT` flag is not set.
    /// - SE_DACL_AUTO_INHERIT_REQ (256 (0x100)): Requests that the provider for the object protected by the SD automatically propagate the DACL to existing child objects. If the provider supports automatic inheritance, it propagates the DACL to any existing child objects, and sets the `SE_DACL_AUTO_INHERITED` bit in the security descriptors of the object and its child objects.
    /// - SE_SACL_AUTO_INHERIT_REQ (512 (0x200)): Requests that the provider for the object protected by the SD automatically propagate the SACL to existing child objects. If the provider supports automatic inheritance, it propagates the SACL to any existing child objects, and sets the `SE_SACL_AUTO_INHERITED` bit in the SDs of the object and its child objects.
    /// - SE_DACL_AUTO_INHERITED (1024 (0x400)): Windows 2000 only. Indicates an SD in which the DACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - SE_SACL_AUTO_INHERITED (2048 (0x800)): Windows 2000: Indicates an SD in which the SACL is set up to support automatic propagation of inheritable ACEs to existing child objects. The system sets this bit when it performs the automatic inheritance algorithm for the object and its existing child objects. This bit is not set in SDs for Windows NT versions 4.0 and earlier, which do not support automatic propagation of inheritable ACEs.
    /// - SE_DACL_PROTECTED (4096 (0x1000)): Windows 2000: Prevents the DACL of the SD from being modified by inheritable ACEs.
    /// - SE_SACL_PROTECTED (8192 (0x2000)): Windows 2000: Prevents the SACL of the SD from being modified by inheritable ACEs.
    /// - SE_SELF_RELATIVE (32768 (0x8000)): Indicates an SD in self-relative format with all of the security information in a contiguous block of memory. If this flag is not set, the SD is in absolute format.
    pub ControlFlags: Option<u32>,
    /// Name of the share.
    pub Name: Option<String>,
}

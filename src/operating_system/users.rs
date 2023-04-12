//! The Users subcategory groups classes that represent user account information, such as group membership details.
//!
//! | Class                                                                 | Description                                                                                                                                      |
//! |-----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Account**](win32-account)                               | Instance class<br/> Represents information about user accounts and group accounts known to the computer system running Windows.<br/> |
//! | [**Win32\_Group**](win32-group)                                   | Instance class<br/> Represents data about a group account.<br/>                                                                      |
//! | [**Win32\_GroupInDomain**](/previous-versions/windows/desktop/cimwin32a/win32-groupindomain)                   | Association class<br/> Identifies the group accounts associated with a Windows NT domain.<br/>                                       |
//! | [**Win32\_GroupUser**](win32-groupuser)                           | Association class<br/> Relates a group and an account that is a member of that group.<br/>                                           |
//! | [**Win32\_LogonSession**](win32-logonsession)                     | Instance class<br/> Describes the logon session or sessions associated with a user logged on to Windows.<br/>                        |
//! | [**Win32\_LogonSessionMappedDisk**](/windows/desktop/CIMWin32Prov/win32-logonsessionmappeddisk) | Instance class<br/> Represents the mapped logical disks associated with the session.<br/>                                            |
//! | [**Win32\_NetworkLoginProfile**](win32-networkloginprofile)       | Instance class<br/> Represents the network login information of a specific user on a computer system running Windows.<br/>           |
//! | [**Win32\_SystemAccount**](win32-systemaccount)                   | Instance class<br/> Represents a system account.<br/>                                                                                |
//! | [**Win32\_UserAccount**](win32-useraccount)                       | Instance class<br/> Represents information about a user account on a computer system running Windows.<br/>                           |
//! | [**Win32\_UserInDomain**](/previous-versions/windows/desktop/cimwin32a/win32-userindomain)                     | Association class<br/> Relates a user account and a Windows NT domain.<br/>                                                          |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows User Accounts
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserAccounts {
    /// Sequence of windows User Accounts
    pub useraccounts: Vec<Win32_UserAccount>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(UserAccounts, useraccounts);

/// The `Win32_UserAccount` WMI class contains information about a user account on a computer system
/// running Windows.
///
/// Note: Because both the Name and Domain are key properties, enumerating Win32_UserAccount on a
/// large network can negatively affect performance. Calling GetObject or querying for a specific
/// instance has less impact.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-useraccount>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_UserAccount {
    AccountType: Option<u32>,
    Caption: Option<String>,
    Description: Option<String>,
    Disabled: Option<bool>,
    Domain: Option<String>,
    FullName: Option<String>,
    InstallDate: Option<WMIDateTime>,
    LocalAccount: Option<bool>,
    Lockout: Option<bool>,
    Name: Option<String>,
    PasswordChangeable: Option<bool>,
    PasswordExpires: Option<bool>,
    PasswordRequired: Option<bool>,
    SID: Option<String>,
    SIDType: Option<u8>,
    Status: Option<String>,
}

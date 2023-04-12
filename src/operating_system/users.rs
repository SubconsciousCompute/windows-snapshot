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
    pub user_accounts: Vec<Win32_UserAccount>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(UserAccounts, user_accounts);

/// Represents the state of Windows user accounts and group accounts
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Accounts {
    /// Sequence of windows Accounts
    pub accounts: Vec<Win32_Account>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Accounts, accounts);

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
    /// Flags that describe the characteristics of a Windows user account.
    ///
    /// - Temporary duplicate account (256): UF_TEMP_DUPLICATE_ACCOUNT: Local user account for
    /// users who have a primary account in another domain. This account provides user access to
    /// this domain only—not to any domain that trusts this domain.
    ///
    /// - Normal account (512): UF_NORMAL_ACCOUNT: Default account type that represents a typical user.
    ///
    /// - Interdomain trust account (2048): UF_INTERDOMAIN_TRUST_ACCOUNT: Account for a system
    /// domain that trusts other domains.
    ///
    /// - Workstation trust account (4096): UF_WORKSTATION_TRUST_ACCOUNT: Computer account for a
    /// computer system running Windows that is a member of this domain.
    ///
    /// - Server trust account (8192): UF_SERVER_TRUST_ACCOUNT: Account for a system backup domain
    /// controller that is a member of this domain.
    AccountType: Option<u32>,
    /// Domain and username of the account.
    Caption: Option<String>,
    /// Description of the account.
    Description: Option<String>,
    /// Windows user account is disabled.
    Disabled: Option<bool>,
    /// Name of the Windows domain to which a user account belongs, for example: "NA-SALES".
    Domain: Option<String>,
    /// Full name of a local user, for example: "Dan Wilson".
    FullName: Option<String>,
    /// Date the object is installed. This property does not need a value to indicate that the
    /// object is installed.
    InstallDate: Option<WMIDateTime>,
    /// If true, the account is defined on the local computer.
    LocalAccount: Option<bool>,
    /// If true, the user account is locked out of the Windows operating system.
    Lockout: Option<bool>,
    /// Name of the Windows user account on the domain that the Domain property of this class specifies.
    ///
    /// Example: "danwilson".
    Name: Option<String>,
    /// If true, the password on this user account can be changed.
    PasswordChangeable: Option<bool>,
    /// If true, the password on this user account expires.
    PasswordExpires: Option<bool>,
    /// If true, a password is required on a Windows user account. If false, this account does not
    /// require a password.
    PasswordRequired: Option<bool>,
    /// Security identifier (SID) for this account.
    /// A SID is a string value of variable length that is used to identify a trustee.
    /// Each account has a unique SID that an authority, such as a Windows domain, issues.
    /// The SID is stored in the security database.
    /// When a user logs on, the system retrieves the user SID from the database,
    /// places the SID in the user access token,
    /// and then uses the SID in the user access token
    /// to identify the user in all subsequent interactions with Windows security.
    /// Each SID is a unique identifier for a user or group,
    /// and a different user or group cannot have the same SID.
    SID: Option<String>,
    /// Enumerated value that specifies the type of SID.
    ///
    /// - SidTypeUser (1)
    /// - SidTypeGroup (2)
    /// - SidTypeDomain (3)
    /// - SidTypeAlias (4)
    /// - SidTypeWellKnownGroup (5)
    /// - SidTypeDeletedAccount (6)
    /// - SidTypeInvalid (7)
    /// - SidTypeUnknown (8)
    /// - SidTypeComputer (9)
    SIDType: Option<u8>,
    /// Current status of an object.
    /// Various operational and nonoperational statuses can be defined.
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail",
    /// which is an element such as a SMART-enabled hard disk drive that may be functioning properly,
    /// but predicts a failure in the near future.
    /// Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service",
    /// which can apply during mirror re-silvering of a disk,
    /// reloading a user permissions list, or other administrative work.
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
}

/// The `Win32_Account` abstract WMI class contains information about user accounts and group accounts
/// known to the computer system running Windows.
/// User or group names recognized by a Windows domain are descendants (or members) of this class.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-account>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Account {
    /// Short description of the object.
    caption: Option<String>,
    /// Description of the object.
    description: Option<String>,
    /// Name of the Windows domain to which a group or user belongs.
    ///
    /// Example: "NA-SALES"
    domain: Option<String>,
    /// Date and time that the object was installed. This property does not require a value to
    /// indicate that the object is installed.
    install_date: Option<WMIDateTime>,
    /// If TRUE, the account is defined on the local machine. To retrieve only accounts defined on
    /// the local machine, design a query that includes the condition "LocalAccount=TRUE".
    local_account: Option<bool>,
    /// Name of the Windows system account on the domain specified by the Domain property of this
    /// class. This property overrides the Name property inherited from CIM_ManagedSystemElement.
    name: Option<String>,
    /// Security identifier (SID) for this account.
    /// A SID is a string value of variable length used to identify a trustee.
    /// Each account has a unique SID issued by an authority (such as a Windows domain),
    /// stored in a security database.
    /// When a user logs on,
    /// the system retrieves the user's SID from the database and places it in the user's access token.
    /// The system uses the SID in the user's access token
    /// to identify the user in all subsequent interactions with Windows security.
    /// When a SID has been used as the unique identifier for a user or group,
    /// it cannot be used again to identify another user or group.
    sid: Option<String>,
    /// Enumerated values that specify the type of security identifier (SID).
    ///
    /// - SidTypeUser (1)
    /// - SidTypeGroup (2)
    /// - SidTypeDomain (3)
    /// - SidTypeAlias (4)
    /// - SidTypeWellKnownGroup (5)
    /// - SidTypeDeletedAccount (6)
    /// - SidTypeInvalid (7)
    /// - SidTypeUnknown (8)
    /// - SidTypeComputer (9)
    sid_type: Option<u8>,
    /// Current status of the object.
    /// Various operational and nonoperational statuses can be defined.
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail"
    /// (an element, such as a SMART-enabled hard disk drive,
    /// may be functioning properly but predicts a failure in the near future).
    /// Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service".
    /// The latter, "Service", can apply during mirror-re-silvering of a disk,
    /// reload of a user permissions list, or other administrative work.
    /// Not all such work is online,
    /// yet the managed element is neither "OK" nor in one of the other states.
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
    status: Option<String>,
}

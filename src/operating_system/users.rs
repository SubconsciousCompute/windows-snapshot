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
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct UserAccounts {
    /// Sequence of windows User Accounts
    pub user_accounts: Vec<Win32_UserAccount>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(UserAccounts, user_accounts);

/// Represents the state of Windows user accounts and group accounts
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct Accounts {
    /// Sequence of windows Accounts
    pub accounts: Vec<Win32_Account>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Accounts, accounts);

/// Represents the state of Windows data about a group account
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct Groups {
    /// Sequence of windows Group
    pub groups: Vec<Win32_Group>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Groups, groups);

/// Represents the state of Windows data about logon session or sessions associated with a user logged
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct LogonSessions {
    /// Sequence of windows logon sessions
    pub logon_sessions: Vec<Win32_LogonSession>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(LogonSessions, logon_sessions);

/// Represents the state of Windows data about network login information
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct NetworkLoginProfiles {
    /// Sequence of windows network login
    pub network_login_profiles: Vec<Win32_NetworkLoginProfile>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(NetworkLoginProfiles, network_login_profiles);

/// Represents the state of Windows system accounts.
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct SystemAccounts {
    /// Sequence of windows SystemAccounts
    pub system_accounts: Vec<Win32_SystemAccount>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(SystemAccounts, system_accounts);

/// The `Win32_UserAccount` WMI class contains information about a user account on a computer system
/// running Windows.
///
/// Note: Because both the Name and Domain are key properties, enumerating `Win32_UserAccount` on a
/// large network can negatively affect performance. Calling `GetObject` or querying for a specific
/// instance has less impact.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-useraccount>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
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
    pub AccountType: Option<u32>,
    /// Domain and username of the account.
    pub Caption: Option<String>,
    /// Description of the account.
    pub Description: Option<String>,
    /// Windows user account is disabled.
    pub Disabled: Option<bool>,
    /// Name of the Windows domain to which a user account belongs, for example: "NA-SALES".
    pub Domain: Option<String>,
    /// Full name of a local user, for example: "Dan Wilson".
    pub FullName: Option<String>,
    /// Date the object is installed. This property does not need a value to indicate that the
    /// object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// If true, the account is defined on the local computer.
    pub LocalAccount: Option<bool>,
    /// If true, the user account is locked out of the Windows operating system.
    pub Lockout: Option<bool>,
    /// Name of the Windows user account on the domain that the Domain property of this class specifies.
    ///
    /// Example: "danwilson".
    pub Name: Option<String>,
    /// If true, the password on this user account can be changed.
    pub PasswordChangeable: Option<bool>,
    /// If true, the password on this user account expires.
    pub PasswordExpires: Option<bool>,
    /// If true, a password is required on a Windows user account. If false, this account does not
    /// require a password.
    pub PasswordRequired: Option<bool>,
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
    pub SID: Option<String>,
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
    pub SIDType: Option<u8>,
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
    pub Status: Option<String>,
}

/// The `Win32_Account` abstract WMI class contains information about user accounts and group accounts
/// known to the computer system running Windows.
/// User or group names recognized by a Windows domain are descendants (or members) of this class.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-account>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Account {
    /// Short description of the object.
    pub caption: Option<String>,
    /// Description of the object.
    pub description: Option<String>,
    /// Name of the Windows domain to which a group or user belongs.
    ///
    /// Example: "NA-SALES"
    pub domain: Option<String>,
    /// Date and time that the object was installed. This property does not require a value to
    /// indicate that the object is installed.
    pub install_date: Option<WMIDateTime>,
    /// If TRUE, the account is defined on the local machine. To retrieve only accounts defined on
    /// the local machine, design a query that includes the condition "LocalAccount=TRUE".
    pub local_account: Option<bool>,
    /// Name of the Windows system account on the domain specified by the Domain property of this
    /// class. This property overrides the Name property inherited from CIM_ManagedSystemElement.
    pub name: Option<String>,
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
    pub sid: Option<String>,
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
    pub sid_type: Option<u8>,
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
    pub status: Option<String>,
}

/// The `Win32_Group WMI` class represents data about a group account.
/// A group account allows access privileges to be changed for a list of users.
/// Example: Marketing2.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-group>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Group {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object
    /// is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object.
    /// Operational and non-operational status can be defined.
    /// Operational status can include "OK", "Degraded", and "Pred Fail".
    /// "Pred Fail" indicates that an element is functioning properly,
    /// but is predicting a failure (for example, a SMART-enabled hard disk drive).
    ///
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service".
    /// "Service" can apply during disk mirror-re-silvering,
    /// reloading a user permissions list, or other administrative work.
    /// Not all such work is online,
    /// but the managed element is neither "OK" nor in one of the other states.
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
    pub Status: Option<String>,
    /// If TRUE, the account is defined on the local machine.
    /// To retrieve only accounts defined on the local machine,
    /// design a query that includes the condition "LocalAccount=TRUE".
    pub LocalAccount: Option<bool>,
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
    pub SID: Option<String>,
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
    pub SIDType: Option<u8>,
    /// Name of the Windows domain to which the group account belongs.
    ///
    /// Example: "NA-SALES"
    pub Domain: Option<String>,
    /// Name of the Windows group account on the domain specified by the Domain property of this class.
    pub Name: Option<String>,
}

/// The `Win32_LogonSession` WMI class
/// describes the logon session or sessions associated with a user
/// logged on to a computer system running Windows.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-logonsession>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LogonSession {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object
    /// is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known.
    /// When subclassed, this property can be overridden to be a key property.
    pub Name: Option<String>,
    /// String that indicates the current status of the object.
    /// Operational and non-operational status can be defined.
    /// Operational status can include "OK", "Degraded", and "Pred Fail".
    /// "Pred Fail" indicates that an element is functioning properly,
    /// but is predicting a failure (for example, a SMART-enabled hard disk drive).
    ///
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service".
    /// "Service" can apply during disk mirror-re-silvering,
    /// reloading a user permissions list, or other administrative work.
    /// Not all such work is online,
    /// but the managed element is neither "OK" nor in one of the other states.
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
    pub Status: Option<String>,
    /// Time at which the session started.
    pub StartTime: Option<WMIDateTime>,
    /// Name of the subsystem used to authenticate the logon session.
    pub AuthenticationPackage: Option<String>,
    /// ID assigned to the logon session.
    pub LogonId: Option<String>,
    /// Numeric value that indicates the type of logon session.
    ///
    /// - 0: Used only by the System account.
    /// - Interactive (2): Intended for users who are interactively using the machine, such as a user being logged on by a terminal server, remote shell, or similar process.
    /// - Network (3): Intended for high-performance servers to authenticate clear text passwords. LogonUser does not cache credentials for this logon type.
    /// - Batch (4): Intended for batch servers, where processes can be executed on behalf of a user without their direct intervention; or for higher performance servers that process many clear-text authentication attempts at a time, such as mail or web servers. LogonUser does not cache credentials for this logon type.
    /// - Service (5): Indicates a service-type logon. The account provided must have the service privilege enabled.
    /// - Proxy (6): Indicates a proxy-type logon.
    /// - Unlock (7): This logon type is intended for GINA DLLs logging on users who are interactively using the machine. This logon type allows a unique audit record to be generated that shows when the workstation was unlocked.
    /// - NetworkCleartext (8): Preserves the name and password in the authentication packages, allowing the server to make connections to other network servers while impersonating the client. This allows a server to accept clear text credentials from a client, call LogonUser, verify that the user can access the system across the network, and still communicate with other servers.
    /// - NewCredentials (9): Allows the caller to clone its current token and specify new credentials for outbound connections. The new logon session has the same local identify, but uses different credentials for other network connections.
    /// - RemoteInteractive (10): Terminal Services session that is both remote and interactive.
    /// - CachedInteractive (11): Attempt cached credentials without accessing the network.
    /// - CachedRemoteInteractive (12): Same as RemoteInteractive. This is used for internal auditing.
    /// - CachedUnlock (13): Workstation logon.
    pub LogonType: Option<u32>,
}

/// The `Win32_NetworkLoginProfile`
/// WMI class represents the network login information of a specific user on a computer system running Windows.
/// This includes, but is not limited to password status,
/// access privileges, disk quotas, and logon directory paths.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkloginprofile>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NetworkLoginProfile {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Account will expire.
    /// This value is calculated from the number of seconds elapsed since 00:00:00,
    /// January 1, 1970, and is set in this format: yyyymmddhhmmss.mmmmmm sutc.
    ///
    /// Example: 20521201000230.000000 000
    pub AccountExpires: Option<WMIDateTime>,

    /// Set of flags that specify the resources a user is authorized to use or modify.
    ///
    /// - 1 (0x1): Printer
    /// - 2 (0x2):  Communication
    /// - 4 (0x4): Server
    /// - 8 (0x8): Accounts
    pub AuthorizationFlags: Option<u32>,
    /// Number of times the user enters a bad password when logging on to a computer system running Windows.
    ///
    /// Example: 0
    pub BadPasswordCount: Option<u32>,
    /// Code page for the user's language of choice. A code page is the character set used.
    pub CodePage: Option<u32>,
    /// Comment or description for this logon profile.
    pub Comment: Option<String>,
    /// Country/region code for the user's language of choice.
    pub CountryCode: Option<u32>,
    /// The properties available to this network profile.
    ///
    /// Properties that can be set include:
    ///
    /// - 1 (0x1): Script: A logon script executed. This value must be set for LAN Manager 2.0.
    /// - 2 (0x2): Account Disabled: The user's account is disabled.
    /// - 8 (0x8): Home Directory Required: A home directory is required.
    /// - 16 (0x10): Lockout: The account is currently locked out. For NetUserSetInfo, this value can be cleared to unlock a previously locked account. This value cannot be used to lock a previously unlocked account.
    /// - 32 (0x20): Password Not Required: No password is required.
    /// - 64 (0x40): Password Cannot Change: The user cannot change the password.
    /// - 128 (0x80): Encrypted Test Password Allowed
    /// - 256 (0x100): Temp Duplicate Account: An account for users whose primary account is in another domain. This account provides user access to this domain, but not to any domain that trusts this domain. The User Manager refers to this account type as a local user account.
    /// - 512 (0x200): Normal Account: Default account type that represents a typical user.
    /// - 2048 (0x800): Interdomain Trust Account: A permit to a trust account for a domain that trusts other domains.
    /// - 4096 (0x1000): Workstation Trust Account: A computer account for a Windows workstation or server that is a member of this domain.
    /// - 8192 (0x2000): Server Trust Account: A computer account for a backup domain controller that is a member of this domain.
    /// - 65536 (0x10000): Do Not Expire Password
    /// - 131072 (0x20000): MNS Logon Account: Majority Node Set (MNS) logon account type that represents an MNS user.
    /// - 262144 (0x40000): Smartcard Required
    /// - 524288 (0x80000): Trusted for Delegation
    /// - 1048576 (0x100000): Not Delegated
    /// - 2097152 (0x200000): Use DES Key Only
    /// - 4194304 (0x400000): Do Not Require Preauthorization
    /// - 8388608 (0x800000): Password Expired: Indicates that the password has expired.
    ///
    /// The following properties describe the account type. Only one value can be set:
    ///
    /// - UF_NORMAL_ACCOUNT
    /// - UF_TEMP_DUPLICATE_ACCOUNT
    /// - UF_WORKSTATION_TRUST_ACCOUNT
    /// - UF_SERVER_TRUST_ACCOUNT
    /// - UF_INTERDOMAIN_TRUST_ACCOUNT
    pub Flags: Option<u32>,
    /// Full name of the user belonging to the network login profile.
    /// This string can be empty if the user chooses not to associate a full name with a user name.
    pub FullName: Option<String>,
    /// Path to the home directory of the user. This string may be empty if the user chooses not to
    /// specify a home directory.
    ///
    /// Example:"\HOMEDIR"
    pub HomeDirectory: Option<String>,
    /// Drive letter assigned to the user's home directory for log on purposes.
    ///
    /// Example: "C:"
    pub HomeDirectoryDrive: Option<String>,
    /// User last logged off the system. This value is calculated from the number of seconds elapsed
    /// since 00:00:00, January 1, 1970. A value of " **************.******+*** " means that the
    /// last logoff time is unknown. The format of this value is yyyymmddhhmmss.mmmmmm sutc. For
    /// information about translating this property into your local time, see WMI Tasks: Dates and
    /// Times.
    ///
    /// Example: 19521201000230.000000 000
    ///
    /// Note: Should be of type WMIDateTime but causes parsing errors due to starting with zeroes.
    pub LastLogoff: Option<String>,
    /// User last logged on to the system.
    /// This value is calculated from the number of seconds elapsed since 00:00:00,
    /// January 1, 1970. The format of this value is yyyymmddhhmmss.mmmmmm sutc.
    /// For information about translating this property into your local time, see WMI Tasks:
    /// Dates and Times.
    ///
    /// Example: 19521201000230.000000 000
    pub LastLogon: Option<WMIDateTime>,
    /// Times during the week when the user can log on.
    /// Each bit represents a unit of time specified by the UnitsPerWeek property.
    /// For instance, if the unit of time is hourly, the first bit (bit 0, word 0) is Sunday,
    /// 0:00 to 0:59, the second bit (bit 1, word 0) is Sunday, 1:00 to 1:59, and so on.
    /// If this member is set to NULL, then there is no time restriction.
    /// The time is set to GMT and must be adjusted for other time zones
    /// (for example, GMT minus 8 hours for PST).
    pub LogonHours: Option<String>,
    /// Name of the server to which logon requests are sent.
    /// Server names should be preceded by two backslashes (\\).
    /// A server name with an asterisk (\\*)
    /// indicates that the logon request can be handled by any logon server.
    /// A null string indicates that requests are sent to the domain controller.
    ///
    /// Example: "\\MyServer"
    pub LogonServer: Option<String>,
    /// Maximum amount of disk space available to the user.
    /// If MaximumStorage is set to USER_MAXSTORAGE_UNLIMITED,
    /// the user is allowed to use all of the available disk space.
    ///
    /// Example: 10000000
    pub MaximumStorage: Option<u64>,
    /// User account on a particular domain or computer.
    /// The number of characters in the name cannot exceed the value of UNLEN.
    ///
    /// Example: "somedomain\johndoe"
    pub Name: Option<String>,
    /// Number of successful times the user tried to log on to this account.
    /// A value of 0xFFFFFFFF indicates that the value is unknown.
    /// This property is maintained separately on each backup domain controller (BDC) in the domain.
    /// To get an accurate value, only the largest value from all BDCs should be used.
    ///
    /// Example: 4
    pub NumberOfLogons: Option<u32>,
    /// Space set aside for use by applications.
    /// This string can be null,
    /// or it can have any number of characters before the terminating null character.
    /// Microsoft products use this member to store user configuration information.
    /// Do not modify this information, because this value is specific to an application.
    pub Parameters: Option<String>,
    /// Length of time a password has been in effect.
    /// This value is measured from the number of seconds elapsed since the password was last changed.
    ///
    /// Example: 00001201000230.000000 000
    ///
    /// Note: Should be of type WMIDateTime but causes parsing errors due to starting with zeroes.
    pub PasswordAge: Option<String>,
    /// Date and time the password expires.
    /// The value is set in this format: yyyymmddhhmmss.mmmmmm sutc
    ///
    /// Example: 19521201000230.000000 000
    pub PasswordExpires: Option<WMIDateTime>,
    /// Relative identifier (RID) of the Primary Global Group for this user.
    /// The identifier verifies the primary group to which the user's profile belongs.
    pub PrimaryGroupId: Option<u32>,
    /// Level of privilege assigned to the usri3_name property.
    ///
    /// - Guest (0)
    /// - User (1)
    /// - Administrator (2)
    pub Privileges: Option<u32>,
    /// Path to the user's profile.
    /// This value can be a null string, a local absolute path, or a UNC path.
    /// A user profile contains settings that are customizable for each user such as the desktop colors.
    ///
    /// Example: "C:\Windows"
    pub Profile: Option<String>,
    /// Directory path to the user's logon script.
    /// A logon script automatically executes a set of commands each time a user logs on to a system.
    ///
    /// Example: "C:\win\profiles\ThomasSteven"
    pub ScriptPath: Option<String>,
    /// Number of time units the week is divided into.
    /// It is used with the LogonHours property to limit user access to the computer.
    ///
    /// Example: 168 (hours per week)
    pub UnitsPerWeek: Option<u32>,
    /// User-defined comment or description for this profile.
    pub UserComment: Option<String>,
    /// RID of the user. The identifier verifies that the user exists and is unique to this domain.
    pub UserId: Option<u32>,
    /// Type of account to which the user has privileges.
    ///
    /// The values are:
    ///
    /// - Normal Account ("Normal Account")
    /// - Duplicate Account ("Duplicate Account")
    /// - Workstation Trust Account ("Workstation Trust Account")
    /// - Server Trust Account ("Server Trust Account")
    /// - Interdomain Trust Account ("Interdomain Trust Account")
    /// - Unknown ("Unknown")
    pub UserType: Option<String>,
    /// Names of workstations from which the user can log on.
    /// Up to eight workstations can be specified; the names must be separated by commas (,).
    /// A null string indicates no restrictions.
    /// To disable logons from all workstations to this account,
    /// set the UF_ACCOUNTDISABLE in the Flags property of this class.
    pub Workstations: Option<String>,
}

/// The `Win32_SystemAccount` WMI class represents a system account.
/// The system account is used by the operating system and services.
/// There are many services and processes within Windows that need the capability to logon internally,
/// for example, during a Windows installation.
/// The system account was designed for that purpose.
///
/// The system account is an internal account that does not show up in User Manager,
/// cannot be added to any groups, and cannot have user rights assigned to it.
/// However, the system account does show up on an NTFS file system volume in file manager,
/// which is located in the Permissions section of the Security menu.
/// By default, the system account is granted full control to all files on an NTFS file system volume,
/// which means
/// that the system account has the same functional privileges as the administrator account.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemaccount>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_SystemAccount {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed.
    /// Lack of a value does not indicate that the object is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object.
    /// Operational and non-operational status can be defined.
    /// Operational status can include "OK", "Degraded", and "Pred Fail".
    /// "Pred Fail" indicates that an element is functioning properly,
    /// but is predicting a failure (for example, a SMART-enabled hard disk drive).
    ///
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service".
    /// "Service" can apply during disk mirror-re-silvering,
    /// reloading a user permissions list, or other administrative work.
    /// Not all such work is online,
    /// but the managed element is neither "OK" nor in one of the other states.
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
    pub Status: Option<String>,
    /// If TRUE, the account is defined on the local machine.
    /// To retrieve only accounts defined on the local machine,
    /// design a query that includes the condition "LocalAccount=TRUE".
    pub LocalAccount: Option<bool>,
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
    pub SID: Option<String>,
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
    pub SIDType: Option<u8>,
    /// Name of the Windows domain to which the system account belongs.
    ///
    /// Example: "NA-SALES"
    pub Domain: Option<String>,
    /// Name of the Windows system account on the domain specified by the Domain property of this class.
    pub Name: Option<String>,
}

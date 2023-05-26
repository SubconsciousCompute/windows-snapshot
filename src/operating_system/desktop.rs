//! The Desktop subcategory groups classes that represent objects that define a specific desktop configuration.
//!
//! | Class                                           | Description                                                                                                                        |
//! |-------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Desktop**](win32-desktop)         | Instance class<br/> Represents the common characteristics of a user's desktop.<br/>                                    |
//! | [**Win32\_Environment**](win32-environment) | Instance class<br/> Represents an environment or system environment setting on a computer system running Windows.<br/> |
//! | [**Win32\_TimeZone**](win32-timezone)       | Instance class<br/> Represents the time zone information for a computer system running Windows.<br/>                   |
//! | [**Win32\_UserDesktop**](win32-userdesktop) | Association class<br/> Relates a user account and the desktop settings that are specific to it.<br/>                   |

use crate::operating_system::users::Win32_UserAccount;
use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's desktops
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Desktops {
    /// Sequence of windows Desktop states
    pub desktops: Vec<Win32_Desktop>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Desktops, desktops);

/// Represents the state of Windows Environment
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Environments {
    /// Sequence of windows Environment states
    pub environments: Vec<Win32_Environment>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Environments, environments);

/// Represents the state of Windows `TimeZone`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeZones {
    /// Sequence of windows TimeZone states
    pub timezones: Vec<Win32_TimeZone>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(TimeZones, timezones);

/// Represents the state of Windows User Desktops
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserDesktops {
    ///  user account and desktop settings that are specific to it
    pub user_desktops: Vec<Win32_UserDesktop>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(UserDesktops, user_desktops);

/// The `Win32_Desktop` WMI class represents the common characteristics of a user's desktop. The
/// properties of this class can be modified by the user to customize the desktop.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-desktop>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Desktop {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Width of the borders around all windows with adjustable borders.
    ///
    /// Example: 3
    pub BorderWidth: Option<u32>,
    /// Fast task switching is turned on. Fast task switching allows the user to switch between
    /// windows using the ALT+TAB keyboard combination.
    pub CoolSwitch: Option<bool>,
    /// Length of time between successive cursor blinks.
    ///
    /// Example: 530
    pub CursorBlinkRate: Option<u32>,
    /// Contents of a window are shown when a user moves the window.
    pub DragFullWindows: Option<bool>,
    /// Spacing of the grid that windows are bound to on the desktop. This makes organizing windows
    /// easier. The spacing is usually fine enough that the user does not notice it.
    ///
    /// Example: 1
    pub GridGranularity: Option<u32>,
    /// Spacing between icons.
    ///
    /// Example: 75
    pub IconSpacing: Option<u32>,
    /// Font used for the names of the icons.
    ///
    /// Example: "MS San Serif"
    pub IconTitleFaceName: Option<String>,
    /// Icon font size.
    ///
    /// Example: 9
    pub IconTitleSize: Option<u32>,
    /// Icon's title text wraps to the next line.
    pub IconTitleWrap: Option<bool>,
    /// Name that identifies the current desktop profile.
    ///
    /// Example: "MainProf"
    pub Name: Option<String>,
    /// Name of the pattern used as the background for the desktop.
    pub Pattern: Option<String>,
    /// Screen saver is active.
    pub ScreenSaverActive: Option<bool>,
    /// Name of the current screen saver executable file.
    ///
    /// Example: "LOGON.SCR"
    pub ScreenSaverExecutable: Option<String>,
    /// Password is enabled for the screen saver.
    pub ScreenSaverSecure: Option<bool>,
    /// Amount of time that passes before the screen saver starts.
    pub ScreenSaverTimeout: Option<u32>,
    /// File name for the wallpaper design on the background of the desktop.
    ///
    /// Example: "WINNT.BMP"
    pub Wallpaper: Option<String>,
    /// Wallpaper is stretched to fill the entire screen. Microsoft Plus! must be installed before
    /// this option is available. If FALSE, the wallpaper retains its original dimensions on the
    /// desktop background.
    pub WallpaperStretched: Option<bool>,
    /// Wallpaper is tiled or centered.
    pub WallpaperTiled: Option<bool>,
}

/// The `Win32_Environment` WMI class represents an environment or system environment setting on a
/// Windows computer system. Querying this class returns environment variables found in:
///
/// `HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\Sessionmanager\Environment`
///
/// and
///
/// `HKEY_USERS\<user>\Environment`
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-environment>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Environment {
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
    /// Character string that specifies the name of a Windows-based environment variable.
    /// By specifying the name of a variable that does not yet exist,
    /// an application creates a new environment variable.
    ///
    /// Example: "Path"
    pub Name: Option<String>,
    /// Indicates whether the variable is a system variable.
    /// A system variable is set by the operating system,
    /// and is independent from user environment settings.
    pub SystemVariable: Option<bool>,
    /// Name of the owner of the environment setting.
    /// It is set to <SYSTEM> for settings that are specific to the Windows-based system
    /// (as opposed to a specific user) and <DEFAULT> for default user settings.
    ///
    /// Example: "JSmith"
    pub UserName: Option<String>,
    /// Placeholder variable of a Windows-based environment variable.
    /// Information like the file system directory can change from computer to computer.
    /// The operating system substitutes placeholders for these.
    ///
    /// Example: "%SystemRoot%"
    pub VariableValue: Option<String>,
}

/// The `Win32_TimeZone`
/// WMI class represents the time zone information for a computer system running Windows,
/// which includes the changes required for transitioning to daylight saving time transition.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-timezone>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_TimeZone {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Current bias for local time translation.
    /// The bias is the difference between Coordinated Universal Time (UTC) and local time.
    /// All translations between UTC and local time are based on the following formula:
    /// UTC = local time - bias.
    /// This property is required.
    pub Bias: Option<i32>,
    /// Bias value to be used during local time translations that occur during daylight saving time.
    /// This property is ignored if a value for the DaylightDay property is not supplied.
    /// The value of this property is added to the Bias property
    /// to form the bias used during daylight time.
    /// In most time zones, the value of this property is -60.
    pub DaylightBias: Option<i32>,
    /// DaylightDayOfWeek of the DaylightMonth when the transition from standard time to daylight
    /// saving time occurs on this operating system.
    ///
    /// Example: If the transition day (DaylightDayOfWeek) occurs on a Sunday, then the value "1"
    /// indicates the first Sunday of the DaylightMonth,
    /// "2" indicates the second Sunday, and so on.
    /// The value "5" indicates the last DaylightDayOfWeek in the month.
    pub DaylightDay: Option<u32>,
    /// Day of the week when the transition from standard time to daylight saving time occurs on an
    /// operating system.
    ///
    /// - Sunday (0)
    /// - Monday (1)
    /// - Tuesday (2)
    /// - Wednesday (3)
    /// - Thursday (4)
    /// - Friday (5)
    /// - Saturday (6)
    ///
    /// Example: 1
    pub DaylightDayOfWeek: Option<u8>,
    /// Hour of the day when the transition from standard time to daylight saving time occurs on an
    /// operating system.
    ///
    /// Example: 2
    pub DaylightHour: Option<u32>,
    /// Millisecond of the DaylightSecond when the transition from standard time to daylight saving
    /// time occurs on an operating system.
    pub DaylightMillisecond: Option<u32>,
    /// Minute of the DaylightHour when the transition from standard time to daylight saving time
    /// occurs on an operating system.
    ///
    /// Example: 59
    pub DaylightMinute: Option<u32>,
    /// Month when the transition from standard time to daylight saving time occurs on an
    /// operating system.
    ///
    /// - January (1)
    /// - February (2)
    /// - March (3)
    /// - April (4)
    /// - May (5)
    /// - June (6)
    /// - July (7)
    /// - August (8)
    /// - September (9)
    /// - October (10)
    /// - November (11)
    /// - December (12)
    pub DaylightMonth: Option<u32>,
    /// Time zone being represented when daylight saving time is in effect.
    ///
    /// Example: "EDT" (Eastern Daylight Time)
    pub DaylightName: Option<String>,
    /// Second of the DaylightMinute when the transition from standard time to daylight saving time
    /// occurs on an operating system.
    ///
    /// Example: 59
    pub DaylightSecond: Option<u32>,
    /// Year when daylight saving time is in effect. This property is not required.
    ///
    /// Example: 1997
    pub DaylightYear: Option<u32>,
    /// Bias value to use when daylight saving time is not in effect. This property is ignored if a
    /// value for StandardDay is not supplied. The value of this property is added to the Bias
    /// property to form the bias during standard time.
    ///
    /// Example: 0
    pub StandardBias: Option<u32>,
    /// StandardDayOfWeek of the StandardMonth when the transition from daylight saving time to
    /// standard time occurs on an operating system.
    ///
    /// If the transition day (StandardDayOfWeek) occurs on a Sunday,
    /// then the value "1" indicates the first Sunday of the StandardMonth,
    /// "2" indicates the second Sunday, and so on.
    /// The value "5" indicates the last StandardDayOfWeek in the month.
    pub StandardDay: Option<u32>,
    /// Day of the week when the transition from daylight saving time to standard time occurs on an
    /// operating system.
    ///
    /// - Sunday (0)
    /// - Monday (1)
    /// - Tuesday (2)
    /// - Wednesday (3)
    /// - Thursday (4)
    /// - Friday (5)
    /// - Saturday (6)
    pub StandardDayOfWeek: Option<u8>,
    /// Hour of the day when the transition from daylight saving time to standard time occurs on an
    /// operating system.
    ///
    /// Example: 11
    pub StandardHour: Option<u32>,
    /// Millisecond of the StandardSecond when the transition from daylight saving time to standard
    /// time occurs on an operating system.
    pub StandardMillisecond: Option<u32>,
    /// Minute of the StandardDay when the transition from daylight saving time to standard time
    /// occurs on an operating system.
    ///
    /// Example: 59
    pub StandardMinute: Option<u32>,
    /// Month when the transition from daylight saving time to standard time occurs on an
    /// operating system.
    ///
    /// - January (1)
    /// - February (2)
    /// - March (3)
    /// - April (4)
    /// - May (5)
    /// - June (6)
    /// - July (7)
    /// - August (8)
    /// - September (9)
    /// - October (10)
    /// - November (11)
    /// - December (12)
    pub StandardMonth: Option<u32>,
    /// Name of the time zone being represented when standard time is in effect.
    ///
    /// Example: "EST" (Eastern Standard Time)
    pub StandardName: Option<String>,
    /// Second of the StandardMinute when the transition from daylight saving time to standard time
    /// occurs on an operating system.
    ///
    /// Example: 59
    pub StandardSecond: Option<u32>,
    /// Year when standard time is in effect. This property is not required.
    ///
    /// Example: 1997
    pub StandardYear: Option<u32>,
}

/// The `Win32_UserDesktop` association WMI class relates a user account and desktop settings that
/// are specific to it.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-userdesktop>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_UserDesktop {
    /// Reference to the instance representing the user account whose desktop can be customized by
    /// the Settings property of this class.
    pub Element: Option<Win32_UserAccount>,
    /// Reference to the instance representing the desktop settings that serve to customize a
    /// specific user account desktop.
    pub Setting: Option<Win32_Desktop>,
}

//! The Desktop subcategory groups classes that represent objects that define a specific desktop configuration.
//!
//! | Class                                           | Description                                                                                                                        |
//! |-------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Desktop**](win32-desktop)         | Instance class<br/> Represents the common characteristics of a user's desktop.<br/>                                    |
//! | [**Win32\_Environment**](win32-environment) | Instance class<br/> Represents an environment or system environment setting on a computer system running Windows.<br/> |
//! | [**Win32\_TimeZone**](win32-timezone)       | Instance class<br/> Represents the time zone information for a computer system running Windows.<br/>                   |
//! | [**Win32\_UserDesktop**](win32-userdesktop) | Association class<br/> Relates a user account and the desktop settings that are specific to it.<br/>                   |

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

/// The `Win32_Desktop` WMI class represents the common characteristics of a user's desktop. The
/// properties of this class can be modified by the user to customize the desktop.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-desktop>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Desktop {
    /// Short textual description of the current object.
    Caption: Option<String>,
    /// Textual description of the current object.
    Description: Option<String>,
    /// Identifier by which the current object is known.
    SettingID: Option<String>,
    /// Width of the borders around all windows with adjustable borders.
    ///
    /// Example: 3
    BorderWidth: Option<u32>,
    /// Fast task switching is turned on. Fast task switching allows the user to switch between
    /// windows using the ALT+TAB keyboard combination.
    CoolSwitch: Option<bool>,
    /// Length of time between successive cursor blinks.
    ///
    /// Example: 530
    CursorBlinkRate: Option<u32>,
    /// Contents of a window are shown when a user moves the window.
    DragFullWindows: Option<bool>,
    /// Spacing of the grid that windows are bound to on the desktop. This makes organizing windows
    /// easier. The spacing is usually fine enough that the user does not notice it.
    ///
    /// Example: 1
    GridGranularity: Option<u32>,
    /// Spacing between icons.
    ///
    /// Example: 75
    IconSpacing: Option<u32>,
    /// Font used for the names of the icons.
    ///
    /// Example: "MS San Serif"
    IconTitleFaceName: Option<String>,
    /// Icon font size.
    ///
    /// Example: 9
    IconTitleSize: Option<u32>,
    /// Icon's title text wraps to the next line.
    IconTitleWrap: Option<bool>,
    /// Name that identifies the current desktop profile.
    ///
    /// Example: "MainProf"
    Name: Option<String>,
    /// Name of the pattern used as the background for the desktop.
    Pattern: Option<String>,
    /// Screen saver is active.
    ScreenSaverActive: Option<bool>,
    /// Name of the current screen saver executable file.
    ///
    /// Example: "LOGON.SCR"
    ScreenSaverExecutable: Option<String>,
    /// Password is enabled for the screen saver.
    ScreenSaverSecure: Option<bool>,
    /// Amount of time that passes before the screen saver starts.
    ScreenSaverTimeout: Option<u32>,
    /// File name for the wallpaper design on the background of the desktop.
    ///
    /// Example: "WINNT.BMP"
    Wallpaper: Option<String>,
    /// Wallpaper is stretched to fill the entire screen. Microsoft Plus! must be installed before
    /// this option is available. If FALSE, the wallpaper retains its original dimensions on the
    /// desktop background.
    WallpaperStretched: Option<bool>,
    /// Wallpaper is tiled or centered.
    WallpaperTiled: Option<bool>,
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
    Caption: Option<String>,
    /// A textual description of the object.
    Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object
    /// is not installed.
    InstallDate: Option<WMIDateTime>,
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
    Status: Option<String>,
    /// Character string that specifies the name of a Windows-based environment variable.
    /// By specifying the name of a variable that does not yet exist,
    /// an application creates a new environment variable.
    ///
    /// Example: "Path"
    Name: Option<String>,
    /// Indicates whether the variable is a system variable.
    /// A system variable is set by the operating system,
    /// and is independent from user environment settings.
    SystemVariable: Option<bool>,
    /// Name of the owner of the environment setting.
    /// It is set to <SYSTEM> for settings that are specific to the Windows-based system
    /// (as opposed to a specific user) and <DEFAULT> for default user settings.
    ///
    /// Example: "JSmith"
    UserName: Option<String>,
    /// Placeholder variable of a Windows-based environment variable.
    /// Information like the file system directory can change from computer to computer.
    /// The operating system substitutes placeholders for these.
    ///
    /// Example: "%SystemRoot%"
    VariableValue: Option<String>,
}

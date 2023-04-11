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

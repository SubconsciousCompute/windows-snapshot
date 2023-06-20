//! Classes in the Mass Storage subcategory represent storage devices such as hard disk drives, CD-ROM drives, and tape drives.
//! 
//! | Class                                                     | Description                                                                                  |
//! |-----------------------------------------------------------|----------------------------------------------------------------------------------------------|
//! | [**Win32\_AutochkSetting**](win32-autochksetting)        | Represents the settings for the autocheck operation of a disk.                               |
//! | [**Win32\_CDROMDrive**](win32-cdromdrive)                | Represents a CD-ROM drive on a computer system running Windows.                              |
//! | [**Win32\_DiskDrive**](win32-diskdrive)                  | Represents a physical disk drive as seen by a computer running the Windows operating system. |
//! | [**Win32\_FloppyDrive**](win32-floppydrive)              | Manages the capabilities of a floppy disk drive.                                             |
//! | [**Win32\_PhysicalMedia**](/previous-versions/windows/desktop/cimwin32a/win32-physicalmedia) | Represents any type of documentation or storage medium.                                      |
//! | [**Win32\_TapeDrive**](win32-tapedrive)                  | Represents a tape drive on a computer system running Windows.                                |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's AutochkSettings
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct AutochkSettings {
    /// Sequence of windows AutochkSettings states
    pub autochk_settings: Vec<Win32_AutochkSetting>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(AutochkSettings, autochk_settings);

/// The `Win32_AutochkSetting` WMI class represents the settings for the autocheck operation of 
/// a disk.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-autochksetting> 
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_AutochkSetting {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// An ID that is used as part of a key for the current object.
    pub SettingID: Option<String>,
    /// The user input delay for autocheck.
    pub UserInputDelay: Option<u32>,
}
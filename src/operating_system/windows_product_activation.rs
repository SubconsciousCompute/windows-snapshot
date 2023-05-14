//! Windows Product Activation (WPA) is an antipiracy technology to reduce the casual copying of software.
//! 
//! | Class                                                                                                               | Description                                                                                                                                                                                       |
//! |---------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_ComputerSystemWindowsProductActivationSetting**](/previous-versions/windows/desktop/licwmiprov/win32-computersystemwindowsproductactivationsetting) | Association class<br/> Relates instances of [**Win32\_ComputerSystem**](win32-computersystem) and [**Win32\_WindowsProductActivation**](/previous-versions/windows/desktop/legacy/aa394520(v=vs.85)).<br/> |
//! | [**Win32\_Proxy**](/previous-versions/windows/desktop/legacy/aa394389(v=vs.85))                                                                                 | Instance class<br/> Contains properties and methods to query and configure an Internet connection related to WPA.<br/>                                                                |
//! | [**Win32\_WindowsProductActivation**](/previous-versions/windows/desktop/legacy/aa394520(v=vs.85))                                           | Instance class<br/> Contains properties and methods related to WPA.<br/>                                                                                                              |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection};

/// Represents the state of Windows Proxys 
/// 
/// Note: this class doesn't exist anymore
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Proxys {
    /// Represents sequence of Windows `Proxys`
    pub proxys: Vec<Win32_Proxy>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(Proxys, proxys);

/// The `Win32_Proxy` WMI class contains properties and methods to query and configure an Internet 
/// connection related to Windows Product Activation (WPA).
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394389(v=vs.85)>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Proxy {
    /// Short textual description of the `CIM_Setting` object. 
    pub Caption: Option<String>,
    /// Textual description of the `CIM_Setting` object.
    pub Description: Option<String>,
    /// Port number configured on the computer for access to the proxy server specified by the 
    /// `ProxyServer` property.
    pub ProxyPortNumber: Option<String>,
    /// Name of the proxy server configured for the user.
    pub ProxyServer: Option<String>,
    /// Name of the computer whose proxy settings are to be accessed.
    pub ServerName: Option<String>,
    /// Identifier by which the `CIM_Setting` object is known.
    pub SettingID: Option<String>,
}
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
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Proxys, proxys);

/// Represents the state of Windows `WindowsProductActivations`
/// 
/// Note: this class doesn't exist anymore
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WindowsProductActivations {
    /// Represents sequence of Windows `WindowsProductActivations`
    pub windows_product_activations: Vec<Win32_WindowsProductActivation>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(WindowsProductActivations, windows_product_activations);

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

/// The `Win32_WindowsProductActivation` WMI class contains properties and methods related to Windows 
/// Product Activation (WPA), such as activation state and grace period. This class also provides the 
/// ability to activate the customer's computer online and offline.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394520(v=vs.85)>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_WindowsProductActivation {
    /// If 1 (one), system activation is pending for the system. The system must be activated within 
    /// the number of days indicated by the `RemainingGracePeriod` property. If 0 (zero), activation 
    /// is not required during a specific time period.
    pub ActivationRequired: Option<u32>,
    /// Short textual description of the `CIM_Setting` object.
    pub Caption: Option<String>,
    /// Textual description of the `CIM_Setting` object.
    pub Description: Option<String>,
    /// If not equal to 0 (zero), and product activation is required, notification reminders (message 
    /// balloons) are enabled and the activation icon appears in the notification tray. If 0 (zero), 
    /// notification reminders and the activation icon are disabled.
    /// 
    /// `Windows XP Professional` : This property is not available. It becomes available in Windows 
    /// XP with SP1.
    pub IsNotificationOn: Option<u32>,
    /// String of 20 characters separated by hyphens in the format, "xxxxx-xxx-xxxxxxx-xxxxx". This is 
    /// the same product ID that is displayed under the `General` tab of the `System Properties` dialog 
    /// in Control Panel.
    pub ProductID: Option<String>,
    /// If this instance represents beta or evaluation media, this represents the number of days remaining 
    /// before expiration of the media. Otherwise, this property is set to the largest possible unsigned value.
    pub RemainingEvaluationPeriod: Option<u32>,
    /// Number of days remaining before activation of the system is required, if the `ActivationRequired` 
    /// property is equal to 1.
    pub RemainingGracePeriod: Option<u32>,
    /// System whose WPA properties and methods are to be accessed. This property is a string that specifies 
    /// the name of the computer or its IP address.
    pub ServerName: Option<String>,
    /// Identifier by which the `CIM_Setting` object is known.
    pub SettingID: Option<String>,
}
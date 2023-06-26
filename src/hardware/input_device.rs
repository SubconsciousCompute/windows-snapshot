//! The Input Devices subcategory groups classes that represent keyboards and pointing devices.
//! 
//! | Class                                                 | Description                                                                                                         |
//! |-------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Keyboard**](win32-keyboard)                | Represents a keyboard installed on a computer system running Windows.                                               |
//! | [**Win32\_PointingDevice**](win32-pointingdevice)    | Represents an input device used to point to and select regions on the display of a computer system running Windows. |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's Keyboards
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct Keyboards {
    /// Sequence of windows Keyboards states
    pub keyboards: Vec<Win32_Keyboard>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Keyboards, keyboards);

/// Represents the state of Windows user's PointingDevices
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct PointingDevices {
    /// Sequence of windows PointingDevices states
    pub pointing_devices: Vec<Win32_PointingDevice>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(PointingDevices, pointing_devices);

/// The `Win32_Keyboard` WMI class represents a keyboard installed on a computer system running Windows.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-keyboard>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Keyboard {
    /// Availability and status of the device.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Running` / `Full Power` (3): Running or Full Power
    /// - `Warning` (4)
    /// - `In Test` (5)
    /// - `Not Applicable` (6)
    /// - `Power Off` (7)
    /// - `Off Line` (8)
    /// - `Off Duty` (9)
    /// - `Degraded` (10)
    /// - `Not Installed` (11)
    /// - `Install Error` (12)
    /// - `Power Save - Unknown` (13): The device is known to be in a power save mode, but its exact status is unknown.
    /// - `Power Save - Low Power Mode` (14): The device is in a power save state but still functioning, and may exhibit degraded performance.
    /// - `Power Save - Standby` (15): The device is not functioning, but could be brought to full power quickly.
    /// - `Power Cycle` (16)
    /// - `Power Save - Warning` (17): The device is in a warning state, though also in a power save mode.
    /// - `Paused` (18): The device is paused.
    /// - `Not Ready` (19): The device is not ready.
    /// - `Not Configured` (20): The device is not configured.
    /// - `Quiesced` (21): The device is quiet. 
    pub Availability: Option<u16>,
    /// Short description of the object a one-line string.
    pub Caption: Option<String>,
    /// Win32 Configuration Manager error code.
    /// 
    /// - `This device is working properly.` (0): Device is working properly.
    /// - `This device is not configured correctly.` (1): Device is not configured correctly.
    /// - `Windows cannot load the driver for this device.` (2)
    /// - `The driver for this device might be corrupted, or your system may be running low on memory or other resources.` (3): Driver for this device might be corrupted, or the system may be low on memory or other resources.
    /// - `This device is not working properly. One of its drivers or your registry might be corrupted.` (4): Device is not working properly. One of its drivers or the registry might be corrupted.
    /// - `The driver for this device needs a resource that Windows cannot manage.` (5): Driver for the device requires a resource that Windows cannot manage.
    /// - `The boot configuration for this device conflicts with other devices.` (6): Boot configuration for the device conflicts with other devices.
    /// - `Cannot filter. (7)
    /// - `The driver loader for the device is missing.` (8): Driver loader for the device is missing.
    /// - `This device is not working properly because the controlling firmware is reporting the resources for the device incorrectly.` (9): Device is not working properly. The controlling firmware is incorrectly reporting the resources for the device.
    /// - `This device cannot start.` (10): Device cannot start.
    /// - `This device failed.` (11): Device failed.
    /// - `This device cannot find enough free resources that it can use.` (12): Device cannot find enough free resources to use.
    /// - `Windows cannot verify this device's resources.` (13): Windows cannot verify the device's resources.
    /// - `This device cannot work properly until you restart your computer.` (14): Device cannot work properly until the computer is restarted.
    /// - `This device is not working properly because there is probably a re-enumeration problem.` (15): Device is not working properly due to a possible re-enumeration problem.
    /// - `Windows cannot identify all the resources this device uses.` (16): Windows cannot identify all of the resources that the device uses.
    /// - `This device is asking for an unknown resource type.` (17): Device is requesting an unknown resource type.
    /// - `Reinstall the drivers for this device.` (18): Device drivers must be reinstalled.
    /// - `Failure using the VxD loader.` (19)
    /// - `Your registry might be corrupted.` (20): Registry might be corrupted.
    /// - `System failure: Try changing the driver for this device. If that does not work, see your hardware documentation. Windows is removing this device.` (21): System failure. If changing the device driver is ineffective, see the hardware documentation. Windows is removing the device.
    /// - `This device is disabled.` (22): Device is disabled.
    /// - `System failure: Try changing the driver for this device. If that doesn't work, see your hardware documentation.` (23): System failure. If changing the device driver is ineffective, see the hardware documentation.
    /// - `This device is not present, is not working properly, or does not have all its drivers installed.` (24): Device is not present, not working properly, or does not have all of its drivers installed.
    /// - `Windows is still setting up this device.` (25): Windows is still setting up the device.
    /// - `Windows is still setting up this device.` (26): Windows is still setting up the device.
    /// - `This device does not have valid log configuration.` (27): Device does not have valid log configuration.
    /// - `The drivers for this device are not installed.` (28): Device drivers are not installed.
    /// - `This device is disabled because the firmware of the device did not give it the required resources.` (29): Device is disabled. The device firmware did not provide the required resources.
    /// - `This device is using an Interrupt Request (IRQ) resource that another device is using.` (30): Device is using an IRQ resource that another device is using.
    /// - `This device is not working properly because Windows cannot load the drivers required for this device.` (31): Device is not working properly. Windows cannot load the required device drivers.
    pub ConfigManagerErrorCode: Option<u32>,
    /// If `TRUE`, the device is using a user-defined configuration.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Name of the first concrete class that appears in the inheritance chain used in the 
    /// creation of an instance. When used with the other key properties of the class, the 
    /// property allows all instances of this class and its subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Address or other identifying information to uniquely name the logical device.
    pub DeviceID: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and corrective actions that 
    /// may be taken.
    pub ErrorDescription: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate that 
    /// the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// If `TRUE`, the device is locked, preventing user input or output.
    pub IsLocked: Option<bool>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Free-form string indicating the layout of the keyboard.
    pub Layout: Option<String>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
    pub Name: Option<String>,
    /// Number of function keys on the keyboard.
    pub NumberOfFunctionKeys: Option<u16>,
    /// Status of hardware-level password enabled at the keyboard (value=4), preventing local 
    /// input.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Disabled` (3)
    /// - `Enabled` (4)
    /// - `Not Implemented` (5)
    pub Password: Option<u16>,
    /// Windows Plug and Play device identifier of the logical device.
    /// 
    /// Example: "*PNP030b"
    pub PNPDeviceID: Option<String>,
    /// Array of the specific power-related capabilities of a logical device.
    /// 
    /// - `Unknown` (0)
    /// - `Not Supported` (1): Power-related capacities are not supported for this device.
    /// - `Disabled` (2)
    /// - `Enabled` (3): The power management features are currently enabled but the exact feature set is unknown or the information is unavailable.
    /// - `Power Saving Modes Entered Automatically` (4): The device can change its power state based on usage or other criteria.
    /// - `Power State Settable` (5): The `SetPowerState` method is supported. This method is found on the parent CIM_LogicalDevice class and can be implemented. For more information, see Designing Managed Object Format (MOF) Classes.
    /// - `Power Cycling Supported` (6): The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle).
    /// - `Timed Power On Supported` (7): Timed Power-On Supported. The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle) and Time set to a specific date and time, or interval, for power-on.
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /// If `TRUE`, the device can be power-managed (can be put into suspend mode, and so on). The 
    /// property does not indicate that power management features are currently enabled, only that 
    /// the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. 
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a 
    /// SMART-enabled hard disk drive, may be functioning properly but predicting a failure in the 
    /// near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service". 
    /// The latter, "Service", could apply during mirror-resilvering of a disk, reload of a user 
    /// permissions list, or other administrative work. Not all such work is online, yet the managed 
    /// element is neither "OK" nor in one of the other states.
    /// 
    /// Values include the following:
    /// - `OK` ("OK")
    /// - `Error` ("Error")
    /// - `Degraded` ("Degraded")
    /// - `Unknown` ("Unknown")
    /// - `Pred Fail` ("Pred Fail")
    /// - `Starting` ("Starting")
    /// - `Stopping` ("Stopping")
    /// - `Service` ("Service")
    /// - `Stressed` ("Stressed")
    /// - `NonRecover` ("NonRecover")
    /// - `No Contact` ("No Contact")
    /// - `Lost Comm` ("Lost Comm")
    pub Status: Option<String>,
    /// State of the logical device. If this property does not apply to the logical device, the 
    /// value 5 (Not Applicable) should be used.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Enabled` (3)
    /// - `Disabled` (4)
    /// - `Not Applicable` (5)
    pub StatusInfo: Option<u16>,
    /// Value of the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
}

/// The `Win32_PointingDevice` WMI class represents an input device used to point to and select 
/// regions on the display of a computer system running Windows. Any device used to manipulate 
/// a pointer, or point to the display on a computer system running Windows is a member of 
/// this class.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-pointingdevice>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PointingDevice {
    /// Availability and status of the device.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Running` / `Full Power` (3): Running or Full Power
    /// - `Warning` (4)
    /// - `In Test` (5)
    /// - `Not Applicable` (6)
    /// - `Power Off` (7)
    /// - `Off Line` (8)
    /// - `Off Duty` (9)
    /// - `Degraded` (10)
    /// - `Not Installed` (11)
    /// - `Install Error` (12)
    /// - `Power Save - Unknown` (13): The device is known to be in a power save mode, but its exact status is unknown.
    /// - `Power Save - Low Power Mode` (14): The device is in a power save state but still functioning, and may exhibit degraded performance.
    /// - `Power Save - Standby` (15): The device is not functioning, but could be brought to full power quickly.
    /// - `Power Cycle` (16)
    /// - `Power Save - Warning` (17): The device is in a warning state, though also in a power save mode.
    /// - `Paused` (18): The device is paused.
    /// - `Not Ready` (19): The device is not ready.
    /// - `Not Configured` (20): The device is not configured.
    /// - `Quiesced` (21): The device is quiet. 
    pub Availability: Option<u16>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// Win32 Configuration Manager error code.
    /// 
    /// - `This device is working properly.` (0): Device is working properly.
    /// - `This device is not configured correctly.` (1): Device is not configured correctly.
    /// - `Windows cannot load the driver for this device.` (2)
    /// - `The driver for this device might be corrupted, or your system may be running low on memory or other resources.` (3): Driver for this device might be corrupted, or the system may be low on memory or other resources.
    /// - `This device is not working properly. One of its drivers or your registry might be corrupted.` (4): Device is not working properly. One of its drivers or the registry might be corrupted.
    /// - `The driver for this device needs a resource that Windows cannot manage.` (5): Driver for the device requires a resource that Windows cannot manage.
    /// - `The boot configuration for this device conflicts with other devices.` (6): Boot configuration for the device conflicts with other devices.
    /// - `Cannot filter. (7)
    /// - `The driver loader for the device is missing.` (8): Driver loader for the device is missing.
    /// - `This device is not working properly because the controlling firmware is reporting the resources for the device incorrectly.` (9): Device is not working properly. The controlling firmware is incorrectly reporting the resources for the device.
    /// - `This device cannot start.` (10): Device cannot start.
    /// - `This device failed.` (11): Device failed.
    /// - `This device cannot find enough free resources that it can use.` (12): Device cannot find enough free resources to use.
    /// - `Windows cannot verify this device's resources.` (13): Windows cannot verify the device's resources.
    /// - `This device cannot work properly until you restart your computer.` (14): Device cannot work properly until the computer is restarted.
    /// - `This device is not working properly because there is probably a re-enumeration problem.` (15): Device is not working properly due to a possible re-enumeration problem.
    /// - `Windows cannot identify all the resources this device uses.` (16): Windows cannot identify all of the resources that the device uses.
    /// - `This device is asking for an unknown resource type.` (17): Device is requesting an unknown resource type.
    /// - `Reinstall the drivers for this device.` (18): Device drivers must be reinstalled.
    /// - `Failure using the VxD loader.` (19)
    /// - `Your registry might be corrupted.` (20): Registry might be corrupted.
    /// - `System failure: Try changing the driver for this device. If that does not work, see your hardware documentation. Windows is removing this device.` (21): System failure. If changing the device driver is ineffective, see the hardware documentation. Windows is removing the device.
    /// - `This device is disabled.` (22): Device is disabled.
    /// - `System failure: Try changing the driver for this device. If that doesn't work, see your hardware documentation.` (23): System failure. If changing the device driver is ineffective, see the hardware documentation.
    /// - `This device is not present, is not working properly, or does not have all its drivers installed.` (24): Device is not present, not working properly, or does not have all of its drivers installed.
    /// - `Windows is still setting up this device.` (25): Windows is still setting up the device.
    /// - `Windows is still setting up this device.` (26): Windows is still setting up the device.
    /// - `This device does not have valid log configuration.` (27): Device does not have valid log configuration.
    /// - `The drivers for this device are not installed.` (28): Device drivers are not installed.
    /// - `This device is disabled because the firmware of the device did not give it the required resources.` (29): Device is disabled. The device firmware did not provide the required resources.
    /// - `This device is using an Interrupt Request (IRQ) resource that another device is using.` (30): Device is using an IRQ resource that another device is using.
    /// - `This device is not working properly because Windows cannot load the drivers required for this device.` (31): Device is not working properly. Windows cannot load the required device drivers.
    pub ConfigManagerErrorCode: Option<u32>,
    /// If `TRUE`, the device is using a user-defined configuration.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation 
    /// of an instance. When used with the other key properties of the class, the property allows 
    /// all instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the pointing device with other devices on the system.
    pub DeviceID: Option<String>,
    /// Type of interface used for the pointing device.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Serial` (3)
    /// - `PS/2` (4)
    /// - `Infrared` (5)
    /// - `HP-HIL` (6)
    /// - `Bus mouse` (7)
    /// - `ADB (Apple Desktop Bus)` (8)
    /// - `Bus mouse DB-9` (160)
    /// - `Bus mouse micro-DIN` (161)
    /// - `USB` (162)
    pub DeviceInterface: Option<u16>,
    /// One of two acceleration values. The sensitivity of the pointing device doubles 
    /// (toggles from the first to the second value) when the pointing device moves a distance 
    /// greater than this threshold value.
    pub DoubleSpeedThreshold: Option<u32>,
    /// If `TRUE`, the error reported in LastErrorCode is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information on any 
    /// corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Configuration of the pointing device for left-hand or right-hand operation.
    /// 
    /// - `Unknown` (0)
    /// - `Not Applicable` (1)
    /// - `Right Handed Operation` (2): Right-Handed Operation
    /// - `Left Handed Operation` (3): Left-Handed Operation
    pub Handedness: Option<u16>,
    /// Type of hardware used for the Windows pointing device.
    /// 
    /// Example: "MICROSOFT PS2 MOUSE"
    pub HardwareType: Option<String>,
    /// Name of the .inf file for the Windows pointing device.
    /// 
    /// Example: "ab.inf"
    pub InfFileName: Option<String>,
    /// Section of the .inf file that holds configuration information for the Windows pointing 
    /// device.
    pub InfSection: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// If `TRUE`, the device is locked, preventing user input or output.
    pub IsLocked: Option<bool>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Name of the processor's manufacturer.
    /// 
    /// Example: "GenuineSilicon"
    pub Manufacturer: Option<String>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
    pub Name: Option<String>,
    /// Number of buttons on the pointing device.
    /// 
    /// Example: 2
    pub NumberOfButtons: Option<u8>,
    /// Windows Plug and Play device identifier of the logical device.
    /// 
    /// Example: "*PNP030b"
    pub PNPDeviceID: Option<String>,
    /// Type of pointing device.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Mouse` (3)
    /// - `Track Ball` (4): Trackball
    /// - `Track Point` (5)
    /// - `Glide Point` (6)
    /// - `Touch Pad` (7)
    /// - `Touch Screen` (8)
    /// - `Mouse - Optical Sensor` (9)
    pub PointingType: Option<u16>,
    /// Array of the specific power-related capabilities of a logical device.
    /// 
    /// - `Unknown` (0)
    /// - `Not Supported` (1): Power-related capacities are not supported for this device.
    /// - `Disabled` (2)
    /// - `Enabled` (3): The power management features are currently enabled but the exact feature set is unknown or the information is unavailable.
    /// - `Power Saving Modes Entered Automatically` (4): The device can change its power state based on usage or other criteria.
    /// - `Power State Settable` (5): The `SetPowerState` method is supported. This method is found on the parent CIM_LogicalDevice class and can be implemented. For more information, see Designing Managed Object Format (MOF) Classes.
    /// - `Power Cycling Supported` (6): The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle).
    /// - `Timed Power On Supported` (7): Timed Power-On Supported. The `SetPowerState` method can be invoked with the PowerState parameter set to 5 (Power Cycle) and Time set to a specific date and time, or interval, for power-on.
    pub PowerManagementCapabilities: Option<Vec<u16>>,
    /// If `TRUE`, the device can be power-managed (can be put into suspend mode, and so on). The 
    /// property does not indicate that power management features are currently enabled, only that 
    /// the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// One of two acceleration threshold values. The system doubles the speed of the pointer 
    /// movement when the pointer device moves a distance greater than this value. Because this 
    /// speed increase occurs after the DoubleSpeedThreshold value has been met, the pointer 
    /// effectively moves at four times its original speed.
    pub QuadSpeedThreshold: Option<u32>,
    /// Tracking resolution.
    /// 
    /// Example: 0
    pub Resolution: Option<u32>,
    /// Rate at which the pointing device is polled for input information.
    pub SampleRate: Option<u32>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. 
    /// Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a 
    /// SMART-enabled hard disk drive, may be functioning properly but predicting a failure in the 
    /// near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service". 
    /// The latter, "Service", could apply during mirror-resilvering of a disk, reload of a user 
    /// permissions list, or other administrative work. Not all such work is online, yet the managed 
    /// element is neither "OK" nor in one of the other states.
    /// 
    /// Values include the following:
    /// - `OK` ("OK")
    /// - `Error` ("Error")
    /// - `Degraded` ("Degraded")
    /// - `Unknown` ("Unknown")
    /// - `Pred Fail` ("Pred Fail")
    /// - `Starting` ("Starting")
    /// - `Stopping` ("Stopping")
    /// - `Service` ("Service")
    /// - `Stressed` ("Stressed")
    /// - `NonRecover` ("NonRecover")
    /// - `No Contact` ("No Contact")
    /// - `Lost Comm` ("Lost Comm")
    pub Status: Option<String>,
    /// State of the logical device. If this property does not apply to the logical device, the 
    /// value 5 (Not Applicable) should be used.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Enabled` (3)
    /// - `Disabled` (4)
    /// - `Not Applicable` (5)
    pub StatusInfo: Option<u16>,
    /// Length of time after which the next interrupt is assumed to indicate the start of a new 
    /// device packet (partial packets are discarded). In the event that an interrupt is lost,
    /// this allows the pointing device driver to synchronize its internal representation of the 
    /// packet state with the hardware state.
    pub Synch: Option<u32>,
    /// Value of the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
}
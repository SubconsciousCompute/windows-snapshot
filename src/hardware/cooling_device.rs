//! The Cooling Devices subcategory groups classes that represent instrumentable fans, temperature probes, and refrigeration devices.
//! 
//! | Class                                                     | Description                                                                 |
//! |-----------------------------------------------------------|-----------------------------------------------------------------------------|
//! | [**Win32\_Fan**](win32-fan)                              | Represents the properties of a fan device in the computer system.           |
//! | [**Win32\_HeatPipe**](win32-heatpipe)                    | Represents the properties of a heat pipe cooling device.                    |
//! | [**Win32\_Refrigeration**](win32-refrigeration)          | Represents the properties of a refrigeration device.                        |
//! | [**Win32\_TemperatureProbe**](win32-temperatureprobe)    | Represents the properties of a temperature sensor (electronic thermometer). |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's fans
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct Fans {
    /// Sequence of windows Fans states
    pub fans: Vec<Win32_Fan>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Fans, fans);

/// Represents the state of Windows user's HeatPipes
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct HeatPipes {
    /// Sequence of windows HeatPipes states
    pub heat_pipes: Vec<Win32_HeatPipe>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(HeatPipes, heat_pipes);

/// Represents the state of Windows user's Refrigerations
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct Refrigerations {
    /// Sequence of windows Refrigerations states
    pub refrigerations: Vec<Win32_Refrigeration>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Refrigerations, refrigerations);

/// Represents the state of Windows user's TemperatureProbes
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct TemperatureProbes {
    /// Sequence of windows TemperatureProbes states
    pub temperature_probes: Vec<Win32_TemperatureProbe>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(TemperatureProbes, temperature_probes);

/// The `Win32_Fan` WMI class represents the properties of a fan device in the computer system. 
/// For example, the CPU cooling fan.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-fan>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Fan {
    /// If `True`, the cooling device provides active (as opposed to passive) cooling.
    pub ActiveCooling: Option<bool>,
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
    /// If `True`, the device is using a user-defined configuration.
    pub ConfigManagerUserConfig: Option<bool>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation 
    /// of an instance. When used with the other key properties of the class, the property allows 
    /// all instances of this class and its subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Currently requested fan speed, defined in revolutions per minute, when a variable speed fan 
    /// is supported (`VariableSpeed` is `TRUE`). The current speed is determined by a sensor 
    /// (`CIM_Tachometer`) that is associated with the fan using the `CIM_AssociatedSensor` relationship.
    pub DesiredSpeed: Option<u64>,
    /// Identifies the fan device. 
    pub DeviceID: Option<String>,
    /// If `True`, the error reported in the `LastErrorCode` property is now cleared.
    pub ErrorCleared: Option<bool>,
    /// Free-form string supplying more information about the error recorded in `LastErrorCode` property, 
    /// and information on any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate that 
    /// the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Label by which the object is known. When subclassed, the property can be overridden to be a 
    /// key property.
    pub Name: Option<String>,
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
    /// If `True`, the device can be power-managed (can be put into suspend mode, and so on). 
    /// The property does not indicate that power management features are currently enabled, 
    /// only that the logical device is capable of power management.
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
    /// If `True`, the fan supports variable speeds.
    pub VariableSpeed: Option<bool>,
}

/// The `Win32_HeatPipe` WMI class represents the properties of a heat pipe cooling device.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-heatpipe>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_HeatPipe {
    /// If `TRUE`, the cooling device provides active cooling not passive.
    pub ActiveCooling: Option<bool>,
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
    /// property allows all instances of this class and its subclasses to be identified 
    /// uniquely.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the heat pipe.
    pub DeviceID: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error that is recorded in `LastErrorCode`, and information 
    /// about any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
    pub Name: Option<String>,
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
    /// If `TRUE`, the device can be power-managed put into suspend mode, and so on. The property 
    /// does not indicate that power management features are enabled currently, only that the 
    /// logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// Current status of the object. Various operational and nonoperational statuses can be 
    /// defined. Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, 
    /// such as a SMART-enabled hard disk drive, may be functioning properly but predicting a 
    /// failure in the near future). Nonoperational statuses include: "Error", "Starting", 
    /// "Stopping", and "Service". The latter, "Service", could apply during mirror-resilvering 
    /// of a disk, reload of a user permissions list, or other administrative work. Not all such 
    /// work is online, yet the managed element is neither "OK" nor in one of the other states.
    /// 
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
    /// Value for the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
}

/// The `Win32_Refrigeration` WMI class represents the properties of a refrigeration device.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-refrigeration>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Refrigeration {
    /// If `TRUE`, the cooling device provides active cooling—not passive cooling.
    pub ActiveCooling: Option<bool>,
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
    /// Short description of the object—a one-line string.
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
    /// Unique identifier of the refrigeration device.
    pub DeviceID: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and any corrective actions 
    /// that may be taken.
    pub ErrorDescription: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
    pub Name: Option<String>,
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
    /// If `TRUE`, the device can be power-managed (can be put into suspend mode, and so on). 
    /// The property does not indicate that power management features are currently enabled, 
    /// only that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// Current status of the object. Various operational and nonoperational statuses can be 
    /// defined. Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such 
    /// as a SMART-enabled hard disk drive, may be functioning properly but predicting a failure in 
    /// the near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and 
    /// "Service". The latter, "Service", could apply during mirror-resilvering of a disk, reload 
    /// of a user permissions list, or other administrative work. Not all such work is online, yet 
    /// the managed element is neither "OK" nor in one of the other states.
    /// 
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
    /// Value for the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
}

/// The `Win32_TemperatureProbe` WMI class represents the properties of a temperature sensor 
/// (electronic thermometer).
/// 
/// Most of the information that the `Win32_TemperatureProbe` WMI class provides comes from 
/// SMBIOS. Real-time readings for the `CurrentReading` property cannot be extracted from SMBIOS 
/// tables. For this reason, current implementations of WMI do not populate the `CurrentReading` 
/// property. The `CurrentReading` property's presence is reserved for future use.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-temperatureprobe>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_TemperatureProbe {
    /// Accuracy of the sensor for the measured property. Its value is recorded as plus or 
    /// minus hundredths of a percent. Accuracy, resolution, and tolerance are used to 
    /// calculate the actual value of the measured physical property. Accuracy may vary and 
    /// depends on whether or not the device is linear over its dynamic range.
    pub Accuracy: Option<i32>,
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
    /// Short description of an object—a one-line string.
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
    /// creation of an instance. When used with the other key properties of a class, this property 
    /// allows all instances of the class and its subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Current implementations of WMI do not populate the `CurrentReading` property. The 
    /// `CurrentReading` property's presence is reserved for future use.
    pub CurrentReading: Option<i32>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the current probe.
    pub DeviceID: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information about any 
    /// corrective actions that you can take.
    pub ErrorDescription: Option<String>,
    /// Date and time the object is installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// If `TRUE`, the sensor is linear over its dynamic range.
    pub IsLinear: Option<bool>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Sensor threshold value to specify the ranges (minimum and maximum values) that identify 
    /// the sensor operating conditions, which can be normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is between `LowerThresholdCritical` and `LowerThresholdFatal`, 
    /// the current state is critical.
    pub LowerThresholdCritical: Option<i32>,
    /// Sensor threshold value to specify the ranges (minimum and maximum values) that identify the 
    /// sensor operating conditions, which can be normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is below `LowerThresholdFatal`, the current state is fatal.
    pub LowerThresholdFatal: Option<i32>,
    /// Sensor threshold value to specify the ranges (minimum and maximum values) that identify 
    /// the sensor operating conditions, which can be normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is between `LowerThresholdNonCritical` and 
    /// `UpperThresholdNonCritical`, the sensor is reporting a normal value. If `CurrentReading` is 
    /// between `LowerThresholdNonCritical` and `LowerThresholdCritical`, the current state is 
    /// noncritical.
    pub LowerThresholdNonCritical: Option<i32>,
    /// Largest value of the measured property that can be read by the numeric sensor.
    pub MaxReadable: Option<i32>,
    /// Smallest value of the measured property that can be read by the numeric sensor.
    pub MinReadable: Option<i32>,
    /// Label for the object. When subclassed, the property can be overridden to be a key property.
    pub Name: Option<String>,
    /// Normal or expected value for the numeric sensor.
    pub NominalReading: Option<i32>,
    /// Normal or expected value for the numeric sensor.
    pub NormalMax: Option<i32>,
    /// Guidance for the user as to the normal minimum range for the numeric sensor.
    pub NormalMin: Option<i32>,
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
    /// If `TRUE`, the device can be power-managed (can be put into suspend mode, and so on). 
    /// The property does not indicate that power management features are currently enabled, 
    /// only that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// Ability of the sensor to resolve differences in the measured property. This value may vary 
    /// depending on whether the device is linear over its dynamic range.
    pub Resolution: Option<u32>,
    /// Current status of the object. Various operational and nonoperational statuses can be 
    /// defined. Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, 
    /// such as a SMART-enabled hard disk drive, may be functioning properly but predicting a 
    /// failure in the near future). Nonoperational statuses include: "Error", "Starting", 
    /// "Stopping", and "Service". The latter, "Service", could apply during mirror-resilvering 
    /// of a disk, reload of a user permissions list, or other administrative work. Not all such 
    /// work is online, yet the managed element is neither "OK" nor in one of the other states.
    /// 
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
    /// Value for the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// Tolerance of the sensor for the measured property. Tolerance, along with resolution 
    /// and accuracy, is used to calculate the actual value of the measured physical property.
    /// Tolerance may vary depending on whether the device is linear over its dynamic range.
    pub Tolerance: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) that identify 
    /// the sensor operating conditions, which can be normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is between `UpperThresholdCritical` and `UpperThresholdFatal`, 
    /// the current state is critical.
    pub UpperThresholdCritical: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) that identify
    /// the sensor operating conditions, which can be normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is above `UpperThresholdFatal`, the current state is fatal.
    pub UpperThresholdFatal: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) that identify 
    /// the sensor operating conditions, which can be normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is between `LowerThresholdNonCritical` and `UpperThresholdNonCritical`, 
    /// the sensor is reporting a normal value. If `CurrentReading` is between 
    /// `UpperThresholdNonCritical` and `UpperThresholdCritical`, the current state is noncritical.
    pub UpperThresholdNonCritical: Option<i32>,
}
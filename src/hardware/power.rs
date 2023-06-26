//! The Power subcategory groups classes that represent power supplies, batteries, and events related to these devices.
//! 
//! 
//! 
//! | Class                                                             | Description                                                                                           |
//! |-------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
//! | [**Win32\_Battery**](win32-battery)                           | Represents a battery connected to the computer system.<br/>                                     |
//! | [**Win32\_CurrentProbe**](win32-currentprobe)                 | Represents the properties of a current monitoring sensor (ammeter).<br/>                        |
//! | [**Win32\_PortableBattery**](win32-portablebattery)           | Represents the properties of a portable battery, such as one used for a notebook computer.<br/> |
//! | [**Win32\_PowerManagementEvent**](win32-powermanagementevent) | Represents power management events resulting from power state changes.<br/>                     |
//! | [**Win32\_VoltageProbe**](win32-voltageprobe)                 | Represents the properties of a voltage sensor (electronic voltmeter).<br/>                      |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's Batteries
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct Batteries {
    /// Sequence of windows Batteries states
    pub batteries: Vec<Win32_Battery>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(Batteries, batteries);

/// Represents the state of Windows user's CurrentProbes
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct CurrentProbes {
    /// Sequence of windows CurrentProbes states
    pub current_probes: Vec<Win32_CurrentProbe>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(CurrentProbes, current_probes);

/// Represents the state of Windows user's PortableBatteries
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct PortableBatteries {
    /// Sequence of windows PortableBatteries states
    pub portable_batteries: Vec<Win32_PortableBattery>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(PortableBatteries, portable_batteries);

/// Represents the state of Windows user's PowerManagementEvents
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct PowerManagementEvents {
    /// Sequence of windows PowerManagementEvents states
    pub power_management_events: Vec<Win32_PowerManagementEvent>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(PowerManagementEvents, power_management_events);

/// The `Win32_Battery` WMI class represents a battery connected to the computer system.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-battery>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_Battery {
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
    /* 
    /// Time required to fully charge the battery. This property is not supported. 
    /// `BatteryRechargeTime` does not have a replacement property, and is now considered obsolete.
    pub BatteryRechargeTime: Option<u32>,
    */
    /// Status of the battery. The value 10 (Undefined) is not valid in the CIM schema because 
    /// in DMI it represents that no battery is installed. In this case, the object should not 
    /// be instantiated. 
    /// 
    /// - `Other` (1): The battery is discharging.
    /// - `Unknown` (2): The system has access to AC so no battery is being discharged. However, the battery is not necessarily charging.
    /// - `Fully Charged` (3)
    /// - `Low` (4)
    /// - `Critical` (5)
    /// - `Charging` (6)
    /// - `Charging and High` (7)
    /// - `Charging and Low` (8)
    /// - `Charging and Critical` (9)
    /// - `Undefined` (10)
    /// - `Partially Charged` (11)
    pub BatteryStatus: Option<u16>,
    /// Short description of the object a one-line string.
    pub Caption: Option<String>,
    /// Enumeration that describes the battery's chemistry.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Lead Acid` (3)
    /// - `Nickel Cadmium` (4)
    /// - `Nickel Metal Hydride` (5)
    /// - `Lithium-ion` (6)
    /// - `Zinc air` (7)
    /// - `Lithium Polymer` (8)
    pub Chemistry: Option<u16>,
    /// Windows Configuration Manager error code.
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
    /// Name of the first concrete class that appears in the inheritance chain used in the creation 
    /// of an instance. When used with the other key properties of the class, the property allows 
    /// all instances of this class and its subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Design capacity of the battery in milliwatt-hours. If the property is not supported, enter 
    /// 0 (zero).
    pub DesignCapacity: Option<u32>,
    /// Design voltage of the battery in millivolts. If the attribute is not supported, enter 0 (zero).
    pub DesignVoltage: Option<u64>,
    /// Identifies the battery.
    /// 
    /// Example: "Internal Battery"
    pub DeviceID: Option<String>,
    /// If `True`, the error reported in the `LastErrorCode` property is now cleared.
    pub ErrorCleared: Option<bool>,
    /// Free-form string that supplies more information about the error recorded in `LastErrorCode` 
    /// property, and information about any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Estimate of the percentage of full charge remaining.
    pub EstimatedChargeRemaining: Option<u16>,
    /// Estimate in minutes of the time to battery charge depletion under the present load conditions 
    /// if the utility power is off, or lost and remains off, or a laptop is disconnected from a power 
    /// source.
    pub EstimatedRunTime: Option<u32>,
    /* 
    /// Amount of time it takes to completely drain the battery after it is fully charged. 
    /// This property is no longer used and is considered obsolete.
    pub ExpectedBatteryLife: Option<u32>,
    */
    /// Battery's expected lifetime in minutes, assuming that the battery is fully charged. 
    /// The property represents the total expected life of the battery, not its current remaining life, 
    /// which is indicated by the `EstimatedRunTime` property.
    pub ExpectedLife: Option<u32>,
    /// Full charge capacity of the battery in milliwatt-hours. Comparison of the value to the 
    /// `DesignCapacity` property determines when the battery requires replacement. A battery's end of 
    /// life is typically when the `FullChargeCapacity` property falls below 80% of the `DesignCapacity` 
    /// property. If the property is not supported, enter 0 (zero).
    pub FullChargeCapacity: Option<u32>,
    /// Date and time the object was installed. This property does not need a value to indicate that 
    /// the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Maximum time, in minutes, to fully charge the battery. The property represents the time to recharge 
    /// a fully depleted battery, not the current remaining charge time, which is indicated in the 
    /// `TimeToFullCharge` property.
    pub MaxRechargeTime: Option<u32>,
    /// Defines the label by which the object is known. When subclassed, the property can be overridden 
    /// to be a key property.
    pub Name: Option<String>,
    /// Windows Plug and Play device identifier of the logical device.
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
    /// Data Specification version number supported by the battery. If the battery does not 
    /// support this function, the value should be left blank.
    pub SmartBatteryVersion: Option<String>,
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
    /// Elapsed time in seconds since the computer system's UPS last switched to battery power, 
    /// or the time since the system or UPS was last restarted, whichever is less. If the battery 
    /// is "on line", 0 (zero) is returned.
    pub TimeOnBattery: Option<u32>,
    /// Remaining time to charge the battery fully in minutes at the current charging rate and usage.
    pub TimeToFullCharge: Option<u32>,
}

/// The `Win32_CurrentProbe` WMI class represents the properties of a current monitoring sensor (ammeter).
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-currentprobe>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_CurrentProbe {
    /// Accuracy of the sensor for the measured property. The value is recorded as plus or minus 
    /// hundredths of a percent. Accuracy, along with resolution and tolerance, is used to calculate 
    /// the actual value of the measured physical property. Accuracy may vary and depends on whether 
    /// or not the device is linear over its dynamic range.
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
    /// Short description of the object a one-line string.
    pub Caption: Option<String>,
    /// Windows Configuration Manager error code.
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
    /// Name of the first concrete class that appears in the inheritance chain used in the creation 
    /// of an instance. When used with the other key properties of the class, the property allows 
    /// all instances of this class and its subclasses to be identified uniquely.
    pub CreationClassName: Option<String>,
    /// Current value indicated by the sensor.
    pub CurrentReading: Option<i32>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the current probe.
    pub DeviceID: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information about any 
    /// corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// If `TRUE`, the sensor is linear over its dynamic range.
    pub IsLinear: Option<bool>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Sensor threshold values specify the ranges (minimum and maximum values) to determine 
    /// whether or not the sensor is operating under normal, noncritical, critical, or fatal 
    /// conditions. If `CurrentReading` is between `LowerThresholdCritical` and `LowerThresholdFatal`, 
    /// the current state is critical.
    pub LowerThresholdCritical: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) for determining 
    /// whether the sensor is operating under normal, noncritical, critical, or fatal conditions. 
    /// If `CurrentReading` is below `LowerThresholdFatal`, the current state is fatal.
    pub LowerThresholdFatal: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) for determining 
    /// whether the sensor is operating under normal, noncritical, critical, or fatal conditions. 
    /// If `CurrentReading` is between `LowerThresholdNonCritical` and `UpperThresholdNonCritical`, the 
    /// sensor is reporting a normal value. If `CurrentReading` is between `LowerThresholdNonCritical` 
    /// and `LowerThresholdCritical`, the current state is noncritical.
    pub LowerThresholdNonCritical: Option<i32>,
    /// Largest value of the measured property that can be read by the numeric sensor.
    pub MaxReadable: Option<i32>,
    /// Smallest value of the measured property that can be read by the numeric sensor.
    pub MinReadable: Option<i32>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
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
    /// If `True`, the device can be power-managed (can be put into suspend mode, and so on). 
    /// The property does not indicate that power management features are currently enabled, 
    /// only that the logical device is capable of power management.
    pub PowerManagementSupported: Option<bool>,
    /// Ability of the sensor to resolve differences in the measured property. This value may 
    /// vary depending on whether the device is linear over its dynamic range.
    pub Resolution: Option<u32>,
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
    /// Value for the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// Tolerance of the sensor for the measured property. Tolerance, along with resolution and 
    /// accuracy, is used to calculate the actual value of the measured physical property. Tolerance 
    /// may vary depending on whether the device is linear over its dynamic range.
    pub Tolerance: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) for determining 
    /// whether the sensor is operating under normal, noncritical, critical, or fatal conditions. 
    /// If `CurrentReading` is between `UpperThresholdCritical` and `UpperThresholdFatal`, the current 
    /// state is critical.
    pub UpperThresholdCritical: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) for determining 
    /// whether the sensor is operating under normal, noncritical, critical, or fatal conditions. 
    /// If `CurrentReading` is above `UpperThresholdFatal`, the current state is fatal.
    pub UpperThresholdFatal: Option<i32>,
    /// Sensor's threshold values specify the ranges (minimum and maximum values) for determining 
    /// whether the sensor is operating under normal, noncritical, critical, or fatal conditions. 
    /// If `CurrentReading` is between `LowerThresholdNonCritical` and `UpperThresholdNonCritical`, the 
    /// sensor is reporting a normal value. If `CurrentReading` is between `UpperThresholdNonCritical` 
    /// and `UpperThresholdCritical`, the current state is noncritical.
    pub UpperThresholdNonCritical: Option<i32>,
}

/// The `Win32_PortableBattery` WMI class contains the properties related to a portable battery, 
/// such as a notebook computer battery.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-portablebattery>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PortableBattery {
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
    /// Description of the battery's charge status. The value 10 (Undefined) is not valid in the Common 
    /// Information Model (CIM) schema because in Desktop Management Interface (DMI) it indicates that 
    /// no battery is installed. In this case, this object should not be instantiated.
    /// 
    /// - `Other` (1): The battery is discharging.
    /// - `Unknown` (2): The system has access to AC so no battery is being discharged. However, the battery is not necessarily charging.
    /// - `Fully Charged` (3)
    /// - `Low` (4)
    /// - `Critical` (5)
    /// - `Charging` (6)
    /// - `Charging and High` (7)
    /// - `Charging and Low` (8)
    /// - `Charging and Critical` (9)
    /// - `Undefined` (10)
    /// - `Partially Charged` (11)
    pub BatteryStatus: Option<u16>,
    /// Multiplication factor of the `DesignCapacity` value to ensure that the milliwatt hour value does 
    /// not overflow for Smart Battery Data Specification (SBDS) implementations.
    pub CapacityMultiplier: Option<u16>,
    /// Short description of the object—a one-line string.
    pub Caption: Option<String>,
    /// Chemistry of the battery.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Lead Acid` (3)
    /// - `Nickel Cadmium` (4)
    /// - `Nickel Metal Hydride` (5)
    /// - `Lithium-ion` (6)
    /// - `Zinc air` (7)
    /// - `Lithium Polymer` (8)
    pub Chemistry: Option<u16>,
    /// Windows Configuration Manager error code.
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
    /// Name of the first concrete class that appears in the inheritance chain used in the creation of 
    /// an instance. When used with the other key properties of the class, the property allows all instances 
    /// of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Design capacity of the battery in milliwatt-hours. If this property is not supported, enter 0 (zero).
    pub DesignCapacity: Option<u32>,
    /// Design voltage of the battery in millivolts. If this attribute is not supported, enter 0 (zero).
    pub DesignVoltage: Option<u64>,
    /// Battery identifier.
    /// 
    /// Example: "Internal Battery"
    pub DeviceID: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Estimate of the percentage of full charge remaining.
    pub EstimatedChargeRemaining: Option<u16>,
    /// Estimate in minutes of the time to battery charge depletion under the present load conditions if the 
    /// utility power is off, or lost and remains off, or a laptop is disconnected from a power source.
    pub EstimatedRunTime: Option<u32>,
    /* 
    /// Not supported.
    pub ExpectedBatteryLife: Option<u32>,
    */
    /// Battery's expected lifetime in minutes, assuming that the battery is fully charged. This property 
    /// represents the total expected life of the battery, not its current remaining life, which is indicated 
    /// by the `EstimatedRunTime` property.
    pub ExpectedLife: Option<u32>,
    /// Full charge capacity of the battery in milliwatt-hours. Comparison of this value to the `DesignCapacity` 
    /// property determines when the battery requires replacement. A battery's end of life is typically when 
    /// the `FullChargeCapacity` property falls below 80% of the `DesignCapacity` property. If this property is not 
    /// supported, enter 0 (zero).
    pub FullChargeCapacity: Option<u32>,
    /// Date and time the object was installed. This property does not need a value to indicate that the object 
    /// is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Physical location of the battery. This property is filled by the computer manufacturer.
    /// 
    /// Example: "In the back, on the left"
    pub Location: Option<String>,
    /// Date when the battery was manufactured.
    pub ManufactureDate: Option<String>,
    /// Manufacturer of the battery.
    pub Manufacturer: Option<String>,
    /// Difference between the highest estimated amount of energy left in the battery and the 
    /// current amount reported by the battery.
    pub MaxBatteryError: Option<u16>,
    /// Maximum time, in minutes, to fully charge the battery. This property represents the time 
    /// to recharge a fully depleted battery, not the current remaining charge time, which is 
    /// indicated in the `TimeToFullCharge` property.
    pub MaxRechargeTime: Option<u32>,
    /// Label by which the object is known. When subclassed, the property can be overridden to be 
    /// a key property.
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
    /// Smart Battery Data Specification version number supported by this battery. If the battery 
    /// does not support this function, the value should be left blank.
    pub SmartBatteryVersion: Option<String>,
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
    /// State of the logical device. If this property does not apply to the logical device, the value 
    /// 5 (Not Applicable) should be used.
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
    /// Elapsed time in seconds since the computer system's UPS last switched to battery power, or the 
    /// time since the system or UPS was last restarted, whichever is less. If the battery is online, 
    /// 0 (zero) is returned.
    pub TimeOnBattery: Option<u32>,
    /// Remaining time in minutes to charge the battery fully at the current charge rate and usage.
    pub TimeToFullCharge: Option<u32>,
}

/// The `Win32_PowerManagementEvent` WMI class represents power management events resulting from power 
/// state changes. These state changes are associated with either the Advanced Power Management (APM) 
/// or the Advanced Configuration and Power Interface (ACPI) system management protocols.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-powermanagementevent>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_PowerManagementEvent {
    /// Descriptor used by the event provider to determine which users can receive the event.
    pub SECURITY_DESCRIPTOR: Option<Vec<u8>>,
    /// Unique value that indicates the time at which the event was generated. This is a 64-bit 
    /// value that represents the number of 100-nanosecond intervals after January 1, 1601. The 
    /// information is in the Coordinated Universal Times (UTC) format.
    pub TIME_CREATED: Option<u64>,
    /// Type of change in the system power state.
    /// 
    /// - `Entering Suspend` (4): While suspended, the computer appears to be off; however, it can be "awakened" in response to various events, including user input (such as moving the mouse or pressing a key on the keyboard). While the computer is suspended, power consumption is reduced to one of several levels depending on how the system is to be used. The lower the level of power consumption, the more time it takes the system to return to the working state. When the computer enters the suspend state, the desktop is locked, and you must press CTRL+ALT+DELETE and provide a valid user name and password to resume operations
    /// - `Resume from Suspend` (7): Indicates that a Resume from Suspend message has been sent, enabling the computer to return to its regular power state.
    /// - `Power Status Change` (10): Indicates a change in the power status of the computer, such as a switch from battery power to AC, or from AC to an uninterruptible power supply. The system also broadcasts this event when remaining battery power slips below the threshold specified by the user or if the battery power changes by a specified percentage.
    /// - `OEM Event` (11): Indicates that an Advanced Power Management (APM) BIOS has sent an OEM event. The value of the event will be captured in the OEMEventCode property. Because some APM BIOS implementations do not provide OEM event notifications, this event might never be broadcast on some computers. APM is a legacy power management scheme. Although still supported, APM has been largely superseded by ACPI (Advanced Configuration and Power Interface).
    /// - `Resume Automatic` (18): Indicates that the computer has awakened in response to an event. If the system detects user activity (such as a mouse click), the ResumeSuspend message will be broadcast, letting applications know that they can resume full interactivity with the user.
    pub EventType: Option<u16>,
    /// System power state defined by the original equipment manufacturer (OEM) when the `EventType` 
    /// property of this class is set to 11 (OEM Event); otherwise, this property is set to `NULL`. 
    /// OEM events are generated when an APM BIOS signals an APM OEM event. OEM event codes are in 
    /// the range 0x0200h - 0x02FFh.
    pub OEMEventCode: Option<u16>,
}
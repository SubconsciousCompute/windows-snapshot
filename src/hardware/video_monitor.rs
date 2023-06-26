//! The Video and Monitors subcategory groups classes that represent monitors, video cards, and their associated settings.
//! 
//! | Class                                                                                 | Description                                                                                                                                                                                                                                                                                                                                                                        |
//! |---------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_DesktopMonitor**](win32-desktopmonitor)                                 | Represents the type of monitor or display device attached to the computer system.<br/>                                                                                                                                                                                                                                                                                       |
//! | [**Win32\_DisplayControllerConfiguration**](win32-displaycontrollerconfiguration) | Represents the video adapter configuration information of a computer system running Windows. This class is obsolete. In place of this class, use the properties in the [**Win32\_VideoController**](win32-videocontroller), [**Win32\_DesktopMonitor**](win32-desktopmonitor), and [CIM\_VideoControllerResolution](cim-videocontrollerresolution) classes.<br/> |
//! | [**Win32\_VideoController**](win32-videocontroller)                               | Represents the capabilities and management capacity of the video controller on a computer system running Windows.<br/>                                                                                                                                                                                                                                                       |
//! | [**Win32\_VideoSettings**](win32-videosettings)                                   | Relates a video controller and video settings that can be applied to it.<br/>                                                                                                                                                                                                                                                                                                |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's DesktopMonitors
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct DesktopMonitors {
    /// Sequence of windows DesktopMonitors states
    pub desktop_monitors: Vec<Win32_DesktopMonitor>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(DesktopMonitors, desktop_monitors);

/// Represents the state of Windows user's DisplayControllerConfigurations
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct DisplayControllerConfigurations {
    /// Sequence of windows DisplayControllerConfigurations states
    pub display_controller_configurations: Vec<Win32_DisplayControllerConfiguration>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(DisplayControllerConfigurations, display_controller_configurations);

/// Represents the state of Windows user's VideoControllers
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct VideoControllers {
    /// Sequence of windows VideoControllers states
    pub video_controllers: Vec<Win32_VideoController>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(VideoControllers, video_controllers);

/// The `Win32_DesktopMonitor` WMI class represents the type of monitor or display device 
/// attached to the computer system.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-desktopmonitor>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_DesktopMonitor {
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
    /// Monitor's bandwidth in megahertz. If unknown, enter 0 (zero).
    pub Bandwidth: Option<u32>,
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
    /// Unique identifier of a desktop monitor.
    pub DeviceID: Option<String>,
    /// Type of desktop monitor or CRT.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `Multiscan Color` (2)
    /// - `Multiscan Monochrome` (3)
    /// - `Fixed Frequency Color` (4)
    /// - `Fixed Frequency Monochrome` (5)
    pub DisplayType: Option<u16>,
    /// If `TRUE`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// Free-form string supplying more information about the error recorded in `LastErrorCode`, and 
    /// information on any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate that 
    /// the object is installed.
    pub InstallDate: Option<String>,
    /// If `TRUE`, the device is locked, preventing user input or output.
    pub IsLocked: Option<bool>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Name of the monitor manufacturer.
    /// 
    /// Example: "NEC"
    pub MonitorManufacturer: Option<String>,
    /// Type of monitor.
    /// 
    /// Example: "NEC 5FGp"
    pub MonitorType: Option<String>,
    /// Label by which the object is known. When subclassed, the property can be overridden to be a key 
    /// property.
    pub Name: Option<String>,
    /// Resolution along the x-axis (horizontal direction) of the monitor.
    pub PixelsPerXLogicalInch: Option<u32>,
    /// Resolution along the y-axis (vertical direction) of the monitor.
    pub PixelsPerYLogicalInch: Option<u32>,
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
    /// Logical height of the display in screen coordinates.
    pub ScreenHeight: Option<u32>,
    /// Logical width of the display in screen coordinates.
    pub ScreenWidth: Option<u32>,
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

/// The `Win32_DisplayControllerConfiguration` WMI class represents the video adapter configuration 
/// information of a computer system running Windows.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-displaycontrollerconfiguration>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_DisplayControllerConfiguration {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// Either the number of bits used to represent the color in this configuration, or the bits in 
    /// each pixel.
    /// 
    /// Example: 8
    pub BitsPerPixel: Option<u32>,
    /// Current number of color planes used in the display configuration. A color plane is another 
    /// way to represent pixel colors. Instead of assigning a single RGB value to each pixel, color 
    /// planes separate the graphic into each of the primary color components (red, green, blue), and 
    /// stores them in their own planes. This allows for greater color depths on 8-bit and 16-bit 
    /// video systems. Present graphics systems have the bitwidth large enough to store color depth 
    /// information, meaning only one color plane is needed.
    /// 
    /// Example: 1
    pub ColorPlanes: Option<u32>,
    /// Number of color indexes in a color table of a display device (if the device has a color depth 
    /// of no more than 8 bits per pixel). For devices with greater color depths, -1 is returned.
    /// 
    /// Example: 256
    pub DeviceEntriesInAColorTable: Option<u32>,
    /// Current number of device-specific pens. A value of 0xFFFFFFFF means the device does not support 
    /// pens. Pens are logical properties that can be assigned by the display controller to display devices, 
    /// and are used to draw lines, borders of polygons, and text.
    /// 
    /// Example: 3
    pub DeviceSpecificPens: Option<u32>,
    /// Current number of pixels in the horizontal direction (x-axis) of the display.
    /// 
    /// Example: 1024
    pub HorizontalResolution: Option<u32>,
    /// Name of the adapter used in this configuration.
    pub Name: Option<String>,
    /// Refresh rate of the video adapter. A value of 0 (zero) or 1 (one) indicates a default rate is being 
    /// used. A value of -1 indicates that an optimal rate is being used.
    /// 
    /// Example: 72
    pub RefreshRate: Option<i32>,
    /// Current number of color index entries reserved for system use. This value is only valid for display 
    /// settings that use an indexed palette. Indexed palettes are not used for color depths greater than 8 
    /// bits per pixel. If the color depth is more than 8 bits per pixel, this value is set to `NULL`.
    /// 
    /// Example: 20
    pub ReservedSystemPaletteEntries: Option<u32>,
    /// Current number of color index entries reserved for system use. This value is only valid for display 
    /// settings that use an indexed palette. Indexed palettes are not used for color depths greater than 8 
    /// bits per pixel. If the color depth is more than 8 bits per pixel, this value is set to `NULL`.
    /// 
    /// Example: 20
    pub SystemPaletteEntries: Option<u32>,
    /// Current number of pixels in the vertical direction (y-axis) of the display.
    /// 
    /// Example: 768
    pub VerticalResolution: Option<u32>,
    /// User-readable description of the current screen resolution and color setting of the display.
    /// 
    /// Example: "1024 768 with 256 colors"
    pub VideoMode: Option<String>,
}

/// The `Win32_VideoController` WMI class represents the capabilities and management capacity of the 
/// video controller on a computer system running Windows.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-videocontroller>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_VideoController {
    /// Array of graphics and 3-D capabilities of the video controller.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `Graphics Accelerator` (2)
    /// - `3D Accelerator` (3): 3-D Accelerator
    pub AcceleratorCapabilities: Option<Vec<u16>>,
    /// General chipset used for this controller to compare compatibilities with the system.
    pub AdapterCompatibility: Option<String>,
    /// Name or identifier of the digital-to-analog converter (DAC) chip. The character set of this 
    /// property is alphanumeric.
    pub AdapterDACType: Option<String>,
    /// Memory size of the video adapter.
    /// 
    /// Example: 64000
    pub AdapterRAM: Option<u32>,
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
    /// Free-form strings providing more detailed explanations for any of the video accelerator features 
    /// indicated in the `AcceleratorCapabilities` array. Note, each entry of this array is related to the 
    /// entry in the `AcceleratorCapabilities` array that is located at the same index.
    pub CapabilityDescriptions: Option<Vec<String>>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// Size of the system's color table. The device must have a color depth of no more than 8 bits per pixel; 
    /// otherwise, this property is not set.
    /// 
    /// Example: 256
    pub ColorTableEntries: Option<u32>,
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
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of an instance. 
    /// When used with the other key properties of the class, this property allows all instances of this class
    /// and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Number of bits used to display each pixel.
    pub CurrentBitsPerPixel: Option<u32>,
    /// Current number of horizontal pixels.
    pub CurrentHorizontalResolution: Option<u32>,
    /// Number of colors supported at the current resolution.
    pub CurrentNumberOfColors: Option<u64>,
    /// Number of columns for this video controller (if in character mode). Otherwise, enter 0 (zero).
    pub CurrentNumberOfColumns: Option<u32>,
    /// Number of rows for this video controller (if in character mode). Otherwise, enter 0 (zero).
    pub CurrentNumberOfRows: Option<u32>,
    /// Frequency at which the video controller refreshes the image for the monitor. A value of 0 
    /// (zero) indicates the default rate is being used, while 0xFFFFFFFF indicates the optimal rate 
    /// is being used.
    pub CurrentRefreshRate: Option<u32>,
    /// Current scan mode.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `Interlaced` (3)
    /// - `Non Interlaced` (4): Noninterlaced
    pub CurrentScanMode: Option<u16>,
    /// Current number of vertical pixels.
    pub CurrentVerticalResolution: Option<u32>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Identifier (unique to the computer system) for this video controller.
    pub DeviceID: Option<String>,
    /// Current number of device-specific pens. A value of 0xffff means that the device does not support pens.
    /// 
    /// Example: 3
    pub DeviceSpecificPens: Option<u32>,
    /// Dither type of the video controller. The property can be one of the predefined values, or a 
    /// driver-defined value greater than or equal to 256. If line art dithering is chosen, the controller 
    /// uses a dithering method that produces well-defined borders between black, white, and gray scalings. 
    /// Line art dithering is not suitable for images that include continuous graduations in intensity and 
    /// hue such as scanned photographs.
    /// 
    /// - `No dithering` (1)
    /// - `Dithering with a coarse brush` (2)
    /// - `Dithering with a fine brush` (3)
    /// - `Line art dithering` (4)
    /// - `Device does gray scaling` (5)
    pub DitherType: Option<u32>,
    /// Last modification date and time of the currently installed video driver.
    pub DriverDate: Option<WMIDateTime>,
    /// Version number of the video driver.
    pub DriverVersion: Option<String>,
    /// If `TRUE`, the error reported in `LastErrorCode` property is now cleared.
    pub ErrorCleared: Option<bool>,
    /// Free-form string supplying more information about the error recorded in `LastErrorCode` property, and 
    /// information on any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Specific value of one of the three possible color-matching methods or intents that should be used by 
    /// default. This property is used primarily for non-ICM applications. ICM applications establish intents 
    /// by using the ICM functions. This property can be a predefined value or a driver defined value greater 
    /// than or equal to 256. Color matching based on saturation is the most appropriate choice for business 
    /// graphs when dithering is not desired. Color matching based on contrast is the most appropriate choice 
    /// for scanned or photographic images when dithering is desired. Color matching optimized to match the 
    /// exact color requested is most appropriate for use with business logos or other images when an exact 
    /// color match is desired.
    /// 
    /// - `Saturation` (1)
    /// - `Contrast` (2)
    /// - `Exact Color` (3)
    pub ICMIntent: Option<u32>,
    /// Method of handling ICM. For non-ICM applications, this property determines if ICM is enabled. For ICM 
    /// applications, the system examines this property to determine how to handle ICM support. This property 
    /// can be a predefined value or a driver-defined value greater than or equal to 256. The value determines 
    /// which system handles image color matching.
    /// 
    /// - `Disabled` (1)
    /// - `Windows` (2)
    /// - `Device Driver` (3)
    /// - `Destination Device` (4)
    pub ICMMethod: Option<u32>,
    /// Path to the video adapter's .inf file.
    /// 
    /// Example: "C:\Windows\SYSTEM32\DRIVERS"
    pub InfFilename: Option<String>,
    /// Section of the .inf file where the Windows video information resides.
    pub InfSection: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate that the object 
    /// is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Name of the installed display device driver.
    pub InstalledDisplayDrivers: Option<String>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Maximum amount of memory supported in bytes.
    pub MaxMemorySupported: Option<u32>,
    /// Maximum number of directly addressable entities supportable by this controller. A value of 0 (zero) 
    /// should be used if the number is unknown.
    pub MaxNumberControlled: Option<u32>,
    /// Maximum refresh rate of the video controller in hertz.
    pub MaxRefreshRate: Option<u32>,
    /// Minimum refresh rate of the video controller in hertz.
    pub MinRefreshRate: Option<u32>,
    /// If `TRUE`, gray scale is used to display images.
    pub Monochrome: Option<bool>,
    /// Label by which the object is known. When subclassed, the property can be overridden to be a key property.
    pub Name: Option<String>,
    /// Current number of color planes. If this value is not applicable for the current video configuration, 
    /// enter 0 (zero).
    pub NumberOfColorPlanes: Option<u16>,
    /// Number of video pages supported given the current resolutions and available memory.
    pub NumberOfVideoPages: Option<u32>,
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
    /// Protocol used by the controller to access "controlled" devices.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `EISA` (3)
    /// - `ISA` (4)
    /// - `PCI` (5)
    /// - `ATA/ATAPI` (6)
    /// - `Flexible Diskette` (7)
    /// - `1496` (8)
    /// - `SCSI Parallel Interface` (9)
    /// - `SCSI Fibre Channel Protocol` (10)
    /// - `SCSI Serial Bus Protocol` (11)
    /// - `SCSI Serial Bus Protocol-2 (1394)` (12)
    /// - `SCSI Serial Storage Architecture` (13)
    /// - `VESA` (14)
    /// - `PCMCIA` (15)
    /// - `Universal Serial Bus` (16)
    /// - `Parallel Protocol` (17)
    /// - `ESCON` (18)
    /// - `Diagnostic` (19)
    /// - `I2C` (20)
    /// - `Power` (21)
    /// - `HIPPI` (22)
    /// - `MultiBus` (23)
    /// - `VME` (24)
    /// - `IPI` (25)
    /// - `IEEE-488` (26)
    /// - `RS232` (27)
    /// - `IEEE 802.3 10BASE5` (28)
    /// - `IEEE 802.3 10BASE2` (29)
    /// - `IEEE 802.3 1BASE5` (30)
    /// - `IEEE 802.3 10BROAD36` (31)
    /// - `IEEE 802.3 100BASEVG` (32)
    /// - `IEEE 802.5 Token-Ring` (33)
    /// - `ANSI X3T9.5 FDDI` (34)
    /// - `MCA` (35)
    /// - `ESDI` (36)
    /// - `IDE` (37)
    /// - `CMD` (38)
    /// - `ST506` (39)
    /// - `DSSI` (40)
    /// - `QIC2` (41)
    /// - `Enhanced ATA/IDE` (42)
    /// - `AGP` (43)
    /// - `TWIRP (two-way infrared)` (44)
    /// - `FIR (fast infrared)` (45)
    /// - `SIR (serial infrared)` (46)
    /// - `IrBus` (47)
    pub ProtocolSupported: Option<u16>,
    /// Number of reserved entries in the system palette. The operating system may reserve entries 
    /// to support standard colors for task bars and other desktop display items. This index is 
    /// valid only if the device driver sets the `RC_PALETTE` bit in the RASTERCAPS index, and is 
    /// available only if the driver is compatible with 16-bit Windows. If the system is not using 
    /// a palette, `ReservedSystemPaletteEntries` is not set.
    pub ReservedSystemPaletteEntries: Option<u32>,
    /// Version number of the initialization data specification (upon which the structure is based).
    pub SpecificationVersion: Option<u32>,
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
    /// Value of the scoping computer's `CreationClassName` property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// Current number of color index entries in the system palette. This index is valid only if 
    /// the device driver sets the `RC_PALETTE` bit in the RASTERCAPS index, and is available only if
    /// the driver is compatible with 16-bit Windows. If the system is not using a palette, 
    /// `SystemPaletteEntries` is not set.
    pub SystemPaletteEntries: Option<u32>,
    /// Date and time this controller was last reset. This could mean the controller was powered down 
    /// or reinitialized.
    pub TimeOfLastReset: Option<WMIDateTime>,
    /// Type of video architecture.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `CGA` (3)
    /// - `EGA` (4)
    /// - `VGA` (5)
    /// - `SVGA` (6)
    /// - `MDA` (7)
    /// - `HGC` (8)
    /// - `MCGA` (9)
    /// - `8514A` (10)
    /// - `XGA` (11)
    /// - `Linear Frame Buffer` (12)
    /// - `PC-98` (160)
    pub VideoArchitecture: Option<u16>,
    /// Type of video memory.
    /// 
    /// - `Other` (1)
    /// - `Unknown` (2)
    /// - `VRAM` (3)
    /// - `DRAM` (4)
    /// - `SRAM` (5)
    /// - `WRAM` (6)
    /// - `EDO RAM` (7)
    /// - `Burst Synchronous DRAM` (8)
    /// - `Pipelined Burst SRAM` (9)
    /// - `CDRAM` (10)
    /// - `3DRAM` (11)
    /// - `SDRAM` (12)
    /// - `SGRAM` (13)
    pub VideoMemoryType: Option<u16>,
    /// Current video mode.
    pub VideoMode: Option<u16>,
    /// Current resolution, color, and scan mode settings of the video controller.
    /// 
    /// Example: "1024 x 768 x 256 colors"
    pub VideoModeDescription: Option<String>,
    /// Free-form string describing the video processor.
    pub VideoProcessor: Option<String>,
}
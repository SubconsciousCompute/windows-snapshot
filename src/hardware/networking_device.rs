//! The Networking Devices subcategory groups classes that represent the network interface controller, its configurations, and its settings.
//! 
//! | Class                                                                           | Description                                                                                                                                                                                                            |
//! |---------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_NetworkAdapter**](win32-networkadapter)                           | Represents a network adapter on a computer system running Windows.<br/>                                                                                                                                          |
//! | [**Win32\_NetworkAdapterConfiguration**](win32-networkadapterconfiguration) | Represents the attributes and behaviors of a network adapter. The class is not guaranteed to be supported after the ratification of the Distributed Management Task Force (DMTF) CIM network specification.<br/> |
//! | [**Win32\_NetworkAdapterSetting**](win32-networkadaptersetting)             | Relates a network adapter and its configuration settings.<br/>                                                                                                                                                   |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's NetworkAdapters
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct NetworkAdapters {
    /// Sequence of windows NetworkAdapters states
    pub network_adapters: Vec<Win32_NetworkAdapter>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(NetworkAdapters, network_adapters);

/// The `Win32_NetworkAdapter` class is deprecated. Use the MSFT_NetAdapter class instead. 
/// The Win32_NetworkAdapterWMI class represents a network adapter of a computer running a 
/// Windows operating system.
/// 
/// `Win32_NetworkAdapter` only supplies IPv4 data.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkadapter>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NetworkAdapter {
    /// Network medium in use. The network adapters are as follows:
    /// - `Ethernet 802.3` ("Ethernet 802.3")
    /// - `Token Ring 802.5` ("Token Ring 802.5")
    /// - `Fiber Distributed Data Interface` (FDDI) ("Fiber Distributed Data Interface (FDDI)")
    /// - `Wide Area Network` (WAN) ("Wide Area Network (WAN)")
    /// - `LocalTalk` ("LocalTalk")
    /// - `Ethernet using DIX header format` ("Ethernet using DIX header format")
    /// - `ARCNET` ("ARCNET")
    /// - `ARCNET` (878.2) ("ARCNET (878.2)")
    /// - `ATM` ("ATM")
    /// - `Wireless` ("Wireless")
    /// - `Infrared Wireless` ("Infrared Wireless")
    /// - `Bpc` ("Bpc")
    /// - `CoWan` ("CoWan")
    /// - `1394` ("1394")
    pub AdapterType: Option<String>,
    /// Network medium in use. Returns the same information as the ``AdapterType` property, 
    /// except that the information is in the form of an integer.
    /// 
    /// - `Ethernet 802.3` (0)
    /// - `Token Ring 802.5` (1)
    /// - `Fiber Distributed Data Interface (FDDI)` (2)
    /// - `Wide Area Network (WAN)` (3)
    /// - `LocalTalk` (4)
    /// - `Ethernet using DIX header format` (5)
    /// - `ARCNET` (6)
    /// - `ARCNET (878.2)` (7)
    /// - `ATM` (8)
    /// - `Wireless` (9)
    /// - `Infrared Wireless` (10)
    /// - `Bpc` (11)
    /// - `CoWan` (12)
    /// - `1394` (13)
    pub AdapterTypeID: Option<u16>,
    /// If `True`, the network adapter can automatically determine the speed of the attached or network media.
    /// 
    /// This property has not been implemented yet. It returns a `NULL` value by default.
    pub AutoSense: Option<bool>,
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
    /// Name of the first concrete class to appear in the inheritance chain used in the creation 
    /// of an instance. When used with the other key properties of the class, the property allows 
    /// all instances of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of the network adapter from other devices on the system.
    pub DeviceID: Option<String>,
    /// If `True`, the error reported in `LastErrorCode` is now cleared.
    pub ErrorCleared: Option<bool>,
    /// More information about the error recorded in `LastErrorCode`, and information about 
    /// any corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Globally unique identifier for the connection.
    pub GUID: Option<String>,
    /// Index number of the network adapter, stored in the system registry.
    /// 
    /// Example: 0
    pub Index: Option<u32>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    /// 
    /// This property has not been implemented yet. It returns a `NULL` value by default.
    pub InstallDate: Option<WMIDateTime>,
    /// If `True`, the network adapter is installed in the system.
    pub Installed: Option<bool>,
    /// Index value that uniquely identifies the local network interface. The value in this 
    /// property is the same as the value in the `InterfaceIndex` property in the instance of 
    /// `Win32_IP4RouteTable` that represents the network interface in the route table.
    pub InterfaceIndex: Option<u32>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Media access control address for this network adapter. A MAC address is a unique 
    /// 48-bit number assigned to the network adapter by the manufacturer. It uniquely 
    /// identifies this network adapter and is used for mapping TCP/IP network communications.
    pub MACAddress: Option<String>,
    /// Name of the network adapter's manufacturer.
    /// 
    /// Example: "3COM"
    pub Manufacturer: Option<String>,
    /// Maximum number of directly addressable ports supported by this network adapter. A 
    /// value of 0 (zero) should be used if the number is unknown.
    pub MaxNumberControlled: Option<u32>,
    /// Maximum speed, in bits per second, for the network adapter.
    /// 
    /// This property has not been implemented yet. It returns a `NULL` value by default.
    pub MaxSpeed: Option<u64>,
    /// Label by which the object is known. When subclassed, the property can be overridden to 
    /// be a key property.
    pub Name: Option<String>,
    /// Name of the network connection as it appears in the Network Connections Control Panel 
    /// program.
    pub NetConnectionID: Option<String>,
    /// State of the network adapter connection to the network.
    /// 
    /// - `Disconnected` (0)
    /// - `Connecting` (1)
    /// - `Connected` (2)
    /// - `Disconnecting` (3)
    /// - `Hardware Not Present` (4)
    /// - `Hardware Disabled` (5)
    /// - `Hardware Malfunction` (6)
    /// - `Media Disconnected` (7)
    /// - `Authenticating` (8)
    /// - `Authentication Succeeded` (9)
    /// - `Authentication Failed` (10)
    /// - `Invalid Address` (11)
    /// - `Credentials Required` (12)
    /// - `Other`: 13–65535
    pub NetConnectionStatus: Option<u16>,
    /// Indicates whether the adapter is enabled or not. If `True`, the adapter is enabled. You 
    /// can enable or disable the NIC by using the `Enable` and `Disable` methods.
    pub NetEnabled: Option<bool>,
    /// Array of network addresses for an adapter.
    /// 
    /// This property has not been implemented yet. It returns a `NULL` value by default.
    pub NetworkAddresses: Option<Vec<String>>,
    /// Network address hard-coded into an adapter. This hard-coded address may be changed by 
    /// firmware upgrade or software configuration. If so, this field should be updated when 
    /// the change is made. The property should be left blank if no hard-coded address exists 
    /// for the network adapter.
    /// 
    /// This property has not been implemented yet. It returns a `NULL` value by default.
    pub PermanentAddress: Option<String>,
    /// Indicates whether the adapter is a physical or a logical adapter. If `True`, the adapter 
    /// is physical.
    pub PhysicalAdapter: Option<bool>,
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
    /// Product name of the network adapter.
    /// 
    /// Example: "Fast EtherLink XL"
    pub ProductName: Option<String>,
    /// Service name of the network adapter. This name is usually shorter than the full product 
    /// name.
    /// 
    /// Example: "Elnkii"
    pub ServiceName: Option<String>,
    /// Estimate of the current bandwidth in bits per second. For endpoints which vary in 
    /// bandwidth or for those where no accurate estimation can be made, this property should 
    /// contain the nominal bandwidth.
    pub Speed: Option<u64>,
    /// Current status of the object.
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
    /// Date and time the network adapter was last reset.
    pub TimeOfLastReset: Option<WMIDateTime>,
}
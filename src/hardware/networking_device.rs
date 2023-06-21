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

/// Represents the state of Windows user's NetworkAdapterConfigurations
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct NetworkAdapterConfigurations {
    /// Sequence of windows NetworkAdapterConfigurations states
    pub network_adapter_configurations: Vec<Win32_NetworkAdapterConfiguration>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(NetworkAdapterConfigurations, network_adapter_configurations);

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

/// The `Win32_NetworkAdapterConfiguration` WMI class represents the attributes and behaviors 
/// of a network adapter. This class includes extra properties and methods that support the 
/// management of the TCP/IP protocol that are independent from the network adapter.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkadapterconfiguration>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NetworkAdapterConfiguration {
    /// Short textual description of the current object.
    pub Caption: Option<String>,
    /// Textual description of the current object.
    pub Description: Option<String>,
    /// Identifier by which the current object is known.
    pub SettingID: Option<String>,
    /// If `TRUE`, TCP/IP transmits Address Resolution Protocol (ARP) queries with source routing 
    /// enabled on Token Ring networks. By default (FALSE), ARP first queries without source 
    /// routing, and then retries with source routing enabled if no reply is received. Source 
    /// routing allows the routing of network packets across different types of networks.
    pub ArpAlwaysSourceRoute: Option<bool>,
    /// If `TRUE`, Ethernet packets follow the IEEE 802.3 Sub-Network Access Protocol (SNAP) 
    /// encoding. Setting this parameter to 1 forces TCP/IP to transmit Ethernet packets by 
    /// using 802.3 SNAP encoding. By default (FALSE), the stack transmits packets in DIX 
    /// Ethernet format.
    pub ArpUseEtherSNAP: Option<bool>,
    /// Valid Windows file path to standard Internet database files (HOSTS, LMHOSTS, NETWORKS, 
    /// and PROTOCOLS). The file path is used by the Windows Sockets interface.
    pub DatabasePath: Option<String>,
    /// If `TRUE`, dead gateway detection occurs. With this feature enabled, Transmission Control 
    /// Protocol (TCP) asks Internet Protocol (IP) to change to a backup gateway if it retransmits 
    /// a segment several times without receiving a response.
    pub DeadGWDetectEnabled: Option<bool>,
    /// Array of IP addresses of default gateways that the computer system uses.
    /// 
    /// Example: "192.168.12.1 192.168.46.1"
    pub DefaultIPGateway: Option<Vec<String>>,
    /// Default Type Of Service (TOS) value set in the header of outgoing IP packets. Request 
    /// for Comments (RFC) 791 defines the values. Default: 0 (zero), Valid Range: 0 - 255.
    pub DefaultTOS: Option<u8>,
    /// Default Time To Live (TTL) value set in the header of outgoing IP packets. The TTL 
    /// specifies the number of routers an IP packet can pass through to reach its destination 
    /// before being discarded. Each router decrements by one the TTL count of a packet as it 
    /// passes through and discards the packets—if the TTL is 0 (zero). Default: 32, Valid 
    /// Range: 1 - 255.
    pub DefaultTTL: Option<u8>,
    /// If `TRUE`, the dynamic host configuration protocol (DHCP) server automatically assigns 
    /// an IP address to the computer system when establishing a network connection.
    pub DHCPEnabled: Option<bool>,
    /// Expiration date and time for a leased IP address that was assigned to the computer by 
    /// the dynamic host configuration protocol (DHCP) server.
    /// 
    /// Example: 20521201000230.000000000
    pub DHCPLeaseExpires: Option<WMIDateTime>,
    /// Date and time the lease was obtained for the IP address assigned to the computer by the 
    /// dynamic host configuration protocol (DHCP) server.
    /// 
    /// Example: 19521201000230.000000000
    pub DHCPLeaseObtained: Option<WMIDateTime>,
    /// IP address of the dynamic host configuration protocol (DHCP) server.
    /// 
    /// Example: "10.55.34.2"
    pub DHCPServer: Option<String>,
    /// Organization name followed by a period and an extension that indicates the type of 
    /// organization, such as "microsoft.com". The name can be any combination of the letters 
    /// A through Z, the numerals 0 through 9, and the hyphen (-), plus the period (.) 
    /// character used as a separator.
    /// 
    /// Example: "microsoft.com"
    pub DNSDomain: Option<String>,
    /// Array of DNS domain suffixes to be appended to the end of host names during name 
    /// resolution. When attempting to resolve a fully qualified domain name (FQDN) from a 
    /// host-only name, the system will first append the local domain name. If this is not 
    /// successful, the system will use the domain suffix list to create additional FQDNs in 
    /// the order listed and query DNS servers for each.
    /// 
    /// Example: "samples.microsoft.com example.microsoft.com"
    pub DNSDomainSuffixSearchOrder: Option<Vec<String>>,
    /// If `TRUE`, the Domain Name System (DNS) is enabled for name resolution over Windows 
    /// Internet Naming Service (WINS) resolution. If the name cannot be resolved using DNS, 
    /// the name request is forwarded to WINS for resolution.
    pub DNSEnabledForWINSResolution: Option<bool>,
    /// Host name used to identify the local computer for authentication by some utilities. 
    /// Other TCP/IP-based utilities can use this value to acquire the name of the local 
    /// computer. Host names are stored on DNS servers in a table that maps names to IP 
    /// addresses for use by DNS. The name can be any combination of the letters A through Z, 
    /// the numerals 0 through 9, and the hyphen (-), plus the period (.) character used as a 
    /// separator. By default, this value is the Microsoft networking computer name, but the 
    /// network administrator can assign another host name without affecting the computer name.
    /// 
    /// Example: "corpdns"
    pub DNSHostName: Option<String>,
    /// Array of server IP addresses to be used in querying for DNS servers.
    pub DNSServerSearchOrder: Option<Vec<String>>,
    /// If `TRUE`, the IP addresses for this connection are registered in DNS under the domain 
    /// name of this connection in addition to being registered under the computer's full DNS 
    /// name. The domain name of this connection is either set using the SetDNSDomain() method 
    /// or assigned by DSCP. The registered name is the host name of the computer with the 
    /// domain name appended.
    pub DomainDNSRegistrationEnabled: Option<bool>,
    /// Memory allocated by IP to store packet data in the router packet queue. When this buffer 
    /// space is filled, the router begins discarding packets at random from its queue. Packet 
    /// queue data buffers are 256 bytes in length, so the value of this parameter should be a 
    /// multiple of 256. Multiple buffers are chained together for larger packets. The IP header 
    /// for a packet is stored separately. This parameter is ignored and no buffers are allocated 
    /// if the IP router is not enabled. The buffer size can range from the network MTU to a value 
    /// smaller than 0xFFFFFFFF. Default: 74240 (fifty 1480-byte packets, rounded to a multiple of 
    /// 256).
    pub ForwardBufferMemory: Option<u32>,
    /// If `TRUE`, the IP addresses for this connection are registered in DNS under the computer's 
    /// full DNS name. The full DNS name of the computer is displayed on the `Network Identification` 
    /// tab in the System application in Control Panel.
    pub FullDNSRegistrationEnabled: Option<bool>,
    /// Array of integer cost metric values (ranging from 1 to 9999) to be used in calculating 
    /// the fastest, most reliable, or least resource-intensive routes. This argument has a 
    /// one-to-one correspondence with the `DefaultIPGateway` property.
    pub GatewayCostMetric: Option<Vec<u16>>,
    /// Extent to which the system supports IP multicast and participates in the Internet Group 
    /// Management Protocol (IGMP). At level 0 (zero), the system provides no multicast support. 
    /// At level 1, the system may only send IP multicast packets. At level 2, the system may send
    /// IP multicast packets and fully participate in IGMP to receive multicast packets.
    /// 
    /// - `No Multicast` (0)
    /// - `IP Multicast` (1)
    /// - `IP & IGMP multicast` (2): IP and IGMP Multicast (default)
    pub IGMPLevel: Option<u8>,
    /// Index number of the Windows network adapter configuration. The index number is used when 
    /// there is more than one configuration available.
    pub Index: Option<u32>,
    /// Index value that uniquely identifies a local network interface. The value in this property 
    /// is the same as the value in the `InterfaceIndex` property in the instance of `Win32_IP4RouteTable` 
    /// that represents the network interface in the route table.
    pub InterfaceIndex: Option<u32>,
    /// Array of all of the IP addresses associated with the current network adapter. This 
    /// property can contain either IPv6 addresses or IPv4 addresses. For more information, 
    /// see IPv6 and IPv4 Support in WMI.
    /// 
    /// Example IPv6 address: "2010:836B:4179::836B:4179"
    pub IPAddress: Option<Vec<String>>,
    /// Cost of using the configured routes for the IP bound adapter and is the weighted value 
    /// for those routes in the IP routing table. If there are multiple routes to a destination 
    /// in the IP routing table, the route with the lowest metric is used. The default value is 
    /// 1.
    pub IPConnectionMetric: Option<u32>,
    /// If `TRUE`, TCP/IP is bound and enabled on this network adapter.
    pub IPEnabled: Option<bool>,
    /// If `TRUE`, IP port security is enabled globally across all IP-bound network adapters and 
    /// the security values associated with individual network adapters are in effect. This 
    /// property is used in conjunction with `IPSecPermitTCPPorts`, `IPSecPermitUDPPorts`, and 
    /// `IPSecPermitIPProtocols`. If `FALSE`, IP filter security is disabled across all network 
    /// adapters and allows all port and protocol traffic to flow unfiltered.
    pub IPFilterSecurityEnabled: Option<bool>,
    /// If `TRUE`, IP port security is enabled globally across all IP-bound network adapters. 
    /// This property is obsolete. In place of this property, you should use `IPFilterSecurityEnabled`.
    pub IPPortSecurityEnabled: Option<bool>,
    /// Array of the protocols permitted to run over the IP. The list of protocols is defined 
    /// using the `EnableIPSec` method. The list will either be empty or contain numeric values. 
    /// A numeric value of 0 (zero) indicates access permission is granted for all protocols. 
    /// An empty string indicates that no protocols are permitted to run when `IPFilterSecurityEnabled` 
    /// is `TRUE`.
    pub IPSecPermitIPProtocols: Option<Vec<String>>,
    /// Array of the ports that will be granted access permission for TCP. The list of protocols 
    /// is defined using the `EnableIPSec` method. The list will either be empty or contain numeric 
    /// values. A numeric value of 0 (zero)indicates access permission is granted for all ports. 
    /// An empty string indicates that no ports are granted access permission when `IPFilterSecurityEnabled` 
    /// is `TRUE`.
    pub IPSecPermitTCPPorts: Option<Vec<String>>,
    /// Array of the ports that will be granted User Datagram Protocol (UDP) access permission. 
    /// The list of protocols is defined using the `EnableIPSec` method. The list will either be 
    /// empty or contain numeric values. A numeric value of 0 (zero) indicates access permission 
    /// is granted for all ports. An empty string indicates that no ports are granted access 
    /// permission when `IPFilterSecurityEnabled` is `TRUE`.
    pub IPSecPermitUDPPorts: Option<Vec<String>>,
    /// Array of all of the subnet masks associated with the current network adapter.
    /// 
    /// Example: "255.255.0.0"
    pub IPSubnet: Option<Vec<String>>,
    /// If `TRUE`, IP zeros-broadcasts are used (0.0.0.0), and the system uses ones-broadcasts 
    /// (255.255.255.255). Computer systems generally use ones-broadcasts, but those derived 
    /// from BSD implementations use zeros-broadcasts. Systems that do not use that same 
    /// broadcasts will not interoperate on the same network. The default is `FALSE`.
    pub IPUseZeroBroadcast: Option<bool>,
    /// The Internetwork Packet Exchange (IPX) technology is no longer supported and this 
    /// property does not contain useful data.
    pub IPXAddress: Option<String>,
    /// The Internetwork Packet Exchange (IPX) technology is no longer supported and this 
    /// property does not contain useful data.
    pub IPXEnabled: Option<bool>,
    /// The Internetwork Packet Exchange (IPX) technology is no longer supported and this 
    /// property does not contain useful data.
    /// 
    /// - `Ethernet II` (0)
    /// - `Ethernet 802.3` (1)
    /// - `Ethernet 802.2` (2)
    /// - `Ethernet SNAP` (3)
    /// - `AUTO` (255)
    pub IPXFrameType: Option<Vec<u32>>,
    /// The Internetwork Packet Exchange (IPX) technology is no longer supported and this 
    /// property does not contain useful data.
    /// 
    /// - `Ethernet` (1)
    /// - `Token ring` (2)
    /// - `FDDI` (3)
    /// - `ARCNET` (8)
    pub IPXMediaType: Option<u32>,
    /// The Internetwork Packet Exchange (IPX) technology is no longer supported and this 
    /// property does not contain useful data.
    pub IPXNetworkNumber: Option<Vec<String>>,
    /// The Internetwork Packet Exchange (IPX) technology is no longer supported and this 
    /// property does not contain useful data.
    pub IPXVirtualNetNumber: Option<String>,
    /// Interval separating Keep Alive Retransmissions until a response is received. After a 
    /// response is received, the delay until the next Keep Alive Transmission is again 
    /// controlled by the value of `KeepAliveTime`. The connection will be aborted after the 
    /// number of retransmissions specified by `TcpMaxDataRetransmissions` have gone unanswered. 
    /// Default: 1000, Valid Range: 1 - 0xFFFFFFFF.
    pub KeepAliveInterval: Option<u32>,
    /// The `KeepAliveTime` property indicates how often the TCP attempts to verify that an 
    /// idle connection is still intact by sending a Keep Alive Packet. A remote system that 
    /// is reachable will acknowledge the keep alive transmission. Keep Alive packets are not 
    /// sent by default. This feature may be enabled in a connection by an application. 
    /// Default: 7,200,000 (two hours).
    pub KeepAliveTime: Option<u32>,
    /// Media Access Control (MAC) address of the network adapter. A MAC address is assigned by 
    /// the manufacturer to uniquely identify the network adapter.
    /// 
    /// Example: "00:80:C7:8F:6C:96"
    pub MACAddress: Option<String>,
    /// Overrides the default Maximum Transmission Unit (MTU) for a network interface. The MTU 
    /// is the maximum packet size (including the transport header) that the transport will 
    /// transmit over the underlying network. The IP datagram can span multiple packets. The 
    /// range of this value spans the minimum packet size (68) to the MTU supported by the 
    /// underlying network.
    pub MTU: Option<u32>,
    /// Number of IP packet headers allocated for the router packet queue. When all headers are 
    /// in use, the router will begin to discard packets from the queue at random. This value 
    /// should be at least as large as the `ForwardBufferMemory` value divided by the maximum IP 
    /// data size of the networks connected to the router. It should be no larger than the 
    /// `ForwardBufferMemory` value divided by 256, since at least 256 bytes of forward buffer 
    /// memory are used for each packet. The optimal number of forward packets for a given 
    /// `ForwardBufferMemory` size depends on the type of traffic on the network. It will be 
    /// somewhere between these two values. If the router is not enabled, this parameter is 
    /// ignored and no headers are allocated. Default: 50, Valid Range: 1 - 0xFFFFFFFE.
    pub NumForwardPackets: Option<u32>,
    /// If `TRUE`, detection of black hole routers occurs while TCP discovers the path of the 
    /// Maximum Transmission Unit. A black hole router does not return ICMP Destination 
    /// Unreachable messages when it needs to fragment an IP datagram with the Don't Fragment 
    /// bit set. TCP depends on receiving these messages to perform Path MTU Discovery. With 
    /// this feature enabled, TCP will try to send segments without the Don't Fragment bit set 
    /// if several retransmissions of a segment go unacknowledged. If the segment is acknowledged 
    /// as a result, the MSS will be decreased and the Don't Fragment bit will be set in future 
    /// packets on the connection. Enabling black hole detection increases the maximum number of 
    /// retransmissions performed for a given segment. The default value of this property is `FALSE`.
    pub PMTUBHDetectEnabled: Option<bool>,
    /// If `TRUE`, the Maximum Transmission Unit (MTU) path is discovered over the path to a 
    /// remote host. By discovering the MTU path and limiting TCP segments to this size, TCP 
    /// can eliminate fragmentation at routers along the path that connect networks with 
    /// different MTUs. Fragmentation adversely affects TCP throughput and network congestion. 
    /// Setting this parameter to FALSE causes an MTU of 576 bytes to be used for all 
    /// connections that are not to machines on the local subnet. The default is `TRUE`.
    pub PMTUDiscoveryEnabled: Option<bool>,
    /// Service name of the network adapter. This name is usually shorter than the full product 
    /// name.
    /// 
    /// Example: "Elnkii"
    pub ServiceName: Option<String>,
    /// Bitmap of the possible settings related to NetBIOS over TCP/IP. Values are identified in 
    /// the following list.
    /// 
    /// - `EnableNetbiosViaDhcp` (0)
    /// - `EnableNetbios` (1)
    /// - `DisableNetbios` (2)
    pub TcpipNetbiosOptions: Option<u32>,
    /// Number of times TCP attempts to retransmit a Connect Request before terminating the 
    /// connection. The initial retransmission timeout is 3 seconds. The retransmission timeout 
    /// doubles for each attempt. Default: 3, Valid Range: 0 - 0xFFFFFFFF.
    pub TcpMaxConnectRetransmissions: Option<u32>,
    /// Number of times TCP retransmits an individual data segment (nonconnect segment) before 
    /// terminating the connection. The retransmission timeout doubles with each successive 
    /// retransmission on a connection. Default: 5, Valid Range: 0 - 0xFFFFFFFF.
    pub TcpMaxDataRetransmissions: Option<u32>,
    /// Maximum number of connections that TCP can have open simultaneously. Default: 0xFFFFFE, 
    /// Valid Range: 0 - 0xFFFFFE.
    pub TcpNumConnections: Option<u32>,
    /// If `TRUE`, TCP uses the RFC 1122 specification for urgent data. If `FALSE` (default), TCP 
    /// uses the mode used by Berkeley Software Design (BSD) derived systems. The two mechanisms 
    /// interpret the urgent pointer differently and are not interoperable. The default value is 
    /// `FALSE`.
    pub TcpUseRFC1122UrgentPointer: Option<bool>,
    /// Maximum TCP Receive Window size offered by the system. The Receive Window specifies the 
    /// number of bytes a sender may transmit without receiving an acknowledgment. In general, 
    /// larger receiving windows will improve performance over high-delay and high-bandwidth 
    /// networks. For efficiency, the receiving window should be an even multiple of the TCP 
    /// Maximum Segment Size (MSS). Default: Four times the maximum TCP data size or an even 
    /// multiple of TCP data size rounded up to the nearest multiple of 8192. Ethernet networks 
    /// default to 8760. Valid range: 0 - 65535.
    /// 
    /// Note: Windows Vista: This property accesses the "CurrentControlSet\\Services\\Tcpip\\Parameters|
    /// TcpWindowSize" registry entry, which is not used in the current implementation of the 
    /// operating system.
    pub TcpWindowSize: Option<u16>,
    /// If `TRUE`, local lookup files are used. Lookup files will contain a map of IP addresses 
    /// to host names. If they exist on the local system, they will be found in 
    /// %SystemRoot%\system32\drivers\etc.
    pub WINSEnableLMHostsLookup: Option<bool>,
    /// Path to a WINS lookup file on the local system. This file will contain a map of IP 
    /// addresses to host names. If the file specified in this property is found, it will be 
    /// copied to the %SystemRoot%\system32\drivers\etc folder of the local system. Valid only 
    /// if the `WINSEnableLMHostsLookup` property is `TRUE`.
    pub WINSHostLookupFile: Option<String>,
    /// IP address for the primary WINS server.
    pub WINSPrimaryServer: Option<String>,
    /// Value appended to the end of the NetBIOS name that isolates a group of computer systems 
    /// communicating with only each other. It is used for all NetBIOS transactions over TCP/IP 
    /// communications from that computer system. Computers configured with identical scope 
    /// identifiers are able to communicate with this computer. TCP/IP clients with different 
    /// scope identifiers disregard packets from computers with this scope identifier. Valid 
    /// only when the `EnableWINS` method executes successfully.
    pub WINSScopeID: Option<String>,
    /// IP address for the secondary WINS server.
    pub WINSSecondaryServer: Option<String>,
}

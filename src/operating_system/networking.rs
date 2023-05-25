//! The Networking subcategory groups classes that represent network connections, network clients, and network connection settings such as the protocol used.
//! 
//! | Class                                                                            | Description                                                                                                                      |
//! |----------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_ActiveRoute**](/previous-versions/windows/desktop/wmiiprouteprov/win32-activeroute)                       | Association class<br/> Relates the current IP4 route to the persisted IP route table.<br/>                           |
//! | [**Win32\_IP4PersistedRouteTable**](/previous-versions/windows/desktop/wmiiprouteprov/win32-ip4persistedroutetable) | Instance class<br/> Represents persisted IP routes.<br/>                                                             |
//! | [**Win32\_IP4RouteTable**](/previous-versions/windows/desktop/wmiiprouteprov/win32-ip4routetable)                   | Instance class<br/> Represents information that governs the routing of network data packets.<br/>                    |
//! | [**Win32\_IP4RouteTableEvent**](/previous-versions/windows/desktop/wmiiprouteprov/win32-ip4routetableevent)         | Event class<br/> Represents IP route change events.<br/>                                                             |
//! | [**Win32\_NetworkClient**](win32-networkclient.md)                              | Instance class<br/> Represents a network client on a computer system running Windows.<br/>                           |
//! | [**Win32\_NetworkConnection**](win32-networkconnection.md)                      | Instance class<br/> Represents an active network connection in a Windows environment.<br/>                           |
//! | [**Win32\_NetworkProtocol**](win32-networkprotocol.md)                          | Instance class<br/> Represents a protocol and its network characteristics on a computer system running Windows.<br/> |
//! | [**Win32\_NTDomain**](/previous-versions/windows/desktop/cimwin32a/win32-ntdomain)                                        | Instance class<br/> Represents a Windows NT domain.<br/>                                                             |
//! | [**Win32\_PingStatus**](/previous-versions/windows/desktop/wmipicmp/win32-pingstatus)                               | Instance class<br/> Represents the values returned by the standard **ping** command.<br/>                            |
//! | [**Win32\_ProtocolBinding**](win32-protocolbinding.md)                          | Association class<br/> Relates a system-level driver, network protocol, and network adapter.<br/>                    |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows IP4PersistedRouteTables
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IP4PersistedRouteTables {
    /// Represents sequence of Windows `IP4PersistedRouteTables`
    pub ip4_persisted_route_tables: Vec<Win32_IP4PersistedRouteTable>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(IP4PersistedRouteTables, ip4_persisted_route_tables);

/// Represents the state of Windows IP4RouteTables
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IP4RouteTables {
    /// Represents sequence of Windows `IP4RouteTables`
    pub ip4_route_tables: Vec<Win32_IP4RouteTable>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(IP4RouteTables, ip4_route_tables);

/// Represents the state of Windows NetworkClients
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkClients {
    /// Represents sequence of Windows `NetworkClients`
    pub nework_clients: Vec<Win32_NetworkClient>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(NetworkClients, nework_clients);

/// Represents the state of Windows NetworkConnections
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkConnections {
    /// Represents sequence of Windows `NetworkConnections`
    pub nework_connections: Vec<Win32_NetworkConnection>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(NetworkConnections, nework_connections);

/// The `Win32_IP4PersistedRouteTable` WMI class represents persisted IP routes. By default, the routes 
/// added to the routing table are not permanent. Rebooting the computer clears the routes from the 
/// table. However, the following command makes the route persist after the computer is restarted: 
/// route -p add.
/// 
/// Persistent entries are automatically inserted again in the route table each time the route table 
/// is rebuilt. The operating system stores persistent routes in the registry. An entry can be removed 
/// through the method call SWbemServices.Delete for scripting or IWbemServices::DeleteInstance for C++ 
/// programming.
/// 
/// This class is only applicable to IPv4 and does not return IPX or IPv6 data. 
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmiiprouteprov/win32-ip4persistedroutetable>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_IP4PersistedRouteTable {
    /// Short description of the object. 
    pub Caption: Option<String>,
    /// Description of the object. 
    pub Description: Option<String>,
    /// Destination IP address for this persisted route.
    pub Destination: Option<String>,
    /// Object was installed. This property does not need a value to indicate that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Mask used in this persisted entry. Use the logical AND operation to combine the mask with the 
    /// destination address. Compare the result to the value in the ipRouteDest field.
    pub Mask: Option<String>,
    /// Primary routing metric for this persisted route. The semantics of this metric are determined by the 
    /// routing protocol specified in the route's ipRouteProto value. If this property is not used, its value 
    /// should be set to -1.
    pub Metric1: Option<i32>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a key property. 
    pub Name: Option<String>,
    /// IP address of the next hop of this persisted route. If the route is bound to an interface that is 
    /// realized via a broadcast medium, this field contains the agent's IP address on that interface.
    pub NextHop: Option<String>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. Operational 
    /// statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a SMART-enabled hard disk drive,
    ///  may be functioning properly but predicting a failure in the near future). Nonoperational statuses include: 
    /// "Error", "Starting", "Stopping", and "Service". The latter, "Service", could apply during mirror-resilvering 
    /// of a disk, reload of a user permissions list, or other administrative work. Not all such work is online, 
    /// yet the managed element is neither "OK" nor in one of the other states.
    /// 
    /// The values are:
    /// - "`OK`"
    /// - "`Error`"
    /// - "`Degraded`"
    /// - "`Unknown`"
    /// - "`Pred Fail`"
    /// - "`Starting`"
    /// - "`Stopping`"
    /// - "`Service`"
    pub Status: Option<String>,
}

/// The `Win32_IP4RouteTable` WMI class represents information that governs the routing of network data packets. 
/// For example, Internet packets are usually sent to a gateway and local packets are routed directly by the 
/// client computer. Administrators can use this information to trace problems associated with misrouted packets, 
/// and also direct a computer to a new gateway as necessary. This class only represents the information shown 
/// as a result of typing the `route print` command from the command prompt.
/// 
/// This class is only applicable to IPv4 and does not return IPX or IPv6 data. 
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmiiprouteprov/win32-ip4routetable>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_IP4RouteTable {
    /// Number of seconds since this route was last updated or otherwise determined to be correct. Whether the 
    /// routing information is outdated can only be determined by knowing the routing protocol by which the 
    /// route was learned.
    pub Age: Option<i32>,
    /// Short description of the object. 
    pub Caption: Option<String>,
    /// Description of the object. 
    pub Description: Option<String>,
    /// Destination IP address for this route.
    pub Destination: Option<String>,
    /// Reference to Management Information Base (MIB) definitions specific to the particular routing protocol 
    /// that handles this route, as determined by the value specified in the route ipRouteProto value. If this
    /// information is not present, its value should be set to the OBJECT IDENTIFIER {0 0}, which is a syntactically 
    /// valid object identifier, and any conforming implementation of ASN.1 and BER must be able to generate and 
    /// recognize this value.
    pub Information: Option<String>,
    /// Object was installed. This property does not need a value to indicate that the object is installed. 
    pub InstallDate: Option<WMIDateTime>,
    /// IP address of the next hop of this route. The value in this property is the same as the value in the 
    /// `InterfaceIndex` property in the instances of `Win32_NetworkAdapter` and `Win32_NetworkAdapterConfiguration` 
    /// that represent the network interface of the next hop of the route.
    /// 
    /// In the case of a route bound to an interface that is realized using a broadcast medium, the value of this 
    /// field is the agent IP address on that interface.
    pub InterfaceIndex: Option<i32>,
    /// Mask used in this entry. Masks should be joined with a logical AND to the destination address before being 
    /// compared to the value in the ipRouteDest field.
    pub Mask: Option<String>,
    /// Primary routing metric for this route. The routing protocol specified in the ipRouteProto value for the 
    /// route determines the interpretation of this property. Set the value of this property to -1 if it is not used.
    pub Metric1: Option<i32>,
    /// Alternate routing metric for this route. The routing protocol specified in the ipRouteProto value for the route 
    /// determines the interpretation of this property. Set the value of this property to -1 if it is not used.
    pub Metric2: Option<i32>,
    /// Alternate routing metric for this route. The routing protocol specified in the ipRouteProto value for the route 
    /// determines the interpretation of this property. Set the value of this property to -1 if it is not used.
    pub Metric3: Option<i32>,
    /// Alternate routing metric for this route. The routing protocol specified in the ipRouteProto value for the route 
    /// determines the interpretation of this property. Set the value of this property to -1 if it is not used.
    pub Metric4: Option<i32>,
    /// Alternate routing metric for this route. The routing protocol specified in the ipRouteProto value for the route 
    /// determines the interpretation of this property. Set the value of this property to -1 if it is not used.
    pub Metric5: Option<i32>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a key property. 
    pub Name: Option<String>,
    /// IP address of the next hop of this route. For a route that is bound to an interface realized via a broadcast 
    /// medium, the value of this field is the agent IP address on that interface.
    pub NextHop: Option<String>,
    /// Routing mechanism through which this route was learned. Inclusion of values for gateway routing protocols does 
    /// not imply that hosts must support those protocols.
    /// 
    /// - other (1)
    /// - local (2)
    /// - netmgmt (3)
    /// - icmp (4)
    /// - egp (5)
    /// - ggp (6)
    /// - hello (7)
    /// - rip (8)
    /// - is-is (9)
    /// - es-is (10)
    /// - ciscoIgrp (11)
    /// - bbnSpfIgp (12)
    /// - ospf (13)
    /// - bgp (14)
    pub Protocol: Option<u32>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. Operational statuses 
    /// include: "OK", "Degraded", and "Pred Fail" (an element, such as a SMART-enabled hard disk disk drive, may be 
    /// functioning properly but predicting a failure in the near future). Nonoperational statuses include: "Error", 
    /// "Starting", "Stopping", and "Service". The latter, "Service", could apply during mirror-resilvering of a disk, 
    /// reload of a user permissions list, or other administrative work. Not all such work is online, yet the managed 
    /// element is neither "OK" nor in one of the other states.
    /// 
    /// The values are:
    /// - "`OK`"
    /// - "`Error`"
    /// - "`Degraded`"
    /// - "`Unknown`"
    /// - "`Pred Fail`"
    /// - "`Starting`"
    /// - "`Stopping`"
    /// - "`Service`"
    pub Status: Option<String>,
    /// Type of route. The following list lists the values.
    /// - `other` (1)
    /// - `invalid` (2)
    /// - `direct` (3)
    /// - `indirect` (4)
    /// 
    /// Values 3 (Direct) and 4 (Indirect) refer to direct and indirect routing in the IP architecture. Setting this 
    /// object to the value 2 (Invalid) invalidates the corresponding entry in the RouteTable object by disassociating 
    /// the entry's destination from the entry's route. An agent may leave an invalid entry in the table. Therefore, 
    /// management stations can receive information from agents that applies to entries not currently in use. To 
    /// interpret such entries, examine the relevant ipRouteType object.
    pub Type: Option<u32>,
}

/// The `Win32_NetworkClient` WMI class represents a network client on a Windows system. Any computer system on the network 
/// with a client relationship to the system is a descendant (or member) of this class.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkclient>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NetworkClient {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object. Operational and non-operational status can be defined. 
    /// Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" indicates that an element is 
    /// functioning properly, but is predicting a failure (for example, a SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can apply during 
    /// disk mirror-resilvering, reloading a user permissions list, or other administrative work. Not all such work is 
    /// online, but the managed element is neither "OK" nor in one of the other states.
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
    /// Name of the manufacturer of the network client running on the computer system running Windows.
    /// 
    /// Example: "Microsoft Corporation"
    pub Manufacturer: Option<String>,
    /// Network name of the network client running on the computer system running Windows.
    /// 
    /// Example: "Microsoft Windows Network"
    pub Name: Option<String>,
}

/// The `Win32_NetworkConnection` WMI classrepresents an active network connection in a Windows-based environment.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-networkconnection>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_NetworkConnection {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// String that indicates the current status of the object. Operational and non-operational status can be defined. 
    /// Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" indicates that an element is 
    /// functioning properly, but is predicting a failure (for example, a SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can apply during 
    /// disk mirror-resilvering, reloading a user permissions list, or other administrative work. Not all such work is 
    /// online, but the managed element is neither "OK" nor in one of the other states.
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
    /// List of access rights to the given file or directory held by the user or group on whose behalf the instance is 
    /// returned. On FAT volumes, the `FULL_ACCESS` value is returned instead, indicating no security has been set on the 
    /// object.
    /// 
    /// - `FILE_READ_DATA (file) or FILE_LIST_DIRECTORY (directory) (1)`: Grants the right to read data from the file. For a directory, this value grants the right to list the contents of the directory.
    /// - `FILE_WRITE_DATA (file) or FILE_ADD_FILE (directory) (2)`: Grants the right to write data to the file. For a directory, this value grants the right to create a file in the directory.
    /// - `FILE_APPEND_DATA (file) or FILE_ADD_SUBDIRECTORY (4)`: Grants the right to append data to the file. For a directory, this value grants the right to create a subdirectory.
    /// - `FILE_READ_EA (8)`: Grants the right to read extended attributes.
    /// - `FILE_WRITE_EA (16)`: Grants the right to write extended attributes.
    /// - `FILE_EXECUTE (file) or FILE_TRAVERSE (directory) (32)`: Grants the right to execute a file. For a directory, the directory can be traversed.
    /// - `FILE_DELETE_CHILD (directory) (64)`: Grants the right to delete a directory and all of the files it contains (its children), even if the files are read-only.
    /// - `FILE_READ_ATTRIBUTES (128)`: Grants the right to read file attributes.
    /// - `FILE_WRITE_ATTRIBUTES (256)`: Grants the right to change file attributes.
    /// - `DELETE (65536)`: Grants delete access.
    /// - `READ_CONTROL (131072)`: Grants read access to the security descriptor and owner.
    /// - `WRITE_DAC (262144)`: Grants write access to the discretionary access control list (DACL).
    /// - `WRITE_OWNER (524288)`: Assigns the write owner.
    /// - `SYNCHRONIZE (1048576)`: Synchronizes access and allows a process to wait for an object to enter the signaled state.
    pub AccessMask: Option<u32>,
    /// Comment supplied by the network provider. 
    pub Comment: Option<String>,
    /// Current state of the network connection.
    /// 
    /// Connected ("Connected")
    /// - `Error` ("Error")
    /// - `Paused` ("Paused")
    /// - `Disconnected` ("Disconnected")
    /// - `Connecting` ("Connecting")
    /// - `Reconnecting` ("Reconnecting")
    pub ConnectionState: Option<String>,
    /// Persistence type of the connection used for connecting to the network.
    /// 
    /// `Current Connection` ("Current Connection")
    /// `Persistent Connection` ("Persistent Connection")
    pub ConnectionType: Option<String>,
    /// Network object should be displayed in a network browsing user interface.
    /// - `Domain` ("Domain")
    /// - `Generic` ("Generic")
    /// - `Server` ("Server")
    /// - `Share` ("Share")
    pub DisplayType: Option<String>,
    /// Local name of the connected network device.
    /// 
    /// Example: "c:\public"
    pub LocalName: Option<String>,
    /// Name of the current network connection. It is the combination of the values in `RemoteName` and `LocalName`.
    /// 
    /// Example: "\\NTRELEASE (c:\public)"
    pub Name: Option<String>,
    /// Connection will be reconnected automatically by the operating system on the next logon.
    pub Persistent: Option<bool>,
    /// Name of the provider that owns the resource. This property can be `NULL` if the provider name is unknown.
    pub ProviderName: Option<String>,
    /// Remote network resource name for a network resource. For a current or persistent connection, `RemoteName` 
    /// contains the network name associated with the name of the value in the `LocalName` property. The name in 
    /// `RemoteName` must follow the network provider's naming conventions.
    /// 
    /// Example: "\\NTRELEASE"
    pub RemoteName: Option<String>,
    /// Full path to the network resource.
    /// 
    /// Example: "\\infosrv1\public"
    pub RemotePath: Option<String>,
    /// Type of resource to enumerate or connect to.
    /// - `Disk` ("Disk")
    /// - `Print` ("Print")
    /// - `Any` ("Any")
    pub ResourceType: Option<String>,
    /// User name or the default user name used to establish a network connection.
    /// 
    /// Example: "SYSTEM"
    pub UserName: Option<String>,
}

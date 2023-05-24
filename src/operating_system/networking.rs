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

/// The Win32_IP4PersistedRouteTable WMI class represents persisted IP routes. By default, the routes 
/// added to the routing table are not permanent. Rebooting the computer clears the routes from the 
/// table. However, the following command makes the route persist after the computer is restarted: 
/// route -p add.
/// 
/// Persistent entries are automatically inserted again in the route table each time the route table 
/// is rebuilt. The operating system stores persistent routes in the registry. An entry can be removed 
/// through the method call SWbemServices.Delete for scripting or IWbemServices::DeleteInstance for C++ 
/// programming.
/// 
/// This class is only applicable to IPv4 and does not return IPX or IPv6 data. For more information, 
/// see IPv6 and IPv4 Support in WMI.
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
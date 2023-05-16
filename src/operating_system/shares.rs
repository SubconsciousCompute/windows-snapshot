//! The Shares subcategory groups classes that represent details of shared resources, such as printers and folders.
//! 
//! | Class                                                       | Description                                                                                                                                                                                          |
//! |-------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_DFSNode**](/previous-versions/windows/desktop/wmipdfs/win32-dfsnode)                     | Association class<br/> Represents a root or junction node of a domain-based or stand-alone distributed file system (DFS).<br/>                                                           |
//! | [**Win32\_DFSNodeTarget**](/previous-versions/windows/desktop/wmipdfs/win32-dfsnodetarget)         | Association class<br/> Represents the relationship of a DFS node to one of its targets.<br/>                                                                                             |
//! | [**Win32\_DFSTarget**](/previous-versions/windows/desktop/wmipdfs/win32-dfstarget)                 | Association class<br/> Represents the target of a DFS node.<br/>                                                                                                                         |
//! | [**Win32\_ServerConnection**](/previous-versions/windows/desktop/wmipsess/win32-serverconnection)   | Instance class<br/> Represents the connections made from a remote computer to a shared resource on the local computer.<br/>                                                              |
//! | [**Win32\_ServerSession**](/previous-versions/windows/desktop/wmipsess/win32-serversession)         | Instance class<br/> Represents the sessions that are established with the local computer by users on a remote computer.<br/>                                                             |
//! | [**Win32\_ConnectionShare**](/previous-versions/windows/desktop/wmipsess/win32-connectionshare)     | Association class<br/> Relates a shared resource on the computer and the connection made to the shared resource.<br/>                                                                    |
//! | [**Win32\_PrinterShare**](win32-printershare)             | Association class<br/> Relates a local printer and the share that represents it as it is viewed over a network.<br/>                                                                     |
//! | [**Win32\_SessionConnection**](/previous-versions/windows/desktop/wmipsess/win32-sessionconnection) | Association class<br/> Represents an association between a session established with the local server by a user on a remote machine, and the connections that depend on the session.<br/> |
//! | [**Win32\_SessionProcess**](win32-sessionprocess)         | Association class<br/> Represents an association between a logon session and the processes associated with that session.<br/>                                                            |
//! | [**Win32\_ShareToDirectory**](win32-sharetodirectory)     | Association class<br/> Relates a shared resource on the computer system and the directory to which it is mapped.<br/>                                                                    |
//! | [**Win32\_Share**](win32-share)                         | Instance class<br/> Represents a shared resource on a computer system running Windows.<br/>                                                                                              |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows ServerConnections
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConnections {
    /// Represents sequence of Windows `ServerConnections`
    pub server_connections: Vec<Win32_ServerConnection>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(ServerConnections, server_connections);

/// The `Win32_ServerConnection` WMI class represents the connections made from a remote computer 
/// to a shared resource on the local computer.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmipsess/win32-serverconnection>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ServerConnection {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object 
    /// is not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be 
    /// a key property. 
    pub Name: Option<String>,
    /// String that indicates the current status of the object. Operational and non-operational status 
    /// can be defined. Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" 
    /// indicates that an element is functioning properly, but is predicting a failure (for example, a 
    /// SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" 
    /// can apply during disk mirror-resilvering, reloading a user permissions list, or other 
    /// administrative work. Not all such work is online, but the managed element is neither "OK" nor 
    /// in one of the other states. This property is inherited from CIM_ManagedSystemElement.
    /// 
    /// Values include the following:
    /// - OK ("OK")
    /// - Error ("Error")
    /// - Degraded ("Degraded")
    /// - Unknown ("Unknown")
    /// - Pred Fail ("Pred Fail")
    /// - Starting ("Starting")
    /// - Stopping ("Stopping")
    /// - Service ("Service")
    /// - Stressed ("Stressed")
    /// - NonRecover ("NonRecover")
    /// - No Contact ("No Contact")
    /// - Lost Comm ("Lost Comm")
    pub Status: Option<String>,
    /// Number of seconds since this connection was established.
    pub ActiveTime: Option<u32>,
    /// Name of the computer from which the connection is established.
    pub ComputerName: Option<String>,
    /// Unique identifier for the connection.
    pub ConnectionID: Option<u32>,
    /// Number of open files associated with this connection.
    pub NumberOfFiles: Option<u32>,
    /// Number of users associated with this connection.
    pub NumberOfUsers: Option<u32>,
    /// Share resource to which the connection is established.
    pub ShareName: Option<String>,
    /// Name of the user that made a connection.
    pub UserName: Option<String>,
}

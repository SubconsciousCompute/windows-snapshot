//! The Users subcategory groups classes that represent storage information.
//! 
//! | Class                                                                   | Description                                                                                                                                  |
//! |-------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_ShadowBy**](/previous-versions/windows/desktop/vsswmi/win32-shadowby)                               | Association class<br/> Represents the association between a shadow copy and the provider that creates the shadow copy.<br/>      |
//! | [**Win32\_ShadowContext**](/previous-versions/windows/desktop/vsswmi/win32-shadowcontext)                     | Association class<br/> Specifies how a shadow copy is to be created, queried, or deleted.<br/>                                   |
//! | [**Win32\_ShadowCopy**](/previous-versions/windows/desktop/legacy/aa394428(v=vs.85))                           | Instance class<br/> Represents a duplicate copy of the original volume at a previous time.<br/>                                  |
//! | [**Win32\_ShadowDiffVolumeSupport**](/previous-versions/windows/desktop/vsswmi/win32-shadowdiffvolumesupport) | Association class<br/> Represents an association between a shadow copy provider and a storage volume.<br/>                       |
//! | [**Win32\_ShadowFor**](/previous-versions/windows/desktop/vsswmi/win32-shadowfor)                             | Association class<br/> Represents an association between a shadow copy and the volume for which the shadow copy is created.<br/> |
//! | [**Win32\_ShadowOn**](/previous-versions/windows/desktop/vsswmi/win32-shadowon)                               | Association class<br/> Represents an association between a shadow copy and where the differential data is written.<br/>          |
//! | [**Win32\_ShadowProvider**](/previous-versions/windows/desktop/vsswmi/win32-shadowprovider)                   | Association class<br/> Represents a component that creates and represents volume shadow copies.<br/>                             |
//! | [**Win32\_ShadowStorage**](/previous-versions/windows/desktop/legacy/aa394433(v=vs.85))                     | Association class<br/> Represents an association between a shadow copy and where the differential data is written.<br/>          |
//! | [**Win32\_ShadowVolumeSupport**](/previous-versions/windows/desktop/vsswmi/win32-shadowvolumesupport)         | Association class<br/> Represents an association between a shadow copy provider with a supported volume.<br/>                    |
//! | [**Win32\_Volume**](/previous-versions/windows/desktop/legacy/aa394515(v=vs.85))                                   | Instance class<br/> Represents an area of storage on a hard disk.<br/>                                                           |
//! | [**Win32\_VolumeUserQuota**](/previous-versions/windows/desktop/vdswmi/win32-volumeuserquota)                 | Association class<br/> Represents a volume to the per volume quota settings.<br/>                                                |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows ShadowCopys
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShadowCopys {
    /// Represents sequence of `ShadowCopys`
    pub shadow_copys: Vec<Win32_ShadowCopy>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(ShadowCopys, shadow_copys);

/// The `Win32_ShadowCopy` class is a storage extent that represents a duplicate copy of the 
/// original volume at a previous time.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa394428(v=vs.85)>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ShadowCopy {
    /// Short textual description of the object. 
    pub Caption: Option<String>,
    /// Textual description of the object. 
    pub Description: Option<String>,
    /// Unique identifier for a shadow copy on the system.
    pub ID: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate that 
    /// the object is installed. This property is inherited from `CIM_LogicalElement`.
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a 
    /// key property. This property is inherited from `CIM_LogicalElement`.
    pub Name: Option<String>,
    /// Unique identifier for a shadow copy set to which the shadow belongs.
    pub SetID: Option<String>,
    /// Unique identifier for a shadow provider that creates a shadow.
    pub ProviderID: Option<String>,
    /// Current status of the object. This property is inherited from CIM_LogicalElement.
    /// 
    /// Values include the following:
    /// - "OK"
    /// - "Error"
    /// - "Degraded"
    /// - "Unknown"
    /// - "Pred Fail"
    /// - "Starting"
    /// - "Stopping"
    /// - "Service"
    /// - "Stressed"
    /// - "NonRecover"
    /// - "No Contact"
    /// - "Lost Comm"
    pub Status: Option<String>,
    /// Number of shadow copies in a shadow copy set to which a shadow copy belongs.
    pub Count: Option<u32>,
    /// Windows object manager name for an underlying storage device that supports the 
    /// original volume.
    pub DeviceObject: Option<String>,
    /// Name of the original volume for which a shadow copy is made.
    pub VolumeName: Option<String>,
    /// Name of the computer that hosts the original volume.
    pub OriginatingMachine: Option<String>,
    /// Name of the computer that services the shadow copy.
    pub ServiceMachine: Option<String>,
    /// File system name of a shadow copy when it is exposed. The `ExposedName` property might 
    /// contain a drive letter or mount point. The `ExposedName` property is `NULL` when a shadow 
    /// copy is hidden or otherwise not exposed.
    pub ExposedName: Option<String>,
    /// Current state of a shadow copy.
    /// 
    /// Value: Meaning
    /// 
    /// 0: Unknown
    /// 1: Preparing
    /// 2: ProcessingPrepare
    /// 3: Prepared
    /// 4: ProcessingPrecommit
    /// 5: Precommitted
    /// 6: ProcessingCommit
    /// 7: Committed
    /// 8: ProcessingPostcommit
    /// 9: Created
    /// 10: Aborted
    /// 11: Deleted
    /// 12: Count
    pub State: Option<u32>,
    /// If `true`, the shadow copy is persistent across reboots.
    pub Persistent: Option<bool>,
    /// If `true`, the shadow copy is created by the Windows Previous Versions component.
    pub ClientAccessible: Option<bool>,
    /// If `true`, the shadow copy is retained after the requestor process ends. If `false`, the shadow 
    /// copy is automatically deleted when the requestor process ends.
    pub NoAutoRelease: Option<bool>,
    /// If `true`, the shadow copy is created with the involvement of the shadow copy writer components.
    pub NoWriters: Option<bool>,
    /// If `true`, the shadow copy can be surfaced on another computer. If `false`, and the volumes are 
    /// surfaced locally, it may not be possible to surface them later on a different computer.
    pub Transportable: Option<bool>,
    /// If `true`, the shadow copy is not currently in the device namespace of a local computer.
    pub NotSurfaced: Option<bool>,
    /// If `true`, the shadow copy is created by a hardware shadow copy provider.
    pub HardwareAssisted: Option<bool>,
    /// If `true`, the shadow copy is created by a differential shadow copy provider. The provider 
    /// can be implemented in hardware or software.
    pub Differential: Option<bool>,
    /// If `true`, the shadow copy is created by a split mirror shadow copy provider. The provider 
    /// can be implemented in hardware or software.
    pub Plex: Option<bool>,
    /// If `true`, the shadow copy is imported to a computer by using the `Import` method and is not 
    /// created by using the `Create` method.
    pub Imported: Option<bool>,
    /// If `true`, the shadow copy is exposed on a remote computer with a network share. If `ExposedRemotely` 
    /// and `ExposedLocally` are not set, the shadow copy is hidden.
    pub ExposedRemotely: Option<bool>,
    /// If `true`, the shadow copy is exposed on the local computer with a drive letter or mount point. 
    /// If `ExposedLocally` and `ExposedRemotely` are not set, the shadow copy is hidden.
    pub ExposedLocally: Option<bool>,
}

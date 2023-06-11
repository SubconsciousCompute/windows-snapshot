//! The Windows Management Instrumentation (WMI) Software Licensing provider retrieves software licensing data. The classes are defined in SppWmi.mof and are located in the "root\cimv2" namespace.
//! 
//! | Classes                              | Description                                                                                                  |
//! | ------------------------------------ | ------------------------------------------------------------------------------------------------------------ |
//! | [**SoftwareLicensingProduct**](https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingproduct)             | Exposes the product-specific properties and methods of the Software Licensing service.                     |
//! | [**SoftwareLicensingService**](https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingservice)             | Exposes the product-independent properties and methods of the Software Licensing service.                   |
//! | [**SoftwareLicensingTokenActivationLicense**](https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingtokenactivationlicense) | Exposes the properties of installed token-based activation licenses.                                        |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows `SoftwareLicensingProducts`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SoftwareLicensingProducts {
    /// Represents data stored in a Windows SoftwareLicensingProducts
    pub software_licensing_products: Vec<SoftwareLicensingProduct>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(SoftwareLicensingProducts, software_licensing_products);

/// Represents the state of Windows `SoftwareLicensingServices`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SoftwareLicensingServices {
    /// Represents data stored in a Windows SoftwareLicensingServices
    pub software_licensing_services: Vec<SoftwareLicensingService>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(SoftwareLicensingServices, software_licensing_services);

/// Represents the state of Windows `SoftwareLicensingTokenActivationLicenses`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SoftwareLicensingTokenActivationLicenses {
    /// Represents data stored in a Windows SoftwareLicensingTokenActivationLicenses
    pub software_licensing_token_activation_licenses: Vec<SoftwareLicensingTokenActivationLicense>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(SoftwareLicensingTokenActivationLicenses, software_licensing_token_activation_licenses);

/// This class exposes the product-specific properties and methods of the Software Licensing service.
/// 
/// <https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingproduct>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SoftwareLicensingProduct {
    /// Specifies the product identifier.
    pub ID: Option<String>,
    /// Specifies the product name.
    pub Name: Option<String>,
    /// Specifies the product description.
    pub Description: Option<String>,
    /// Specifies the ID of current product application.
    pub ApplicationID: Option<String>,
    /// Specifies the software licensing server URL for the process certificate.
    pub ProcessorURL: Option<String>,
    /// Specifies the software licensing server URL for the binding certificate.
    pub MachineURL: Option<String>,
    /// Specifies the software licensing server URL for the product certificate.
    pub ProductKeyURL: Option<String>,
    /// Specifies the software licensing server URL for the user license.
    pub UseLicenseURL: Option<String>,
    /// Specifies the license status of this product application. The following values are possible.
    /// 
    /// Value: Description
    /// - 0: Unlicensed
    /// - 1: Licensed
    /// - 2: OOBGrace
    /// - 3: OOTGrace
    /// - 4: NonGenuineGrace
    /// - 5: Notification
    /// - 6: ExtendedGrace
    pub LicenseStatus: Option<u32>,
    /// Specifies the license status. Provides additional information about why a computer is in a 
    /// specific licensing state.
    pub LicenseStatusReason: Option<u32>,
    /// Specifies the remaining time, in minutes, before the parent application goes into notification 
    /// mode. For volume clients, this is the remaining time before reactivation is required.
    pub GracePeriodRemaining: Option<u32>,
    /// Specifies the expiration date of this product application. After this date, the `LicenseStatus` 
    /// property is set to Unlicensed and cannot be activated.
    pub EvaluationEndDate: Option<WMIDateTime>,
    /// Specifies the offline installation identifier of this product application. Used for offline 
    /// activation. Returns a `null` value if a product key is not installed.
    pub OfflineInstallationId: Option<String>,
    /// Specifies the last five characters of the product key. Returns a `null` value if a product key 
    /// is not installed.
    pub PartialProductKey: Option<String>,
    /// Specifies the product key ID. Returns a `null` value if a product key is not installed.
    pub ProductKeyID: Option<String>,
    /// Specifies the group identifier for the SKU used to determine license relationships for add-ons.
    pub LicenseFamily: Option<String>,
    /// Specifies the dependency identifier for the set of SKUs used to determine license relationships 
    /// for add-ons.
    pub LicenseDependsOn: Option<String>,
    /// Indicates `TRUE` if the product is identified as an add-on license.
    pub LicenseIsAddon: Option<bool>,
    /// Specifies the frequency, in minutes, of how often a client will contact the KMS host before the 
    /// product is licensed.
    pub VLActivationInterval: Option<u32>,
    /// Specifies the frequency, in minutes, of how often a client will contact the KMS host after the 
    /// product is licensed.
    pub VLRenewalInterval: Option<u32>,
    /// Specifies the KMS product key ID. Returns `null` if it is not applicable.
    pub KeyManagementServiceProductKeyID: Option<String>,
    /// Specifies the name of the KMS host. Returns null if `SetKeyManagementServiceMachine` 
    /// has not been called.
    pub KeyManagementServiceMachine: Option<String>,
    /// Specifies the TCP port that is used by clients to send KMS-activation requests. 
    /// Returns 0 if `SetKeyManagementServicePort` has not been called.
    pub KeyManagementServicePort: Option<u32>,
    /// Specifies the last discovered KMS host name through DNS.
    pub DiscoveredKeyManagementServiceMachineName: Option<String>,
    /// Specifies the last discovered KMS host port through DNS.
    pub DiscoveredKeyManagementServiceMachinePort: Option<u32>,
    /// Indicates whether KMS is enabled on the computer. The following values are possible.
    /// 
    /// Value: Description
    /// - 0: False
    /// - 1: True
    pub IsKeyManagementServiceMachine: Option<u32>,
    /// Specifies the count of currently active KMS clients on the KMS host. A value of -1 
    /// indicates the host is not enabled as a KMS or that it has not received any 
    /// client-licensing requests.
    pub KeyManagementServiceCurrentCount: Option<u32>,
    /// Specifies the minimum number of clients that are required to connect to a KMS host to 
    /// enable volume licensing.
    pub RequiredClientCount: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 0 (Unlicensed).
    pub KeyManagementServiceUnlicensedRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 1 (Licensed).
    pub KeyManagementServiceLicensedRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 2 (OOBGrace).
    pub KeyManagementServiceOOBGraceRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 3 (OOTGrace).
    pub KeyManagementServiceOOTGraceRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 4 
    /// (NonGenuineGrace).
    pub KeyManagementServiceNonGenuineGraceRequests: Option<u32>,
    /// Specifies the count of valid KMS requests.
    pub KeyManagementServiceTotalRequests: Option<u32>,
    /// Specifies the count of failed KMS requests.
    pub KeyManagementServiceFailedRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 5 
    /// (Notification).
    pub KeyManagementServiceNotificationRequests: Option<u32>,
    /// Specifies the genuine status for the product application.
    pub GenuineStatus: Option<u32>,
    /// Specifies the extended grace time, in minutes, before the parent application goes into 
    /// notification mode.
    pub ExtendedGrace: Option<u32>,
    /// Specifies the ID of the token-based activation license that activated the product.
    pub TokenActivationILID: Option<String>,
    /// Specifies the version of the token-based activation license that activated the product.
    pub TokenActivationILVID: Option<u32>,
    /// Specifies the grant number in the token-based activation license that activated the product.
    pub TokenActivationGrantNumber: Option<u32>,
    /// Specifies the thumbprint of the certificate that activated the product.
    pub TokenActivationCertificateThumbprint: Option<String>,
    /// Specifies additional information for token-based activation.
    pub TokenActivationAdditionalInfo: Option<String>,
    /// Specifies the trusted time for the product.
    pub TrustedTime: Option<WMIDateTime>,
}

/// This class exposes the product-independent properties and methods of the Software Licensing service.
/// 
/// <https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingservice>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SoftwareLicensingService {
    /// Specifies the version of the Software Licensing service.
    pub Version: Option<String>,
    /// Specifies the registered key management service machine name. Returns null if 
    /// `SetKeyManagementServiceMachine` has not been called.
    pub KeyManagementServiceMachine: Option<String>,
    /// Indicates whether the machine has a key management service (KMS) enabled. The following values 
    /// are possible.
    /// 
    /// Value: Description
    /// - 0: False
    /// - 1: True
    pub IsKeyManagementServiceMachine: Option<u32>,
    /// Specifies the frequency, in minutes, of how often a client should contact the KMS machine before 
    /// the client is licensed.
    pub VLActivationInterval: Option<u32>,
    /// Specifies the frequency, in minutes, of how often the current machine should contact the KMS machine 
    /// after the client is licensed.
    pub VLRenewalInterval: Option<u32>,
    /// Specifies the count of currently active volume clients. A value of -1 indicates that the machine 
    /// is not enabled as a KMS or that it has not received any client licensing-requests.
    pub KeyManagementServiceCurrentCount: Option<u32>,
    /// Specifies the minimum number of clients required to connect to a KMS machine to enable volume 
    /// licensing.
    pub RequiredClientCount: Option<u32>,
    /// Specifies the KMS product key ID. Returns null if not applicable.
    pub KeyManagementServiceProductKeyID: Option<String>,
    /// Specifies the last discovered KMS host name through DNS.
    pub DiscoveredKeyManagementServiceMachineName: Option<String>,
    /// Specifies the last discovered KMS host port through DNS.
    pub DiscoveredKeyManagementServiceMachinePort: Option<u32>,
    /// Indicates whether the licensing policy cache needs to be updated. The following values are possible.
    /// 
    /// Value: Description
    /// - 0: Refresh not required.
    /// - 1: Refresh required.
    pub PolicyCacheRefreshRequired: Option<u32>,
    /// Specifies the unique identifier for this volume client machine. The client includes this CMID 
    /// in requests it sends to the KMS.
    pub ClientMachineID: Option<String>,
    /// Specifies the remaining number of times that the client can be successfully rearmed.
    pub RemainingWindowsReArmCount: Option<u32>,
    /// Specifies the TCP port on which the KMS host listens for activation requests.
    pub KeyManagementServiceListeningPort: Option<u32>,
    /// Indicates the DNS publishing status of a KMS host. The following values are possible.
    /// 
    /// Value: Description
    /// - 0: Disabled
    /// - 1: Auto-publish enabled (default)
    pub KeyManagementServiceDnsPublishing: Option<bool>,
    /// Indicates the thread priority status of the KMS. The following values are possible.
    /// 
    /// Value: Description
    /// - 0: Normal priority (default)
    /// - 1: Low priority
    pub KeyManagementServiceLowPriority: Option<bool>,
    /// Indicates the caching status of the KMS host name and port. The following values are possible.
    /// 
    /// Value: Description
    /// - 0: Caching disabled
    /// - 1: Caching enabled (default)
    pub KeyManagementServiceHostCaching: Option<bool>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 0 (Unlicensed).
    pub KeyManagementServiceUnlicensedRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 1 (Licensed).
    pub KeyManagementServiceLicensedRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 2 (OOBGrace).
    pub KeyManagementServiceOOBGraceRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 3 (OOTGrace).
    pub KeyManagementServiceOOTGraceRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 4 (NonGenuineGrace).
    pub KeyManagementServiceNonGenuineGraceRequests: Option<u32>,
    /// Specifies the total count of valid KMS requests.
    pub KeyManagementServiceTotalRequests: Option<u32>,
    /// Specifies the total count of failed KMS requests.
    pub KeyManagementServiceFailedRequests: Option<u32>,
    /// Specifies the count of KMS requests from clients with `LicenseStatus` set to 5 (Notification).
    pub KeyManagementServiceNotificationRequests: Option<u32>,
    /// Specifies the ID of the token-based activation license that activated the computer.
    pub TokenActivationILID: Option<String>,
    /// Specifies the version of the token-based activation license that activated the computer.
    pub TokenActivationILVID: Option<u32>,
    /// Specifies the grant number in the token-based activation license that activated the computer.
    pub TokenActivationGrantNumber: Option<u32>,
    /// Specifies the thumbprint of the certificate that activated the computer.
    pub TokenActivationCertificateThumbprint: Option<String>,
    /// Specifies additional information for token-based activation.
    pub TokenActivationAdditionalInfo: Option<String>,
    // /// Indicates whether the volume activation through key management service is disabled.
    // pub KeyManagementServiceActivationDisabled: Option<bool>,
}

/// This class exposes properties of installed token-based activation licenses.
/// 
/// <https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingtokenactivationlicense>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SoftwareLicensingTokenActivationLicense {
    /// Specifies a GUID that is used by the Software Licensing service to uniquely identify an XRML 
    /// license.
    pub ID: Option<String>,
    /// Specifies a GUID that is used to identify the IL to the customer. The ILID is not unique, unless 
    /// combined with the ILVID.
    pub ILID: Option<String>,
    /// Specifies a version number that is used along with the ILID to allow customers to version their 
    /// licenses.
    pub ILVID: Option<u32>,
    /// Specifies an HRESULT returned from the issuance license (IL) authorization.
    pub AuthorizationStatus: Option<u32>,
    /// Specifies a UTC datetime after which the IL cannot be used for token activation.
    pub ExpirationDate: Option<WMIDateTime>,
    /// Specifies optional text provided by the customer and included in the IL.
    pub Description: Option<String>,
    /// Specifies optional text to provide additional metadata.
    pub AdditionalInfo: Option<String>,
}

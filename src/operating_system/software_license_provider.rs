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

/// Represents the state of Windows SoftwareLicensingProducts
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SoftwareLicensingProducts {
    /// Represents data stored in a Windows SoftwareLicensingProducts
    pub software_licensing_products: Vec<SoftwareLicensingProduct>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(SoftwareLicensingProducts, software_licensing_products);

/// This class exposes the product-specific properties and methods of the Software Licensing service.
/// 
/// <https://learn.microsoft.com/en-gb/previous-versions/windows/desktop/sppwmi/softwarelicensingproduct>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
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

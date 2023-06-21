//! The Telephony subcategory groups classes that represent "plain old telephone" modem devices and their associated serial connections.
//! 
//! | Class                                                               | Description                                                                                                                                |
//! |---------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_POTSModem**](win32-potsmodem)                         | Represents the services and characteristics of a Plain Old Telephone Service (POTS) modem on a computer system running Windows.<br/> |
//! | [**Win32\_POTSModemToSerialPort**](win32-potsmodemtoserialport) | Relates a modem and the serial port the modem uses.<br/>                                                                             |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows user's POTSModems
#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
pub struct POTSModems {
    /// Sequence of windows POTSModems states
    pub pot_modems: Vec<Win32_POTSModem>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(POTSModems, pot_modems);

/// The `Win32_POTSModem` WMI class represents the services and characteristics of a Plain Old 
/// Telephone Service (POTS) modem on a computer system running Windows.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-potsmodem>
#[derive(Default, Deserialize, Serialize, Debug, Clone, Hash)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_POTSModem {
    /// Current auto-answer or callback setting for the modem.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `Disabled` (2)
    /// - `Manual Answer` (3)
    /// - `Auto Answer` (4)
    /// - `Auto Answer with Call-Back` (5)
    pub AnswerMode: Option<u16>,
    /// Port to which the POTS modem is attached.
    /// 
    /// Example: "COM1"
    pub AttachedTo: Option<String>,
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
    /// Command string used to detect a dial tone before dialing.
    /// 
    /// Example: "X4"
    pub BlindOff: Option<String>,
    /// Command string used to dial whether or not there is a dial tone.
    /// 
    /// Example: "X3"
    pub BlindOn: Option<String>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// All modem connection protocols with which this modem device is compatible.
    pub CompatibilityFlags: Option<String>,
    /// Data compression characteristics of the modem.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1): Unknown
    /// - `No Compression` (2): Other
    /// - `MNP 5` (3): No Compression
    /// - `V.42bis` (4): MNP 5
    /// - `5`: V.42bis
    pub CompressionInfo: Option<u16>,
    /// Command string used to disable hardware data compression.
    /// 
    /// Example: "S46=136"
    pub CompressionOff: Option<String>,
    /// Command string used to enable hardware data compression.
    /// 
    /// Example: "S46=138"
    pub CompressionOn: Option<String>,
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
    /// Modem initialization string. This property is made up of command strings from other 
    /// properties of this class.
    pub ConfigurationDialog: Option<String>,
    /// Array of countries/regions in which the modem can operate.
    pub CountriesSupported: Option<Vec<String>>,
    /// Country/region for which the modem is currently programmed. When multiple countries/
    /// regions are supported, this property defines which one is currently selected for use.
    pub CountrySelected: Option<String>,
    /// Name of the first concrete class to appear in the inheritance chain used in the 
    /// creation of an instance. When used with the other key properties of the class, the 
    /// property allows all instances of this class and its subclasses to be uniquely 
    /// identified.
    pub CreationClassName: Option<String>,
    /// List of currently defined passwords for the modem. This array may be left blank for 
    /// security reasons.
    pub CurrentPasswords: Option<Vec<String>>,
    /// Control settings for a serial communications device, in this case, the modem device.
    pub DCB: Option<Vec<u8>>,
    /// If `TRUE`, this POTS modem is the default modem on the computer system running Windows.
    pub Default: Option<Vec<u8>>,
    /// Description of the object.
    pub Description: Option<String>,
    /// Unique identifier of this POTS modem from other devices on the system.
    pub DeviceID: Option<String>,
    /// Name of the device loader for the modem. A device loader loads and manages device 
    /// drivers and enumerators for a given device.
    pub DeviceLoader: Option<String>,
    /// Physical type of the modem.
    /// 
    /// The values are:
    /// - `Null Modem` ("Null Modem")
    /// - `Internal Modem` ("Internal Modem")
    /// - `External Modem` ("External Modem")
    /// - `PCMCIA Modem` ("PCMCIA Modem")
    /// - `Unknown` ("Unknown")
    pub DeviceType: Option<String>,
    /// Type of dialing method used.
    /// 
    /// - `Unknown` (0)
    /// - `Tone` (1)
    /// - `Pulse` (2)
    pub DialType: Option<u16>,
    /// Date of the modem driver.
    pub DriverDate: Option<WMIDateTime>,
    /// If `TRUE`, the error reported in LastErrorCode is now cleared.
    pub ErrorCleared: Option<bool>,
    /// Command string used to enable error correction control when establishing a connection. 
    /// This increases the reliability of the connection.
    /// 
    /// Example: "+Q5S36=4S48=7"
    pub ErrorControlForced: Option<String>,
    /// Error correction characteristics of the modem.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `No Error Correction` (2)
    /// - `MNP 4` (3)
    /// - `LAPM` (4)
    pub ErrorControlInfo: Option<u16>,
    /// Command string used to disable error control.
    /// 
    /// Example: "+Q6S36=3S48=128"
    pub ErrorControlOff: Option<String>,
    /// Command string used to enable error control.
    /// 
    /// Example: "+Q5S36=7S48=7"
    pub ErrorControlOn: Option<String>,
    /// More information about the error recorded in `LastErrorCode`, and information on any 
    /// corrective actions that may be taken.
    pub ErrorDescription: Option<String>,
    /// Command string used to enable hardware flow control. Flow control consists of signals 
    /// sent between computers that verify that both computers are ready to transmit or receive 
    /// data.
    /// 
    /// Example: "&K1"
    pub FlowControlHard: Option<String>,
    /// Command string used to disable flow control. Flow control consists of signals sent 
    /// between computers that verify that both computers are ready to transmit or receive 
    /// data.
    /// 
    /// Example: "&K0"
    pub FlowControlOff: Option<String>,
    /// Command string used to enable software flow control. Flow control consists of signals 
    /// sent between computers that verify that both computers are ready to transmit or 
    /// receive data.
    /// 
    /// Example: "&K2"
    pub FlowControlSoft: Option<String>,
    /// Multiplier used with the `InactivityTimeout` property to calculate the timeout period of 
    /// a connection.
    pub InactivityScale: Option<String>,
    /// Time limit (in seconds) for automatic disconnection of the phone line, if no data is 
    /// exchanged. A value of 0 (zero) indicates that this feature is present but not enabled.
    pub InactivityTimeout: Option<u32>,
    /// Index number for this POTS modem.
    /// 
    /// Example: 0
    pub Index: Option<u32>,
    /// The device instance ID for this POTS modem.
    /// 
    /// Example: "1&08"
    /// 
    /// Windows Server 2012 R2, Windows 8.1, Windows Server 2012, Windows 8, Windows Server 2008 
    /// R2, Windows 7, Windows Server 2008 and Windows Vista: This property is available beginning 
    /// with Windows Server 2016 and Windows 10.
    pub IndexEx: Option<String>,
    /// Date and time the object was installed. This property does not need a value to indicate 
    /// that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Last error code reported by the logical device.
    pub LastErrorCode: Option<u32>,
    /// Maximum settable communication speed for accessing the phone system.
    pub MaxBaudRateToPhone: Option<u32>,
    /// Maximum settable communication speed to the COM port for an external modem. Enter 0 
    /// (zero) if not applicable.
    pub MaxBaudRateToSerialPort: Option<u32>,
    /// Number of passwords definable in the modem itself. If this feature is not supported, 
    /// enter 0 (zero).
    pub MaxNumberOfPasswords: Option<u16>,
    /// Model of this POTS modem.
    /// 
    /// Example: "Sportster 56K External"
    pub Model: Option<String>,
    /// Path to this modem's .inf file. This file contains initialization information for the 
    /// modem and its driver.
    /// 
    /// Example: "C:\Windows\INF"
    pub ModemInfPath: Option<String>,
    /// Name of the section in the modem's .inf file that contains information about the modem.
    pub ModemInfSection: Option<String>,
    /// Command string used to instruct the modem to use Bell modulations for 300 and 1200 bps.
    /// 
    /// Example: "B1"
    pub ModulationBell: Option<String>,
    /// Command string used to instruct the modem to use CCITT modulations for 300 and 1200 bps.
    /// 
    /// Example: "B0"
    pub ModulationCCITT: Option<String>,
    /// Modulation scheme of the modem.
    /// 
    /// - `Unknown `(0)
    /// - `Other `(1)
    /// - `Not Supported `(2)
    /// - `Bell 103 `(3)
    /// - `Bell 212A `(4)
    /// - `V.22bis `(5)
    /// - `V.32 `(6)
    /// - `V.32bis `(7)
    /// - `V.turbo `(8)
    /// - `V.FC `(9)
    /// - `V.34 `(10)
    /// - `V.34bis` (11)
    pub ModulationScheme: Option<u16>,
    /// Label by which the object is known. When subclassed, the property can be overridden 
    /// to be a key property.
    pub Name: Option<String>,
    /// Windows Plug and Play device identifier of the logical device.
    /// 
    /// Example: "*PNP030b"
    pub PNPDeviceID: Option<String>,
    /// Definition of the port used for this modem.
    /// 
    /// - ("00"): Parallel Port
    /// - ("01"): Serial Port
    /// - ("02"): Modem
    pub PortSubClass: Option<String>,
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
    /// Dialing prefix used to access an outside line.
    pub Prefix: Option<String>,
    /// List of all the properties (and their values) for this modem.
    pub Properties: Option<Vec<u8>>,
    /// Network path to the computer that provides the modem services.
    pub ProviderName: Option<String>,
    /// Command string used to instruct the modem to use pulse mode for dialing. Pulse dialing 
    /// is necessary for phone lines that are unable to handle tone dialing.
    pub Pulse: Option<String>,
    /// Command string used to reset the modem for the next call.
    /// 
    /// Example: "AT&F"
    pub Reset: Option<String>,
    /// Response this modem might report to the operating system during the connection process. 
    /// The first two characters specify the type of response. The second two characters specify 
    /// information about the connection being made. The second two characters are used only for 
    /// Negotiation Progress or Connect response codes. The next eight characters specify the 
    /// modem-to-modem line speed negotiated in bits per second (bps). The characters represent 
    /// a 32-bit unsigned long integer format (byte and word reversed). The last eight characters 
    /// indicate that the modem is changing to a different port or Data Terminal Equipment (DTE) 
    /// speed. Usually this field is not used because modems make connections at a locked port 
    /// speed regardless of the modem-to-modem or Data Communications Equipment (DCE) speed.
    pub ResponsesKeyName: Option<String>,
    /// Number of rings before the modem answers an incoming call.
    pub RingsBeforeAnswer: Option<u8>,
    /// Command string used to turn the modem speaker on after dialing a number, and turning 
    /// the speaker off when a connection has been established.
    /// 
    /// Example: "M1"
    pub SpeakerModeDial: Option<String>,
    /// Command string used to turn the modem speaker off.
    /// 
    /// Example: "M0"
    pub SpeakerModeOff: Option<String>,
    /// Command string used to turn the modem speaker on.
    /// 
    /// Example: "M2"
    pub SpeakerModeOn: Option<String>,
    /// Command string used to instruct the modem to turn the speaker on (until a connection 
    /// is established).
    /// 
    /// Example: "M3"
    pub SpeakerModeSetup: Option<String>,
    /// Command string used to set the modem speaker to the highest volume.
    /// 
    /// Example: "L3"
    pub SpeakerVolumeHigh: Option<String>,
    /// Describes the volume level of the audible tones from the modem.
    /// 
    /// - `Unknown` (0)
    /// - `Other` (1)
    /// - `Not Supported` (2)
    /// - `High` (3)
    /// - `Medium` (4)
    /// - `Low` (5)
    /// - `Off` (6)
    /// - `Auto` (7)
    pub SpeakerVolumeInfo: Option<u16>,
    /// Command string used to set the modem speaker to the lowest volume.
    /// 
    /// Example: "L1"
    pub SpeakerVolumeLow: Option<String>,
    /// Command string used to set the modem speaker to a medium volume.
    /// 
    /// Example: "L2"
    pub SpeakerVolumeMed: Option<String>,
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
    /// Type of characters used for text passed through the modem.
    /// 
    /// The values are:
    /// - `ASCII string format` ("ASCII string format")
    /// - `DBCS string format` ("DBCS string format")
    /// - `UNICODE string format` ("UNICODE string format")
    pub StringFormat: Option<String>,
    /// If TRUE, the modem supports callback.
    pub SupportsCallback: Option<bool>,
    /// If TRUE, synchronous, as well as asynchronous, communication is supported.
    pub SupportsSynchronousConnect: Option<bool>,
    /// Value of the scoping computer's CreationClassName property.
    pub SystemCreationClassName: Option<String>,
    /// Name of the scoping system.
    pub SystemName: Option<String>,
    /// String that marks the end of a command string.
    /// 
    /// Example: "<cr"
    pub Terminator: Option<String>,
    /// Date and time the modem was last reset.
    pub TimeOfLastReset: Option<WMIDateTime>,
    /// Command string that instructs the modem to use tone mode for dialing. The phone line 
    /// must support tone dialing.
    /// 
    /// Example: "T"
    pub Tone: Option<String>,
    /// Command strings used to activate the voice capabilities of a voice modem.
    /// 
    /// Example: "AT+V"
    pub VoiceSwitchFeature: Option<String>,
}
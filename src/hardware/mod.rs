//! [Computer System Hardware Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes)
//! 
//! The Computer System Hardware category groups classes together that represent hardware related 
//! objects. Examples include input devices, hard disks, expansion cards, video devices, networking 
//! devices, and system power.
//! 
//! The Hardware System category is grouped into the following subcategories:
//! 
//! - [Cooling Device Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#cooling-device-classes)
//! - [Input Device Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#input-device-classes)
//! - [Mass Storage Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#mass-storage-classes)
//! - [Motherboard, Controller, and Port Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#motherboard-controller-and-port-classes) (don't do)
//! - [Networking Device Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#networking-device-classes)
//! - [Power Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#power-classes)
//! - [Printing Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#printing-classes) (don't do)
//! - [Telephony Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#telephony-classes)
//! - [Video and Monitor Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/computer-system-hardware-classes#video-and-monitor-classes)

pub mod cooling_device;
pub mod input_device;
pub mod mass_storage;
pub mod networking_device;
pub mod telephony;
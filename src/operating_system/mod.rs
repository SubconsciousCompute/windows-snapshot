//! [Operating System Classes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes)
//!
//! The Operating System category groups classes that represent operating system related objects.
//! They denote the various configurations and settings that define a computing environment.
//! Examples include: the boot configuration, Component Object Model (COM) settings, desktop
//! environment settings, drivers, security settings, user settings, and registry settings.
//!
//! The Operating System category is grouped into the following subcategories:
//!
//! - [COM](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#com)
//! - [Desktop](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#desktop)
//! - [Drivers](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#drivers)
//! - [Event log](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#windows-event-log)
//! - [File system](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#file-system)
//! - [Job objects](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#job-objects)
//! - [Memory and page files](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#memory-and-page-files)
//! - [Multimedia audio or visual](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#multimedia-audio-or-visual)
//! - [Networking](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#networking)
//! - [Operating system events](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#operating-system-events)
//! - [Operating system settings](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#operating-system-settings)
//! - [Processes](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#processes)
//! - [Registry](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#registry)
//! - [Scheduler jobs](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#scheduler-jobs)
//! - [Security](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#security)
//! - [Services](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#services)
//! - [Shares](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#shares)
//! - [Start menu](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#start-menu)
//! - [Storage](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#storage)
//! - [Users](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#users)
//! - [Windows product activation](https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/operating-system-classes#windows-product-activation)

pub mod desktop;
pub mod drivers;
pub mod file_system;
pub mod processes;
pub mod registry;
pub mod services;
pub mod users;
pub mod event_log;
pub mod memory_and_page_files;
pub mod scheduler_jobs;
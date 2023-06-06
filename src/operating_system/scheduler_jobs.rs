//! | Class                                                | Description                                                                                                                                                                                                                                                           |
//! |------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_CurrentTime**](/previous-versions/windows/desktop/wmitimepprov/win32-currenttime) | Abstract class<br/> Represents an instance in time as component seconds, minutes, day of the week, and so on.<br/>                                                                                                                                        |
//! | [**Win32\_ScheduledJob**](win32-scheduledjob)      | Instance class<br/> Represents a job scheduled using the Windows schedule service.<br/>                                                                                                                                                                   |
//! | [**Win32\_LocalTime**](/previous-versions/windows/desktop/wmitimepprov/win32-localtime)     | Instance class<br/> Represents a point in time returned as [**Win32\_LocalTime**](/previous-versions/windows/desktop/wmitimepprov/win32-localtime) objects that result from a query. The **Hour** property is returned as the local time in a 24-hour clock.<br/>                                |
//! | [**Win32\_UTCTime**](/previous-versions/windows/desktop/wmitimepprov/win32-utctime)         | Instance class<br/> Represents a point in time that is returned as [**Win32\_UTCTime**](/previous-versions/windows/desktop/wmitimepprov/win32-utctime) objects that result from a query. The **Hour** property is returned as the coordinated universal time (UTC) time in a 24 hour clock.<br/> |

use crate::update;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows `ScheduledJobs`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScheduledJobs {
    /// Represents sequence of Windows `ScheduledJobs`
    pub scheduled_jobs: Vec<Win32_ScheduledJob>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(ScheduledJobs, scheduled_jobs);

/// Represents the state of Windows `LocalTimes`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalTimes {
    /// Represents sequence of Windows `LocalTimes`
    pub local_times: Vec<Win32_LocalTime>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(LocalTimes, local_times);

/// Represents the state of Windows `UTCTimes`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UTCTimes {
    /// Represents sequence of Windows `UTCTimes`
    pub utc_times: Vec<Win32_UTCTime>,
    /// When was the record last updated
    pub last_updated: SystemTime,
    /// Signifies change in state
    /// 
    /// - TRUE : The state changed since last UPDATE
    /// - FALSE : The state is the same as last UPDATE
    pub state_change: bool,
}

update!(UTCTimes, utc_times);

/// The `Win32_ScheduledJob` WMI class represents a job created with the `AT` command.
/// 
/// Note: The `Win32_ScheduledJob` class does not represent a job created with the Scheduled Task Wizard 
/// from the Control Panel. You cannot change a task created by WMI in the Scheduled Tasks UI. 
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-scheduledjob>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ScheduledJob {
    /// A short textual description of the object.
    pub Caption: Option<String>,
    /// A textual description of the object.
    pub Description: Option<String>,
    /// Indicates when the object was installed. Lack of a value does not indicate that the object is 
    /// not installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Label by which the object is known. When subclassed, this property can be overridden to be a 
    /// key property.
    pub Name: Option<String>,
    /// String that indicates the current status of the object. Operational and non-operational status 
    /// can be defined. Operational status can include "OK", "Degraded", and "Pred Fail". "Pred Fail" 
    /// indicates that an element is functioning properly, but is predicting a failure (for example, a 
    /// SMART-enabled hard disk drive).
    /// 
    /// Non-operational status can include "Error", "Starting", "Stopping", and "Service". "Service" can 
    /// apply during disk mirror-resilvering, reloading a user permissions list, or other administrative 
    /// work. Not all such work is online, but the managed element is neither "OK" nor in one of the 
    /// other states.
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
    /// Length of time the job has been executing.
    pub ElapsedTime: Option<WMIDateTime>,
    /// User is notified upon job completion or failure.
    pub Notify: Option<String>,
    /// User who submitted the job.
    pub Owner: Option<String>,
    /// Importance of a job's execution.
    pub Priority: Option<u32>,
    /// Time that the job was submitted.
    pub TimeSubmitted: Option<WMIDateTime>,
    /// Time at which the job is invalid or should be stopped.
    pub UntilTime: Option<WMIDateTime>,
    /// Name of the command, batch program, or binary file (and command-line arguments) that the schedule 
    /// service uses to invoke the job.
    /// 
    /// Example: "defrag /q/f"
    pub Command: Option<String>,
    /// Days of the month when the job is scheduled to run. If a job is scheduled to run on multiple days 
    /// of the month, these values can be joined in a logical OR. For example, if a job is to run on the 
    /// 1st and 16th of each month, the value of the `DaysOfMonth` property would be 1 OR 32768.
    /// 
    /// - `1` (1), 1st
    /// - `2` (2), 2nd
    /// - `3` (4), 3rd
    /// - `4` (8), 4th
    /// - `5` (16), 5th
    /// - `6` (32), 6th
    /// - `7` (64), 7th
    /// - `8` (128), 8th
    /// - `9` (256), 9th
    /// - `10` (512), 10th
    /// - `11` (1024), 11th
    /// - `12` (2048), 12th
    /// - `13` (4096), 13th
    /// - `14` (8192), 14th
    /// - `15` (16384), 15th
    /// - `16` (32768), 16th
    /// - `17` (65536), 17th
    /// - `18` (131072), 18th
    /// - `19` (262144), 19th
    /// - `20` (524288), 20th
    /// - `21` (1048576), 21st
    /// - `22` (2097152), 22nd
    /// - `23` (4194304), 23rd
    /// - `24` (8388608), 24th
    /// - `25` (16777216), 25th
    /// - `26` (33554432), 26th
    /// - `27` (67108864), 27th
    /// - `28` (134217728), 28th
    /// - `29` (268435456), 29th
    /// - `30` (536870912), 30th
    /// - `31` (1073741824), 31st
    pub DaysOfMonth: Option<u32>,
    /// Days of the week when a job is scheduled to run. If a job is scheduled to run on multiple days 
    /// of the week, the values can be joined in a logical OR. For example, if a job is scheduled to 
    /// run on Mondays, Wednesdays, and Fridays the value of the `DaysOfWeek` property would be 1 OR 4 OR 16.
    /// 
    /// - `Monday` (1)
    /// - `Tuesday` (2)
    /// - `Wednesday` (4)
    /// - `Thursday` (8)
    /// - `Friday` (16)
    /// - `Saturday` (32)
    /// - `Sunday` (64)
    pub DaysOfWeek: Option<u32>,
    /// Specified job is interactive, which means that a user can give input to a scheduled job while 
    /// it is executing.
    pub InteractWithDesktop: Option<bool>,
    /// Identifying number of the job. It is used by methods as a handle to one job being scheduled on 
    /// this computer.
    pub JobId: Option<u32>,
    /// Status of execution the last time this job was scheduled to run.
    /// - `Success` ("Success")
    /// - `Failure` ("Failure")
    pub JobStatus: Option<String>,
    /// Scheduled job runs repeatedly on the days that the job is scheduled. If `False`, then the job is 
    /// run one time.
    pub RunRepeatedly: Option<bool>,
    /// UTC time to run the job, in the form of "YYYYMMDDHHMMSS.MMMMMM(+-)OOO", where "YYYYMMDD" must be 
    /// replaced by "********". The replacement is necessary because the scheduling service only allows 
    /// jobs to be configured to run one time, or run on a day of the month or week. A job cannot be run 
    /// on a specific date.
    /// 
    /// The "(+-)OOO" section of the `StartTime` property value is the current bias for local time translation. 
    /// The bias is the difference between the UTC time and local time. To calculate the bias for your time 
    /// zone, multiply the number of hours that your time zone is ahead or behind Greenwich Mean Time (GMT) 
    /// by 60 (use a positive number for the number of hours if your time zone is ahead of GMT and a negative 
    /// number if your time zone is behind GMT). Add an additional 60 to your calculation if your time zone 
    /// is using daylight savings time. For example, the Pacific Standard Time zone is eight hours behind GMT, 
    /// therefore the bias is equals to -420 (-8 * 60 + 60) when daylight savings time is in use and -480 
    /// (-8 * 60) when daylight savings time is not in use. You can also determine the value of the bias by 
    /// querying the bias property of the Win32_TimeZone class.
    /// 
    /// For example: "********123000.000000-420" specifies 14.30 (2:30 P.M.) PST with daylight savings time 
    /// in effect.
    pub StartTime: Option<WMIDateTime>,
}

/// The `Win32_LocalTime` WMI class describes a point in time returned as `Win32_LocalTime` objects that result 
/// from a query. These are returned as the value for the `TargetInstance` property in the `__InstanceModificationEvent` 
/// system class. The Hour property is returned as the local time on a 24-hour clock.
/// 
/// Note: The smallest time segment supported is 1 second.
///
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmitimepprov/win32-localtime>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_LocalTime {
    /// Current day that matches the query (1 31).
    pub Day: Option<u32>,
    /// Current day of the current week that matches the query (0 6). By convention, the value 0 is always Sunday, 
    /// regardless of the culture or the locale set on the machine.
    pub DayOfWeek: Option<u32>,
    /// Current hour of the current day (0 23).
    pub Hour: Option<u32>,
    /// Not implemented.
    pub Milliseconds: Option<u32>,
    /// Current minute (0 59).
    pub Minute: Option<u32>,
    /// Current month that matches the query (1 12).
    pub Month: Option<u32>,
    /// Current quarter of the current year (1 4).
    pub Quarter: Option<u32>,
    /// Current second of the current minute (0 59).
    pub Second: Option<u32>,
    /// Current week (1 6) in the current month (1 12).
    pub WeekInMonth: Option<u32>,
    /// Current year that matches the query (4 digits).
    pub Year: Option<u32>,
}

/// The `Win32_UTCTimeWMI` class describes a point in time that is returned as `Win32_UTCTime` objects 
/// that result from a query. These are returned as the value for the `TargetInstance` property in the 
/// `__InstanceModificationEvent` system class. The Hour property is returned as the Coordinated 
/// Universal Time (UTC) time on a 24 hour clock.
/// 
/// Note: The smallest time segment supported is a second.
/// 
/// <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/wmitimepprov/win32-utctime>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_UTCTime {
    /// Current day that matches the query (1 31).
    pub Day: Option<u32>,
    /// Current day of the current week that matches the query (0 6). By convention, the value 0 (zero) 
    /// is always Sunday, regardless of the culture or the locale set on the machine.
    pub DayOfWeek: Option<u32>,
    /// Current hour of the current day (0 23).
    pub Hour: Option<u32>,
    /// Not implemented.
    pub Milliseconds: Option<u32>,
    /// Current minute (0 59).
    pub Minute: Option<u32>,
    /// Current month that matches the query (1 12).
    pub Month: Option<u32>,
    /// Current quarter of the current year (1 4).
    pub Quarter: Option<u32>,
    /// Current second of the current minute (0 59).
    pub Second: Option<u32>,
    /// Current week in the current month (1 6).
    pub WeekInMonth: Option<u32>,
    /// Current year matching the query (4 digits).
    pub Year: Option<u32>,
}
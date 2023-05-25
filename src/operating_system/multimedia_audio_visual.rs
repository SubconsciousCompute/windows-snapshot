//! The class in the Multimedia Audio or Visual subcategory represents properties of the audio or video codec installed on the computer system.
//! 
//! | Class                                       | Description                                                                                                |
//! |---------------------------------------------|------------------------------------------------------------------------------------------------------------|
//! | [**Win32\_CodecFile**](win32-codecfile) | Instance class<br/> Represents the audio or video codec installed on the computer system.<br/> |

use crate::{update};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

/// Represents the state of Windows CodecFiles
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CodecFiles {
    /// Represents sequence of Windows `CodecFiles`
    pub codec_files: Vec<Win32_CodecFile>,
    /// When was the record last updated
    pub last_updated: SystemTime,
}

update!(CodecFiles, codec_files);

/// The `Win32_CodecFile` WMI class represents the audio or video codec installed on the computer 
/// system. Codecs convert one media format type to another, typically a compressed format to an 
/// uncompressed format. The name "codec" is derived from a combination of compress and decompress. 
/// For example, a codec can convert a compressed format, such as MS-ADPCM, to an uncompressed 
/// format such as PCM, which most audio hardware can play directly.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-codecfile>
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_CodecFile {
    /// Bitmask that represents the access rights required to access or perform specific operations 
    /// on the codec file. For bit values, see File and Directory Access Rights Constants.
    /// 
    /// Note: On FAT volumes, the FULL_ACCESS value is returned instead, which indicates no security 
    /// has been set on the object.
    /// 
    /// `FILE_READ_DATA` (file) or `FILE_LIST_DIRECTORY` (directory) (1)
    /// `FILE_WRITE_DATA` (file) or `FILE_ADD_FILE` (directory) (2)
    /// `FILE_APPEND_DATA` (file) or `FILE_ADD_SUBDIRECTORY` (directory) (4)
    /// `FILE_READ_EA` (8)
    /// `FILE_WRITE_EA` (16)
    /// `FILE_EXECUTE` (file) or `FILE_TRAVERSE` (directory) (32)
    /// `FILE_DELETE_CHILD` (directory) (64)
    /// `FILE_READ_ATTRIBUTES` (128)
    /// `FILE_WRITE_ATTRIBUTES` (256)
    /// `DELETE` (65536)
    /// `READ_CONTROL` (131072)
    /// `WRITE_DAC` (262144)
    /// `WRITE_OWNER` (524288)
    /// `SYNCHRONIZE` (1048576)
    pub AccessMask: Option<u32>,
    /// If `True`, the file should be archived.
    pub Archive: Option<bool>,
    /// Short description of the object.
    pub Caption: Option<String>,
    /// If `True`, the file is compressed.
    pub Compressed: Option<bool>,
    /// Algorithm or tool used to compress the logical file. If it is not possible (or not desired) to 
    /// describe the compression scheme (perhaps because it is not known), use the following words: 
    /// "Unknown" to represent that it is not known whether the logical file is compressed or not; 
    /// "Compressed" to represent that the file is compressed but either its compression scheme is not 
    /// known or not disclosed; and "Not Compressed" to represent that the logical file is not compressed.
    pub CompressionMethod: Option<String>,
    /// Name of the first concrete class to appear in the inheritance chain used in the creation of an 
    /// instance. When used with the other key properties of the class, the property allows all instances 
    /// of this class and its subclasses to be uniquely identified.
    pub CreationClassName: Option<String>,
    /// File creation date.
    pub CreationDate: Option<WMIDateTime>,
    /// Class of the computer system.
    pub CSCreationClassName: Option<String>,
    /// String representing the name of the computer system.
    pub CSName: Option<String>,
    /// Full name of the codec driver. This string is intended to be displayed in large (descriptive) spaces.
    /// 
    /// Example: "Microsoft PCM Converter"
    pub Description: Option<String>,
    /// Drive letter (including colon) of the file.
    /// 
    /// Example: "c:"
    pub Drive: Option<String>,
    /// DOS-compatible file name for this file.
    /// 
    /// Example: "c:\progra~1"
    pub EightDotThreeFileName: Option<String>,
    /// If `True`, the file is encrypted.
    pub Encrypted: Option<bool>,
    /// Algorithm or tool used to encrypt the logical file. If it is not possible (or not desired) to 
    /// describe the encryption scheme (perhaps for security reasons), use the following words: "Unknown" 
    /// to represent that it is not known whether the logical file is encrypted or not; "Encrypted" to 
    /// represent that the file is encrypted but either its encryption scheme is not known or not disclosed; 
    /// and "Not Encrypted" to represent that the logical file is not encrypted.
    pub EncryptionMethod: Option<String>,
    /// File name extension (without the dot).
    /// 
    /// Examples: "txt", "mof", "mdb"
    pub Extension: Option<String>,
    /// Name (without the extension) of the file.
    /// 
    /// Example: "autoexec"
    pub FileName: Option<String>,
    /// Size of the file (in bytes).
    pub FileSize: Option<u64>,
    /// File type (indicated by the `Extension` property).
    pub FileType: Option<String>,
    /// Class of the file system.
    pub FSCreationClassName: Option<String>,
    /// Name of the file system.
    pub FSName: Option<String>,
    /// Codec represented by this class.
    /// 
    /// The values are:
    /// - "Audio"
    /// - "Video"
    /// 
    /// `Audio` ("Audio")
    /// 
    /// `Video` ("Video")
    pub Group: Option<String>,
    /// If `True`, the file is hidden.
    pub Hidden: Option<bool>,
    /// Object was installed. This property does not require a value to indicate that the object is installed.
    pub InstallDate: Option<WMIDateTime>,
    /// Number of "file opens" that are currently active against the file.
    pub InUseCount: Option<u64>,
    /// File was last accessed.
    pub LastAccessed: Option<WMIDateTime>,
    /// File was last modified.
    pub LastModified: Option<WMIDateTime>,
    /// Manufacturer string from version resource, if one is present.
    pub Manufacturer: Option<String>,
    /// Inherited name that serves as a key of a logical file instance within a file system. Full path names 
    /// should be provided.
    /// 
    /// Example: "C:\Windows\system\win.ini"
    pub Name: Option<String>,
    /// Path of the file. This includes leading and trailing backslashes.
    /// 
    /// Example: "\windows\system\"
    pub Path: Option<String>,
    /// File can be read.
    pub Readable: Option<bool>,
    /// Current status of the object. Various operational and nonoperational statuses can be defined. Operational statuses include: "OK", "Degraded", and "Pred Fail" (an element, such as a SMART-enabled hard disk drive, may be functioning properly but predicting a failure in the near future). Nonoperational statuses include: "Error", "Starting", "Stopping", and "Service". The latter, "Service", could apply during mirror-resilvering of a disk, reload of a user permissions list, or other administrative work. Not all such work is online, yet the managed element is neither "OK" nor in one of the other states.
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
    /// If `True`, the file is a system file.
    pub System: Option<bool>,
    /// Version string from version resource, if one is present.
    pub Version: Option<String>,
    /// If `True`, the file can be written.
    pub Writeable: Option<bool>,
}

//! # windows-snapshot
//!
//! The aim is to provide a snapshot of Windows system asynchronously. It achieves this by using `WMI`.
//!
//! Our global state is `windows_snapshot::state::Windows` which stores the states of Windows machine at any given instance.
//! Each state member can be updated synchronously or asynchronously, alone or alongside other states.
//!
//! Example:
//!
//! ```rust
//! use windows_snapshot::COMLibrary;
//!
//! #[tokio::main]
//! async fn main() {
//!     let _com_con = COMLibrary::new().unwrap(); // initialise security context
//!
//!     let mut k = windows_snapshot::state::Windows::default();
//!
//!     k.async_update().await;
//!     //k.update(); // for synchronous update
//!
//!     println!("{k:#?}");
//! }
//! ```

pub mod operating_system;
pub mod state;

pub use wmi::COMLibrary;

/// Macro to automatically make `update` and `async_update` for a given state field
#[macro_export]
macro_rules! update {
    ($struct_name: ident, $struct_field: ident) => {
        impl $struct_name {
            /// Update fields synchronously
            pub fn update(&mut self) {
                let com_con = unsafe { COMLibrary::assume_initialized() };

                let wmi_con = WMIConnection::new(com_con).unwrap();

                self.last_updated = SystemTime::now();
                self.$struct_field = wmi_con.query().unwrap();
            }

            /// Update fields asynchronously
            pub async fn async_update(&mut self) {
                let com_con = unsafe { COMLibrary::assume_initialized() };

                let wmi_con = WMIConnection::new(com_con).unwrap();

                self.last_updated = SystemTime::now();
                self.$struct_field = wmi_con.async_query().await.unwrap();
            }
        }

        impl Default for $struct_name {
            /// `last_updated` defaults to the the current `SystemTime`
            fn default() -> Self {
                $struct_name {
                    $struct_field: Default::default(),
                    last_updated: SystemTime::now(),
                }
            }
        }
    };
}

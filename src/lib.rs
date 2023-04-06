pub mod operating_system;
pub mod state;

#[macro_export]
macro_rules! update {
    ($struct_field: ident) => {
        pub fn update(&mut self) {
            let com_con = unsafe { COMLibrary::assume_initialized() };

            let wmi_con = WMIConnection::new(com_con).unwrap();

            self.last_updated = SystemTime::now();
            self.$struct_field = wmi_con.query().unwrap();
        }

        pub async fn async_update(&mut self) {
            let com_con = unsafe { COMLibrary::assume_initialized() };

            let wmi_con = WMIConnection::new(com_con).unwrap();

            self.last_updated = SystemTime::now();
            self.$struct_field = wmi_con.async_query().await.unwrap();
        }
    };
}

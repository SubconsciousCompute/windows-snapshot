// cargo run --bin get_fields --release

use wmi::*;
use std::collections::HashMap;
use wmi::Variant;

pub fn main() {
    let wmi_con = WMIConnection::new(COMLibrary::new().unwrap()).unwrap();
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_ServerConnection").unwrap();

    print!("{:?}", results);
}
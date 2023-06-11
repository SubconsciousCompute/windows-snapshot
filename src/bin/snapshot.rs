// cargo run --bin snapshot --release

use windows_snapshot::COMLibrary;
use std::time::SystemTime;
use wmi::WMIConnection;
use std::collections::hash_map::DefaultHasher;
use windows_snapshot::{Hash, Hasher};

#[tokio::main]
async fn main() {
    let _com_con = COMLibrary::new().unwrap(); // initialise security context

    let mut k = windows_snapshot::state::Windows::default();

    //k.async_update().await;
    //k.update(); // for synchronous update

    // println!("{k:#?}");
    // k.async_update().await;
    // k.startup_commands.update();

    let wmi_con = WMIConnection::new(_com_con).unwrap();

    k.desktops.last_updated = SystemTime::now();
    k.desktops.desktops = wmi_con.async_query().await.unwrap();

    let mut hasher = DefaultHasher::new();
    k.desktops.desktops.hash(&mut hasher);

    println!("{:#?}", k.desktops);
    println!("hash: {:?}", hasher.finish());
}

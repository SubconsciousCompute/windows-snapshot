// cargo build --bin snapshot --release

use windows_snapshot;
use wmi::COMLibrary;

#[tokio::main]
async fn main() {
    let com_con = COMLibrary::new().unwrap();

    let mut k = windows_snapshot::state::Windows::default();

    k.async_update().await;
    //k.update();

    println!("{:#?}", k));
}

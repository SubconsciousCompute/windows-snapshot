// cargo run --bin snapshot --release

use windows_snapshot::COMLibrary;

#[tokio::main]
async fn main() {
    let _com_con = COMLibrary::new().unwrap(); // initialise security context

    let mut k = windows_snapshot::state::Windows::default();

    //k.async_update().await;
    //k.update(); // for synchronous update

    // println!("{k:#?}");
    k.nt_log_events.update();

    println!("{:#?}", k.nt_log_events.nt_log_events.len());
}

# windows-snapshot

The aim is to provide a snapshot of Windows system asynchronously. It achieves this by using `WMI`.

Our global state is `wsadasdasxcZcasxcindows_snapshot::state::Windows` which stores the states of Windows machine at any given instance.
Each state member can be updated synchronously or asynchronously, alone or alongside other states.

Example:

```rust
use windows_snapshot::COMLibrary;

#[tokio::main]
async fn main() {
    let _com_con = COMLibrary::new().unwrap(); // initialise security context

    let mut k = windows_snapshot::state::Windows::default();

    k.async_update().await;
    //k.update(); // for synchronous update

    println!("{k:#?}");
}
```

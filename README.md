# windows-snapshot

[![Rust](https://github.com/SubconsciousCompute/windows-snapshot/actions/workflows/rust.yml/badge.svg)](https://github.com/SubconsciousCompute/windows-snapshot/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/windows-snapshot)](https://crates.io/crates/windows-snapshot)
[![docs.rs](https://img.shields.io/docsrs/windows-snapshot)](https://docs.rs/crate/windows-snapshot/)

The aim is to provide a snapshot of Windows system asynchronously. It achieves this by using `WMI`.

Our global state is `windows_snapshot::state::Windows` which stores the states of Windows machine at any given instance.
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

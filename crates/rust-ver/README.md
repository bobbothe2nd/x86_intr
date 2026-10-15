# LLVM Version (`rust-ver`)

Parse LLVM version output from `rustc` and provides functions to configure an API around it.

## Usage

In `build.rs`:

```rust
use rust_ver::{build::{rerun_if_rustc_changed, cfg_llvm_threshold, cfg_rust_channel, cfg_rust_threshold}, ver::Version};

fn main() {
    rerun_if_rustc_changed();

    cfg_llvm_threshold(
        "llvm_at_least_24",
        Version::new(24, 0, 0),
    ).unwrap();

    cfg_rust_threshold(
        "rust_at_least_1_90",
        Version::new(1, 90, 0),
    ).unwrap();

    cfg_llvm_ver("llvm_ver")?;
    cfg_rust_ver("rust_ver")?;

    env_llvm_ver("LLVM_VERSION")?;
    env_rust_ver("RUST_VERSION")?;

    cfg_rust_channel("rust_channel").unwrap();
}
```

Then use it like this:

```rust
const LLVM_VERSION: &str = env!("LLVM_VERSION");
const RUST_VERSION: &str = env!("RUST_VERSION");

#[cfg(llvm_at_least_24)]
mod llvm_24;

#[cfg(not(llvm_at_least_24))]
mod llvm_old;

#[cfg(rust_channel = "nightly")]
fn nightly_only() {
    // ...
}

#[cfg(rust_at_least_1_90)]
fn implementation() {
    // ...
}

#[cfg(llvm_ver = "23.1.1")]
fn llvm_2311() {
    // ...
}

#[cfg(rust_ver = "1.99.0")]
fn rust_199() {
    // ...
}
```

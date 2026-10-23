//! Build-time utilities for detecting compile-time Rust/LLVM version

use crate::{
    Error, ver::{ToVersion, Version, llvm_ver, rust_channel, rust_ver},
};

/// Creates a `cfg(name)` true only if the detected LLVM is at least `threshold`
pub fn cfg_llvm_threshold<V: ToVersion>(name: &str, threshold: V) -> Result<(), Error> {
    println!("cargo:rustc-check-cfg=cfg({name})");

    let version = llvm_ver()?;

    if version >= threshold.to_version()? {
        println!("cargo:rustc-cfg={name}");
    }

    Ok(())
}

/// Creates a `cfg(name)` with the value of the detected LLVM version
///
/// Emits the exact version, not a compatibility check.
pub fn cfg_llvm_ver(name: &str) -> Result<Version, Error> {
    println!("cargo:rustc-check-cfg=cfg({name}, values(any()))");

    let version = llvm_ver()?;

    println!("cargo:rustc-cfg={name}=\"{version}\"");

    Ok(version)
}

/// Creates a compile-time environment variable `name` with the value of the detected LLVM version
pub fn env_llvm_ver(name: &str) -> Result<Version, Error> {
    let version = llvm_ver()?;

    println!("cargo:rustc-env={name}={version}");

    Ok(version)
}

/// Creates a `cfg(name)` true only if the detected `rustc` is at least `threshold`
pub fn cfg_rust_threshold<V: ToVersion>(name: &str, threshold: V) -> Result<(), Error> {
    println!("cargo:rustc-check-cfg=cfg({name})");

    let version = rust_ver()?;

    if version >= threshold.to_version()? {
        println!("cargo:rustc-cfg={name}");
    }

    Ok(())
}

/// Creates a `cfg(name)` with the value of the detected `rustc` version
///
/// Emits the exact version, not a compatibility check.
pub fn cfg_rust_ver(name: &str) -> Result<Version, Error> {
    println!("cargo:rustc-check-cfg=cfg({name}, values(any()))");

    let version = rust_ver()?;

    println!("cargo:rustc-cfg={name}=\"{version}\"");

    Ok(version)
}

/// Creates a compile-time environment variable `name` with the value of the detected `rustc` version
pub fn env_rust_ver(name: &str) -> Result<Version, Error> {
    let version = rust_ver()?;

    println!("cargo:rustc-env={name}={version}");

    Ok(version)
}

/// Creates a `cfg(name)` with the value of the detected `rustc` channel
pub fn cfg_rust_channel(name: &str) -> Result<(), Error> {
    println!("cargo:rustc-check-cfg=cfg({name}, values(\"nightly\", \"stable\", \"beta\", \"dev\"))");

    let channel = rust_channel()?;

    println!("cargo:rustc-cfg={name}=\"{channel}\"");

    Ok(())
}

/// Improves the correctness of build scripts that detect the Rust build environment.
///
/// This tells cargo to rerun the build script if the environment variable `RUSTC` changes.
pub fn rerun_if_rustc_changed() {
    println!("cargo:rerun-if-env-changed=RUSTC");
}

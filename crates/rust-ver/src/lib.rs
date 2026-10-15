//! Check Rust/LLVM versions at build time

#![forbid(missing_docs)]

pub mod build;

pub mod ver;

/// Generic error type for `rust-ver`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// if the target fails to execute `rustc`
    RustcFailed,

    /// if `rustc --version --verbose` outputs an invalid value
    InvalidOutput,

    /// if the parsed version is invalid
    UnsupportedVersion,

    /// if the parsed channel is invalid
    UnsupportedChannel,
}

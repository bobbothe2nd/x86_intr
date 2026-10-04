//! Implementing missing intrinsics from `core::arch`
//!
//! Derived partly from `core_arch/missing-x86.md`

#![allow(
    non_snake_case,
    non_camel_case_types,
    clippy::missing_safety_doc,
)]
#![forbid(missing_docs)]
#![no_std]

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

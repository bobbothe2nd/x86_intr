//! Atomically test CPUID:0.0 and CPUID:8000'0000.0 to efficiently cache vendor and leaf.
//!
//! Used internally by macro

#[cfg(all(target_has_atomic = "32", target_has_atomic = "8"))]
mod atomic;

#[cfg(not(all(target_has_atomic = "32", target_has_atomic = "8")))]
mod nonatomic;

#[cfg(all(target_has_atomic = "32", target_has_atomic = "8"))]
pub use atomic::*;

#[cfg(not(all(target_has_atomic = "32", target_has_atomic = "8")))]
pub use nonatomic::*;

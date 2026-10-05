use core::arch::asm;

use crate::x86_64::{split_u64, u32x2};

/// Serialize instruction execution, ensuring all modifications to flags, registers, and memory by previous instructions are completed before the next instruction is fetched.
///
/// Requires `serialize`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_serialize)
#[inline(always)]
pub unsafe fn _serialize() {
    unsafe {
        asm!("serialize");
    }
}

/// Sets up a hardware monitored address range containing `p`.
///
/// Address range should be writeback memory caching type.
///
/// Requires `waitpkg`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_umonitor)
#[inline(always)]
pub unsafe fn _umonitor(p: *const u8) {
    unsafe {
        asm!(
            "umonitor {p}",
            p = in(reg) p,
            options(nostack, preserves_flags, readonly),
        );
    }
}

/// Consumes monitor and awaits either TSC reaches deadline or write to monitored address.
///
/// Bit 0 of `control` selects between a lower power (cleared) or faster wakeup (set).
///
/// Requires `waitpkg`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_umwait)
#[inline(always)]
pub unsafe fn _umwait(control: u32, deadline: u64) {
    let u32x2 { lo, hi } = split_u64(deadline);

    unsafe {
        asm!(
            "umwait {control:e}",
            control = in(reg) control,
            in("eax") lo,
            in("edx") hi,
            options(nostack, readonly),
        );
    }
}

/// Sets up a hardware monitored address range containing `p`.
///
/// Address range should be writeback memory caching type.
///
/// Requires `waitpkg`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_monitor)
#[inline(always)]
pub unsafe fn _mm_monitor(p: *const u8, extensions: u32, hints: u32) {
    unsafe {
        asm!(
            "monitor",
            in("rax") p,
            in("ecx") extensions,
            in("edx") hints,
            options(nostack, preserves_flags),
        );
    }
}

/// Consumes monitor and awaits either TSC reaches deadline or write to monitored address.
///
/// Requires `monitor`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mwait)
#[inline(always)]
pub unsafe fn _mm_mwait(extensions: u32, hints: u32) {
    unsafe {
        asm!(
            "mwait",
            in("ecx") extensions,
            in("eax") hints,
            options(nostack, preserves_flags, readonly),
        );
    }
}

/// Sets up a hardware monitored address range containing `p`.
///
/// Requires `monitorx`
#[inline(always)]
pub unsafe fn _monitorx(p: *const u8, extensions: u32, hints: u32) {
    unsafe {
        asm!(
            "monitorx",
            in("rax") p,
            in("ecx") extensions,
            in("edx") hints,
            options(nostack, preserves_flags),
        );
    }
}
 
/// Sleeps until TSC reaches `timeout`.
///
/// Bit 0 of `control` selects between a lower power (cleared) or faster wakeup (set).
///
/// Requires `waitpkg`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tpause)
#[inline(always)]
pub unsafe fn _tpause(control: u32, timeout: u64) {
    let u32x2 { lo, hi } = split_u64(timeout);

    unsafe {
        asm!(
            "tpause {control:e}",
            control = in(reg) control,
            in("eax") lo,
            in("edx") hi,
            options(nostack, nomem),
        );
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use crate::is_cpuid_feature_detected;

    use super::*;

    #[test]
    fn serialize_if_supported() {
        if !is_cpuid_feature_detected!("serialize") {
            return;
        }

        let value = 0u64;

        unsafe {
            _umonitor((&raw const value).cast::<u8>());
        }

        let start = unsafe { core::arch::x86_64::_rdtsc() };

        let timeout = start + 1000;

        unsafe {
            _tpause(0, timeout);
        }
    }

    #[test]
    fn umonitor_if_supported() {
        if !is_cpuid_feature_detected!("waitpkg") {
            return;
        }

        let value = 0u64;

        unsafe {
            _umonitor((&raw const value).cast::<u8>());
        }
    }

    #[test]
    fn umwait_if_supported() {
        if !is_cpuid_feature_detected!("waitpkg") {
            return;
        }

        let start = unsafe { core::arch::x86_64::_rdtsc() };

        let timeout = start + 10000;

        unsafe {
            _umwait(0, timeout);
        }
    }

    #[test]
    fn monitorx_if_supported() {
        if !is_cpuid_feature_detected!("monitorx") {
            return;
        }

        let value = 0u64;

        unsafe {
            _monitorx((&raw const value).cast::<u8>(), 0, 0);
        }
    }

    #[test]
    fn tpause_if_supported() {
        if !is_cpuid_feature_detected!("waitpkg") {
            return;
        }

        let start = unsafe { core::arch::x86_64::_rdtsc() };

        let timeout = start + 10000;

        unsafe {
            _tpause(0, timeout);
        }
    }
}

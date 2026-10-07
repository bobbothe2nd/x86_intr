use core::arch::asm;

/// Enqueues command pointed to by `src` into `dst`.
///
/// Requires `enqcmd` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_enqcmd)
#[inline(always)]
pub unsafe fn _enqcmd(dst: *mut u64, src: *const u64) {
    unsafe {
        asm!(
            "enqcmd {dst:r}, [{src:r}]",
            dst = in(reg) dst,
            src = in(reg) src,
            options(nostack, preserves_flags),
        );
    }
}

/// Enqueues command pointed to by `src` into `dst`.
///
/// Requires `enqcmd` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_enqcmds)
#[inline(always)]
pub unsafe fn _enqcmds(dst: *mut u64, src: *const u64) {
    unsafe {
        asm!(
            "enqcmds {dst:r}, [{src:r}]",
            dst = in(reg) dst,
            src = in(reg) src,
            options(nostack, preserves_flags),
        );
    }
}

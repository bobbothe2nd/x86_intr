use core::arch::asm;

/// Reads the processor ID.
///
/// Requires `rdpid`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdpid)
#[inline(always)]
pub unsafe fn _rdpid() -> u32 {
    let mut pid;

    unsafe {
        asm!(
            "rdpid {pid:r}",
            pid = out(reg) pid,
            options(nostack, preserves_flags)
        );
    }

    pid
}

/// Provides a hint to the processor to selectively reset the prediction history of the current logical processor.
///
/// If `hints` contains any bits not set in the `IA32_HRESET_ENABLE` MSR, this will cause a (#GP).
/// This is a privileged instruction.
///
/// Requires `hreset`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_hreset)
#[inline(always)]
pub unsafe fn _hreset<const HINTS: i32>() {
    unsafe {
        asm!(
            "hreset {HINTS}",
            HINTS = const HINTS,
            options(nostack, preserves_flags, readonly)
        );
    }
}

/// Invalidate mappings in the TLBs and paging-structure caches for the PCID.
///
/// Requires `rdpid`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_invpcid)
#[inline(always)]
pub unsafe fn _invpcid(typ: u32, descriptor: *mut u128) {
    unsafe {
        asm!(
            "invpcid {typ:e}, [{ptr:r}]",
            typ = in(reg) typ,
            ptr = in(reg) descriptor,
            options(nostack, preserves_flags)
        );
    }
}

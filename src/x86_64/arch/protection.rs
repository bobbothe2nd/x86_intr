use core::arch::asm;

/// Writes `val` into PKRU.
///
/// Requires `pku` and `ospke`.
#[inline(always)]
pub unsafe fn _wrpkru(val: u32) {
    unsafe {
        asm!(
            "wrpkru",
            in("eax") val,
            in("ecx") 0,
            in("edx") 0,
            options(nostack, preserves_flags, readonly)
        );
    }
}

/// Reads the value of PKRU.
///
/// Requires `pku` and `ospke`.
#[inline(always)]
pub unsafe fn _rdpkru() -> u32 {
    let val;

    unsafe {
        asm!(
            "rdpkru",
            out("eax") val,
            in("ecx") 0,
            options(nostack, preserves_flags, readonly)
        );
    }

    val
}

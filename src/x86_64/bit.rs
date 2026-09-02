use core::arch::asm;

/// Count the number of bits set to 1 in unsigned 32-bit integer `a`, and return that count.
///
/// Requires `popcnt`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_popcnt_u32)
#[inline(always)]
pub unsafe fn _mm_popcnt_u32(a: u32) -> i32 {
    let mut dst = 0;

    unsafe {
        asm!(
            "popcnt {dst:e} {src:e}",
            dst = inout(reg) dst,
            src = in(reg) a,
            options(nostack, preserves_flags)
        );
    }

    dst
}

/// Count the number of bits set to 1 in unsigned 64-bit integer `a`, and return that count.
///
/// Requires `popcnt`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_popcnt_u64)
#[inline(always)]
pub unsafe fn _mm_popcnt_u64(a: u64) -> i64 {
    let mut dst = 0;

    unsafe {
        asm!(
            "popcnt {dst:r} {src:r}",
            dst = inout(reg) dst,
            src = in(reg) a,
            options(nostack, preserves_flags)
        );
    }

    dst
}

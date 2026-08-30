use core::arch::asm;

/// Invalidates and flushes the cache line that contains `p` from all levels of the cache hierarchy.
///
/// Similar to [`_mm_clflush`](core::arch::x86_64::_mm_clflush), optimized for flushing cache lines in parallel.
/// Will be deprecated when `core::arch::x86_64::_mm_clflushopt` is stabilized.
///
/// Requires `clflushopt` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_clflushopt)
#[inline(always)]
pub unsafe fn _mm_clflushopt(p: *const u8) {
    unsafe {
        asm!(
            "clflushopt [{p}]",
            p = in(reg) p,
            options(nostack),
        );
    }
}

/// Writes back to memory the cache line (if modified) that contains the linear address specified with the memory operand from any level of the cache hierarchy in the cache coherence domain.
///
/// Requires `clwb` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_clwb)
#[inline(always)]
pub unsafe fn _mm_clwb(p: *const u8) {
    unsafe {
        asm!(
            "clwb [{p}]",
            p = in(reg) p,
            options(nostack, preserves_flags),
        );
    }
}

/// Hint to hardware to move the cache line containing m8 to a more distant level of the cache without writing back to memory.
///
/// Requires `cldemote` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cldemote)
#[inline(always)]
pub unsafe fn _mm_cldemote(p: *const u8) {
    unsafe {
        asm!(
            "cldemote [{p}]",
            p = in(reg) p,
            options(nostack, preserves_flags),
        );
    }
}

/// Hint to hardware to move the cache line containing m8 to a more distant level of the cache without writing back to memory.
///
/// Requires `clzero` feature
#[inline(always)]
pub unsafe fn _mm_clzero(p: *const u8) {
    unsafe {
        asm!(
            "clzero {p}",
            p = in(reg) p,
            options(nostack, preserves_flags),
        );
    }
}

/// Move `u32` directly and atomically
///
/// Requires `movdiri` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_directstoreu_u32)
#[inline(always)]
pub unsafe fn _directstoreu_u32(dst: *mut u32, val: u32) {
    unsafe {
        asm!(
            "movdiri [{dst:r}], {val:e}",
            dst = in(reg) dst,
            val = in(reg) val,
            options(nostack, preserves_flags),
        );
    }
}

/// Move `u64` directly and atomically
///
/// Requires `movdiri` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_directstoreu_u64)
#[inline(always)]
pub unsafe fn _directstoreu_u64(dst: *mut u64, val: u64) {
    unsafe {
        asm!(
            "movdiri [{dst:r}], {val:r}",
            dst = in(reg) dst,
            val = in(reg) val,
            options(nostack, preserves_flags),
        );
    }
}

/// Move 64 bytes directly and atomically
///
/// Requires `movdir64b` feature
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_directstoreu_u64)
#[inline(always)]
pub unsafe fn _movdir64b(dst: *mut u64, src: *const u8) {
    unsafe {
        asm!(
            "movdir64b {dst:r}, [{src:r}]",
            dst = in(reg) dst,
            src = in(reg) src,
            options(nostack, preserves_flags),
        );
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use crate::is_cpuid_feature_detected;

    use super::*;

    #[test]
    fn movdir64b_if_supported() {
        #[repr(align(512))]
        struct Aligned([u128; 4]);

        if !is_cpuid_feature_detected!("movdir64b") {
            return;
        }

        let mut dst = Aligned([0u128; 4]);
        let src = Aligned([12345u128; 4]);

        unsafe {
            _movdir64b((&raw mut dst).cast(), (&raw const src).cast());
        }

        assert_eq!(dst.0, src.0);
    }

    #[test]
    fn movdiri_if_supported() {
        if !is_cpuid_feature_detected!("movdiri") {
            return;
        }

        let mut dst = 0u32;
        let src = 1234u32;

        unsafe {
            _directstoreu_u32((&raw mut dst).cast(), src);
        }

        assert_eq!(dst, src);

        let mut dst = 0u64;
        let src = 1234u64;

        unsafe {
            _directstoreu_u64((&raw mut dst).cast(), src);
        }

        assert_eq!(dst, src);
    }

    #[test]
    fn cldemote_if_supported() {
        if !is_cpuid_feature_detected!("cldemote") {
            return;
        }

        let mut value = 0_u32;

        unsafe {
            _mm_cldemote((&raw const value).cast());
        }

        value = 123;

        assert_eq!(value, 123);
    }

    #[test]
    fn clwb_if_supported() {
        if !is_cpuid_feature_detected!("clwb") {
            return;
        }

        let mut value = 0_u32;

        unsafe {
            _mm_clwb((&raw const value).cast());
        }

        value = 123;

        assert_eq!(value, 123);
    }

    #[test]
    fn clflushopt_if_supported() {
        if !is_cpuid_feature_detected!("clflushopt") {
            return;
        }

        let value = 15_u32;

        unsafe {
            _mm_clflushopt((&raw const value).cast());
        }

        assert_eq!(value, 15);
    }

    #[test]
    fn clzero_if_supported() {
        if !is_cpuid_feature_detected!("clzero") {
            return;
        }

        let value = 15_u32;

        unsafe {
            _mm_clzero((&raw const value).cast());
        }

        assert_eq!(value, 15);
    }
}

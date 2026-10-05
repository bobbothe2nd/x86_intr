use core::arch::asm;

/// Read the FS segment base register and return the 32-bit result.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_readfsbase_u32)
#[inline(always)]
pub unsafe fn _readfsbase_u32() -> u32 {
    let mut dst;

    unsafe {
        asm!(
            "rdfsbase {dst:e}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Read the FS segment base register and return the 64-bit result.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_readfsbase_u64)
#[inline(always)]
pub unsafe fn _readfsbase_u64() -> u64 {
    let mut dst;

    unsafe {
        asm!(
            "rdfsbase {dst}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Read the GS segment base register and return the 32-bit result.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_readgsbase_u32)
#[inline(always)]
pub unsafe fn _readgsbase_u32() -> u32 {
    let mut dst;

    unsafe {
        asm!(
            "rdgsbase {dst:e}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Read the GS segment base register and return the 64-bit result.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_readgsbase_u64)
#[inline(always)]
pub unsafe fn _readgsbase_u64() -> u64 {
    let mut dst;

    unsafe {
        asm!(
            "rdgsbase {dst}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Write the unsigned 32-bit integer a to the FS segment base register.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_writefsbase_u32)
#[inline(always)]
pub unsafe fn _writefsbase_u32(a: u32) {
    unsafe {
        asm!(
            "wrfsbase {a:e}",
            a = in(reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Write the unsigned 64-bit integer a to the FS segment base register.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_writefsbase_u64)
#[inline(always)]
pub unsafe fn _writefsbase_u64(a: u64) {
    unsafe {
        asm!(
            "wrfsbase {a}",
            a = in(reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Write the unsigned 32-bit integer a to the GS segment base register.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_writegsbase_u32)
#[inline(always)]
pub unsafe fn _writegsbase_u32(a: u32) {
    unsafe {
        asm!(
            "wrgsbase {a:e}",
            a = in(reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Write the unsigned 64-bit integer a to the GS segment base register.
///
/// Requires `fsgsbase`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_writegsbase_u64)
#[inline(always)]
pub unsafe fn _writegsbase_u64(a: u64) {
    unsafe {
        asm!(
            "wrgsbase {a}",
            a = in(reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }
}

use core::arch::asm;

/// Load 16 bits from memory, perform a byte swap operation, and return the result.
///
/// Requires `movbe`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_loadbe_i16)
#[inline(always)]
pub unsafe fn _loadbe_i16(ptr: *const i16) -> i16 {
    let mut dst;

    unsafe {
        asm!(
            "movbe {dst:x}, word ptr [{ptr}]",
            dst = out(reg) dst,
            ptr = in(reg) ptr,
            options(nostack, preserves_flags)
        );
    }

    dst
}

/// Load 32 bits from memory, perform a byte swap operation, and return the result.
///
/// Requires `movbe`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_loadbe_i32)
#[inline(always)]
pub unsafe fn _loadbe_i32(ptr: *const i32) -> i32 {
    let mut dst;

    unsafe {
        asm!(
            "movbe {dst:e}, dword ptr [{ptr}]",
            dst = out(reg) dst,
            ptr = in(reg) ptr,
            options(nostack, preserves_flags)
        );
    }

    dst
}

/// Load 64 bits from memory, perform a byte swap operation, and return the result.
///
/// Requires `movbe`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_loadbe_i64)
#[inline(always)]
pub unsafe fn _loadbe_i64(ptr: *const i64) -> i64 {
    let mut dst;

    unsafe {
        asm!(
            "movbe {dst:r}, qword ptr [{ptr}]",
            dst = out(reg) dst,
            ptr = in(reg) ptr,
            options(nostack, preserves_flags)
        );
    }

    dst
}

/// Perform a bit swap operation of the 16 bits in data, and store the results to memory.
///
/// Requires `movbe`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_storebe_i16)
#[inline(always)]
pub unsafe fn _storebe_i16(ptr: *mut i16, data: i16) {
    unsafe {
        asm!(
            "movbe word ptr [{ptr}], {data:x}",
            data = in(reg) data,
            ptr = in(reg) ptr,
            options(nostack, preserves_flags)
        );
    }
}

/// Perform a bit swap operation of the 32 bits in data, and store the results to memory.
///
/// Requires `movbe`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_loadstorebe_i32)
#[inline(always)]
pub unsafe fn _storebe_i32(ptr: *mut i32, data: i32) {
    unsafe {
        asm!(
            "movbe dword ptr [{ptr}], {data:e}",
            data = in(reg) data,
            ptr = in(reg) ptr,
            options(nostack, preserves_flags)
        );
    }
}

/// Perform a bit swap operation of the 64 bits in data, and store the results to memory.
///
/// Requires `movbe`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_storebe_i64)
#[inline(always)]
pub unsafe fn _storebe_i64(ptr: *mut i64, data: i64) {
    unsafe {
        asm!(
            "movbe qword ptr [{ptr}], {data:r}",
            data = in(reg) data,
            ptr = in(reg) ptr,
            options(nostack, preserves_flags)
        );
    }
}

use core::arch::asm;

/// Mark shadow stack pointed to by `p` as not busy.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_clrssbsy)
#[inline(always)]
pub unsafe fn _clrssbsy(p: *mut u8) {
    unsafe {
        asm!(
            "clrssbsy [{p:r}]",
            p = in(reg) p,
            options(nostack)
        );
    }
}

polymorphic! {
    #[inline(always)]
    #[doc = concat!(
        "Read the current shadow stack pointer, and return the result.",
        "\n\n",
        "Requires `cet_ss`",
        "\n\n",
        "[Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_get_ssp)",
    )]
    pub unsafe fn _get_ssp() -> match {
        u32 => {
            let dst: u32;

            unsafe {
                asm!(
                    "rdsspd {dst:r}",
                    dst = out(reg) dst,
                    options(nostack, preserves_flags)
                );
            }

            dst
        }
        u64 => {
            let dst: u64;

            unsafe {
                asm!(
                    "rdsspq {dst:r}",
                    dst = out(reg) dst,
                    options(nostack, preserves_flags)
                );
            }

            dst
        }
    }
}

/// Read the low 32-bits of the current shadow stack pointer, and store the result in dst.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdsspd_i32)
#[inline(always)]
pub unsafe fn _rdsspd_i32() -> i32 {
    let mut dst = 0;

    unsafe {
        asm!(
            "rdsspd {dst:r}",
            dst = out(reg) dst,
            options(nostack, preserves_flags)
        );
    }

    dst
}

/// Read the current shadow stack pointer, and store the result in dst.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdsspq_i64)
#[inline(always)]
pub unsafe fn _rdsspq_i64() -> i64 {
    let dst;

    unsafe {
        asm!(
            "rdsspd {dst:r}",
            dst = out(reg) dst,
            options(nostack, preserves_flags)
        );
    }

    dst
}

/// Increment the shadow stack pointer by 4 times the value specified in bits \[7:0\] of a.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_inc_ssp)
#[inline(always)]
pub unsafe fn _inc_ssp(a: u32) {
    unsafe {
        asm!(
            "incsspd {a:r}",
            a = in(reg) a,
            options(nostack, preserves_flags)
        );
    }
}

/// Increment the shadow stack pointer by 4 times the value specified in bits \[7:0\] of a.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_incsspd)
#[inline(always)]
pub unsafe fn _incsspd(a: i32) {
    unsafe {
        asm!(
            "incsspd {a:r}",
            a = in(reg) a,
            options(nostack, preserves_flags)
        );
    }
}

/// Increment the shadow stack pointer by 8 times the value specified in bits \[7:0\] of a.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_incsspq)
#[inline(always)]
pub unsafe fn _incsspq(a: i32) {
    unsafe {
        asm!(
            "incsspq {a:r}",
            a = in(reg) a,
            options(nostack, preserves_flags)
        );
    }
}

/// Restore the saved shadow stack pointer from the shadow stack restore token previously created on shadow stack by [`_saveprevssp`].
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rstorssp)
#[inline(always)]
pub unsafe fn _rstorssp(p: *mut u8) {
    unsafe {
        asm!(
            "rstorssp [{p:r}]",
            p = in(reg) p,
            options(nostack)
        );
    }
}

/// Save the previous shadow stack pointer context.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_saveprevssp)
#[inline(always)]
pub unsafe fn _saveprevssp(p: *mut u8) {
    unsafe {
        asm!(
            "saveprevssp [{p:r}]",
            p = in(reg) p,
            options(nostack, preserves_flags)
        );
    }
}

/// Mark shadow stack pointed to by `IA32_PL0_SSP` as busy.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_setssbsy)
#[inline(always)]
pub unsafe fn _setssbsy() {
    unsafe {
        asm!("setssbsy", options(nostack, nomem, preserves_flags));
    }
}

/// Write 32-bit value in val to a shadow stack page in memory specified by p.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_wrssd)
#[inline(always)]
pub unsafe fn _wrssd(val: i32, p: *mut u8) {
    unsafe {
        asm!(
            "wrssd {val:e}, [{p:r}]",
            val = in(reg) val,
            p = in(reg) p,
            options(nostack, preserves_flags)
        );
    }
}

/// Write 64-bit value in val to a shadow stack page in memory specified by p.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_wrssq)
#[inline(always)]
pub unsafe fn _wrssq(val: i64, p: *mut u8) {
    unsafe {
        asm!(
            "wrssq {val:r}, [{p:r}]",
            val = in(reg) val,
            p = in(reg) p,
            options(nostack, preserves_flags)
        );
    }
}

/// Write 32-bit value in val to a user shadow stack page in memory specified by p.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_wrussd)
#[inline(always)]
pub unsafe fn _wrussd(val: i32, p: *mut u8) {
    unsafe {
        asm!(
            "wrussd {val:e}, [{p:r}]",
            val = in(reg) val,
            p = in(reg) p,
            options(nostack, preserves_flags)
        );
    }
}

/// Write 64-bit value in val to a user shadow stack page in memory specified by p.
///
/// Requires `cet_ss`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_wrussq)
#[inline(always)]
pub unsafe fn _wrussq(val: i64, p: *mut u8) {
    unsafe {
        asm!(
            "wrussq {val:r}, [{p:r}]",
            val = in(reg) val,
            p = in(reg) p,
            options(nostack, preserves_flags)
        );
    }
}

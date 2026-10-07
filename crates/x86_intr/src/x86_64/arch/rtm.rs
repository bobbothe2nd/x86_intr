use core::arch::asm;

/// Force an RTM abort. The EAX register is updated to reflect an XABORT instruction caused the abort, and the `imm8` parameter will be provided in bits [31:24] of EAX.
/// Following an RTM abort, the logical processor resumes execution at the fallback address computed through the outermost XBEGIN instruction.
///
/// Requires `rtm`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xabort)
#[inline(always)]
pub unsafe fn _xabort<const IMM8: u8>() {
    unsafe {
        asm!(
            "xabort {IMM8}",
            IMM8 = const IMM8,
            in("eax") (IMM8 as u32) << 24,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Specify the start of an RTM code region.
/// If the logical processor was not already in transactional execution, then this call causes the logical processor to transition into transactional execution.
/// On an RTM abort, the logical processor discards all architectural register and memory updates performed during the RTM execution,
/// restores architectural state, and starts execution beginning at the fallback address computed from the outermost XBEGIN instruction.
/// Return status of ~0 (0xFFFF) if continuing inside transaction; all other codes are aborts.
///
/// Requires `rtm`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xbegin)
// #[inline(always)]
pub unsafe fn _xbegin() -> u32 {
    let dst;

    unsafe {
        asm!(
            "xbegin 2f",
            "xor {dst:e}, {dst:e}",
            "jmp 3f",
            "2:",
            "mov {dst:e}, eax",
            "3:",
            dst = out(reg) dst,
            options(nostack, nomem)
        );
    }

    dst
}

/// Specify the end of an RTM code region.
/// If this corresponds to the outermost scope, the logical processor will attempt to commit the logical processor state atomically.
/// If the commit fails, the logical processor will perform an RTM abort.
///
/// Requires `rtm`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xend)
#[inline(always)]
pub unsafe fn _xend() {
    unsafe {
        asm!("xend", options(nostack, nomem, preserves_flags));
    }
}

/// Query the transactional execution status, return 1 if inside a transactionally executing RTM or HLE region, and return 0 otherwise.
///
/// Requires `rtm`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xtest)
#[inline(always)]
pub unsafe fn _xtest() -> u8 {
    let dst;

    unsafe {
        asm!(
            "xtest",
            "setnz {dst}",
            dst = out(reg_byte) dst,
            options(nostack, nomem)
        );
    }

    dst
}

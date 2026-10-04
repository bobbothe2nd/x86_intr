use core::arch::asm;

/// Consumes monitor and awaits either timeout or write to monitored address.
///
/// Bit 0 of `hints` selects between a lower power (cleared) or faster wakeup (set).
///
/// Bit 1 of `extensions` selects between a timeout and no timeout.
///
/// Requires `monitorx`
///
/// When linking the `c/mwaitx.c` shim fails (common on Windows), this function has to
/// avoid the builtin register allocator because of `bx` not being supported. This is
/// automatically handled by `build.rs`.
///
/// As a result of the C shim, [`mwaitx_asm`] can sometimes outperform this because
/// (on my system at least) the C intrinsic pushes `rbx` to the stack to save it's value
/// rather than allocating an additional GPR.
#[inline(never)]
pub unsafe fn _mwaitx(extensions: u32, hints: u32, timeout: u32) {
    #[cfg(mwaitx_intr)]
    unsafe {
        unsafe extern "C" {
            fn rust_mwaitx(
                extensions: u32,
                hints: u32,
                timeout: u32,
            );
        }

        rust_mwaitx(extensions, hints, timeout);
    }

    #[cfg(not(mwaitx_intr))]
    unsafe {
        mwaitx_asm(extensions, hints, timeout);
    }
}

/// Consumes monitor and awaits either timeout or write to monitored address.
///
/// Same API as [`_mwaitx`].
///
/// This intrinsic uses the `bx` register which is reserved by LLVM. Until that gets fixed,
/// the register is temporarily copied to another register, which has to be initialized to
/// zero due to another limitation of inline assembly. That is, you cant allocate uninitialized
/// variables to a register. Then, timeout is written to `ebx`. After the `mwaitx` instruction,
/// the temporary value gets copied back into `rbx` like nothing ever happened.
///
/// This obviously hurts performance. Thats why [`mwaitx_no_timeout`] exists.
#[inline(never)]
pub unsafe fn mwaitx_asm(extensions: u32, hints: u32, timeout: u32) {
    let _rbx: u64;

    unsafe {
        asm!(
            "mov {tmp:r}, rbx",
            "mov ebx, {timeout:e}",
            "mwaitx",
            "mov rbx, {tmp:r}",
            tmp = out(reg) _rbx,
            timeout = in(reg) timeout,
            in("eax") hints,
            in("ecx") extensions,
            options(nostack, preserves_flags),
        );
    }
}

/// Equivalent to `_mwaitx` with `extensions=2` but with better performance.
///
/// The difference is that this doesn't touch `bx`.
#[inline(never)]
pub unsafe fn mwaitx_no_timeout(hints: u32) {
    unsafe {
        asm!(
            "mwaitx",
            in("eax") hints,
            in("ecx") 2,
            options(nostack, preserves_flags),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::is_cpuid_feature_detected;

    #[test]
    fn mwaitx_if_supported() {
        if !is_cpuid_feature_detected!("monitorx") {
            return;
        }

        unsafe {
            super::_mwaitx(0, 0, 10_000);
        }
    }
}

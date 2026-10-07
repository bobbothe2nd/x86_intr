use core::arch::asm;

/// Clear the user interrupt flag (UIF).
///
/// Requires `uintr`.
#[inline(always)]
pub unsafe fn _clui() {
    unsafe {
        asm!("clui", options(nostack, nomem));
    }
}

/// Sets the user interrupt flag (UIF).
///
/// Requires `uintr`.
#[inline(always)]
pub unsafe fn _stui() {
    unsafe {
        asm!("stui", options(nostack, nomem));
    }
}

/// Send user interprocessor interrupts specified in `__a`.
///
/// Requires `tsxldtrk`.
#[inline(always)]
pub unsafe fn _senduipi(__a: u64) {
    unsafe {
        asm!(
            "senduipi {a:r}",
            a = in(reg) __a,
            options(nostack, nomem)
        );
    }
}

/// Store the current user interrupt flag (UIF) in unsigned 8-bit integer dst.
///
/// Requires `uintr`.
#[inline(always)]
pub unsafe fn _testui() {
    unsafe {
        asm!("testui", options(nostack, nomem));
    }
}

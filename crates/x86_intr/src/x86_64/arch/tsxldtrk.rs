use core::arch::asm;

/// Mark the end of a TSX (HLE/RTM) suspend load address tracking region.
///
/// Requires `tsxldtrk`.
#[inline(always)]
pub unsafe fn _xresldtrk() {
    unsafe {
        asm!("xresldtrk", options(nostack, nomem, preserves_flags));
    }
}

/// Mark the start of a TSX (HLE/RTM) suspend load address tracking region.
///
/// Requires `tsxldtrk`.
#[inline(always)]
pub unsafe fn _xsusldtrk() {
    unsafe {
        asm!("xsusldtrk", options(nostack, nomem, preserves_flags));
    }
}

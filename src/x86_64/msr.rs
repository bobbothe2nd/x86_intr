use core::arch::asm;

use crate::x86_64::{concat_u32, split_u64, u32x2};

/// Reads the contents of a 64-bit MSR specified in `__A` into `dst`.
///
/// Requires `user_msr` feature
#[inline(always)]
pub unsafe fn _urdmsr(__A: u64) -> u64 {
    let lo: u32;
    let hi: u32;

    unsafe {
        asm!(
            "urdmsr",
            in("ecx") __A as u32,
            out("eax") lo,
            out("edx") hi,
        );
    }

    concat_u32(lo, hi)
}

/// Writes the contents of `__B` into the 64-bit MSR specified in `__A`.
///
/// Requires `user_msr` feature
#[inline(always)]
pub unsafe fn _uwrmsr(__A: u64, __B: u64) {
    let u32x2 { lo, hi} = split_u64(__B);

    unsafe {
        asm!(
            "uwrmsr",
            in("ecx") __A as u32,
            in("eax") lo,
            in("edx") hi,
        );
    }
}

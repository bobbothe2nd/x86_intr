use core::arch::asm;

/// Atomically add a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aadd_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "aadd [{a}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically add a 64-bit value at memory operand `__A` and a 64-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aadd_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "aadd [{a}], {b}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically and a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _andd_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "andd [{a}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically and a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _andd_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "andd [{a}], {b}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically or a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aor_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "aor [{a}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically or a 64-bit value at memory operand `__A` and a 64-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aor_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "aor [{a}], {b}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically xor a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _axor_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "axor [{a}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

/// Atomically xor a 64-bit value at memory operand `__A` and a 64-bit `__B`, and store the result to the same memory location.
///
/// This is a relaxed atomic read-modify-write operation. It does not provide
/// acquire, release, or sequentially-consistent ordering for accesses to
/// other memory locations.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _axor_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "axor [{a}], {b}",
            a = in(reg) __A,
            b = in(reg) __B,
            options(nostack, preserves_flags),
        );
    }
}

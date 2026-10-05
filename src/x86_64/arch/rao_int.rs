use core::arch::asm;

/// Atomically add a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aadd_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "aadd [{a:r}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically add a 64-bit value at memory operand `__A` and a 64-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aadd_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "aadd [{a:r}], {b:r}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically and a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _andd_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "andd [{a:r}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically and a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _andd_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "andd [{a:r}], {b:r}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically or a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aor_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "aor [{a:r}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically or a 64-bit value at memory operand `__A` and a 64-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _aor_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "aor [{a:r}], {b:r}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically xor a 32-bit value at memory operand `__A` and a 32-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _axor_i32(__A: *mut i32, __B: i32) {
    unsafe {
        asm!(
            "axor [{a:r}], {b:e}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

/// Atomically xor a 64-bit value at memory operand `__A` and a 64-bit `__B`, and store the result to the same memory location.
///
/// Requires `rao_int`
#[inline(always)]
pub unsafe fn _axor_i64(__A: *mut i64, __B: i64) {
    unsafe {
        asm!(
            "axor [{a:r}], {b:r}",
            a = in(reg) __A,
            b = in(reg) __B,
        );
    }
}

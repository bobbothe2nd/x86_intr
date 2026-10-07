use core::{arch::asm, fmt};

use intr_gen::intr_asm;

use crate::x86_64::{ValidSimdReg, private::Sealed};

/// A zero-sized compile-time handle identifying MMX register `mmR`
#[derive(Clone, Copy)]
pub struct __m64<const R: u8>
where 
    Self: ValidSimdReg;

impl<const R: u8> Sealed for __m64<R>
where 
    Self: ValidSimdReg,
{}

impl ValidSimdReg for __m64<0> {}
impl ValidSimdReg for __m64<1> {}
impl ValidSimdReg for __m64<2> {}
impl ValidSimdReg for __m64<3> {}
impl ValidSimdReg for __m64<4> {}
impl ValidSimdReg for __m64<5> {}
impl ValidSimdReg for __m64<6> {}
impl ValidSimdReg for __m64<7> {}

impl<const R: u8> fmt::Debug for __m64<R>
where 
    Self: ValidSimdReg,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mm{R}")
    }
}

/// Add packed 16-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_pi16)
#[inline(always)]
pub unsafe fn _mm_add_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 32-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_pi32)
#[inline(always)]
pub unsafe fn _mm_add_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 8-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_pi8)
#[inline(always)]
pub unsafe fn _mm_add_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed signed 16-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_pi16)
#[inline(always)]
pub unsafe fn _mm_adds_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddsw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed signed 8-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_pi8)
#[inline(always)]
pub unsafe fn _mm_adds_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddsb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_pu16)
#[inline(always)]
pub unsafe fn _mm_adds_pu16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddusw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_pu8)
#[inline(always)]
pub unsafe fn _mm_adds_pu8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddusb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise AND of 64 bits (representing integer data) in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_and_si64)
#[inline(always)]
pub unsafe fn _mm_and_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pand mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise NOT of 64 bits (representing integer data) in `a` and then AND with `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_andnot_si64)
#[inline(always)]
pub unsafe fn _mm_andnot_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pandn mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 16-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_pi16)
#[inline(always)]
pub unsafe fn _mm_cmpeq_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpeqw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 32-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_pi32)
#[inline(always)]
pub unsafe fn _mm_cmpeq_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpeqd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_pi8)
#[inline(always)]
pub unsafe fn _mm_cmpeq_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpeqb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 16-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_pi16)
#[inline(always)]
pub unsafe fn _mm_cmpgt_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpgtw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 32-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_pi32)
#[inline(always)]
pub unsafe fn _mm_cmpgt_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpgtd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_pi8)
#[inline(always)]
pub unsafe fn _mm_cmpgt_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpgtb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtm64_si64)
#[inline(always)]
pub unsafe fn _mm_cvtm64_si64<const A: u8>(_a: __m64<A>) -> i64
where 
    __m64<A>: ValidSimdReg,
{
    let dst;

    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "movq {dst}, mm{A}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags),
        );
    }

    dst
}

/// Copy 32-bit integer `a` to the lower elements of `dst`, and zero the upper element of `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi32_si64)
#[inline(always)]
pub unsafe fn _mm_cvtsi32_si64<const DST: u8>(a: i32) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{DST} = DST[0..8],
            "movd mm{DST}, {src:e}",
            src = in(reg) a,
            options(nostack, nomem, preserves_flags),
        );
    }

    __m64
}

/// Copy 64-bit integer `a`to `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi64_m64)
#[inline(always)]
pub unsafe fn _mm_cvtsi64_m64<const DST: u8>(a: i64) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{DST} = DST[0..8],
            "movq mm{DST}, {src}",
            src = in(reg) a,
            options(nostack, nomem, preserves_flags),
        );
    }

    __m64
}

/// Empty the MMX state, which marks the x87 FPU registers as available for use by x87 instructions. This instruction must be used at the end of all MMX technology procedures.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_empty)
#[inline(always)]
pub unsafe fn _m_empty() {
    unsafe {
        asm!(
            "emms",
            lateout("mm0") _,
            lateout("mm1") _,
            lateout("mm2") _,
            lateout("mm3") _,
            lateout("mm4") _,
            lateout("mm5") _,
            lateout("mm6") _,
            lateout("mm7") _,
            options(nostack, nomem, preserves_flags),
        );
    }
}

/// Empty the MMX state, which marks the x87 FPU registers as available for use by x87 instructions. This instruction must be used at the end of all MMX technology procedures.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_empty)
#[inline(always)]
pub unsafe fn _mm_empty() {
    unsafe {
        _m_empty();
    }
}

/// Copy 32-bit integer `a` to the lower elements of `dst`, and zero the upper element of `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_from_int)
#[inline(always)]
pub unsafe fn _m_from_int<const DST: u8>(a: i32) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{DST} = DST[0..8],
            "movd mm{DST}, {src:e}",
            src = in(reg) a,
            options(nostack, nomem, preserves_flags),
        );
    }

    __m64
}

/// Copy 64-bit integer `a`to `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_from_int64)
#[inline(always)]
pub unsafe fn _m_from_int64<const DST: u8>(a: i64) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{DST} = DST[0..8],
            "movq mm{DST}, {src}",
            src = in(reg) a,
            options(nostack, nomem, preserves_flags),
        );
    }

    __m64
}

/// Multiply packed signed 16-bit integers in `a` and `b`, producing intermediate signed 32-bit integers. Horizontally add adjacent pairs of intermediate 32-bit integers, and pack the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_madd_pi16)
#[inline(always)]
pub unsafe fn _mm_madd_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pmaddwd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Multiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the high 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mulhi_pi16)
#[inline(always)]
pub unsafe fn _mm_mulhi_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pmulhw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Multiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the low 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mullo_pi16)
#[inline(always)]
pub unsafe fn _mm_mullo_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pmullw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Multiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the low 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_or_si64)
#[inline(always)]
pub unsafe fn _mm_or_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "por mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 16-bit integers from `a` and `b` to packed 8-bit integers using signed saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_packs_pi16)
#[inline(always)]
pub unsafe fn _mm_packs_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "packsswb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 32-bit integers from `a` and `b` to packed 16-bit integers using signed saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_packs_pi32)
#[inline(always)]
pub unsafe fn _mm_packs_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "packssdw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 16-bit integers from `a` and `b` to packed 8-bit integers using unsigned saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_packs_pu16)
#[inline(always)]
pub unsafe fn _mm_packs_pu16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "packuswb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 32-bit integers from `a` and `b` to packed 16-bit integers using signed saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_packssdw)
#[inline(always)]
pub unsafe fn _m_packssdw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "packssdw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 16-bit integers from `a` and `b` to packed 8-bit integers using signed saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_packsswb)
#[inline(always)]
pub unsafe fn _m_packsswb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "packsswb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 16-bit integers from `a` and `b` to packed 8-bit integers using unsigned saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_packuswb)
#[inline(always)]
pub unsafe fn _m_packuswb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "packuswb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 8-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddb)
#[inline(always)]
pub unsafe fn _m_paddb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 32-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddd)
#[inline(always)]
pub unsafe fn _m_paddd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 8-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddsb)
#[inline(always)]
pub unsafe fn _m_paddsb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddsb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 16-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddsw)
#[inline(always)]
pub unsafe fn _m_paddsw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddsw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 8-bit unsigned integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddusb)
#[inline(always)]
pub unsafe fn _m_paddusb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddusb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 16-bit unsigned integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddusw)
#[inline(always)]
pub unsafe fn _m_paddusw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddusw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Add packed 16-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_paddw)
#[inline(always)]
pub unsafe fn _m_paddw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "paddw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise AND of 64 bits (representing integer data) in `a` and `b`, and store the result in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pand)
#[inline(always)]
pub unsafe fn _m_pand<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pand mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise NOT of 64 bits (representing integer data) in `a` and then AND with `b`, and store the result in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pandn)
#[inline(always)]
pub unsafe fn _m_pandn<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pandn mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pcmpeqb)
#[inline(always)]
pub unsafe fn _m_pcmpeqb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpeqb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 32-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pcmpeqd)
#[inline(always)]
pub unsafe fn _m_pcmpeqd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpeqd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 16-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pcmpeqw)
#[inline(always)]
pub unsafe fn _m_pcmpeqw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpeqw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pcmpgtb)
#[inline(always)]
pub unsafe fn _m_pcmpgtb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpgtb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 32-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pcmpgtd)
#[inline(always)]
pub unsafe fn _m_pcmpgtd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpgtd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compare packed 16-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pcmpgtw)
#[inline(always)]
pub unsafe fn _m_pcmpgtw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pcmpgtw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Multiply packed signed 16-bit integers in `a` and `b`, producing intermediate signed 32-bit integers. Horizontally add adjacent pairs of intermediate 32-bit integers, and pack the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pmaddwd)
#[inline(always)]
pub unsafe fn _m_pmaddwd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pmaddwd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Multiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the high 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pmulhw)
#[inline(always)]
pub unsafe fn _m_pmulhw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pmulhw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Multiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the low 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pmullw)
#[inline(always)]
pub unsafe fn _m_pmullw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pmullw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise OR of 64 bits (representing integer data) in `a` and `b`, and store the result in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_por)
#[inline(always)]
pub unsafe fn _m_por<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "por mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pslld)
#[inline(always)]
pub unsafe fn _m_pslld<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pslld mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` left by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pslldi)
#[inline(always)]
pub unsafe fn _m_pslldi<const IMM8: u8, const A: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pslld mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psllq)
#[inline(always)]
pub unsafe fn _m_psllq<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllq mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` left by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psllqi)
#[inline(always)]
pub unsafe fn _m_psllqi<const IMM8: u8, const A: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllq mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 16-bit integers in `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psllw)
#[inline(always)]
pub unsafe fn _m_psllw<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllw mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 16-bit integers in `a` left by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psllwi)
#[inline(always)]
pub unsafe fn _m_psllwi<const IMM8: u8, const A: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllw mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 32-bit integers in `a` right by `count` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrad)
#[inline(always)]
pub unsafe fn _m_psrad<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrad mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 32-bit integers in `a` right by `imm8` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psradi)
#[inline(always)]
pub unsafe fn _m_psradi<const IMM8: u8, const A: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrad mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 16-bit integers in `a` right by `count` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psraw)
#[inline(always)]
pub unsafe fn _m_psraw<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psraw mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 16-bit integers in `a` right by `imm8` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrawi)
#[inline(always)]
pub unsafe fn _m_psrawi<const IMM8: u8, const A: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psraw mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrld)
#[inline(always)]
pub unsafe fn _m_psrld<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrld mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrldi)
#[inline(always)]
pub unsafe fn _m_psrldi<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrld mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` right by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrlq)
#[inline(always)]
pub unsafe fn _m_psrlq<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlq mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrlqi)
#[inline(always)]
pub unsafe fn _m_psrlqi<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlq mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrlw)
#[inline(always)]
pub unsafe fn _m_psrlw<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlw mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers `a` right by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psrlwi)
#[inline(always)]
pub unsafe fn _m_psrlwi<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlw mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubb)
#[inline(always)]
pub unsafe fn _m_psubb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubd)
#[inline(always)]
pub unsafe fn _m_psubd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubsb)
#[inline(always)]
pub unsafe fn _m_psubsb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubsb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubsw)
#[inline(always)]
pub unsafe fn _m_psubsw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubsw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubusb)
#[inline(always)]
pub unsafe fn _m_psubusb<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubusb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed unsigned 16-bit integers in `b` from packed unsigned 16-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubusw)
#[inline(always)]
pub unsafe fn _m_psubusw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubusw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_psubw)
#[inline(always)]
pub unsafe fn _m_psubw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 8-bit integers from the high half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_punpckhbw)
#[inline(always)]
pub unsafe fn _m_punpckhbw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckhbw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 32-bit integers from the high half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_punpckhdq)
#[inline(always)]
pub unsafe fn _m_punpckhdq<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckhdq mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 16-bit integers from the high half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_punpckhwd)
#[inline(always)]
pub unsafe fn _m_punpckhwd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckhwd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 8-bit integers from the low half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_punpcklbw)
#[inline(always)]
pub unsafe fn _m_punpcklbw<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpcklbw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 32-bit integers from the low half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_punpckldq)
#[inline(always)]
pub unsafe fn _m_punpckldq<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckldq mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 16-bit integers from the low half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_punpcklwd)
#[inline(always)]
pub unsafe fn _m_punpcklwd<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpcklwd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise XOR of 64 bits (representing integer data) in `a` and `b`, and store the result in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_pxor)
#[inline(always)]
pub unsafe fn _m_pxor<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pxor mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Return vector of type [`__m64`] with all elements set to zero.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setzero_si64)
#[inline(always)]
pub unsafe fn _mm_setzero_si64<const DST: u8>() -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{DST} = DST[0..8],
            "pxor mm{DST}, mm{DST}",
            options(nostack, nomem, preserves_flags),
        );
    }

    __m64
}

/// Shift packed 16-bit integers in `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sll_pi16)
#[inline(always)]
pub unsafe fn _mm_sll_pi16<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllw mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sll_pi32)
#[inline(always)]
pub unsafe fn _mm_sll_pi32<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pslld mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sll_pi64)
#[inline(always)]
pub unsafe fn _mm_sll_pi64<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllq mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers in `a` left by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_pi16)
#[inline(always)]
pub unsafe fn _mm_slli_pi16<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllw mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` left by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_pi32)
#[inline(always)]
pub unsafe fn _mm_slli_pi32<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pslld mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` left by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_pi64)
#[inline(always)]
pub unsafe fn _mm_slli_pi64<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psllq mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sra_pi16)
#[inline(always)]
pub unsafe fn _mm_sra_pi16<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psraw mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sra_pi32)
#[inline(always)]
pub unsafe fn _mm_sra_pi32<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrad mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srai_pi16)
#[inline(always)]
pub unsafe fn _mm_srai_pi16<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psraw mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in sign bits, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srai_pi32)
#[inline(always)]
pub unsafe fn _mm_srai_pi32<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrad mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srl_pi16)
#[inline(always)]
pub unsafe fn _mm_srl_pi16<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlw mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srl_pi32)
#[inline(always)]
pub unsafe fn _mm_srl_pi32<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrld mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srl_pi64)
#[inline(always)]
pub unsafe fn _mm_srl_pi64<const A: u8, const COUNT_REG: u8>(a: __m64<A>, _count: __m64<COUNT_REG>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<COUNT_REG>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlq mm{A}, mm{COUNT_REG}",
            COUNT_REG = const COUNT_REG,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 16-bit integers in `a` right by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_pi16)
#[inline(always)]
pub unsafe fn _mm_srli_pi16<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlw mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_pi32)
#[inline(always)]
pub unsafe fn _mm_srli_pi32<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrld mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Shift 64-bit integer `a` right by `count` while shifting in zeros, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_pi64)
#[inline(always)]
pub unsafe fn _mm_srli_pi64<const A: u8, const IMM8: u8>(a: __m64<A>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psrlq mm{A}, {IMM8}",
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_pi16)
#[inline(always)]
pub unsafe fn _mm_sub_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_pi32)
#[inline(always)]
pub unsafe fn _mm_sub_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_pi8)
#[inline(always)]
pub unsafe fn _mm_sub_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_pi16)
#[inline(always)]
pub unsafe fn _mm_subs_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubsw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_pi8)
#[inline(always)]
pub unsafe fn _mm_subs_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubsb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed unsigned 16-bit integers in `b` from packed unsigned 16-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_pu16)
#[inline(always)]
pub unsafe fn _mm_subs_pu16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubusw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit integers in `a` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_pu8)
#[inline(always)]
pub unsafe fn _mm_subs_pu8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "psubusb mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Copy the lower 32-bit integer in `a` to `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_to_int)
#[inline(always)]
pub unsafe fn _m_to_int<const A: u8>(_a: __m64<A>) -> i32
where 
    __m64<A>: ValidSimdReg,
{
    let dst;

    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "movd {dst:e}, mm{A}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags),
        );
    }

    dst
}

/// Copy 64-bit integer `a` to `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_m_to_int64)
#[inline(always)]
pub unsafe fn _m_to_int64<const A: u8>(_a: __m64<A>) -> i64
where 
    __m64<A>: ValidSimdReg,
{
    let dst;

    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "movq {dst:e}, mm{A}",
            dst = out(reg) dst,
            options(nostack, nomem, preserves_flags),
        );
    }

    dst
}

/// Unpack and interleave 16-bit integers from the high half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_pi16)
#[inline(always)]
pub unsafe fn _mm_unpackhi_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckhwd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 32-bit integers from the high half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_pi32)
#[inline(always)]
pub unsafe fn _mm_unpackhi_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckhdq mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 8-bit integers from the high half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_pi8)
#[inline(always)]
pub unsafe fn _mm_unpackhi_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckhbw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 16-bit integers from the low half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_pi16)
#[inline(always)]
pub unsafe fn _mm_unpacklo_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpcklwd mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 32-bit integers from the low half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_pi32)
#[inline(always)]
pub unsafe fn _mm_unpacklo_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpckldq mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Unpack and interleave 8-bit integers from the low half of `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_pi8)
#[inline(always)]
pub unsafe fn _mm_unpacklo_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "punpcklbw mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise XOR of 64 bits (representing integer data) in `a` and `b`, and store the result in `dst`.
///
/// Requires `mmx`
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_xor_si64)
#[inline(always)]
pub unsafe fn _mm_xor_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            mm{A} = A[0..8],
            "pxor mm{A}, mm{B}",
            B = const B,
            options(nostack, nomem, preserves_flags),
        );
    }

    a
}

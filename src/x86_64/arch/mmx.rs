use core::{arch::asm, fmt, marker::PhantomData};

use crate::x86_64::ValidSimdReg;

/// Represents an `mm` SIMD register (formatted `mm{R}`)
#[derive(Clone, Copy)]
pub struct __m64<const R: u8>(PhantomData<u64>)
where 
    Self: ValidSimdReg;

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

impl<const R: u8> __m64<R>
where 
    Self: ValidSimdReg,
{
    /// Allocates the register without initializing
    #[inline(always)]
    #[must_use = "Allocating a register is meaningless without using it"]
    pub const fn allocate() -> Self {
        Self(PhantomData)
    }
}

/// Add packed 16-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_add_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed 32-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_add_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddd mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed 8-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_add_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed signed 16-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_adds_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddsw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed signed 8-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_adds_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddsb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_adds_pu16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddusw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_adds_pu8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddusb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Add packed unsigned 8-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_and_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pand mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compute the bitwise NOT of 64 bits (representing integer data) in `a` and then AND with `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_andnot_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pandn mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 16-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cmpeq_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pcmpeqw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 32-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cmpeq_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pcmpeqd mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for equality, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cmpeq_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pcmpeqb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 16-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cmpgt_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pcmpgtw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 32-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cmpgt_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pcmpgtd mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cmpgt_pi8<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pcmpgtb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cvtm64_si64<const A: u8>(_a: __m64<A>) -> i64
where 
    __m64<A>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "movq {dst}, mm{A}",
            dst = out(reg) dst,
            A = const A,
            options(nostack, preserves_flags),
        );
    }

    dst
}

/// Copy 32-bit integer `a` to the lower elements of dst, and zero the upper element of `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cvtsi32_si64<const DST: u8>(a: i32) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "movd mm{DST}, {src:e}",
            src = in(reg) a,
            DST = const DST,
            options(nostack, preserves_flags),
        );
    }

    __m64::allocate()
}

/// Copy 64-bit integer `a`to `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_cvtsi64_m64<const DST: u8>(a: i64) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "movq mm{DST}, {src}",
            src = in(reg) a,
            DST = const DST,
            options(nostack, preserves_flags),
        );
    }

    __m64::allocate()
}

/// Empty the MMX state, which marks the x87 FPU registers as available for use by x87 instructions. This instruction must be used at the end of all MMX technology procedures.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _m_empty() {
    unsafe {
        asm!(
            "emms",
            options(nostack, nomem, preserves_flags),
        );
    }
}

/// Empty the MMX state, which marks the x87 FPU registers as available for use by x87 instructions. This instruction must be used at the end of all MMX technology procedures.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_empty() {
    unsafe {
        asm!(
            "emms",
            options(nostack, nomem, preserves_flags),
        );
    }
}

/// Copy 32-bit integer `a` to the lower elements of dst, and zero the upper element of `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _m_from_int<const DST: u8>(a: i32) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "movd mm{DST}, {src:e}",
            src = in(reg) a,
            DST = const DST,
            options(nostack, preserves_flags),
        );
    }

    __m64::allocate()
}

/// Copy 64-bit integer `a`to `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _m_from_int64<const DST: u8>(a: i64) -> __m64<DST>
where 
    __m64<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "movq mm{DST}, {src}",
            src = in(reg) a,
            DST = const DST,
            options(nostack, preserves_flags),
        );
    }

    __m64::allocate()
}

/// Multiply packed signed 16-bit integers in `a` and `b`, producing intermediate signed 32-bit integers. Horizontally add adjacent pairs of intermediate 32-bit integers, and pack the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_madd_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pmaddwd mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// MMultiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the high 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_mulhi_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pmulhw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// MMultiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the low 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_mullo_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "pmullw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// MMultiply the packed signed 16-bit integers in `a` and `b`, producing intermediate 32-bit integers, and store the low 16 bits of the intermediate integers in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_or_si64<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "por mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 16-bit integers from `a` and `b` to packed 8-bit integers using signed saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_packs_pi16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "packsswb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 32-bit integers from `a` and `b` to packed 16-bit integers using signed saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_packs_pi32<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "packssdw mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

/// Convert packed signed 32-bit integers from `a` and `b` to packed 16-bit integers using unsigned saturation, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_packs_pu16<const A: u8, const B: u8>(a: __m64<A>, _b: __m64<B>) -> __m64<A>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
{
    unsafe {
        asm!(
            "packuswb mm{A}, mm{B}",
            A = const A,
            B = const B,
            options(nostack, preserves_flags),
        );
    }

    a
}

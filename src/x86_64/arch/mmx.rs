use core::{arch::asm, marker::PhantomData};

use crate::x86_64::ValidSimdReg;

/// Represents an `mm` SIMD register (formatted `mm{R}`)
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

impl<const R: u8> __m64<R>
where 
    Self: ValidSimdReg,
{
    /// Allocates the register without initializing
    pub const fn allocate() -> Self {
        Self(PhantomData)
    }
}

/// Add packed 16-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_add_pi16<const A: u8, const B: u8, const OUT: u8>(_a: __m64<A>, _b: __m64<B>) -> __m64<OUT>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
    __m64<OUT>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddw mm{A}, mm{B}",
            A = const A,
            B = const B,
        );
    }

    __m64::allocate()
}

/// Add packed 32-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_add_pi32<const A: u8, const B: u8, const OUT: u8>(_a: __m64<A>, _b: __m64<B>) -> __m64<OUT>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
    __m64<OUT>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddd mm{A}, mm{B}",
            A = const A,
            B = const B,
        );
    }

    __m64::allocate()
}

/// Add packed 8-bit integers in `a` and `b`, and store the results in `dst`.
///
/// Requires `mmx`
#[inline(always)]
pub unsafe fn _mm_add_pi8<const A: u8, const B: u8, const OUT: u8>(_a: __m64<A>, _b: __m64<B>) -> __m64<OUT>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
    __m64<OUT>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddb mm{A}, mm{B}",
            A = const A,
            B = const B,
        );
    }

    __m64::allocate()
}

/// Add packed signed 16-bit integers in a and b using saturation, and store the results in dst.
#[inline(always)]
pub unsafe fn _mm_adds_pi16<const A: u8, const B: u8, const OUT: u8>(_a: __m64<A>, _b: __m64<B>) -> __m64<OUT>
where 
    __m64<A>: ValidSimdReg,
    __m64<B>: ValidSimdReg,
    __m64<OUT>: ValidSimdReg,
{
    unsafe {
        asm!(
            "paddsw mm{A}, mm{B}",
            A = const A,
            B = const B,
        );
    }

    __m64::allocate()
}

use core::marker::PhantomData;

use crate::x86_64::ValidSimdReg;

/// Represents an `mm` SIMD register (formatted `mm{R}`)
#[allow(private_bounds)]
pub struct __m64<const R: usize>(PhantomData<u64>)
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

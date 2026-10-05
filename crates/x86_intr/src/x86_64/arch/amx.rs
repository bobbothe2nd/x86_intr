use core::marker::PhantomData;

use crate::x86_64::ValidSimdReg;

/// Represents an AMX tile
#[allow(private_bounds)]
pub struct __tile1024i<const R: usize>(PhantomData<[u8; 1024]>)
where
    Self: ValidSimdReg;

impl ValidSimdReg for __tile1024i<0> {}
impl ValidSimdReg for __tile1024i<1> {}
impl ValidSimdReg for __tile1024i<2> {}
impl ValidSimdReg for __tile1024i<3> {}
impl ValidSimdReg for __tile1024i<4> {}
impl ValidSimdReg for __tile1024i<5> {}
impl ValidSimdReg for __tile1024i<6> {}
impl ValidSimdReg for __tile1024i<7> {}

use core::marker::PhantomData;

use crate::x86_64::ValidSimdReg;

/// Represents an AMX tile
pub struct __tile1024i<const R: u8>(PhantomData<[u8; 1024]>)
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

impl<const R: u8> __tile1024i<R>
where 
    Self: ValidSimdReg,
{
    /// Allocates the register without initializing
    pub const fn allocate() -> Self {
        Self(PhantomData)
    }
}

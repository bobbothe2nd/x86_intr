pub(crate) mod feature;

pub use feature::{test, vendor};

macro_rules! polymorphic {
    {
        $(
            $(#[$attrs:meta])*
            pub fn $name:ident() -> match {
                $(
                    $variants:ty => $body:block
                )*
            }
        )*
    } => {
        $(
            #[allow(non_camel_case_types)]
            trait $name {
                fn $name() -> Self;
            }

            $(
                impl $name for $variants {
                    #[inline(always)]
                    fn $name() -> Self $body
                }
            )*

            $(#[$attrs])*
            #[allow(private_bounds)]
            pub fn $name<T: $name>() -> T {
                T::$name()
            }
        )*
    };
}

mod amx;
mod bit;
mod cache;
mod cet_ss;
mod command;
mod cpu;
mod memory;
mod mmx;
mod msr;
mod mwaitx;
mod protection;
mod tls;
mod tsxldtrk;
mod uintr;
mod wait;

pub mod arch {
    //! Re-exported intrinsics.

    use super::*;

    pub use amx::*;
    pub use bit::*;
    pub use cache::*;
    pub use cet_ss::*;
    pub use command::*;
    pub use cpu::*;
    pub use memory::*;
    pub use mmx::*;
    pub use msr::*;
    pub use mwaitx::*;
    pub use protection::*;
    pub use tls::*;
    pub use tsxldtrk::*;
    pub use uintr::*;
    pub use wait::*;
}

trait ValidSimdReg {}

#[repr(C)]
struct u32x2 {
    lo: u32,
    hi: u32,
}

#[inline(always)]
fn concat_u32(lo: u32, hi: u32) -> u64 {
    u64::from(lo) | (u64::from(hi) << 32)
}

#[inline(always)]
const fn split_u64(x: u64) -> u32x2 {
    unsafe {
        core::mem::transmute::<u64, u32x2>(x)
    }
}

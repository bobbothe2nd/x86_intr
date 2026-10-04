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
    pub use mwaitx::*;
    pub use memory::*;
    pub use protection::*;
    pub use tls::*;
    pub use tsxldtrk::*;
    pub use uintr::*;
    pub use wait::*;
}

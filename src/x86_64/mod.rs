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

mod bit;
mod cache;
mod cet_ss;
mod command;
mod cpu;
mod memory;
mod protection;
mod tls;
mod tsxldtrk;
mod uintr;
mod wait;

pub mod arch {
    //! Re-exported intrinsics.

    pub use super::bit::*;
    pub use super::cache::*;
    pub use super::cet_ss::*;
    pub use super::command::*;
    pub use super::cpu::*;
    pub use super::memory::*;
    pub use super::protection::*;
    pub use super::tls::*;
    pub use super::tsxldtrk::*;
    pub use super::uintr::*;
    pub use super::wait::*;
}

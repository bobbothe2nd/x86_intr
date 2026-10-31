/// Trait used to determine a valid SIMD register type
pub trait ValidSimdReg: private::Sealed {}

mod private {
    pub trait Sealed {}
}

#[repr(C)]
struct u32x2 {
    lo: u32,
    hi: u32,
}

#[inline(always)]
const fn concat_u32(lo: u32, hi: u32) -> u64 {
    (lo as u64) | ((hi as u64) << 32)
}

#[inline(always)]
const fn split_u64(x: u64) -> u32x2 {
    unsafe { core::mem::transmute::<u64, u32x2>(x) }
}

#[cfg(test)]
mod tests {
    use super::{concat_u32, split_u64, u32x2};

    #[test]
    fn splut_concat_uint() {
        let x = 0xABCDEF987654321;
        let u32x2 { hi, lo } = split_u64(x);
        let y = concat_u32(lo, hi);
        assert_eq!(x, y);
    }
}

macro_rules! polymorphic {
    {
        $(
            $(#[$attrs:meta])*
            pub unsafe fn $name:ident() -> match {
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
            pub unsafe fn $name<T: $name>() -> T {
                T::$name()
            }
        )*
    };
}

pub(crate) mod feature;

pub use feature::{test, vendor};

pub mod arch;

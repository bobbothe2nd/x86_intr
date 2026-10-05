trait Supported {}

struct Mmx<const R: usize>;

impl Supported for Mmx<0> {}
impl Supported for Mmx<1> {}
impl Supported for Mmx<2> {}
impl Supported for Mmx<3> {}
impl Supported for Mmx<4> {}
impl Supported for Mmx<5> {}
impl Supported for Mmx<6> {}
impl Supported for Mmx<7> {}

#[inline(always)]
#[asm_cartesian::asm_cartesian(
    paddb,
    D = mm { 0..8 },
    A = mm { 0..8 }
)]
unsafe fn paddb<const D: usize, const A: usize>(
    _dst: Mmx<D>,
    _a: Mmx<A>,
)
where
    Mmx<D>: Supported,
    Mmx<A>: Supported,
{}

fn main() {
    unsafe {
        paddb::<0, 1>(Mmx, Mmx);
        paddb::<3, 5>(Mmx, Mmx);
        paddb::<7, 2>(Mmx, Mmx);
    }
}

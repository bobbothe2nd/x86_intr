trait Supported {}

struct Tmm<const R: usize>;

impl Supported for Tmm<0> {}
impl Supported for Tmm<1> {}
impl Supported for Tmm<2> {}
impl Supported for Tmm<3> {}
impl Supported for Tmm<4> {}
impl Supported for Tmm<5> {}
impl Supported for Tmm<6> {}
impl Supported for Tmm<7> {}

#[inline(always)]
#[asm_cartesian::asm_cartesian(
    tdpbf16ps,
    D = tmm { 0..8 },
    A = tmm { 0..8 },
    B = tmm { 0..8 },
)]
unsafe fn tdpbf16ps<const D: usize, const A: usize, const B: usize>(_dst: Tmm<D>, _a: Tmm<A>, _b: Tmm<B>)
where 
    Tmm<D>: Supported,
    Tmm<A>: Supported,
    Tmm<B>: Supported,
{}

fn main() {
        println!("start");

    unsafe {
        this::<12>();
        println!("again");

        tdpbf16ps::<1, 0, 3>(Tmm, Tmm, Tmm);

        // tdpbf16ps::<21, 0, 3>(Tmm, Tmm, Tmm);
    }
}

fn this<const N: usize>() {
    unsafe {
        core::arch::asm!(
            "tdpbf16ps tmm1, tmm{}, tmm3",
            const N,
        );
    }
}

use core::{arch::asm, fmt, marker::PhantomData};

use crate::x86_64::ValidSimdReg;

/// Represents an AMX tile
#[derive(Clone, Copy)]
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

impl<const R: u8> fmt::Debug for __tile1024i<R>
where 
    Self: ValidSimdReg,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tmm{R}")
    }
}

impl<const R: u8> __tile1024i<R>
where 
    Self: ValidSimdReg,
{
    /// Allocates the register without initializing
    #[inline(always)]
    #[must_use = "Allocating a register is meaningless without using it"]
    pub const fn allocate() -> Self {
        Self(PhantomData)
    }
}

/// Same as [`_tile_cmmimfp16ps`]
///
/// Requires `amx-complex`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_cmmimfp16ps)
#[inline(always)]
pub unsafe fn __tile_cmmimfp16ps<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_cmmimfp16ps::<DST, SRC0, SRC1>();
    }
}

/// Perform matrix multiplication of two tiles containing complex elements and accumulate the results into a packed single precision tile.
/// Each dword element in input tiles `a` and `b` is interpreted as a complex number with FP16 real part and FP16 imaginary part.
/// Calculates the imaginary part of the result. For each possible combination of (row of `a`, column of `b`), it performs a set of multiplication and accumulations on all corresponding complex numbers (one from a and one from `b`).
/// The imaginary part of the a element is multiplied with the real part of the corresponding `b` element, and the real part of the a element is multiplied with the imaginary part of the corresponding `b` elements.
/// The two accumulated results are added, and then accumulated into the corresponding row and column of `dst`.
///
/// Requires `amx-complex`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_cmmimfp16ps)
#[inline(always)]
pub unsafe fn _tile_cmmimfp16ps<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tcmmimfp16ps tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`_tile_cmmrlfp16ps`]
///
/// Requires `amx-complex`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_cmmrlfp16ps)
#[inline(always)]
pub unsafe fn __tile_cmmrlfp16ps<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_cmmrlfp16ps::<DST, SRC0, SRC1>();
    }
}

/// Perform matrix multiplication of two tiles containing complex elements and accumulate the results into a packed single precision tile.
/// Each dword element in input tiles `a` and `b` is interpreted as a complex number with FP16 real part and FP16 imaginary part.
/// Calculates the real part of the result. For each possible combination of (row of `a`, column of `b`), it performs a set of multiplication and accumulations on all corresponding complex numbers (one from a and one from `b`).
/// The real part of the a element is multiplied with the real part of the corresponding `b` element, and the negated imaginary part of the a element is multiplied with the imaginary part of the corresponding `b` elements.
/// The two accumulated results are added, and then accumulated into the corresponding row and column of `dst`.
///
/// Requires `amx-complex`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_cmmrlfp16ps)
#[inline(always)]
pub unsafe fn _tile_cmmrlfp16ps<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tcmmrlfp16ps tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`__tile_dpbf16ps`]
///
/// Requires `amx-bf16`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpbf16ps)
#[inline(always)]
pub unsafe fn __tile_dpbf16ps<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_dpbf16ps::<DST, SRC0, SRC1>();
    }
}

/// Compute dot-product of BF16 (16-bit) floating-point pairs in tiles `a` and `b`, accumulating the intermediate single-precision (32-bit) floating-point elements with elements in `dst`, and store the 32-bit result back to tile `dst`.
///
/// Requires `amx-bf16`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpbf16ps)
#[inline(always)]
pub unsafe fn _tile_dpbf16ps<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tdpbf16ps tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`_tile_dpbssd`]
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpbssd)
#[inline(always)]
pub unsafe fn __tile_dpbssd<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_dpbssd::<DST, SRC0, SRC1>();
    }
}

/// Compute dot-product of bytes in tiles with a source/destination accumulator.
/// Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding signed 8-bit integers in `b`, producing 4 intermediate 32-bit results.
/// Sum these 4 results with the corresponding 32-bit integer in `dst`, and store the 32-bit result back to tile `dst`.
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbssd)
#[inline(always)]
pub unsafe fn _tile_dpbssd<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tdpbssd tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`_tile_dpbsud`]
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpbsud)
#[inline(always)]
pub unsafe fn __tile_dpbsud<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_dpbsud::<DST, SRC0, SRC1>();
    }
}

/// Compute dot-product of bytes in tiles with a source/destination accumulator.
/// Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding unsigned 8-bit integers in `b`, producing 4 intermediate 32-bit results.
/// Sum these 4 results with the corresponding 32-bit integer in `dst`, and store the 32-bit result back to tile `dst`.
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbsud)
#[inline(always)]
pub unsafe fn _tile_dpbsud<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tdpbsud tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`_tile_dpbusd`]
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpbusd)
#[inline(always)]
pub unsafe fn __tile_dpbusd<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_dpbusd::<DST, SRC0, SRC1>();
    }
}

/// Compute dot-product of bytes in tiles with a source/destination accumulator.
/// Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in `b`, producing 4 intermediate 32-bit results.
/// Sum these 4 results with the corresponding 32-bit integer in `dst`, and store the 32-bit result back to tile `dst`.
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbusd)
#[inline(always)]
pub unsafe fn _tile_dpbusd<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tdpbusd tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`_tile_dpbuud`]
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpbuud)
#[inline(always)]
pub unsafe fn __tile_dpbuud<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_dpbuud::<DST, SRC0, SRC1>();
    }
}

/// Compute dot-product of bytes in tiles with a source/destination accumulator.
/// Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in `a` with corresponding unsigned 8-bit integers in `b`, producing 4 intermediate 32-bit results.
/// Sum these 4 results with the corresponding 32-bit integer in `dst`, and store the 32-bit result back to tile `dst`.
///
/// Requires `amx-int8`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbuud)
#[inline(always)]
pub unsafe fn _tile_dpbuud<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tdpbuud tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Same as [`_tile_dpfp16ps`]
///
/// Requires `amx-fp16`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_dpfp16ps)
#[inline(always)]
pub unsafe fn __tile_dpfp16ps<const DST: u8, const SRC0: u8, const SRC1: u8>(_dst: *mut __tile1024i<DST>, _src0: __tile1024i<SRC0>, _src1: __tile1024i<SRC1>)
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        _tile_dpfp16ps::<DST, SRC0, SRC1>();
    }
}

/// Compute dot-product of bytes in tiles with a source/destination accumulator.
/// Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in `a` with corresponding unsigned 8-bit integers in `b`, producing 4 intermediate 32-bit results.
/// Sum these 4 results with the corresponding 32-bit integer in `dst`, and store the 32-bit result back to tile `dst`.
///
/// Requires `amx-fp16`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpfp16ps)
#[inline(always)]
pub unsafe fn _tile_dpfp16ps<const DST: u8, const SRC0: u8, const SRC1: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tdpfp16ps tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            DST = const DST,
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Implementation of tile config structure used by [`_tile_loadconfig`]
#[repr(C, align(64))]
pub struct TileCfg {
    /// selects the supported configuration of the tiles that will be used
    ///
    /// only valid non-INIT value is 1
    pub palette: u8,

    /// used for storing the restart values for interrupted operations
    pub start_row: u8,

    /// must be zero
    pub _reserved0: [u8; 14],

    /// tile 0 bytes/row
    pub tile0_colsb: u16,

    /// tile 1 bytes/row
    pub tile1_colsb: u16,

    /// tile 2 bytes/row
    pub tile2_colsb: u16,

    /// tile 3 bytes/row
    pub tile3_colsb: u16,

    /// tile 4 bytes/row
    pub tile4_colsb: u16,

    /// tile 5 bytes/row
    pub tile5_colsb: u16,

    /// tile 6 bytes/row
    pub tile6_colsb: u16,

    /// tile 7 bytes/row
    pub tile7_colsb: u16,

    /// must be zero
    pub _reserved1: [u8; 16],

    /// tile 0 rows
    pub tile0_rows: u8,

    /// tile 1 rows
    pub tile1_rows: u8,

    /// tile 2 rows
    pub tile2_rows: u8,

    /// tile 3 rows
    pub tile3_rows: u8,

    /// tile 4 rows
    pub tile4_rows: u8,

    /// tile 5 rows
    pub tile5_rows: u8,

    /// tile 6 rows
    pub tile6_rows: u8,

    /// tile 7 rows
    pub tile7_rows: u8,

    /// must be zero
    pub _reserved2: [u8; 8],
}

/// Load tile configuration from a 64-byte memory location specified by `mem_addr`.
/// The tile configuration format is specified below, and includes the tile type pallette, the number of bytes per row, and the number of rows.
/// If the specified pallette_id is zero, that signifies the init state for both the tile config and the tile data, and the tiles are zeroed.
/// Any invalid configurations will result in #GP fault.
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_loadconfig)
#[inline(always)]
pub unsafe fn _tile_loadconfig(mem_addr: *const TileCfg) {
    unsafe {
        asm!(
            "ldtilecfg [{addr}]",
            addr = in(reg) mem_addr,
            options(nostack, preserves_flags)
        );
    }
}

/// Same as [`_tile_loadd`]
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_loadd)
#[inline(always)]
pub unsafe fn __tile_loadd<const DST: u8>(_dst: *mut __tile1024i<DST>, base: *const u8, stride: usize)
where 
    __tile1024i<DST>: ValidSimdReg,
{
    unsafe {
        _tile_loadd(base, stride);
    }
}

/// Load tile rows from memory specifieid by `base` address and `stride` into destination tile `dst` using the tile configuration previously configured via `_tile_loadconfig`.
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_loadd)
#[inline(always)]
pub unsafe fn _tile_loadd<const DST: u8>(base: *const u8, stride: usize)
where 
    __tile1024i<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tileloadd tmm{DST}, [{addr} + {stride}]",
            DST = const DST,
            addr = in(reg) base,
            stride = in(reg) stride,
            options(nostack, preserves_flags)
        );
    }
}

/// Release the tile configuration to return to the init state, which releases all storage it currently holds.
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_release)
#[inline(always)]
pub unsafe fn _tile_release() {
    unsafe {
        asm!(
            "tilerelease",
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Stores the current tile configuration to a 64-byte memory location specified by `mem_addr`.
/// The tile configuration format is specified below, and includes the tile type pallette, the number of bytes per row, and the number of rows.
/// If tiles are not configured, all zeroes will be stored to memory.
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_storeconfig)
#[inline(always)]
pub unsafe fn _tile_storeconfig(mem_addr: *mut TileCfg) {
    unsafe {
        asm!(
            "sttilecfg [{addr}]",
            addr = in(reg) mem_addr,
            options(nostack, preserves_flags)
        );
    }
}

/// Same as [`_tile_stored`]
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_stored)
#[inline(always)]
pub unsafe fn __tile_stored<const SRC: u8>(base: *mut u8, stride: usize, _src: __tile1024i<SRC>)
where 
    __tile1024i<SRC>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tilestored [{base} + {stride}], tmm{SRC}",
            SRC = const SRC,
            base = in(reg) base,
            stride = in(reg) stride,
            options(nostack, preserves_flags)
        );
    }
}

/// Store the tile specified by src to memory specifieid by base address and stride using the tile configuration previously configured via `_tile_loadconfig`.
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_stored)
#[inline(always)]
pub unsafe fn _tile_stored<const SRC: u8>(base: *mut u8, stride: usize)
where 
    __tile1024i<SRC>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tilestored [{base} + {stride}], tmm{SRC}",
            SRC = const SRC,
            base = in(reg) base,
            stride = in(reg) stride,
            options(nostack, preserves_flags)
        );
    }
}

/// Same as [`_tile_stream_loadd`]
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_stream_loadd)
#[inline(always)]
pub unsafe fn __tile_stream_loadd<const DST: u8>(_dst: *mut __tile1024i<DST>, base: *const u8, stride: usize)
where 
    __tile1024i<DST>: ValidSimdReg,
{
    unsafe {
        _tile_stream_loadd(base, stride);
    }
}

/// Load tile rows from memory specifieid by `base` address and stride into destination tile `dst` using the tile configuration previously configured via `_tile_loadconfig`.
/// This intrinsic provides a hint to the implementation that the data will likely not be reused in the near future and the data caching can be optimized accordingly.
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_stream_loadd)
#[inline(always)]
pub unsafe fn _tile_stream_loadd<const DST: u8>(base: *const u8, stride: usize)
where 
    __tile1024i<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tileloaddt1 tmm{DST}, [{addr} + {stride}]",
            DST = const DST,
            addr = in(reg) base,
            stride = in(reg) stride,
            options(nostack, preserves_flags)
        );
    }
}

/// Same as [`_tile_zero`]
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__tile_zero)
#[inline(always)]
pub unsafe fn __tile_zero<const DST: u8>(_dst: *mut __tile1024i<DST>)
where 
    __tile1024i<DST>: ValidSimdReg,
{
    unsafe {
        _tile_zero::<DST>();
    }
}

/// Zero the tile specified by `dst`
///
/// Requires `amx-tile`.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_zero)
#[inline(always)]
pub unsafe fn _tile_zero<const DST: u8>()
where 
    __tile1024i<DST>: ValidSimdReg,
{
    unsafe {
        asm!(
            "tilezero tmm{DST}",
            DST = const DST,
            options(nostack, nomem, preserves_flags)
        );
    }
}

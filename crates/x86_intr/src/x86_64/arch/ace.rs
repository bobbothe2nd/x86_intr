use core::arch::{
    asm,
    x86_64::{__m128i, __m256i, __m512, __m512h, __m512i},
};

use intr_gen::intr_asm;

use crate::{arch::__tile1024i, ValidSimdReg};

/// Convert one packed FP16 vector to BF8 E5M2, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtph_bf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2bf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert one packed FP16 vector to HF8 E4M3, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtph_hf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2hf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert one packed FP16 vector to BF8 E5M2, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtphs_bf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2bf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert one packed FP16 vector to HF8 E4M3, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtphs_hf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2hf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert two packed FP16 vector to BF8 E5M2, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2ph_bf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2bf8 {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert two packed FP16 vector to HF8 E4M3, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2ph_hf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2hf8 {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert two packed FP16 vector to BF8 E5M2, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2phs_bf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2bf8s {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert two packed FP16 vector to HF8 E4M3, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2phs_hf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2hf8s {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert two packed FP32 vectors to one packed FP16 vector. Supports embedded rounding at 512-bit width.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2ps_phx(a: __m512, b: __m512) -> __m512h {
    let dst;

    unsafe {
        asm!(
            "vcvt2ps2phx {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP16 to BF8 E5M2 using bias rounding term, non-saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasph_bf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2bf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP16 to HF8 E4M3 using bias rounding term, non-saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasph_hf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2hf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP16 to BF8 E5M2 using bias rounding term, saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasphs_bf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2bf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP16 to HF8 E4M3 using bias rounding term, saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasphs_hf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2hf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP8 E4M3 to packed FP16. Conversion is exact; no rounding required.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvthf8_ph(a: __m256i) -> __m512h {
    let dst;

    unsafe {
        asm!(
            "vcvthf82ph {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(ymm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using RNE rounding, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtps_bf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2bf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using RNE rounding, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtpss_bf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2bf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RNE rounding, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtps_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2hf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RNE rounding, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtpss_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2hf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RTO rounding, non-saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtrops_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtrop2hf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RTO rounding, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtropss_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtrop2hf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using bias rounding, non-saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasps_bf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2bf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using bias rounding, saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiaspss_bf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2bf8s {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using bias rounding, non-saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasps_hf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2hf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using bias rounding, saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiaspss_hf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2hf8s {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
        );
    }

    dst
}

/// Convert packed BF8 E5M2 to packed FP32. Conversion is exact; output register is 4x wider than input.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbf8_ps(a: __m128i) -> __m512 {
    let dst;

    unsafe {
        asm!(
            "vcvtbf82ps {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(xmm_reg) a,
        );
    }

    dst
}

/// Convert packed HF8 E4M3 to packed FP32. Conversion is exact; output register is 4x wider than input.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvthf8_ps(a: __m128i) -> __m512 {
    let dst;

    unsafe {
        asm!(
            "vcvthf82ps {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(xmm_reg) a,
        );
    }

    dst
}

/// Same as [`_mm512_cvtbf8_bf4s`]
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtf8_bf4s(a: __m512i) -> __m256i {
    unsafe { _mm512_cvtbf8_bf4s(a) }
}

/// Convert packed BF8 E5M2 to packed FP4 E2M1, saturating. Output register is half the width of input.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbf8_bf4s(a: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf82bf4s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed HF8 E4M3 to packed FP4 E2M1, saturating. Output register is half the width of input.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvthf8_bf4s(a: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvthf82bf4s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Convert packed FP4 E2M1 to packed HF8 E4M3. Conversion is exact; output register is 2x wider than input.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbf4_hf8(a: __m256i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf42hf8 {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(ymm_reg) a,
        );
    }

    dst
}

/// Same as [`_mm512_cvtbf8_bf6s`]
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtf8_bf6s(a: __m512i) -> __m512i {
    unsafe { _mm512_cvtbf8_bf6s(a) }
}

/// Convert packed BF8 E5M2 to packed BF6 E3M2, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbf8_bf6s(a: __m512i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf82bf6s {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Same as [`_mm512_cvtbfhf8`]
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtfhf8(a: __m512i) -> __m512i {
    unsafe { _mm512_cvtbfhf8(a) }
}

/// Convert packed BF8 E5M2 to packed HF6 E2M3, saturating.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbfhf8(a: __m512i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf62hf8 {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// Truncate packed 32-bit signed integers to 8-bit with symmetric signed saturation, storing to destination register or memory.
///
/// Requires `avx512-bw`/`avx512bw`
#[target_feature(enable = "avx512bw")]
pub unsafe fn _mm512_cvtssepi32_epi8(a: __m512i) -> __m128i {
    let dst;

    unsafe {
        asm!(
            ".insn <encoding 0x62 0xF3 0x0F 0x38 0x41 /r>, {dst}, {a}",
            // "vpmovssdb {dst}, {a}",
            dst = out(xmm_reg) dst,
            a = in(zmm_reg) a,
        );
    }

    dst
}

/// #define ACE_UNPACKB_SIZE(n) (((n) & 0x7) << 2)
pub const fn ace_unpackb_size(n: u8) -> u8 {
    (n & 0x7) << 2
}

/// #define ACE_UNPACKB_START(s) (((s) & 0x3) << 0)
pub const fn ace_unpackb_start(s: u8) -> u8 {
    s & 0x3
}

/// #define ACE_UNPACKB_SEXT (1 << 5)
pub const ACE_UNPACKB_SEXT: u8 = 1 << 5;

/// Unpack packed sub-byte elements (2–7 bits) to bytes; imm8 selects element size, start offset within the packed block, and optional sign extension.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_unpackb<const IMM8: u8>(a: __m512i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vunpackb {dst}, {a}, {IMM8}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            IMM8 = const IMM8,
        );
    }

    dst
}

/// Read a row from a tile register to a ZMM vector register.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_movrow<const TSRC: u8>(row: u32) -> __m512i
where
    __tile1024i<TSRC>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "tilemovrow {dst}, tmm{TSRC}, {row:e}",
            dst = out(zmm_reg) dst,
            row = in(reg) row,
            TSRC = const TSRC,
        );
    }

    dst
}

/// Write a row to a tile register from a ZMM vector register.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_setrow<const TDST: u8>(row: u32, src: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "tilemovrow tmm{TDST}, {src}, {row:e}",
            row = in(reg) row,
            src = in(zmm_reg) src,
        );
    }
}

/// Write a column to a tile register from a ZMM vector register.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_setcol<const TDST: u8>(col: u32, src: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "tilemovcol tmm{TDST}, {src}, {col:e}",
            col = in(reg) col,
            src = in(zmm_reg) src,
        );
    }
}

/// Read a row from a tile register to a ZMM vector register.
///
/// Requires `avx10.2`
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_cvtrowd2p<const TSRC: u8>(row: u32) -> __m512i
where
    __tile1024i<TSRC>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "tcvtrowd2ps {dst}, tmm{TSRC}, {row:e}",
            dst = out(zmm_reg) dst,
            row = in(reg) row,
            TSRC = const TSRC,
        );
    }

    dst
}

/// #define ACE_SCALE_A(g) (((g) & 0x3) << 4)
pub const fn ace_scale_a(g: u8) -> u8 {
    (g & 0x3) << 4
}

/// #define ACE_SCALE_B(g) (((g) & 0x3) << 0)
pub const fn ace_scale_b(g: u8) -> u8 {
    g & 0x3
}

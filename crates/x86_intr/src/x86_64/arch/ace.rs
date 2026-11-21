use core::arch::{
    asm,
    x86_64::{__m256i, __m512, __m512h, __m512i},
};

#[cfg(feature = "avx10")]
use core::arch::x86_64::__m128i;

use intr_gen::intr_asm;

use crate::{arch::__tile1024i, ValidSimdReg};

/// Convert one packed FP16 vector to BF8 E5M2, non-saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtph_bf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2bf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert one packed FP16 vector to HF8 E4M3, non-saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtph_hf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2hf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert one packed FP16 vector to BF8 E5M2, saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtphs_bf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2bf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert one packed FP16 vector to HF8 E4M3, saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtphs_hf8(a: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtph2hf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert two packed FP16 vector to BF8 E5M2, non-saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2ph_bf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2bf8 {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert two packed FP16 vector to HF8 E4M3, non-saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2ph_hf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2hf8 {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert two packed FP16 vector to BF8 E5M2, saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2phs_bf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2bf8s {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert two packed FP16 vector to HF8 E4M3, saturating.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2phs_hf8(a: __m512h, b: __m512h) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvt2ph2hf8s {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert two packed FP32 vectors to one packed FP16 vector. Supports embedded rounding at 512-bit width.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvt2ps_phx(a: __m512, b: __m512) -> __m512h {
    let dst;

    unsafe {
        asm!(
            "vcvt2ps2phx {dst}, {a}, {b}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP16 to BF8 E5M2 using bias rounding term, non-saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasph_bf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2bf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP16 to HF8 E4M3 using bias rounding term, non-saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasph_hf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2hf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP16 to BF8 E5M2 using bias rounding term, saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasphs_bf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2bf8s {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP16 to HF8 E4M3 using bias rounding term, saturating. `a` contains the INT8 bias values; `b` contains the FP16 values.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasphs_hf8(a: __m512i, b: __m512h) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasph2hf8s {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP8 E4M3 to packed FP16. Conversion is exact; no rounding required.
///
/// Requires `avx10.1`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvthf8_ph(a: __m256i) -> __m512h {
    let dst;

    unsafe {
        asm!(
            "vcvthf82ph {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(ymm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using RNE rounding, non-saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtps_bf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2bf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using RNE rounding, saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtpss_bf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2bf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RNE rounding, non-saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtps_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2hf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RNE rounding, saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtpss_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtps2hf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RTO rounding, non-saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtrops_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtrop2hf8 {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using RTO rounding, saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtropss_hf8(a: __m512) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtrop2hf8s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using bias rounding, non-saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasps_bf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2bf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to BF8 E5M2 using bias rounding, saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiaspss_bf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2bf8s {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using bias rounding, non-saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiasps_hf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2hf8 {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP32 to HF8 E4M3 using bias rounding, saturating. Bias rounding term is Operand 2.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbiaspss_hf8(a: __m512, b: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbiasps2hf8s {dst}, {a}, {b}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            b = in(zmm_reg) b,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed BF8 E5M2 to packed FP32. Conversion is exact; output register is 4x wider than input.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbf8_ps(a: __m128i) -> __m512 {
    let dst;

    unsafe {
        asm!(
            "vcvtbf82ps {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(xmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed HF8 E4M3 to packed FP32. Conversion is exact; output register is 4x wider than input.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvthf8_ps(a: __m128i) -> __m512 {
    let dst;

    unsafe {
        asm!(
            "vcvthf82ps {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(xmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed BF8 E5M2 to packed FP4 E2M1, saturating. Output register is half the width of input.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtf8_bf4s(a: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf82bf4s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed HF8 E4M3 to packed FP4 E2M1, saturating. Output register is half the width of input.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvthf8_bf4s(a: __m512i) -> __m256i {
    let dst;

    unsafe {
        asm!(
            "vcvthf82bf4s {dst}, {a}",
            dst = out(ymm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed FP4 E2M1 to packed HF8 E4M3. Conversion is exact; output register is 2x wider than input.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtbf4_hf8(a: __m256i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf42hf8 {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(ymm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed BF8 E5M2 to packed BF6 E3M2, saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtf8_bf6s(a: __m512i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf82bf6s {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Convert packed BF8 E5M2 to packed HF6 E2M3, saturating.
///
/// Requires `avx10.2`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_cvtf6_hf8(a: __m512i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vcvtbf62hf8 {dst}, {a}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Truncate packed 32-bit signed integers to 8-bit with symmetric signed saturation, storing to destination register or memory.
///
/// Requires `avx512-bw`/`avx512bw`
#[cfg(feature = "avx10")]
#[inline(never)]
#[target_feature(enable = "avx512bw")]
pub unsafe fn _mm512_cvtssepi32_epi8(a: __m512i) -> __m128i {
    let dst;

    unsafe {
        asm!(
            "vpmovssdb {dst}, {a}",
            dst = out(xmm_reg) dst,
            a = in(zmm_reg) a,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// `#define ACE_UNPACKB_SIZE(n) (((n) & 0x7) << 2)`
#[inline(always)]
pub const fn ACE_UNPACKB_SIZE(n: u8) -> u8 {
    (n & 0x7) << 2
}

/// `#define ACE_UNPACKB_START(s) (((s) & 0x3) << 0)`
#[inline(always)]
pub const fn ACE_UNPACKB_START(s: u8) -> u8 {
    s & 0x3
}

/// `#define ACE_UNPACKB_SEXT (1 << 5)`
pub const ACE_UNPACKB_SEXT: u8 = 1 << 5;

/// Unpack packed sub-byte elements (2–7 bits) to bytes; imm8 selects element size, start offset within the packed block, and optional sign extension.
///
/// Requires `avx10.2`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_unpackb<const IMM8: u8>(a: __m512i) -> __m512i {
    let dst;

    unsafe {
        asm!(
            "vunpackb {dst}, {a}, {IMM8}",
            dst = out(zmm_reg) dst,
            a = in(zmm_reg) a,
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Read a row from a tile register to a ZMM vector register.
///
/// Requires `ace`
#[inline(never)]
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
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Write a row to a tile register from a ZMM vector register.
///
/// Requires `ace`
#[inline(never)]
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
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Write a column to a tile register from a ZMM vector register.
///
/// Requires `ace`
#[inline(never)]
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
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Read a row from a tile register to a ZMM vector register.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_cvtrowd2ps<const TSRC: u8>(row: u32) -> __m512i
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
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Move a tile row to a ZMM vector register, converting FP32 to BF16 packed into the high word of each dword.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_cvtrowps2bf16h<const TSRC: u8>(row: u32) -> __m512i
where
    __tile1024i<TSRC>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "tcvtrowps2bf16h {dst}, tmm{TSRC}, {row:e}",
            dst = out(zmm_reg) dst,
            row = in(reg) row,
            TSRC = const TSRC,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Move a tile row to a ZMM vector register, converting FP32 to BF16 packed into the low word of each dword.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_cvtrowps2bf16l<const TSRC: u8>(row: u32) -> __m512i
where
    __tile1024i<TSRC>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "tcvtrowps2bf16l {dst}, tmm{TSRC}, {row:e}",
            dst = out(zmm_reg) dst,
            row = in(reg) row,
            TSRC = const TSRC,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Move a tile row to a ZMM vector register, converting FP32 to FP16 packed into the high word of each dword.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_cvtrowps2phh<const TSRC: u8>(row: u32) -> __m512i
where
    __tile1024i<TSRC>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "tcvtrowps2phh {dst}, tmm{TSRC}, {row:e}",
            dst = out(zmm_reg) dst,
            row = in(reg) row,
            TSRC = const TSRC,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// Move a tile row to a ZMM vector register, converting FP32 to FP16 packed into the low word of each dword.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_cvtrowps2phl<const TSRC: u8>(row: u32) -> __m512i
where
    __tile1024i<TSRC>: ValidSimdReg,
{
    let dst;

    unsafe {
        asm!(
            "tcvtrowps2phl {dst}, tmm{TSRC}, {row:e}",
            dst = out(zmm_reg) dst,
            row = in(reg) row,
            TSRC = const TSRC,
            options(nostack, nomem, preserves_flags)
        );
    }

    dst
}

/// `#define ACE_SCALE_A(g) (((g) & 0x3) << 4)`
#[inline(always)]
pub const fn ACE_SCALE_A(g: u8) -> u8 {
    (g & 0x3) << 4
}

/// `#define ACE_SCALE_B(g) (((g) & 0x3) << 0)`
#[inline(always)]
pub const fn ACE_SCALE_B(g: u8) -> u8 {
    g & 0x3
}

/// Rank-4 MX FP8 outer product. A = BF8 E5M2, B = BF8 E5M2. OCP MX block scaling via BSR. FP32 accumulate.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4mxbf8ps<const TDST: u8, const IMM8: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4mxbf8ps tmm{TDST}, {src1}, {src2}, {IMM8}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 MX FP8 outer product. A = BF8 E5M2, B = HF8 E4M3. OCP MX block scaling via BSR. FP32 accumulate.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4mxbhf8ps<const TDST: u8, const IMM8: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4mxbhf8ps tmm{TDST}, {src1}, {src2}, {IMM8}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 MX FP8 outer product. A = HF8 E4M3, B = BF8 E5M2. OCP MX block scaling via BSR. FP32 accumulate.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4mxhbf8ps<const TDST: u8, const IMM8: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4mxhbf8ps tmm{TDST}, {src1}, {src2}, {IMM8}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 MX FP8 outer product. A = HF8 E4M3, B = HF8 E4M3. OCP MX block scaling via BSR. FP32 accumulate.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4mxhf8ps<const TDST: u8, const IMM8: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4mxhf8ps tmm{TDST}, {src1}, {src2}, {IMM8}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 MX INT8 outer product. A = MX INT8 signed, B = MX INT8 signed. OCP MX block scaling via BSR. FP32 accumulate.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4mxbssps<const TDST: u8, const IMM8: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4mxbssps tmm{TDST}, {src1}, {src2}, {IMM8}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            IMM8 = const IMM8,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-2 BF16 outer product. A = BF16, B = BF16. FP32 accumulate. No block scaling.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top2bf16ps<const TDST: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top2bf16ps tmm{TDST}, {src1}, {src2}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 byte outer product. A = INT8 signed, B = INT8 signed. INT32 accumulate. No block scaling.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4bssd<const TDST: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4bssd tmm{TDST}, {src1}, {src2}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 byte outer product. A = INT8 signed, B = INT8 unsigned. INT32 accumulate. No block scaling.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4bsud<const TDST: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4bsud tmm{TDST}, {src1}, {src2}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 byte outer product. A = INT8 unsigned, B = INT8 signed. INT32 accumulate. No block scaling.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4busd<const TDST: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4busd tmm{TDST}, {src1}, {src2}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Rank-4 byte outer product. A = INT8 unsigned, B = INT8 unsigned. INT32 accumulate. No block scaling.
///
/// Requires `ace`
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _tile_top4buud<const TDST: u8>(src1: __m512i, src2: __m512i)
where
    __tile1024i<TDST>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{TDST} = TDST[0..8],
            "top4buud tmm{TDST}, {src1}, {src2}",
            src1 = in(zmm_reg) src1,
            src2 = in(zmm_reg) src2,
            options(nostack, nomem, preserves_flags)
        );
    }
}

/// Initialize all bytes in BSR bsr0 to 0x7F (representing E8M0 scale = 1).
///
/// Requires `ace`
#[cfg(feature = "ace_bsr")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _bsrinit() {
    unsafe {
        asm!(
            "bsrinit",
            options(nostack, preserves_flags)
        );
    }
}

/// Write full BSR bsr0 from two ZMM sources: zmm1 provides A-scales, zmm2/m512 provides B-scales.
///
/// Requires `ace`
#[cfg(feature = "ace_bsr")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _bsrmovf(a_scales: __m512i, b_scales: __m512i) {
    unsafe {
        asm!(
            "bsrmovf bsr0, {a_scales}, {b_scales}",
            a_scales = in(zmm_reg) a_scales,
            b_scales = in(zmm_reg) b_scales,
            options(nostack, preserves_flags)
        );
    }
}

/// Move the A-scale half of BSR bsr0 (high 512 bits). Write form: bsr0 ← zmm1/m512.
///
/// Requires `ace`
#[cfg(feature = "ace_bsr")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _bsrmovh(a_scales: __m512i) {
    unsafe {
        asm!(
            "bsrmovh bsr0, {a_scales}",
            a_scales = in(zmm_reg) a_scales,
            options(nostack, preserves_flags)
        );
    }
}

/// Move the A-scale half of BSR bsr0 (high 512 bits). Read form: zmm1/m512 ← bsr0.
///
/// Requires `ace`
#[cfg(feature = "ace_bsr")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _bsrmovh_r() -> __m512i {
    let a_scales;

    unsafe {
        asm!(
            "bsrmovh {a_scales}, bsr0",
            a_scales = out(zmm_reg) a_scales,
            options(nostack, preserves_flags)
        );
    }

    a_scales
}

/// Move the B-scale half of BSR bsr0 (low 512 bits). Write form: bsr0 ← zmm1/m512.
///
/// Requires `ace`
#[cfg(feature = "ace_bsr")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _bsrmovl(b_scales: __m512i) {
    unsafe {
        asm!(
            "bsrmovl bsr0, {b_scales}",
            b_scales = in(zmm_reg) b_scales,
            options(nostack, preserves_flags)
        );
    }
}

/// Move the B-scale half of BSR bsr0 (low 512 bits). Read form: zmm1/m512 ← bsr0.
///
/// Requires `ace`
#[cfg(feature = "ace_bsr")]
#[inline(never)]
#[target_feature(enable = "avx512f")]
pub unsafe fn _bsrmovl_r() -> __m512i {
    let b_scales;

    unsafe {
        asm!(
            "bsrmovl {b_scales}, bsr0",
            b_scales = out(zmm_reg) b_scales,
            options(nostack, preserves_flags)
        );
    }

    b_scales
}

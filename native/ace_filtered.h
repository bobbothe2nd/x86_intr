/* ACE v1.15 instruction mnemonics from the ACE specification. */
/* cvtph_bf8 — BF8 (E5M2), non-saturating */
// VCVTPH2BF8
__m256i _mm512_cvtph_bf8(__m512h a);
/* cvtph_hf8 — HF8 (E4M3), non-saturating */
// VCVTPH2HF8
__m256i _mm512_cvtph_hf8(__m512h a);
/* cvtphs_bf8 — BF8 (E5M2), saturating */
// VCVTPH2BF8S
__m256i _mm512_cvtphs_bf8(__m512h a);
/* cvtphs_hf8 — HF8 (E4M3), saturating */
// VCVTPH2HF8S
__m256i _mm512_cvtphs_hf8(__m512h a);
/* cvt2ph_bf8 — BF8 (E5M2), non-saturating */
// VCVT2PH2BF8
__m512i _mm512_cvt2ph_bf8(__m512h a, __m512h b);
/* cvt2ph_hf8 — HF8 (E4M3), non-saturating */
// VCVT2PH2HF8
__m512i _mm512_cvt2ph_hf8(__m512h a, __m512h b);
/* cvt2phs_bf8 — BF8 (E5M2), saturating */
// VCVT2PH2BF8S
__m512i _mm512_cvt2phs_bf8(__m512h a, __m512h b);
/* cvt2phs_hf8 — HF8 (E4M3), saturating */
// VCVT2PH2HF8S
__m512i _mm512_cvt2phs_hf8(__m512h a, __m512h b);
/* cvt2ps_phx — FP32-pair→FP16 */
// VCVT2PS2PHX
__m512h _mm512_cvt2ps_phx(__m512 a, __m512 b);
/* cvtbiasph_bf8 — BF8 (E5M2), non-saturating */
// VCVTBIASPH2BF8
__m256i _mm512_cvtbiasph_bf8(__m512i a, __m512h b);
/* cvtbiasph_hf8 — HF8 (E4M3), non-saturating */
// VCVTBIASPH2HF8
__m256i _mm512_cvtbiasph_hf8(__m512i a, __m512h b);
/* cvtbiasphs_bf8 — BF8 (E5M2), saturating */
// VCVTBIASPH2BF8S
__m256i _mm512_cvtbiasphs_bf8(__m512i a, __m512h b);
/* cvtbiasphs_hf8 — HF8 (E4M3), saturating */
// VCVTBIASPH2HF8S
__m256i _mm512_cvtbiasphs_hf8(__m512i a, __m512h b);
/* cvthf8_ph — FP8 E4M3→FP16 */
// VCVTHF82PH
__m512h _mm512_cvthf8_ph(__m256i a);

/* OCP / AVX10.2 */
// VCVTPS2BF8
__m256i _mm512_cvtps_bf8(__m512 a);
// VCVTPS2BF8S
__m256i _mm512_cvtpss_bf8(__m512 a);
// VCVTPS2HF8
__m256i _mm512_cvtps_hf8(__m512 a);
// VCVTPS2HF8S
__m256i _mm512_cvtpss_hf8(__m512 a);
// VCVTROPS2HF8
__m256i _mm512_cvtrops_hf8(__m512 a);
// VCVTROPS2HF8S
__m256i _mm512_cvtropss_hf8(__m512 a);
// VCVTBIASPS2BF8
__m256i _mm512_cvtbiasps_bf8(__m512 a, __m512i b);
// VCVTBIASPS2BF8S
__m256i _mm512_cvtbiaspss_bf8(__m512 a, __m512i b);
// VCVTBIASPS2HF8
__m256i _mm512_cvtbiasps_hf8(__m512 a, __m512i b);
// VCVTBIASPS2HF8S
__m256i _mm512_cvtbiaspss_hf8(__m512 a, __m512i b);
// VCVTBF82PS
__m512 _mm512_cvtbf8_ps(__m128i a);
// VCVTHF82PS
__m512 _mm512_cvthf8_ps(__m128i a);
// VCVTBF82BF4S
__m256i _mm512_cvtf8_bf4s(__m512i a);
// VCVTBF42HF8
__m512i _mm512_cvtbf4_hf8(__m256i a);
// VCVTBF82BF6S
__m512i _mm512_cvtf8_bf6s(__m512i a);
// VCVTBF62HF8
__m512i _mm512_cvtf6_hf8(__m512i a);
// VPMOVSSDB
__m128i _mm512_cvtssepi32_epi8(__m512i a);

/* Compose the imm8 argument for _mm512_unpackb:
 * ACE_UNPACKB_SIZE(n) imm8[4:2] element size in bits (2..7)
 * ACE_UNPACKB_START(s) imm8[1:0] start offset (0..3, subject to size)
 * ACE_UNPACKB_SEXT imm8[5] sign-extend output (all sizes)
 * imm8[7:6] are reserved and must be zero.
 */
#define ACE_UNPACKB_SIZE(n) (((n) & 0x7) << 2)
#define ACE_UNPACKB_START(s) (((s) & 0x3) << 0)
#define ACE_UNPACKB_SEXT (1 << 5)
/* unpackb — Sub-byte element extraction */
// VUNPACKB
__m512i _mm512_unpackb(__m512i a, unsigned int b);

/* ACE tile operations */
// TILEMOVROW
__m512i _tile_movrow(const __tile1024i *tsrc, unsigned int row);

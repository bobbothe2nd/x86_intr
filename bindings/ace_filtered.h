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
/* _tile_setrow — write direction */
void _tile_setrow(__tile1024i *tdst, unsigned int row, __m512i src);
/* _tile_setcol — Write only: tmm←zmm col */
void _tile_setcol(__tile1024i *tdst, unsigned int row, __m512i src);

// === TODO ===

/* _tile_cvtrowd2ps — Move row INT32→FP32 */
__m512 _tile_cvtrowd2ps(const __tile1024i *tsrc, unsigned int row);
/* _tile_cvtrowps2bf16h — high half-word of each dword */
__m512i _tile_cvtrowps2bf16h(const __tile1024i *tsrc, unsigned int row);
/* _tile_cvtrowps2bf16l — low half-word of each dword */
__m512i _tile_cvtrowps2bf16l(const __tile1024i *tsrc, unsigned int row);
/* _tile_cvtrowps2phh — high half-word of each dword */
__m512i _tile_cvtrowps2phh(const __tile1024i *tsrc, unsigned int row);
/* _tile_cvtrowps2phl — low half-word of each dword */
__m512i _tile_cvtrowps2phl(const __tile1024i *tsrc, unsigned int row);
/* Compose the imm8 scale-group selector with | :
* ACE_SCALE_A(g) imm8[5:4] A-input scale group (0..3)
* ACE_SCALE_B(g) imm8[1:0] B-input scale group (0..3)
* imm8[7:6] and imm8[3:2] are reserved and must be zero. */
#define ACE_SCALE_A(g) (((g) & 0x3) << 4)
#define ACE_SCALE_B(g) (((g) & 0x3) << 0)
/* _tile_top4mxbf8ps — BF8 × BF8 */
void _tile_top4mxbf8ps(__tile1024i *tdst, __m512i src1, __m512i src2, const int imm8);
/* _tile_top4mxbhf8ps — BF8 × HF8 */
void _tile_top4mxbhf8ps(__tile1024i *tdst, __m512i src1, __m512i src2, const int imm8);
/* _tile_top4mxhbf8ps — HF8 × BF8 */
void _tile_top4mxhbf8ps(__tile1024i *tdst, __m512i src1, __m512i src2, const int imm8);
/* _tile_top4mxhf8ps — HF8 × HF8 */
void _tile_top4mxhf8ps(__tile1024i *tdst, __m512i src1, __m512i src2, const int imm8);
/* _tile_top4mxbssps — MX INT8 rank-4 OP S×S; BSR scaled; FP32 accum */
void _tile_top4mxbssps(__tile1024i *tdst, __m512i src1, __m512i src2, const int imm8);
/* _tile_top2bf16ps — BF16 rank-2 OP; FP32 accum; no scaling */
void _tile_top2bf16ps(__tile1024i *tdst, __m512i src1, __m512i src2);
/* _tile_top4bssd — signed A × signed B */
void _tile_top4bssd(__tile1024i *tdst, __m512i src1, __m512i src2);
/* _tile_top4bsud — signed A × unsigned B */
void _tile_top4bsud(__tile1024i *tdst, __m512i src1, __m512i src2);
/* _tile_top4busd — unsigned A × signed B */
void _tile_top4busd(__tile1024i *tdst, __m512i src1, __m512i src2);
/* _tile_top4buud — unsigned A × unsigned B */
void _tile_top4buud(__tile1024i *tdst, __m512i src1, __m512i src2);

/* _bsrinit — Init BSR to 0x7F (E8M0 scale=1) */
void _bsrinit(void);
/* _bsrmovf — Write full BSR from two ZMMs */
void _bsrmovf(__m512i a_scales, __m512i b_scales);
/* _bsrmovh — write direction */
void _bsrmovh(__m512i a_scales);
/* _bsrmovh_r — read direction */
__m512i _bsrmovh_r(void);
/* _bsrmovl — write direction */
void _bsrmovl(__m512i b_scales);
/* _bsrmovl_r — read direction */
__m512i _bsrmovl_r(void);

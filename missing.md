Copied from Rust `stdarch/crates/core_arch/missing-x86.md` to track Intel intrinsics.

<details><summary>["AVX512_FP16"]</summary><p>

  * [ ] [`_mm256_set1_pch`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_set1_pch)
  * [ ] [`_mm512_set1_pch`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_set1_pch)
  * [ ] [`_mm_set1_pch`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_set1_pch)
</p></details>


<details><summary>["CET_SS"]</summary><p>

  * [x] [`_clrssbsy`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_clrssbsy)
  * [x] [`_get_ssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_get_ssp)
  * [x] [`_get_ssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_get_ssp)
  * [x] [`_inc_ssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_inc_ssp)
  * [x] [`_incsspd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_incsspd)
  * [x] [`_incsspq`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_incsspq)
  * [x] [`_rdsspd_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rdsspd_i32)
  * [x] [`_rdsspq_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rdsspq_i64)
  * [x] [`_rstorssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rstorssp)
  * [x] [`_saveprevssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_saveprevssp)
  * [x] [`_setssbsy`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_setssbsy)
  * [x] [`_wrssd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrssd)
  * [x] [`_wrssq`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrssq)
  * [x] [`_wrussd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrussd)
  * [x] [`_wrussq`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrussq)
</p></details>


<details><summary>["CLDEMOTE"]</summary><p>

  * [x] [`_mm_cldemote`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cldemote)
</p></details>


<details><summary>["CLWB"]</summary><p>

  * [x] [`_mm_clwb`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_clwb)
</p></details>


<details><summary>["CMPCCXADD"]</summary><p>

  * [ ] [`_cmpccxadd_epi32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_cmpccxadd_epi32)
  * [ ] [`_cmpccxadd_epi64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_cmpccxadd_epi64)
</p></details>


<details><summary>["ENQCMD"]</summary><p>

  * [x] [`_enqcmd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_enqcmd)
  * [x] [`_enqcmds`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_enqcmds)
</p></details>


<details><summary>["FSGSBASE"]</summary><p>

  * [x] [`_readfsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readfsbase_u32)
  * [x] [`_readfsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readfsbase_u64)
  * [x] [`_readgsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readgsbase_u32)
  * [x] [`_readgsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readgsbase_u64)
  * [x] [`_writefsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writefsbase_u32)
  * [x] [`_writefsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writefsbase_u64)
  * [x] [`_writegsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writegsbase_u32)
  * [x] [`_writegsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writegsbase_u64)
</p></details>


<details><summary>["HRESET"]</summary><p>

  * [x] [`_hreset`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_hreset)
</p></details>


<details><summary>["INVPCID"]</summary><p>

  * [x] [`_invpcid`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_invpcid)
</p></details>


<details><summary>["MONITOR"]</summary><p>

  * [x] [`_mm_monitor`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_monitor)
  * [x] [`_mm_mwait`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mwait)
</p></details>


<details><summary>["MOVBE"]</summary><p>

  * [x] [`_loadbe_i16`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_loadbe_i16)
  * [x] [`_loadbe_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_loadbe_i32)
  * [x] [`_loadbe_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_loadbe_i64)
  * [x] [`_storebe_i16`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_storebe_i16)
  * [x] [`_storebe_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_storebe_i32)
  * [x] [`_storebe_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_storebe_i64)
</p></details>


<details><summary>["MOVDIR64B"]</summary><p>

  * [x] [`_movdir64b`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_movdir64b)
</p></details>


<details><summary>["MOVDIRI"]</summary><p>

  * [x] [`_directstoreu_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_directstoreu_u32)
  * [x] [`_directstoreu_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_directstoreu_u64)
</p></details>


<details><summary>["PCONFIG"]</summary><p>

  * [ ] [`_pconfig_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_pconfig_u32)
</p></details>


<details><summary>["POPCNT"]</summary><p>

  * [x] [`_mm_popcnt_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_popcnt_u32)
  * [x] [`_mm_popcnt_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_popcnt_u64)
</p></details>


<details><summary>["PREFETCHI"]</summary><p>

  * [ ] [`_m_prefetchit0`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_m_prefetchit0)
  * [ ] [`_m_prefetchit1`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_m_prefetchit1)
</p></details>


<details><summary>["RAO_INT"]</summary><p>

  * [x] [`_aadd_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aadd_i32)
  * [x] [`_aadd_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aadd_i64)
  * [x] [`_aand_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aand_i32)
  * [x] [`_aand_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aand_i64)
  * [x] [`_aor_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aor_i32)
  * [x] [`_aor_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aor_i64)
  * [x] [`_axor_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_axor_i32)
  * [x] [`_axor_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_axor_i64)
</p></details>


<details><summary>["RDPID"]</summary><p>

  * [x] [`_rdpid_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rdpid_u32)
</p></details>


<details><summary>["SERIALIZE"]</summary><p>

  * [x] [`_serialize`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_serialize)
</p></details>


<details><summary>["SSE"]</summary><p>

  * [ ] [`_mm_free`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_free)
  * [ ] [`_mm_malloc`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_malloc)
</p></details>


<details><summary>["TSXLDTRK"]</summary><p>

  * [x] [`_xresldtrk`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_xresldtrk)
  * [x] [`_xsusldtrk`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_xsusldtrk)
</p></details>


<details><summary>["UINTR"]</summary><p>

  * [x] [`_clui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_clui)
  * [x] [`_senduipi`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_senduipi)
  * [x] [`_stui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_stui)
  * [x] [`_testui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_testui)
</p></details>


<details><summary>["USER_MSR"]</summary><p>

  * [x] [`_urdmsr`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_urdmsr)
  * [x] [`_uwrmsr`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_uwrmsr)
</p></details>


<details><summary>["WAITPKG"]</summary><p>

  * [x] [`_tpause`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_tpause)
  * [x] [`_umonitor`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_umonitor)
  * [x] [`_umwait`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_umwait)
</p></details>


<details><summary>["WBNOINVD"]</summary><p>

  * [x] [`_wbnoinvd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wbnoinvd)
</p></details>

todo in MMX:

__m64 _m_packssdw (__m64 a, __m64 b)
packsswb
__m64 _m_packsswb (__m64 a, __m64 b)
packuswb
__m64 _m_packuswb (__m64 a, __m64 b)
paddb
__m64 _m_paddb (__m64 a, __m64 b)
paddd
__m64 _m_paddd (__m64 a, __m64 b)
paddsb
__m64 _m_paddsb (__m64 a, __m64 b)
paddsw
__m64 _m_paddsw (__m64 a, __m64 b)
paddusb
__m64 _m_paddusb (__m64 a, __m64 b)
paddusw
__m64 _m_paddusw (__m64 a, __m64 b)
paddw
__m64 _m_paddw (__m64 a, __m64 b)
pand
__m64 _m_pand (__m64 a, __m64 b)
pandn
__m64 _m_pandn (__m64 a, __m64 b)
pcmpeqb
__m64 _m_pcmpeqb (__m64 a, __m64 b)
pcmpeqd
__m64 _m_pcmpeqd (__m64 a, __m64 b)
pcmpeqw
__m64 _m_pcmpeqw (__m64 a, __m64 b)
pcmpgtb
__m64 _m_pcmpgtb (__m64 a, __m64 b)
pcmpgtd
__m64 _m_pcmpgtd (__m64 a, __m64 b)
pcmpgtw
__m64 _m_pcmpgtw (__m64 a, __m64 b)
pmaddwd
__m64 _m_pmaddwd (__m64 a, __m64 b)
pmulhw
__m64 _m_pmulhw (__m64 a, __m64 b)
pmullw
__m64 _m_pmullw (__m64 a, __m64 b)
por
__m64 _m_por (__m64 a, __m64 b)
pslld
__m64 _m_pslld (__m64 a, __m64 count)
pslld
__m64 _m_pslldi (__m64 a, int imm8)
psllq
__m64 _m_psllq (__m64 a, __m64 count)
psllq
__m64 _m_psllqi (__m64 a, int imm8)
psllw
__m64 _m_psllw (__m64 a, __m64 count)
psllw
__m64 _m_psllwi (__m64 a, int imm8)
psrad
__m64 _m_psrad (__m64 a, __m64 count)
psrad
__m64 _m_psradi (__m64 a, int imm8)
psraw
__m64 _m_psraw (__m64 a, __m64 count)
psraw
__m64 _m_psrawi (__m64 a, int imm8)
psrld
__m64 _m_psrld (__m64 a, __m64 count)
psrld
__m64 _m_psrldi (__m64 a, int imm8)
psrlq
__m64 _m_psrlq (__m64 a, __m64 count)
psrlq
__m64 _m_psrlqi (__m64 a, int imm8)
psrlw
__m64 _m_psrlw (__m64 a, __m64 count)
psrlw
__m64 _m_psrlwi (__m64 a, int imm8)
psubb
__m64 _m_psubb (__m64 a, __m64 b)
psubd
__m64 _m_psubd (__m64 a, __m64 b)
psubsb
__m64 _m_psubsb (__m64 a, __m64 b)
psubsw
__m64 _m_psubsw (__m64 a, __m64 b)
psubusb
__m64 _m_psubusb (__m64 a, __m64 b)
psubusw
__m64 _m_psubusw (__m64 a, __m64 b)
psubw
__m64 _m_psubw (__m64 a, __m64 b)
punpckhbw
__m64 _m_punpckhbw (__m64 a, __m64 b)
punpckhdq
__m64 _m_punpckhdq (__m64 a, __m64 b)
punpcklbw
__m64 _m_punpckhwd (__m64 a, __m64 b)
punpcklbw
__m64 _m_punpcklbw (__m64 a, __m64 b)
punpckldq
__m64 _m_punpckldq (__m64 a, __m64 b)
punpcklwd
__m64 _m_punpcklwd (__m64 a, __m64 b)
pxor
__m64 _m_pxor (__m64 a, __m64 b)
...
__m64 _mm_set_pi16 (short e3, short e2, short e1, short e0)
...
__m64 _mm_set_pi32 (int e1, int e0)
...
__m64 _mm_set_pi8 (char e7, char e6, char e5, char e4, char e3, char e2, char e1, char e0)
...
__m64 _mm_set1_pi16 (short a)
...
__m64 _mm_set1_pi32 (int a)
...
__m64 _mm_set1_pi8 (char a)
...
__m64 _mm_setr_pi16 (short e3, short e2, short e1, short e0)
...
__m64 _mm_setr_pi32 (int e1, int e0)
...
__m64 _mm_setr_pi8 (char e7, char e6, char e5, char e4, char e3, char e2, char e1, char e0)
pxor
__m64 _mm_setzero_si64 (void)
psllw
__m64 _mm_sll_pi16 (__m64 a, __m64 count)
pslld
__m64 _mm_sll_pi32 (__m64 a, __m64 count)
psllq
__m64 _mm_sll_si64 (__m64 a, __m64 count)
psllw
__m64 _mm_slli_pi16 (__m64 a, int imm8)
pslld
__m64 _mm_slli_pi32 (__m64 a, int imm8)
psllq
__m64 _mm_slli_si64 (__m64 a, int imm8)
psraw
__m64 _mm_sra_pi16 (__m64 a, __m64 count)
psrad
__m64 _mm_sra_pi32 (__m64 a, __m64 count)
psraw
__m64 _mm_srai_pi16 (__m64 a, int imm8)
psrad
__m64 _mm_srai_pi32 (__m64 a, int imm8)
psrlw
__m64 _mm_srl_pi16 (__m64 a, __m64 count)
psrld
__m64 _mm_srl_pi32 (__m64 a, __m64 count)
psrlq
__m64 _mm_srl_si64 (__m64 a, __m64 count)
psrlw
__m64 _mm_srli_pi16 (__m64 a, int imm8)
psrld
__m64 _mm_srli_pi32 (__m64 a, int imm8)
psrlq
__m64 _mm_srli_si64 (__m64 a, int imm8)
psubw
__m64 _mm_sub_pi16 (__m64 a, __m64 b)
psubd
__m64 _mm_sub_pi32 (__m64 a, __m64 b)
psubb
__m64 _mm_sub_pi8 (__m64 a, __m64 b)
psubsw
__m64 _mm_subs_pi16 (__m64 a, __m64 b)
psubsb
__m64 _mm_subs_pi8 (__m64 a, __m64 b)
psubusw
__m64 _mm_subs_pu16 (__m64 a, __m64 b)
psubusb
__m64 _mm_subs_pu8 (__m64 a, __m64 b)
movd
int _m_to_int (__m64 a)
movq
__int64 _m_to_int64 (__m64 a)
punpcklbw
__m64 _mm_unpackhi_pi16 (__m64 a, __m64 b)
punpckhdq
__m64 _mm_unpackhi_pi32 (__m64 a, __m64 b)
punpckhbw
__m64 _mm_unpackhi_pi8 (__m64 a, __m64 b)
punpcklwd
__m64 _mm_unpacklo_pi16 (__m64 a, __m64 b)
punpckldq
__m64 _mm_unpacklo_pi32 (__m64 a, __m64 b)
punpcklbw
__m64 _mm_unpacklo_pi8 (__m64 a, __m64 b)
pxor
__m64 _mm_xor_si64 (__m64 a, __m64 b)

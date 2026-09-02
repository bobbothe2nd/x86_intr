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

  * [ ] [`_aadd_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aadd_i32)
  * [ ] [`_aadd_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aadd_i64)
  * [ ] [`_aand_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aand_i32)
  * [ ] [`_aand_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aand_i64)
  * [ ] [`_aor_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aor_i32)
  * [ ] [`_aor_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_aor_i64)
  * [ ] [`_axor_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_axor_i32)
  * [ ] [`_axor_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_axor_i64)
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

  * [ ] [`_xresldtrk`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_xresldtrk)
  * [ ] [`_xsusldtrk`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_xsusldtrk)
</p></details>


<details><summary>["UINTR"]</summary><p>

  * [ ] [`_clui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_clui)
  * [ ] [`_senduipi`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_senduipi)
  * [ ] [`_stui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_stui)
  * [ ] [`_testui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_testui)
</p></details>


<details><summary>["USER_MSR"]</summary><p>

  * [ ] [`_urdmsr`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_urdmsr)
  * [ ] [`_uwrmsr`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_uwrmsr)
</p></details>


<details><summary>["WAITPKG"]</summary><p>

  * [x] [`_tpause`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_tpause)
  * [x] [`_umonitor`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_umonitor)
  * [x] [`_umwait`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_umwait)
</p></details>


<details><summary>["WBNOINVD"]</summary><p>

  * [x] [`_wbnoinvd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wbnoinvd)
</p></details>

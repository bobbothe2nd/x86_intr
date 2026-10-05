# `x86_intr`

Additional x86/x86-64 intrinsics for instructions and ISA extensions not yet available through stable `core::arch`. These intrinsics will probably be deprecated as new intrinsics are stabilized/added in `core::arch`. More intrinsics will be added soon.

## Intrinsics Included

All intrinsics are exported in `x86_intr::arch`.

Intel intrinsics:

* [`_clrssbsy`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_clrssbsy)
* [`_get_ssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_get_ssp)
* [`_get_ssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_get_ssp)
* [`_inc_ssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_inc_ssp)
* [`_incsspd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_incsspd)
* [`_incsspq`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_incsspq)
* [`_rdsspd_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rdsspd_i32)
* [`_rdsspq_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rdsspq_i64)
* [`_rstorssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rstorssp)
* [`_saveprevssp`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_saveprevssp)
* [`_setssbsy`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_setssbsy)
* [`_wrssd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrssd)
* [`_wrssq`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrssq)
* [`_wrussd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrussd)
* [`_wrussq`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wrussq)
* [`_mm_cldemote`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cldemote)
* [`_mm_clwb`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_clwb)
* [`_enqcmd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_enqcmd)
* [`_enqcmds`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_enqcmds)
* [`_readfsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readfsbase_u32)
* [`_readfsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readfsbase_u64)
* [`_readgsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readgsbase_u32)
* [`_readgsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_readgsbase_u64)
* [`_writefsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writefsbase_u32)
* [`_writefsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writefsbase_u64)
* [`_writegsbase_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writegsbase_u32)
* [`_writegsbase_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_writegsbase_u64)
* [`_hreset`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_hreset)
* [`_invpcid`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_invpcid)
* [`_mm_monitor`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_monitor)
* [`_mm_mwait`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mwait)
* [`_loadbe_i16`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_loadbe_i16)
* [`_loadbe_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_loadbe_i32)
* [`_loadbe_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_loadbe_i64)
* [`_storebe_i16`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_storebe_i16)
* [`_storebe_i32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_storebe_i32)
* [`_storebe_i64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_storebe_i64)
* [`_movdir64b`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_movdir64b)
* [`_directstoreu_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_directstoreu_u32)
* [`_directstoreu_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_directstoreu_u64)
* [`_mm_popcnt_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_popcnt_u32)
* [`_mm_popcnt_u64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_popcnt_u64)
* [`_rdpid_u32`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_rdpid_u32)
* [`_serialize`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_serialize)
* [`_xresldtrk`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_xresldtrk)
* [`_xsusldtrk`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_xsusldtrk)
* [`_clui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_clui)
* [`_senduipi`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_senduipi)
* [`_stui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_stui)
* [`_testui`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_testui)
* [`_urdmsr`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_urdmsr)
* [`_uwrmsr`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_uwrmsr)
* [`_tpause`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_tpause)
* [`_umonitor`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_umonitor)
* [`_umwait`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_umwait)
* [`_wbnoinvd`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_wbnoinvd)

AMD intrinsics:

* `_mm_clzero`
* `_monitorx`
* `_mwaitx` (and `mwaitx_no_timeout`/`mwaitx_asm` for optimization)

Other intrinsics:

* `_rdpkru`
* `_wrpkru`

## Feature Detection

This is different from `std::arch::is_x86_feature_detected` because it only checks CPUID bits. This does not guarantee some features are usable, but the instructions related to that feature will be encoded. All intrinsics in this crate will document exactly what they require to be safe.

```rust
if x86_intr::is_cpuid_feature_detected("waitpkg", "tsc") {
    while !flag.load(Ordering::Acquire) {
        unsafe {
            x86_intr::arch::_umonitor(&raw const flag);

            let start = core::arch::x86_64::_rdtsc();

            x86_intr::arch::_umwait(1, start + 10_000);
        }
    }
} else {
    while !flag.load(Ordering::Acquire) {
        spin_loop();
    }
}
```

## Safety

This crate uses a lot of inline assembly. This is because not all x86_64 instructions are represented through `core::arch::x86_64`. This crate aims to improve that coverage by using inline assembly.

Make sure to check for the appropriate CPUID features for each instruction before using it.

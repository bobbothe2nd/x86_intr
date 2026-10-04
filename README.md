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

## Performance

A useful benchmark is comparing `umontior`/`umwait` to `park`/`unpark` and `spin_loop`:

```text
delay: 1µs
  spin         mean  459.467µs median  540.900µs p95  788.000µs p99  968.800µs iterations     9692.6
  umonitor     mean  500.366µs median  541.400µs p95  777.000µs p99  876.100µs iterations        9.6
  park         mean  599.592µs median  631.700µs p95  910.600µs p99    1.230ms iterations        0.0

delay: 10µs
  spin         mean  630.562µs median  540.800µs p95    1.213ms p99    2.504ms iterations    10000.0
  umonitor     mean  532.741µs median  537.900µs p95  657.500µs p99    1.186ms iterations       10.1
  park         mean  641.643µs median  631.100µs p95    1.047ms p99    1.347ms iterations        0.0

delay: 100µs
  spin         mean  745.528µs median  585.300µs p95    1.804ms p99    4.620ms iterations     8586.0
  umonitor     mean  969.090µs median  759.400µs p95    2.108ms p99    2.928ms iterations        5.7
  park         mean  829.893µs median  748.600µs p95    1.393ms p99    1.568ms iterations        0.0

delay: 1ms
  spin         mean    1.634ms median    1.620ms p95    2.198ms p99    4.786ms iterations    29874.7
  umonitor     mean    1.815ms median    1.546ms p95    4.899ms p99    9.372ms iterations       28.3
  park         mean    1.681ms median    1.599ms p95    2.526ms p99    3.702ms iterations        0.0

delay: 10ms
  spin         mean   10.458ms median   10.391ms p95   11.026ms p99   14.694ms iterations   156947.1
  umonitor     mean   10.397ms median   10.365ms p95   10.854ms p99   11.176ms iterations      191.6
  park         mean   10.725ms median   10.648ms p95   11.479ms p99   12.699ms iterations        0.0

delay: 100ms
  spin         mean  100.671ms median  100.420ms p95  100.965ms p99  122.762ms iterations  1489072.1
  umonitor     mean  100.474ms median  100.384ms p95  100.776ms p99  107.708ms iterations     1841.3
  park         mean  100.637ms median  100.614ms p95  100.891ms p99  103.158ms iterations        0.0
```

samples: 100, warmup: 10

Using a monitor can watch a memory address for changes non-atomically while avoiding data races. It does have on major caveat though. It sometimes can return early for too long timeouts. Thats why the benchmark includes an iteration counter. It still has significantly less loads than a simple spin loop.

It's also important to check if the `waitpkg` feature is available on the target. For this reason, `x86_intr` provides a macro for checking CPUID features. It not only supports more features than `is_cpuid_feature_detected`, it also doesn't require an operating system. It has the same API too.

Another important note is that Intel `umwait` and AMD `mwaitx` are significantly different. `mwaitx` has to temporarily allocate a register to avoid clobbering `ebx`. It sounds simple, but it can hurt performance. To avoid this, there are two `mwaitx` intrinsics exposed by this crate: `_mwaitx` and `mwaitx_no_timeout`. `mwaitx_no_timeout` doesn't have a leading underscore because it isn't an Intel intrinsic. Here, it's necessary because crates can't use LLVM intrinsics and `bx` is reserved by LLVM. More information in the documentation of `_mwaitx`.

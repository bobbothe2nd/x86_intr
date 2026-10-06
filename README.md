# `x86_intr`

Additional x86/x86-64 intrinsics for instructions and ISA extensions not yet available through stable `core::arch`. These intrinsics will probably be deprecated as new intrinsics are stabilized/added in `core::arch`. More intrinsics will be added soon.

## Intrinsics Included

All intrinsics are exported in `x86_intr::arch`.

Intel intrinsics:

* [CET_SS](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=CET_SS)
* [CLDEMOTE](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=CLDEMOTE)
* [CLWB](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=CLWB)
* [ENQCMD](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=ENQCMD)
* [FSGSBASE](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=FSGSBASE)
* [HRESET](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=HRESET)
* [INVPCID](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=INVPCID)
* [MONITOR](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=MONITOR)
* [MOVBE](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=MOVBE)
* [MOVDIR64B](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=MOVDIR64B)
* [MOVDIRI](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=MOVDIRI)
* [POPCNT](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=POPCNT)
* [RAO_INT](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=RAO_INT)
* [RDPID](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=RDPID)
* [SERIALIZE](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=SERIALIZE)
* [TSXLDTRK](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=TSXLDTRK)
* [TSXLDTRK](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=TSXLDTRK)
* [UINTR](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=UINTR)
* [USER_MSR](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=USER_MSR)
* [WAITPKG](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=WAITPKG)
* [WBNOINVD](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#othertechs=WBNOINVD)

AMD intrinsics:

* `_mm_clzero`
* `_monitorx`
* `_mwaitx` (and `mwaitx_no_timeout`/`mwaitx_asm` for optimization)
* `_rdpru`

Other intrinsics:

* `_rdpkru`
* `_wrpkru`

Experimental support for:

* [AMX](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#techs=AMX)
* [MMX](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#techs=MMX)

AMX is currently unstable and MMX will likely never be supported by Rust due to being almost completely obsolete.

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

Note TSC is guaranteed support by x86_64

## Safety

This crate uses a lot of inline assembly. This is because not all x86_64 instructions are represented through `core::arch::x86_64`. This crate aims to improve that coverage by using inline assembly.

Make sure to check for the appropriate CPUID features for each instruction before using it.

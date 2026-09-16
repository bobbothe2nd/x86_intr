/// Tests a bit in CPUID feature leaves by their literal name or explicitly.
///
/// ```rust
/// if x86_intr::is_cpuid_feature_detected!("fpu") {
///     // there is an onboard x87 FPU
/// }
/// ```
#[macro_export]
macro_rules! is_cpuid_feature_detected {
    (CPUID.1.0:ecx[$bit:tt]) => {
        ($crate::test::cpuid_1_0_ecx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.1.0:edx[$bit:tt]) => {
        ($crate::test::cpuid_1_0_edx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.7.0:eax[$bit:tt]) => {
        ($crate::test::cpuid_7_0_eax() & (1u32 << $bit)) != 0u32
    };

    (CPUID.7.0:ebx[$bit:tt]) => {
        ($crate::test::cpuid_7_0_ebx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.7.0:ecx[$bit:tt]) => {
        ($crate::test::cpuid_7_0_ecx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.7.0:edx[$bit:tt]) => {
        ($crate::test::cpuid_7_0_edx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.0x8000_0001 .0:edx[$bit:tt]) => {
        ($crate::test::cpuid_8000_0001_0_edx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.0x8000_0008 .0:ebx[$bit:tt]) => {
        ($crate::test::cpuid_8000_0008_0_ebx() & (1u32 << $bit)) != 0u32
    };

    (CPUID.$leaf:tt.$subleaf:tt:$reg:ident[$bit:tt]) => {
        (::core::arch::x86_64::__cpuid_count($leaf, $subleaf).$reg & (1u32 << $bit)) != 0u32
    };

    ("fpu") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[0])
        }
    };

    ("vme") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[1])
        }
    };

    ("de") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[2])
        }
    };

    ("pse") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[3])
        }
    };

    ("tsc") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[4])
        }
    };

    ("msr") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[5])
        }
    };

    ("pae") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[6])
        }
    };

    ("mce") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[7])
        }
    };

    ("cx8") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[8])
        }
    };

    ("apic") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[9])
        }
    };

    ("sep") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[11])
        }
    };

    ("mtrr") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[12])
        }
    };

    ("pge") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[13])
        }
    };

    ("mca") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[14])
        }
    };

    ("cmov") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[15])
        }
    };

    ("pat") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[16])
        }
    };

    ("pse-36") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[17])
        }
    };

    ("psn") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[18])
        }
    };

    ("clfsh") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[19])
        }
    };

    ("ds") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[21])
        }
    };

    ("acpi") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[22])
        }
    };

    ("mmx") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[23])
        }
    };

    ("fxsr") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[24])
        }
    };

    ("sse") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[25])
        }
    };

    ("sse2") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[26])
        }
    };

    ("ss") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[27])
        }
    };

    ("htt") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[28])
        }
    };

    ("tm") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[29])
        }
    };

    ("ia64") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[30])
        }
    };

    ("pbe") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:edx[31])
        }
    };

    ("sse3") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[0])
        }
    };

    ("pclmulqdq") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[1])
        }
    };

    ("dtes64") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[2])
        }
    };

    ("monitor") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[3])
        }
    };

    ("ds-cpl") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[4])
        }
    };

    ("vmx") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[5])
        }
    };

    ("smx") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[6])
        }
    };

    ("est") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[7])
        }
    };

    ("tm2") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[8])
        }
    };

    ("ssse3") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[9])
        }
    };

    ("cnxt-id") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[10])
        }
    };

    ("sdbg") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[11])
        }
    };

    ("fma") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[12])
        }
    };

    ("cx16") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[13])
        }
    };

    ("xptr") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[14])
        }
    };

    ("pdcm") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[15])
        }
    };

    ("pcid") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[17])
        }
    };

    ("dca") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[18])
        }
    };

    ("sse4.1") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[19])
        }
    };

    ("sse4.2") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[20])
        }
    };

    ("x2apic") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[21])
        }
    };

    ("movbe") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[22])
        }
    };

    ("popcnt") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[23])
        }
    };

    ("tsc-deadline") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[24])
        }
    };

    ("aes-ni") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[25])
        }
    };

    ("xsave") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[26])
        }
    };

    ("osxsave") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[27])
        }
    };

    ("avx") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[28])
        }
    };

    ("f16c") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[20])
        }
    };

    ("rdmd") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[30])
        }
    };

    ("hypervisor") => {
        if $crate::test::cpuid_0_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1.0:ecx[31])
        }
    };

    ("fsgsbase") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[0])
        }
    };

    ("tsc_adjust") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[1])
        }
    };

    ("sgx") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[2])
        }
    };

    ("bmi1") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[3])
        }
    };

    ("hle") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[4])
        }
    };

    ("avx2") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[5])
        }
    };

    ("fdp-excptn-only") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[6])
        }
    };

    ("smep") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[7])
        }
    };

    ("bmi2") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[8])
        }
    };

    ("erms") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[9])
        }
    };

    ("invpcid") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[10])
        }
    };

    ("rtm") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[11])
        }
    };

    ("rdt-m") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_intel_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[12])
        }
    }};

    ("pqm") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_amd_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[12])
        }
    }};

    ("fcs_fds_deprecation") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[13])
        }
    };

    ("mpx") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[14])
        }
    };

    ("rdt-a") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_intel_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[15])
        }
    }};

    ("pqe") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_amd_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[15])
        }
    }};

    ("avx512-f") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[16])
        }
    };

    ("avx512-dq") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[17])
        }
    };

    ("rdseed") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[18])
        }
    };

    ("adx") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[19])
        }
    };

    ("smap") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[20])
        }
    };

    ("avx512-ifma") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[21])
        }
    };

    ("pcommit") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[22])
        }
    };

    ("clflushopt") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[23])
        }
    };

    ("clwb") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[24])
        }
    };

    ("pt") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[25])
        }
    };

    ("avx512-pf") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[26])
        }
    };

    ("avx512-er") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[27])
        }
    };

    ("avx512-cd") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[28])
        }
    };

    ("sha") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[29])
        }
    };

    ("avx512-bw") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[30])
        }
    };

    ("avx512-vl") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ebx[31])
        }
    };

    ("prefetchwt1") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[0])
        }
    };

    ("avx512-vbmi") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[1])
        }
    };

    ("umip") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[2])
        }
    };

    ("pku") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[3])
        }
    };

    ("ospke") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[4])
        }
    };

    ("waitpkg") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[5])
        }
    };

    ("avx512-vbmi2") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[6])
        }
    };

    ("cet_ss") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[7])
        }
    };

    ("shstk") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[7])
        }
    };

    ("gfni") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[8])
        }
    };

    ("vaes") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[9])
        }
    };

    ("vpclmulqdq") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[10])
        }
    };

    ("avx512-vnni") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[11])
        }
    };

    ("avx512-bitalg") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[12])
        }
    };

    ("tme_en") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[13])
        }
    };

    ("avx512-vpopcntdq") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[14])
        }
    };

    ("la57") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[16])
        }
    };

    ("rdpid") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[22])
        }
    };

    ("kl") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[23])
        }
    };

    ("bus-lock-detect") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[24])
        }
    };

    ("cldemote") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[25])
        }
    };

    ("movdiri") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[27])
        }
    };

    ("movdir64b") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[28])
        }
    };

    ("enqcmd") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[29])
        }
    };

    ("sgx-lc") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[30])
        }
    };

    ("pks") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:ecx[31])
        }
    };

    ("sgx-keys") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[1])
        }
    };

    ("avx512-4vnniw") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[2])
        }
    };

    ("avx512-4fmaps") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[3])
        }
    };

    ("fsrm") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[4])
        }
    };

    ("uintr") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[5])
        }
    };

    ("avx512-vp2intersect") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[8])
        }
    };

    ("srbds-ctrl") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[9])
        }
    };

    ("md-clear") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[10])
        }
    };

    ("rtm-always-abort") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[11])
        }
    };

    ("rtm-force-abort") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[13])
        }
    };

    ("serialize") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[14])
        }
    };

    ("hybrid") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[15])
        }
    };

    ("tsxldtrk") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[16])
        }
    };

    ("pconfig") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[18])
        }
    };

    ("lbr") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[19])
        }
    };

    ("cet-ibt") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[20])
        }
    };

    ("amx-bf16") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[22])
        }
    };

    ("avx512-fp16") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[23])
        }
    };

    ("amx-tile") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[24])
        }
    };

    ("amx-int8") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[25])
        }
    };

    ("ibrs") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[26])
        }
    };

    ("stibp") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[27])
        }
    };

    ("ssbd") => {
        if $crate::test::cpuid_0_0_eax() < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7.0:edx[31])
        }
    };

    ("sha256") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[0])
        }
    };

    ("sm3") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[1])
        }
    };

    ("sm4") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[2])
        }
    };

    ("rao-int") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[3])
        }
    };

    ("avx-vnni") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[4])
        }
    };

    ("avx512-bf16") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[5])
        }
    };

    ("lass") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[6])
        }
    };

    ("cmpccxadd") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[7])
        }
    };

    ("archperfmonext") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[8])
        }
    };

    ("fzrm") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[10])
        }
    };

    ("fsrcs") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[11])
        }
    };

    ("fred") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[17])
        }
    };

    ("lkgs") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[18])
        }
    };

    ("wrmsms") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[19])
        }
    };

    ("nmi_src") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[20])
        }
    };

    ("amx-fp16") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[21])
        }
    };

    ("hreset") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[22])
        }
    };

    ("avx-ifma") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[23])
        }
    };

    ("lam") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[26])
        }
    };

    ("msrlist") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[27])
        }
    };

    ("invd_disable_post_bios_done") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[30])
        }
    };

    ("movrs") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[31])
        }
    };

    ("ppin") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ebx[0])
        }
    };

    ("pbndkb") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ebx[1])
        }
    };

    ("rdt_m_asym") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[0])
        }
    };

    ("rdt_a_asym") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[1])
        }
    };

    ("msr_imm") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[5])
        }
    };

    ("ace") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[11])
        }
    };

    ("avx-vnni-int8") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[4])
        }
    };

    ("avx-ne-convert") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[5])
        }
    };

    ("amx-complex") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[8])
        }
    };

    ("avx-vnni-int16") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[10])
        }
    };

    ("utmr") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[13])
        }
    };

    ("prefetchi") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[14])
        }
    };

    ("user_msr") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[15])
        }
    };

    ("uiret-uif-from-rflags") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[17])
        }
    };

    ("cet-sss") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[18])
        }
    };

    ("avx10") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[19])
        }
    };

    ("APX_F") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[21])
        }
    };

    ("SEC_TEE_ATTESTATION") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[22])
        }
    };

    ("mwait") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[23])
        }
    };

    ("slsm") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[24])
        }
    };

    ("psfd") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[0])
        }
    };

    ("ipred_ctrl") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[1])
        }
    };

    ("rrsba_ctrl") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[2])
        }
    };

    ("ddpd_u") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[3])
        }
    };

    ("bhi_ctrl") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[4])
        }
    };

    ("mcdt_no") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[5])
        }
    };

    ("uc_lock_disable") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[6])
        }
    };

    ("monitor_mitg_no") => {
        if $crate::test::cpuid_0_0_eax() < 7
        || $crate::test::cpuid_7_0_eax() < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[7])
        }
    };

    ("syscall") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[11])
            || $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[10])
        }
    };

    ("nx") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[20])
        }
    };

    ("sem") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[21])
        }
    };

    ("mmxext") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[22])
        }
    };

    ("lm") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[29])
        }
    };

    ("3dnowext") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[30])
        }
    };

    ("3dnow") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[31])
        }
    };

    ("lahf_lm") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[0])
        }
    };

    ("cmp_legacy") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[1])
        }
    };

    ("svm") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[2])
        }
    };

    ("extapic") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[3])
        }
    };

    ("abm") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[4])
        }
    };

    ("sse4a") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[5])
        }
    };

    ("misalignedsse") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[6])
        }
    };

    ("3dnowprefetch") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[7])
        }
    };

    ("osvw") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[8])
        }
    };

    ("ibs") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[9])
        }
    };

    ("xop") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[10])
        }
    };

    ("skinit") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[11])
        }
    };

    ("wdt") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[12])
        }
    };

    ("lwp") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[15])
        }
    };

    ("fma4") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[16])
        }
    };

    ("tce") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[17])
        }
    };

    ("nodeid_msr") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[19])
        }
    };

    ("tbm") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[21])
        }
    };

    ("topoext") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[22])
        }
    };

    ("perfctr_core") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[23])
        }
    };

    ("perfctr_nb") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[24])
        }
    };

    ("dbx") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[26])
        }
    };

    ("perftsc") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[27])
        }
    };

    ("monitorx") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[29])
        }
    };

    ("clzero") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[0])
        }
    };

    ("retired_instr") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[1])
        }
    };

    ("xrstor_fp_err") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[2])
        }
    };

    ("invlpgb") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[3])
        }
    };

    ("rdpru") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[4])
        }
    };

    ("xotext") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[5])
        }
    };

    ("mbe") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[6])
        }
    };

    ("mcommit") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[8])
        }
    };

    ("wbnoinvd") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[9])
        }
    };

    ("LBR_EXT_V1") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[10])
        }
    };

    ("IBPB") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[12])
        }
    };

    ("wbinvd_int") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[13])
        }
    };

    ("IBRS") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[14])
        }
    };

    ("STIBP") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[15])
        }
    };

    ("ibrsAlwaysOn") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[16])
        }
    };

    ("StibpAlwaysOn") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[17])
        }
    };

    ("ibrs_preferred") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[18])
        }
    };

    ("ibrs_same_mode_protection") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[19])
        }
    };

    ("no_efer_lmsle") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[20])
        }
    };

    ("invlpgb_nested") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[21])
        }
    };

    ("LBR_TSX") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[22])
        }
    };

    ("ppin") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[23])
        }
    };

    ("ssbd") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[24])
        }
    };

    ("ssbd_legacy") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[25])
        }
    };

    ("ssbd_no") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[26])
        }
    };

    ("cppc") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[27])
        }
    };

    ("psfd") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[28])
        }
    };

    ("btc_no") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[29])
        }
    };

    ("IBPB_RET") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[30])
        }
    };

    ("branch_sampling") => {
        if $crate::test::cpuid_8000_0000_0_eax() < 0x8000_0008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0008 .0:ebx[31])
        }
    };

    ($($features:tt),*) => {
        (true $(
            && $crate::is_cpuid_feature_detected!($features)
        )*)
    }
}

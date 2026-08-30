/// Tests a bit in CPUID feature leaves by their literal name or explicitly.
///
/// ```rust
/// if x86_intr::is_cpuid_feature_detected!("fpu") {
///     // there is an onboard x87 FPU
/// }
/// ```
#[macro_export]
macro_rules! is_cpuid_feature_detected {
    (CPUID.$leaf:tt.$subleaf:tt:$reg:ident[$bit:tt]) => {
        (::core::arch::x86_64::__cpuid_count($leaf, $subleaf).$reg & (1u32 << $bit)) != 0u32
    };

    ("fpu") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[0])
        }
    };

    ("vme") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[1])
        }
    };

    ("de") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[2])
        }
    };

    ("pse") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[3])
        }
    };

    ("tsc") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[4])
        }
    };

    ("msr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[5])
        }
    };

    ("pae") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[6])
        }
    };

    ("mce") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[7])
        }
    };

    ("cx8") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[8])
        }
    };

    ("apic") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[9])
        }
    };

    ("sep") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[11])
        }
    };

    ("mtrr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[12])
        }
    };

    ("pge") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[13])
        }
    };

    ("mca") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[14])
        }
    };

    ("cmov") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[15])
        }
    };

    ("pat") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[16])
        }
    };

    ("pse-36") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[17])
        }
    };

    ("psn") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[18])
        }
    };

    ("clfsh") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[19])
        }
    };

    ("ds") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[21])
        }
    };

    ("acpi") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[22])
        }
    };

    ("mmx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[23])
        }
    };

    ("fxsr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[24])
        }
    };

    ("sse") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[25])
        }
    };

    ("sse2") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[26])
        }
    };

    ("ss") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[27])
        }
    };

    ("htt") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[28])
        }
    };

    ("tm") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[29])
        }
    };

    ("ia64") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[30])
        }
    };

    ("pbe") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:edx[31])
        }
    };

    ("sse3") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[0])
        }
    };

    ("pclmulqdq") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[1])
        }
    };

    ("dtes64") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[2])
        }
    };

    ("monitor") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[3])
        }
    };

    ("ds-cpl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[4])
        }
    };

    ("vmx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[5])
        }
    };

    ("smx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[6])
        }
    };

    ("est") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[7])
        }
    };

    ("tm2") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[8])
        }
    };

    ("ssse3") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[9])
        }
    };

    ("cnxt-id") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[10])
        }
    };

    ("sdbg") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[11])
        }
    };

    ("fma") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[12])
        }
    };

    ("cx16") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[13])
        }
    };

    ("xptr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[14])
        }
    };

    ("pdcm") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[15])
        }
    };

    ("pcid") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[17])
        }
    };

    ("dca") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[18])
        }
    };

    ("sse4.1") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[19])
        }
    };

    ("sse4.2") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[20])
        }
    };

    ("x2apic") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[21])
        }
    };

    ("movbe") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[22])
        }
    };

    ("popcnt") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[23])
        }
    };

    ("tsc-deadline") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[24])
        }
    };

    ("aes-ni") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[25])
        }
    };

    ("xsave") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[26])
        }
    };

    ("osxsave") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[27])
        }
    };

    ("avx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[28])
        }
    };

    ("f16c") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[20])
        }
    };

    ("rdmd") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[30])
        }
    };

    ("hypervisor") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.1 .0:ecx[31])
        }
    };

    ("fsgsbase") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[0])
        }
    };

    ("tsc_adjust") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[1])
        }
    };

    ("sgx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[2])
        }
    };

    ("bmi1") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[3])
        }
    };

    ("hle") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[4])
        }
    };

    ("avx2") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[5])
        }
    };

    ("fdp-excptn-only") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[6])
        }
    };

    ("smep") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[7])
        }
    };

    ("bmi2") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[8])
        }
    };

    ("erms") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[9])
        }
    };

    ("invpcid") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[10])
        }
    };

    ("rtm") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[11])
        }
    };

    ("rdt-m") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_intel_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[12])
        }
    }};

    ("pqm") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_amd_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[12])
        }
    }};

    ("fcs_fds_deprecation") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[13])
        }
    };

    ("mpx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[14])
        }
    };

    ("rdt-a") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_intel_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[15])
        }
    }};

    ("pqe") => {{
        let cpuid = ::core::arch::x86_64::__cpuid_count(0, 0);
        if cpuid.eax < 7 || $crate::arch::is_amd_raw(cpuid) {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[15])
        }
    }};

    ("avx512-f") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[16])
        }
    };

    ("avx512-dq") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[17])
        }
    };

    ("rdseed") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[18])
        }
    };

    ("adx") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[19])
        }
    };

    ("smap") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[20])
        }
    };

    ("avx512-ifma") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[21])
        }
    };

    ("pcommit") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[22])
        }
    };

    ("clflushopt") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[23])
        }
    };

    ("clwb") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[24])
        }
    };

    ("pt") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[25])
        }
    };

    ("avx512-pf") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[26])
        }
    };

    ("avx512-er") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[27])
        }
    };

    ("avx512-cd") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[28])
        }
    };

    ("sha") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[29])
        }
    };

    ("avx512-bw") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[30])
        }
    };

    ("avx512-vl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ebx[31])
        }
    };

    ("prefetchwt1") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[0])
        }
    };

    ("avx512-vbmi") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[1])
        }
    };

    ("umip") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[2])
        }
    };

    ("pku") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[3])
        }
    };

    ("ospke") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[4])
        }
    };

    ("waitpkg") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[5])
        }
    };

    ("avx512-vbmi2") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[6])
        }
    };

    ("cet_ss") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[7])
        }
    };

    ("shstk") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[7])
        }
    };

    ("gfni") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[8])
        }
    };

    ("vaes") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[9])
        }
    };

    ("vpclmulqdq") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[10])
        }
    };

    ("avx512-vnni") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[11])
        }
    };

    ("avx512-bitalg") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[12])
        }
    };

    ("tme_en") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[13])
        }
    };

    ("avx512-vpopcntdq") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[14])
        }
    };

    ("la57") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[16])
        }
    };

    ("rdpid") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[22])
        }
    };

    ("kl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[23])
        }
    };

    ("bus-lock-detect") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[24])
        }
    };

    ("cldemote") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[25])
        }
    };

    ("movdiri") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[27])
        }
    };

    ("movdir64b") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[28])
        }
    };

    ("enqcmd") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[29])
        }
    };

    ("sgx-lc") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[30])
        }
    };

    ("pks") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:ecx[31])
        }
    };

    ("sgx-keys") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[1])
        }
    };

    ("avx512-4vnniw") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[2])
        }
    };

    ("avx512-4fmaps") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[3])
        }
    };

    ("fsrm") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[4])
        }
    };

    ("uintr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[5])
        }
    };

    ("avx512-vp2intersect") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[8])
        }
    };

    ("srbds-ctrl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[9])
        }
    };

    ("md-clear") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[10])
        }
    };

    ("rtm-always-abort") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[11])
        }
    };

    ("rtm-force-abort") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[13])
        }
    };

    ("serialize") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[14])
        }
    };

    ("hybrid") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[15])
        }
    };

    ("tsxldtrk") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[16])
        }
    };

    ("pconfig") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[18])
        }
    };

    ("lbr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[19])
        }
    };

    ("cet-ibt") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[20])
        }
    };

    ("amx-bf16") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[22])
        }
    };

    ("avx512-fp16") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[23])
        }
    };

    ("amx-tile") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[24])
        }
    };

    ("amx-int8") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[25])
        }
    };

    ("ibrs") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[26])
        }
    };

    ("stibp") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[27])
        }
    };

    ("ssbd") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .0:edx[31])
        }
    };

    ("sha256") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[0])
        }
    };

    ("sm3") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[1])
        }
    };

    ("sm4") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[2])
        }
    };

    ("rao-int") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[3])
        }
    };

    ("avx-vnni") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[4])
        }
    };

    ("avx512-bf16") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[5])
        }
    };

    ("lass") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[6])
        }
    };

    ("cmpccxadd") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[7])
        }
    };

    ("archperfmonext") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[8])
        }
    };

    ("fzrm") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[10])
        }
    };

    ("fsrcs") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[11])
        }
    };

    ("fred") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[17])
        }
    };

    ("lkgs") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[18])
        }
    };

    ("wrmsms") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[19])
        }
    };

    ("nmi_src") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[20])
        }
    };

    ("amx-fp16") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[21])
        }
    };

    ("hreset") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[22])
        }
    };

    ("avx-ifma") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[23])
        }
    };

    ("lam") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[26])
        }
    };

    ("msrlist") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[27])
        }
    };

    ("invd_disable_post_bios_done") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[30])
        }
    };

    ("movrs") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:eax[31])
        }
    };

    ("ppin") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ebx[0])
        }
    };

    ("pbndkb") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ebx[1])
        }
    };

    ("RDT_M_ASYM") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[0])
        }
    };

    ("RDT_A_ASYM") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[1])
        }
    };

    ("MSR_IMM") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[5])
        }
    };

    ("ace") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:ecx[11])
        }
    };

    ("avx-vnni-int8") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[4])
        }
    };

    ("avx-ne-convert") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[5])
        }
    };

    ("amx-complex") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[8])
        }
    };

    ("avx-vnni-int16") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[10])
        }
    };

    ("utmr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[13])
        }
    };

    ("prefetchi") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[14])
        }
    };

    ("user_msr") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[15])
        }
    };

    ("uiret-uif-from-rflags") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[17])
        }
    };

    ("cet-sss") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[18])
        }
    };

    ("avx10") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[19])
        }
    };

    ("APX_F") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[21])
        }
    };

    ("SEC_TEE_ATTESTATION") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[22])
        }
    };

    ("mwait") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[23])
        }
    };

    ("slsm") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax == 0 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .1:edx[24])
        }
    };

    ("psfd") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[0])
        }
    };

    ("ipred_ctrl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[1])
        }
    };

    ("rrsba_ctrl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[2])
        }
    };

    ("ddpd_u") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[3])
        }
    };

    ("bhi_ctrl") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[4])
        }
    };

    ("mcdt_no") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[5])
        }
    };

    ("UC_LOCK_DISABLE") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[6])
        }
    };

    ("monitor_mitg_no") => {
        if ::core::arch::x86_64::__cpuid_count(0, 0).eax < 7
        || ::core::arch::x86_64::__cpuid_count(7, 0).eax < 2 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.7 .2:edx[7])
        }
    };

    ("syscall") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[11])
            || $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[10])
        }
    };

    ("nx") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[20])
        }
    };

    ("sem") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[21])
        }
    };

    ("mmxext") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[22])
        }
    };

    ("lm") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[29])
        }
    };

    ("3dnowext") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[30])
        }
    };

    ("3dnow") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:edx[31])
        }
    };

    ("lahf_lm") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[0])
        }
    };

    ("cmp_legacy") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[1])
        }
    };

    ("svm") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[2])
        }
    };

    ("extapic") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[3])
        }
    };

    ("abm") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[4])
        }
    };

    ("sse4a") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[5])
        }
    };

    ("misalignedsse") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[6])
        }
    };

    ("3dnowprefetch") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[7])
        }
    };

    ("osvw") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[8])
        }
    };

    ("ibs") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[9])
        }
    };

    ("xop") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[10])
        }
    };

    ("skinit") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[11])
        }
    };

    ("wdt") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[12])
        }
    };

    ("lwp") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[15])
        }
    };

    ("fma4") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[16])
        }
    };

    ("tce") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[17])
        }
    };

    ("nodeid_msr") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[19])
        }
    };

    ("tbm") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[21])
        }
    };

    ("topoext") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[22])
        }
    };

    ("perfctr_core") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[23])
        }
    };

    ("perfctr_nb") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[24])
        }
    };

    ("dbx") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[26])
        }
    };

    ("perftsc") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[27])
        }
    };

    ("monitorx") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x8000_0001 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x8000_0001 .0:ecx[29])
        }
    };

    ("clzero") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[0])
        }
    };

    ("retired_instr") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[1])
        }
    };

    ("xrstor_fp_err") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[2])
        }
    };

    ("invlpgb") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[3])
        }
    };

    ("rdpru") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[4])
        }
    };

    ("xotext") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[5])
        }
    };

    ("mbe") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[6])
        }
    };

    ("mcommit") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[8])
        }
    };

    ("wbnoinvd") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[9])
        }
    };

    ("LBR_EXT_V1") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[10])
        }
    };

    ("IBPB") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[12])
        }
    };

    ("wbinvd_int") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[13])
        }
    };

    ("IBRS") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[14])
        }
    };

    ("STIBP") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[15])
        }
    };

    ("ibrsAlwaysOn") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[16])
        }
    };

    ("StibpAlwaysOn") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[17])
        }
    };

    ("ibrs_preferred") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[18])
        }
    };

    ("ibrs_same_mode_protection") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[19])
        }
    };

    ("no_efer_lmsle") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[20])
        }
    };

    ("invlpgb_nested") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[21])
        }
    };

    ("LBR_TSX") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[22])
        }
    };

    ("ppin") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[23])
        }
    };

    ("ssbd") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[24])
        }
    };

    ("ssbd_legacy") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[25])
        }
    };

    ("ssbd_no") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[26])
        }
    };

    ("cppc") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[27])
        }
    };

    ("psfd") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[28])
        }
    };

    ("btc_no") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[29])
        }
    };

    ("IBPB_RET") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[30])
        }
    };

    ("branch_sampling") => {
        if ::core::arch::x86_64::__cpuid_count(0x8000_0000, 0).eax < 0x80000008 {
            false
        } else {
            $crate::is_cpuid_feature_detected!(CPUID.0x80000008 .0:ebx[31])
        }
    };
}

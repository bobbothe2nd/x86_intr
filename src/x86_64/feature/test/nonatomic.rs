use core::arch::x86_64::__cpuid_count;

/// Get EAX register of CPUID:8000'0000.0
#[inline(always)]
pub fn cpuid_8000_0000_eax() -> u32 {
    __cpuid_count(8000_0000, 0).eax
}

/// Get EAX register of CPUID:7.0
#[inline(always)]
pub fn cpuid_7_eax() -> u32 {
    __cpuid_count(7, 0).eax
}

/// Get EAX register of CPUID:0.0
#[inline(always)]
pub fn cpuid_0_0_eax() -> u32 {
    __cpuid_count(0, 0).eax
}

/// Get EBX, ECX, EDX registers of CPUID:0.0
#[inline(always)]
pub fn cpuid_0_0_ebx_ecx_edx() -> (u32, u32, u32) {
    let cpuid = __cpuid_count(0, 0);

    (cpuid.ebx, cpuid.ecx, cpuid.edx)
}

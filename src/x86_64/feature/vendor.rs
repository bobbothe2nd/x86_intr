use core::arch::x86_64::{__cpuid_count, CpuidResult};

pub fn is_amd() -> bool {
    let cpuid = __cpuid_count(0, 0);
    is_amd_raw(cpuid)
}

pub const fn is_amd_raw(cpuid: CpuidResult) -> bool {
    cpuid.ebx == u32::from_le_bytes(*b"Auth")
        && cpuid.edx == u32::from_le_bytes(*b"enti")
        && cpuid.ecx == u32::from_le_bytes(*b"cAMD")
}

pub fn is_intel() -> bool {
    let cpuid = __cpuid_count(0, 0);
    is_intel_raw(cpuid)
}

pub const fn is_intel_raw(cpuid: CpuidResult) -> bool {
    cpuid.ebx == u32::from_le_bytes(*b"Genu")
        && cpuid.edx == u32::from_le_bytes(*b"ineI")
        && cpuid.ecx == u32::from_le_bytes(*b"ntel")
}

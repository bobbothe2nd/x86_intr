//! Check vendor from CPUID.

use super::test::cpuid_0_0_ebx_ecx_edx;

/// Checks CPUID:0.0 for "AuthenticAMD".
#[inline(always)]
pub fn is_amd() -> bool {
    let cpuid = cpuid_0_0_ebx_ecx_edx();

    cpuid.0 == u32::from_le_bytes(*b"Auth")
        && cpuid.1 == u32::from_le_bytes(*b"enti")
        && cpuid.2 == u32::from_le_bytes(*b"cAMD")
}

/// Checks CPUID:0.0 for "GenuineIntel".
#[inline(always)]
pub fn is_intel() -> bool {
    let cpuid = cpuid_0_0_ebx_ecx_edx();

    cpuid.0 == u32::from_le_bytes(*b"Genu")
        && cpuid.1 == u32::from_le_bytes(*b"ineI")
        && cpuid.2 == u32::from_le_bytes(*b"ntel")
}

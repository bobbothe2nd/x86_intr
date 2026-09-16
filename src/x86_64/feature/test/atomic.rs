use core::{
    arch::x86_64::{__cpuid_count, CpuidResult},
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};

static CPUID_SET: AtomicU8 = AtomicU8::new(0);

static CPUID_0_0_EAX: AtomicU32 = AtomicU32::new(0);
static CPUID_0_0_EBX: AtomicU32 = AtomicU32::new(0);
static CPUID_0_0_ECX: AtomicU32 = AtomicU32::new(0);
static CPUID_0_0_EDX: AtomicU32 = AtomicU32::new(0);

static CPUID_7_0_EAX: AtomicU32 = AtomicU32::new(0);

static CPUID_8000_0000_0_EAX: AtomicU32 = AtomicU32::new(0);

/// Get EAX register of CPUID:8000'0000.0
#[inline(always)]
pub fn cpuid_8000_0000_eax() -> u32 {
    if CPUID_SET.load(Ordering::Relaxed) & 2 != 0 {
        CPUID_8000_0000_0_EAX.load(Ordering::Relaxed)
    } else {
        let cpuid = __cpuid_count(8000_0000, 0);
        CPUID_8000_0000_0_EAX.store(cpuid.eax, Ordering::Relaxed);
        CPUID_SET.fetch_or(2, Ordering::Release);
        cpuid.eax
    }
}

/// Get EAX register of CPUID:7.0
#[inline(always)]
pub fn cpuid_7_eax() -> u32 {
    if CPUID_SET.load(Ordering::Relaxed) & 4 != 0 {
        CPUID_7_0_EAX.load(Ordering::Relaxed)
    } else {
        let cpuid = __cpuid_count(7, 0);
        CPUID_7_0_EAX.store(cpuid.eax, Ordering::Relaxed);
        CPUID_SET.fetch_or(4, Ordering::Release);
        cpuid.eax
    }
}

#[inline(always)]
fn set_0_0() -> CpuidResult {
    let cpuid = __cpuid_count(0, 0);

    CPUID_0_0_EAX.store(cpuid.eax, Ordering::Relaxed);
    CPUID_0_0_EBX.store(cpuid.ebx, Ordering::Relaxed);
    CPUID_0_0_ECX.store(cpuid.ecx, Ordering::Relaxed);
    CPUID_0_0_EDX.store(cpuid.edx, Ordering::Relaxed);

    CPUID_SET.fetch_or(1, Ordering::Release);

    cpuid
}

/// Get EAX register of CPUID:0.0
#[inline(always)]
pub fn cpuid_0_0_eax() -> u32 {
    if CPUID_SET.load(Ordering::Relaxed) & 1 != 0 {
        CPUID_0_0_EAX.load(Ordering::Relaxed)
    } else {
        set_0_0().eax
    }
}

/// Get EBX, ECX, EDX registers of CPUID:0.0
#[inline(always)]
pub fn cpuid_0_0_ebx_ecx_edx() -> (u32, u32, u32) {
    if CPUID_SET.load(Ordering::Relaxed) & 1 != 0 {
        (
            CPUID_0_0_EBX.load(Ordering::Relaxed),
            CPUID_0_0_ECX.load(Ordering::Relaxed),
            CPUID_0_0_EDX.load(Ordering::Relaxed),
        )
    } else {
        let cpuid = set_0_0();

        (cpuid.ebx, cpuid.ecx, cpuid.edx)
    }
}

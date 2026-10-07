//! Atomically test CPUID leaves to efficiently cache vendor and leaf.
//!
//! Used internally by macro

use core::{
    arch::x86_64::{__cpuid_count, CpuidResult},
    sync::atomic::{AtomicU32, AtomicU8, Ordering},
};

static CPUID_SET: AtomicU8 = AtomicU8::new(0);

static CPUID_0_0_EAX: AtomicU32 = AtomicU32::new(0);
static CPUID_0_0_EBX: AtomicU32 = AtomicU32::new(0);
static CPUID_0_0_ECX: AtomicU32 = AtomicU32::new(0);
static CPUID_0_0_EDX: AtomicU32 = AtomicU32::new(0);

static CPUID_1_0_ECX: AtomicU32 = AtomicU32::new(0);
static CPUID_1_0_EDX: AtomicU32 = AtomicU32::new(0);

static CPUID_7_0_EAX: AtomicU32 = AtomicU32::new(0);
static CPUID_7_0_EBX: AtomicU32 = AtomicU32::new(0);
static CPUID_7_0_ECX: AtomicU32 = AtomicU32::new(0);
static CPUID_7_0_EDX: AtomicU32 = AtomicU32::new(0);

static CPUID_8000_0000_0_EAX: AtomicU32 = AtomicU32::new(0);
static CPUID_8000_0001_0_EDX: AtomicU32 = AtomicU32::new(0);
static CPUID_8000_0008_0_EBX: AtomicU32 = AtomicU32::new(0);

/// Get EAX register of CPUID:7.0
#[deprecated = "use cpuid_7_0_eax instead"]
#[inline(always)]
pub fn cpuid_7_eax() -> u32 {
    cpuid_7_0_eax()
}

/// Get EAX register of CPUID:8000'0000.0
#[deprecated = "use cpuid_8000_0000_0_eax instead"]
#[inline(always)]
pub fn cpuid_8000_0000_eax() -> u32 {
    cpuid_8000_0000_0_eax()
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

macro_rules! cpuid_load {
    ($name:ident, CPUID:$leaf:literal.$subleaf:literal:$reg:ident, $id:literal, $static:ident, $($statics:ident=$in_reg:ident),*) => {
        #[doc = concat!("Gets `", stringify!($reg), "` register of CPUID:", stringify!($leaf), ".", stringify!($subleaf))]
        #[inline(always)]
        pub fn $name() -> u32 {
            if CPUID_SET.load(Ordering::Acquire) & (1 << $id) != 0 {
                $static.load(Ordering::Relaxed)
            } else {
                let cpuid = __cpuid_count($leaf, $subleaf);
                $(
                    $statics.store(cpuid.$in_reg, Ordering::Relaxed);
                )*
                CPUID_SET.fetch_or(1 << $id, Ordering::Release);
                cpuid.$reg
            }
        }
    };
}

cpuid_load!(cpuid_1_0_ecx, CPUID:1 .0:ecx, 1, CPUID_1_0_ECX, CPUID_1_0_ECX=ecx, CPUID_1_0_EDX=edx);
cpuid_load!(cpuid_1_0_edx, CPUID:1 .0:edx, 1, CPUID_1_0_EDX, CPUID_1_0_ECX=ecx, CPUID_1_0_EDX=edx);

cpuid_load!(cpuid_7_0_eax, CPUID:7 .0:eax, 2, CPUID_7_0_EAX, CPUID_7_0_EAX=eax, CPUID_7_0_EBX=ebx, CPUID_7_0_ECX=ecx, CPUID_7_0_EDX=edx);
cpuid_load!(cpuid_7_0_ebx, CPUID:7 .0:ebx, 2, CPUID_7_0_EBX, CPUID_7_0_EAX=eax, CPUID_7_0_EBX=ebx, CPUID_7_0_ECX=ecx, CPUID_7_0_EDX=edx);
cpuid_load!(cpuid_7_0_ecx, CPUID:7 .0:ecx, 2, CPUID_7_0_ECX, CPUID_7_0_EAX=eax, CPUID_7_0_EBX=ebx, CPUID_7_0_ECX=ecx, CPUID_7_0_EDX=edx);
cpuid_load!(cpuid_7_0_edx, CPUID:7 .0:edx, 2, CPUID_7_0_EDX, CPUID_7_0_EAX=eax, CPUID_7_0_EBX=ebx, CPUID_7_0_ECX=ecx, CPUID_7_0_EDX=edx);

cpuid_load!(cpuid_8000_0000_0_eax, CPUID:8000_0000 .0:eax, 3, CPUID_8000_0000_0_EAX, CPUID_8000_0000_0_EAX=eax);
cpuid_load!(cpuid_8000_0001_0_edx, CPUID:8000_0001 .0:edx, 4, CPUID_8000_0001_0_EDX, CPUID_8000_0001_0_EDX=edx);
cpuid_load!(cpuid_8000_0008_0_ebx, CPUID:8000_0008 .0:ebx, 5, CPUID_8000_0008_0_EBX, CPUID_8000_0008_0_EBX=ebx);

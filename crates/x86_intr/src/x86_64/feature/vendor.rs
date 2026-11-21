//! Check vendor from CPUID.

use super::test::cpuid_0_0_ebx_ecx_edx;

macro_rules! check_vendor {
    ($name:ident, $full:literal, $ebx:literal $ecx:literal $edx:literal) => {
        #[doc = concat!("Checks CPUID:0.0 for \"", $full, "\".")]
        #[inline(always)]
        pub fn $name() -> bool {
            let cpuid = cpuid_0_0_ebx_ecx_edx();

            cpuid.0 == u32::from_le_bytes(*$ebx)
                && cpuid.1 == u32::from_le_bytes(*$ecx)
                && cpuid.2 == u32::from_le_bytes(*$edx)
        }
    };
}

check_vendor!(is_intel, "GenuineIntel", b"Genu" b"ineI" b"ntel");
check_vendor!(is_umc, "UMC UMC UMC ", b"UMC " b"UMC " b"UMC ");
check_vendor!(is_amd, "AuthenticAMD", b"Auth" b"enti" b"cAMD");
check_vendor!(is_cyrix, "CyrixInstead", b"Cyri" b"xIns" b"tead");
check_vendor!(is_nexgen, "NexGenDriven", b"NexG" b"enDr" b"iven");
check_vendor!(is_ti, "TI CPU@Work ", b"TI C" b"PU@W" b"ork ");
check_vendor!(is_centaur, "CentaurHauls", b"Cent" b"aurH" b"auls");
check_vendor!(is_rise, "RiseRiseRise", b"Rise" b"Rise" b"Rise");
check_vendor!(is_sis, "SiS SiS SiS ", b"SiS " b"SiS " b"SiS ");
check_vendor!(is_transmeta, "GenuineTMx86", b"Genu" b"ineT" b"Mx86");
check_vendor!(is_vortex86, "Vortex86 SoC", b"Vort" b"ex86" b" SoC");
check_vendor!(is_rdc, "Genuine  RDC", b"Genu" b"ine " b" RDC");
check_vendor!(is_hygon, "HygonGenuine", b"Hygo" b"nGen" b"uine");
check_vendor!(is_zhaoxin, "  Shanghai  ", b"  Sh" b"angh" b"ai  ");
check_vendor!(is_evolutionary, "EVOLUTIONARY", b"EVOL" b"UTIO" b"NARY");

check_vendor!(is_ao486_old, "GenuineAO486", b"Genu" b"ineA" b"O486");
check_vendor!(is_ao486, "MiSTer AO486", b"MiST" b"er A" b"O486");

check_vendor!(is_msvpc7, "Virtual CPU ", b"Virt" b"ual " b"CPU ");
check_vendor!(is_connectix, "ConnectixCPU ", b"Conn" b"exti" b"xCPU");
check_vendor!(is_insignia586, "Insignia 586", b"Insi" b"gnia" b" 586");
check_vendor!(is_compaq, "Compaq FX!32", b"Comp" b"aq F" b"X!32");
check_vendor!(is_neko, "Neko Project", b"Neko" b" Pro" b"ject");

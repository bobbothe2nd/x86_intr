fn main() {
    println!("cargo:rustc-check-cfg=cfg(mwaitx_intr)");

    let target = std::env::var("TARGET").unwrap();

    if target.contains("x86_64") || target.contains("i686") {
        let success = cc::Build::new()
            .file("src/c/mwaitx.c")
            .try_compile("x86_intrinsics_shim")
            .is_ok();

        if success {
            println!("cargo:rustc-cfg=mwaitx_intr");
        }
    }
}

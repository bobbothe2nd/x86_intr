fn main() {
    println!("cargo:rustc-check-cfg=cfg(c_shim)");

    let target = std::env::var("TARGET").unwrap();

    if target.contains("x86_64") || target.contains("i686") {
        let success = cc::Build::new()
            .cargo_warnings(false)
            .file("src/c/intr.c")
            .flag("-mmwaitx")
            .flag("-mprefetchwt1")
            .try_compile("x86_intrinsics_shim")
            .is_ok();

        if success {
            println!("cargo:rustc-cfg=c_shim");
        }
    }
}

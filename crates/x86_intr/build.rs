use rust_ver::{build::{cfg_llvm_threshold, rerun_if_rustc_changed}, ver::Version};

fn main() {
    cfg_llvm_threshold(
        "llvm_avx10_2",
        Version::new(23, 1, 4),
    ).expect("failed to set LLVM version threshold");

    rerun_if_rustc_changed();
}

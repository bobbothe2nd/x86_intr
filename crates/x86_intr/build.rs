use rust_ver::{build::{cfg_llvm_threshold, rerun_if_rustc_changed}, ver::Version};

fn main() {
    cfg_llvm_threshold(
        "llvm_insn",
        // no LLVM versions support .insn directives
        Version::new(u32::MAX, 0, 0),
    ).expect("failed to set LLVM version threshold");

    rerun_if_rustc_changed();
}

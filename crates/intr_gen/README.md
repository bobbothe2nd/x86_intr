# Intrinsic Generator

This crate currently exposes one macro for abstracting inline assembly. That is, `intr_asm`.

Its API is shown below:

```rust
pub unsafe fn _tile_dpbusd<const DST: u8, const SRC0: u8, const SRC1: u8>()
where
    __tile1024i<DST>: ValidSimdReg,
    __tile1024i<SRC0>: ValidSimdReg,
    __tile1024i<SRC1>: ValidSimdReg,
{
    unsafe {
        intr_asm!(
            tmm{DST} = DST[0..8],
            "tdpbusd tmm{DST}, tmm{SRC0}, tmm{SRC1}",
            SRC0 = const SRC0,
            SRC1 = const SRC1,
            options(nostack, nomem, preserves_flags)
        );
    }
}
```

That `tmm{DST} = DST[0..8]` is the special part. It marks the output of this assembly is in a `tmm` register specified by the constant `DST`. It gets redefined as `DST` as an assembly-accessible constant used for formatting the register to be used. The valid range for this register is `tmm{0..8}` or `tmm{0..=7}`. It marks the used register as clobbered and an output for this code. That's it.

It doesn't sound like much, but for registers that are clobber-only, it reduces boilerplate a LOT.

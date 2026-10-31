// asm!() but with support for in("r17")/in("mm0")/in("tmm0") and [late]out("r17") and in[late]out(...same as in...)
// support clobber("r17")

enum RegArgTy {
    /// `in()`
    In,

    /// `inout()`
    InOut,

    /// `out()`
    Out,

    /// `lateout()`
    LateOut,

    /// `inlateout()`
    InLateOut,
}

enum RegClass {
    /// `r{16 + self.0}`
    Apx(u8),

    /// `tmm{self.0}`
    Tmm(u8),

    /// `mm{self.0}`
    Mm(u8),
}

enum RegArg {
    InOut {
        ty: RegArgTy,
        class: RegClass,
    },

    Clobber {
        reg: Box<str>,
    }
}

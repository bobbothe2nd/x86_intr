__attribute__((always_inline))
static inline void mwaitx(unsigned int extensions,
                          unsigned int hints,
                          unsigned int timeout)
{
    __asm__ volatile (
        "mwaitx"
        :
        : "a"(hints),
          "c"(extensions),
          "b"(timeout)
        : "memory"
    );
}

void rust_mwaitx(unsigned int extensions,
                 unsigned int hints,
                 unsigned int timeout)
{
    mwaitx(extensions, hints, timeout);
}

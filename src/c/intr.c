#include <x86intrin.h>

void rust_mwaitx(
    unsigned int extensions,
    unsigned int hints,
    unsigned int timeout
) {
    __builtin_ia32_mwaitx(extensions, hints, timeout);
}

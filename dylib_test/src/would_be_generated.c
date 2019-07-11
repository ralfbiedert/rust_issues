#include <stdint.h>

extern int32_t get_version();

// In reality these would be auto-generated doc tests extracted from parsing a `mylib.h` ...
int32_t test_harness() {
    int32_t rval = get_version();
    if (rval != 0) return rval;

    return 0;
}
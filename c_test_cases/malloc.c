#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

int main() {
    uint8_t* x = nullptr;

    // Test malloc
    x = malloc(100);
    assert(x != nullptr);
    free(x);

    // Test calloc
    x = calloc(100, 1);
    assert(x != nullptr);
    for (int i = 0; i < 100; i++) {
        // Calloc should zero things out
        assert(x[i] == 0);
    }
    free(x);

    // Test freeing nullptr
    free(nullptr);

    // Test realloc
    // This is currently not working because realloc was not implemented
    // correctly
    x = malloc(10);
    memset(x, 0xAB, 10);
    assert(x != nullptr);
    x = realloc(x, 100);
    for (int i = 0; i < 10; i++) {
        assert(x[i] == 0xAB);
    }
    memset(x, 0xBA, 100);
    free(x);

    // MAke sure alignment works
    x = aligned_alloc(0x100, 100);
    assert((((uintptr_t)x) & 0xFF) == 0);
    free(x);

    return 0;
}

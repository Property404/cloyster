#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

int main() {
    uint8_t* x = NULL;

    // Test malloc
    x = malloc(100);
    assert(x != NULL);
    free(x);

    // Test calloc
    x = calloc(100, 1);
    assert(x != NULL);
    for (int i = 0; i < 100; i++) {
        // Calloc should zero things out
        assert(x[i] == 0);
    }
    free(x);

    // Test freeing NULL
    free(NULL);

    // Test realloc
    // This is currently not working because realloc was not implemented
    // correctly
    x = malloc(10);
    memset(x, 0xAB, 10);
    assert(x != NULL);
    x = realloc(x, 100);
    for(int i=0; i < 10; i++) {
        assert(x[i] == 0xAB);
    }
    memset(x, 0xBA, 100);
    free(x);

    return 0;
}

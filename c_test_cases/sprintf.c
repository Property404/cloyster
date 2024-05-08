#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char* s = malloc(100);
    memset(s, 'x', 100);
    assert(sprintf(s, "Hello, world!") >= 0);
    assert(!strcmp(s, "Hello, world!"));

    assert(sprintf(s, "Hello, %d!", 42) >= 0);
    assert(!strcmp(s, "Hello, 42!"));

    assert(sprintf(s, "Hi") >= 0);
    assert(!strcmp(s, "Hi"));

    free(s);
}

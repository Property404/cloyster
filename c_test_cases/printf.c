#include <assert.h>
#include <stdio.h>

int main() {
    printf("Integer(d): %d\n", 42);
    printf("Integer(i): %i\n", 42);
    printf("Pointer: %p\n", (void*)42);
    printf("Null: %p\n", NULL);
    printf("Hex: %x\n", 42);
    printf("Hex: %X\n", 42);
    printf("Binary: %b\n", 42);
    printf("Binary: %B\n", 42);
    printf("Negative int: %d\n", -42);
    printf("Percent: %%\n");
    printf("Bad UTF-8: \xc0\x80\n");
    printf("Bad UTF-8 as arg: %s\n", "\xc0\x80");
    printf("Multiple args: %x%s%p\n", 42, "hello", NULL);

    // Check return value
    assert(printf("Hello %s", "World\n") == 12);
    assert(printf("%s", "") == 0);
    return 0;
}

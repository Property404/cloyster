// 12345
// This program opens and reads itself
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main() {
    printf("Opening %s\n", __FILE__);
    FILE* fp = fopen(__FILE__, "rb");
    printf("checking\n");
    assert(fp != nullptr);

    char buffer[9] = {0};
    printf("Reading\n");
    assert(fread(buffer, 1, sizeof(buffer) - 1, fp) == sizeof(buffer) - 1);

    printf("Comparing\n");
    assert(memcmp(buffer, "// 12345", sizeof(buffer)) == 0);

    assert(fclose(fp) == 0);

    printf("OK\n");
    return 0;
}

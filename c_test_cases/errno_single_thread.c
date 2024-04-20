#include <assert.h>
#include <errno.h>
#include <stdio.h>

int main() {
    printf("Errno: %d\n", errno);
    assert(errno == 0);
    errno = 5;
    assert(errno == 5);
    return 0;
}

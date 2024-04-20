#include <assert.h>
#include <stdio.h>
#include <time.h>

const time_t TIME_THIS_PROGRAM_WAS_WRITTEN = 1713622848;

int main() {
    const time_t t = time(NULL);

    // Make sure we're not using 32-bit time
    assert(sizeof(t) > 4);

    assert(t > TIME_THIS_PROGRAM_WAS_WRITTEN);

    return 0;
}

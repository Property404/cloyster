#include <assert.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

const time_t TIME_THIS_PROGRAM_WAS_WRITTEN = 1713622848;

int main() {
    // Make sure we're not using 32-bit time
    assert(sizeof(time_t) > 4);

    const time_t start = time(NULL);
    assert(start > TIME_THIS_PROGRAM_WAS_WRITTEN);

    struct timespec sleep_time = {.tv_nsec = 0, .tv_sec = 1};
    assert(nanosleep(&sleep_time, NULL) == 0);

    const time_t end = time(NULL);
    assert(end > start);
    assert(end < start + 5);

    return 0;
}

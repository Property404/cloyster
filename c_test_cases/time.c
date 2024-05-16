#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

constexpr time_t TIME_THIS_PROGRAM_WAS_WRITTEN = 1713622848;

static void test_clock_gettime() {
    // Get a reference time
    const time_t reference_time = time(nullptr);
    assert(reference_time > TIME_THIS_PROGRAM_WAS_WRITTEN);

    // Make sure that clock_gettime() and time() are approx the same
    struct timespec ts = {0};
    clock_gettime(0, &ts);
    assert(ts.tv_sec >= reference_time);
    assert(ts.tv_sec <= reference_time + 2);
}

static void test_time() {
    // assert the time returned is a sane value
    const time_t tval = time(nullptr);
    assert(tval > TIME_THIS_PROGRAM_WAS_WRITTEN);

    // Make sure we're using 64-bit timer (or higher, thought that's unlikely)
    assert(sizeof(tval) >= sizeof(uint64_t));
}

// Make sure we can sleep and that approximately the right amount of time
// elapses
static void test_nanosleep() {
    const time_t start = time(nullptr);
    assert(start > TIME_THIS_PROGRAM_WAS_WRITTEN);

    struct timespec sleep_time = {.tv_nsec = 0, .tv_sec = 1};
    assert(nanosleep(&sleep_time, nullptr) == 0);

    const time_t end = time(nullptr);
    assert(end > start);
    assert(end < start + 5);
}

int main() {
    test_clock_gettime();
    test_time();
    test_nanosleep();
    return 0;
}

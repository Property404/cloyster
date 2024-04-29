#include <assert.h>
#include <math.h>
#include <stdlib.h>

int main() {
    assert(abs(-3) == 3);
    assert(abs(0) == 0);
    assert(abs(3) == 3);
    assert(fabs(-34.0) > 33.9 && fabs(-34.0) < 34.1);
    assert(fabsf(-34.0f) > 33.9 && fabsf(-34.0f) < 34.1);
    assert(fabs(34.0) > 33.9 && fabs(34.0) < 34.1);
    assert(fabsf(34.0f) > 33.9 && fabsf(34.0f) < 34.1);
    return 0;
}

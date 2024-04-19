// This tests TLS (Thread Local Storage) for a single-threaded program
#include <stdio.h>

__thread int tls_var = 5;
__thread int tls_var2 = 3;
extern int __tdata_start;

int main() {
    printf("tdata_start: %d\n", __tdata_start);

    printf("tls_var: %d\n", tls_var);
    tls_var += 50;
    printf("tls_var: %d\n", tls_var);

    printf("tls_var2: %d\n", tls_var2);
    tls_var2 *= 8;
    printf("tls_var2: %d\n", tls_var2);
    return 0;
}

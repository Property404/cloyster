#include <stdio.h>
#include <string.h>

// Normalize the values of strcmp/strncmp
// `strcmp` and friends aren't specified to return anything specific, except 0.
// Just "negative," "positive," or 0
static int norm(int x) {
    if (x < 0) {
        return -1;
    } else if (x > 0) {
        return 1;
    } else {
        return 0;
    }
}

int main() {
    printf("%d\n", norm(strcmp("a", "b")));
    printf("%d\n", norm(strcmp("", "b")));
    printf("%d\n", norm(strcmp("", "")));
    printf("%d\n", norm(strcmp("apple", "appl")));
    for (int i = 0; i < 10; i++) {
        printf("%d\n", norm(strncmp("a", "b", i)));
        printf("%d\n", norm(strncmp("", "", i)));
        printf("%d\n", norm(strncmp("a", "", i)));
        printf("%d\n", norm(strncmp("", "b", i)));
        printf("%d\n", norm(strncmp("apple", "apple", i)));
        printf("%d\n", norm(strncmp("apple", "orange", i)));
    }
}

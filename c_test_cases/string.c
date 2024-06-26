#include <stdio.h>
#include <stdlib.h>
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

void strcmp_test() {
    printf("%d\n", norm(strcmp("a", "b")));
    printf("%d\n", norm(strcmp("", "b")));
    printf("%d\n", norm(strcmp("", "")));
    printf("%d\n", norm(strcmp("apple", "appl")));
    printf("%d\n", norm(strcasecmp("apple", "apple")));
    printf("%d\n", norm(strcasecmp("apple", "aPPLE")));
}

void strncmp_test() {
    for (int i = 0; i < 10; i++) {
        printf("%d\n", norm(strncmp("a", "b", i)));
        printf("%d\n", norm(strncmp("a", "B", i)));
        printf("%d\n", norm(strncmp("", "", i)));
        printf("%d\n", norm(strncmp("a", "", i)));
        printf("%d\n", norm(strncmp("", "b", i)));
        printf("%d\n", norm(strncmp("apple", "apple", i)));
        printf("%d\n", norm(strncmp("apple", "aPPLe", i)));
        printf("%d\n", norm(strncmp("apple", "orange", i)));
        printf("%d\n", norm(strncasecmp("apple", "apple", i)));
        printf("%d\n", norm(strncasecmp("apple", "aPPLe", i)));
    }
}

void substr_test() {
    printf("%s\n", strstr("apple", "le"));
    printf("%p\n", strstr("apple", "te"));
    printf("%s\n", strchr("apple", 'p'));
    printf("%s\n", strrchr("apple", 'p'));
    printf("%p\n", strrchr("apple", 'x'));
}

void strcpy_test() {
    char* dst = malloc(100);
    printf("%s\n", strcpy(dst, "ORANGE"));
    printf("%s\n", strcpy(dst, "apple"));
    printf("%s\n", strncpy(dst, "apple", 2));
    printf("%s\n", strncpy(dst, "apple", 0));
    printf("%s\n", stpcpy(dst, "O"));
    free(dst);
}

int main() {
    strcmp_test();
    strncmp_test();
    substr_test();
    strcpy_test();
    return 0;
}

#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

int main(int argc, char** argv) {
    printf("argc: %d\n", argc);
    printf("argv[0]: %s\n\n", argv[0]);

    for(int i=0; i <= 15; i++) {
        if (i) {
            putchar(' ');
        }
        if (i%3 && i%5) {
            printf("%d",i);
        } else if (i%3) {
            printf("Buzz");
        } else if (i%5) {
            printf("Fizz");
        } else {
            printf("FizzBuzz");
        }
    }
    puts("\n");

    printf("The current unix time is: %d\n", time(NULL));
}

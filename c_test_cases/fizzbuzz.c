#include <stdio.h>

int main() {
    for(int i=0; i <= 15; i++) {
        if (i%3 && i%5) {
            printf("%d\n",i);
        } else if (i%3) {
            puts("Buzz");
        } else if (i%5) {
            puts("Fizz");
        } else {
            puts("FizzBuzz");
        }
    }

    return 0;
}

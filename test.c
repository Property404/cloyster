#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

int main() {
    const char* message = "Hello, world!";
    for (int i=0; i<5;i++) {
        printf("%s <%d>\n", message, i);
    }
}

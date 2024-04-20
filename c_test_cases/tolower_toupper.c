#include <assert.h>
#include <ctype.h>
int main() {
    assert(toupper('x') == 'X');
    assert(toupper('X') == 'X');
    assert(toupper('0') == '0');
    assert(tolower('x') == 'x');
    assert(tolower('X') == 'x');
    assert(tolower('0') == '0');
    return 0;
}

#include <unistd.h>

int main(void) {
    write(1, "9876543210\n", 11);
    while (1) {} // Oh no, how did this get here?
    return (0);
}

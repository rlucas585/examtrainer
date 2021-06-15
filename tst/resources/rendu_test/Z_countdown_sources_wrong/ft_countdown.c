#include <unistd.h>

int main(void) {
    write(1, "987543210\n", 11);
    return (0);
}

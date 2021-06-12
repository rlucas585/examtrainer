#include <unistd.h>

int main(void) {
    write(1, "987543210\n", 10);
    return (0);
}

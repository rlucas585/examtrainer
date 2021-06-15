#include <unistd.h>

int has_a(const char* str) {
    size_t i = 0;
    while (str[i]) {
        if (str[i] == 'a')
            return 1;
        i++;
    }
    return 0;
}

void put_newline(void) {
    write(1, "\n", 1);
}

void put_a(void) {
    write(1, "a\n", 2);
}

int main(int argc, char** argv) {
    if (argc != 2) {
        put_a();
    } else if (has_a(argv[1])) {
        put_a();
    } else {
        put_newline();
    }
    return 0;
}

#include <unistd.h>

static size_t ft_strlen(const char* s) {
    size_t i = 0;
    while (s[i])
        i++;
    return i;
}

static void ft_putstr(const char* s) {
    write(1, s, ft_strlen(s));
}

void hello_world(void) {
    ft_putstr("hello world!\n");
}

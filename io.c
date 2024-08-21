#include <stdint.h>

#define SYS_READ  0   // syscall number for sys_read
#define SYS_WRITE 1   // syscall number for sys_write
#define STDIN     0   // file descriptor for stdin
#define STDOUT    1   // file descriptor for stdout

// Function to read a single character from stdin
char my_getchar() {
    char c;
    asm volatile (
        "syscall"
        : "=a" (c)              // output: store the read character in c
        : "a" (SYS_READ),        // input: syscall number for read
          "D" (STDIN),           // input: file descriptor (stdin)
          "S" (&c),              // input: pointer to the buffer to store the character
          "d" (1)                // input: number of bytes to read
        : "rcx", "r11", "memory" // clobbered registers
    );
    return c;
}

// Function to write a single character to stdout
void my_putchar(char c) {
    asm volatile (
        "syscall"
        :
        : "a" (SYS_WRITE),       // input: syscall number for write
          "D" (STDOUT),          // input: file descriptor (stdout)
          "S" (&c),              // input: pointer to the character to write
          "d" (1)                // input: number of bytes to write
        : "rcx", "r11", "memory" // clobbered registers
    );
}


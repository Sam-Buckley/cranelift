section .text
    global _start
    global putchar
    global getchar
    extern main

_start:
    ; test putchar and getchar
    ; put address of the character to print in RSI

    ; Call main function
    call main ; Store the return value in EBX register

    ; Exit program
    mov edi, eax
    mov eax, 60
    syscall

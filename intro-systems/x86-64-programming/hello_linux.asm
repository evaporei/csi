; global variables
section .data
    ; variable "msg" points to "hello world\n"
    ; 10 = \n = 0xA
    ; size/length = 12 (5 + 1 + 5 + 1)
    msg db  "hello world", 10
    ; MSG_LEN: equ $ - msg
    ; NR_WRITE: equ 1
    ; STDOUT: equ 1

; CPU instructions
section .text
    ; export "_start" symbol so linker can find it
    global _start

; program's entry point
; aka, first instruction to execute is at this address
_start:
    mov rax, 1; write syscall number (__NR_WRITE)
    mov rdi, 1; STDOUT
    mov rsi, msg
    mov rdx, 12; len(msg)
    syscall
    mov rax, 60; exit syscall number
    mov rdi, 0; 0 argument, it could be `xor rdi, rdi`
    syscall

# 1. Assembling and running with nasm

```bash
# fish
nasm -g -f elf64 hello.asm; and ld -o hello hello.o; and ./hello
# bash/zsh
nasm -g -f elf64 hello.asm && ld -o hello hello.o && ./hello
```

# 2. Adding numbers 1 to n

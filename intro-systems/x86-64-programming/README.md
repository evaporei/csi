## 1. Assembling and running with nasm

```bash
# fish
nasm -g -f elf64 hello_linux.asm; and ld -o hello hello.o; and ./hello
# bash/zsh
nasm -g -f elf64 hello_linux.asm && ld -o hello hello.o && ./hello
```

## 2, 3 and 4

Just run `make`.

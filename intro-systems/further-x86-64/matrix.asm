section .text
global index
index:
	; rdi: matrix
	; rsi: rows
	; rdx: cols
	; rcx: rindex
	; r8: cindex

	; // Some C
	; int rows = 3;
	; int cols = 4;
	; // Accessing elements using single index
	; int rindex = 1;
	; int cindex = 2;
	; int index = rindex * cols + cindex;
	; int value_flat = *(*matrix + index);

	imul rcx, rdx ; rindex * cols
	add rcx, r8 ; + cindex
	mov rax, [rdi + rcx * 4] ; int = 4 bytes
	ret

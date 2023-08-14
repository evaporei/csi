; int binary_convert(char *bits) {
;     int result = 0;
;
;     for (char *c = bits; *c != '\0'; c++) {
;         if (*c == '1') {
;             result = (result << 1) + 1;
;         } else if (*c == '0') {
;             result = result << 1;
;         }
;     }
;
;     return result;
; }
section .text
global binary_convert
binary_convert:
	mov rax, 0
	; rcx = offset that we're accessing `bits`
	mov rcx, 0
	jmp .loop
.loop:
	; ebx = current char being accessed in `bits`
	movzx ebx, byte [rdi + rcx]
	inc rcx
	cmp ebx, 49
	je .add_one
	cmp ebx, 48
	je .add_zero
	cmp ebx, 0
	je .end
	jmp .loop
.add_one:
	; rax = (rax << 1) + 1
	shl rax, 1
	inc rax
	jmp .loop
.add_zero:
	; rax = rax << 1
	shl rax, 1
	jmp .loop
.end:
	ret

; sample base code in C
; int sum_to_n(n) {
;     int total = 0;
;
;     for (int i = 0; i <= n; i++)
;         total += i;
;
;     return total;
; }
section .text
global sum_to_n

; n = rdi
; total will go into rax
sum_to_n:
	;; for optimized version, uncomment line below:
	; jmp sum_to_n_opt

	; total = 0
	mov eax, 0
	; i = n
	mov ecx, edi
.loop:
	cmp ecx, 0
	je .out
	add eax, ecx
	dec ecx
	jmp .loop
.out:
	ret

; n * (n + 1) / 2
sum_to_n_opt:
	mov ebx, edi
	inc ebx
	mov eax, edi
	imul eax, ebx
	xor rdx, rdx
	mov ecx, 2
	idiv ecx
	ret

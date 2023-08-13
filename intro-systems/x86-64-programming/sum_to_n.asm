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
	; total = 0
	mov eax, 0
	; i = n
	mov ecx, edi
	jmp .loop
.loop:
	cmp ecx, 0
	je .out
	add eax, ecx
	dec ecx
	jmp .loop
.out:
	ret

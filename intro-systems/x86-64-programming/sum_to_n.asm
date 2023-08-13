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
	mov rax, 0
	ret

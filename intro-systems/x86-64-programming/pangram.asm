; ; simple (lowercase + no err handling):
; bool checkIfPangram(char * sentence){
;     unsigned int letters = 0;
;
;     for (char *c = sentence; *c != '\0'; c++) {
;         letters = letters | (1 << (*c - 97));
;     }
;
;     return letters == 0x03ffffff;
; }
; ; complete:
; bool pangram(char * sentence){
;     unsigned int letters = 0;
;
;     for (char *c = sentence; *c != '\0'; c++) {
;         letters |= (1 << ((*c | 32) - 97));
;     }
;
;     return (letters & 0x03ffffff) == 0x03ffffff;
; }
section .text
global pangram
pangram:
	mov eax, 0
	mov edx, 0
.loop:
	; ecx = current char being accessed
	movzx ecx, byte [rdi]
	cmp ecx, 0
	je .end
	inc rdi
	or ecx, 32 ; "to lowercase"
	sub ecx, 97 ; 'a' -> will give 0-25 value
	bts edx, ecx
	jmp .loop
.end:
	and edx, 0x03ffffff
	cmp edx, 0x03ffffff
	sete al
	ret

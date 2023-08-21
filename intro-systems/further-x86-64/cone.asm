default rel

section .text
global volume
volume:
    ; xmm0 = radius
    ; xmm1 = height
    ; V = π * r² * h / 3
    mulss xmm0, xmm0 ; r²
    mulss xmm0, [pi] ; * PI
    mulss xmm0, xmm1 ; * h
    divss xmm0, [three] ; / 3
 	ret

section .rodata
pi: dd 3.14159
three: dd 3.0

; .PI:
;     dq 3.14159
; .THREE:
;     dq 3.0
; .PI: ; 0x400921f9f01b866e
;     .long 0x400921f9
;     .long 0x3ffccccc

; float float_mov(float v1, float *src, float *dst) {
;     float v2 = *src;
;     *dst = v1;
;     return v2;
; }

; v1 in %xmm0, src in %rdi, dst in %rsi
; float_mov:
;     vmovaps xmm1, xmm0 ; Copy v1
;     vmovss xmm0, [rdi] ; Read v2 from src
;     vmovss [rsi], xmm1 ; Write v1 to dst
;     ret


; double cel2fahr(double temp)
; {
;     return 1.8 * temp + 32.0;
; }
; cel2fahr:
;     vmulsd .LC2(%rip), %xmm0, %xmm0; Multiply by 1.8
;     vaddsd .LC3(%rip), %xmm0, %xmm0; Add 32.0
;     ret
; .LC2:
;     .long 3435973837; Low-order 4 bytes of 1.8
;     .long 1073532108; High-order 4 bytes of 1.8
; .LC3:
;     .long 0; Low-order 4 bytes of 32.0
;     .long 1077936128; High-order 4 bytes of 32.0

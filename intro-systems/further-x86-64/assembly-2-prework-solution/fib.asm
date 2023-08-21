section .text
global fib
fib:
    ; edi = int n
    ; eax = n
    ; if n <= 1 return n;
	mov eax, edi	; Base case:
	cmp edi, 1		; if (n <= 1) return n
	jle .end

    ; save caller's rbx
	push rbx		; Store rbx on stack so that we can use it...
    ; ebx = n (edi)
 	mov ebx, edi	; ... to store n
    ; edi -= 1
 	sub edi, 1

    ; fib (n - 1)
 	call fib		; Compute fib(n - 1)...
    ; save r12
    push r12		; ... and store the result in r12
    ; eax = r12d
    mov	r12d, eax

    ; save rcx to realign stack?
	push rcx		; Realign stack to 16 bytes by pushing junk
    lea edi, [rbx-2]
 	call fib		; Calculate fib(n-2)...
	add	eax, r12d 	; ...and add result to previously calculated fib(n-1)
 
	pop rcx					
 	pop r12
 	pop rbx
.end:
	ret

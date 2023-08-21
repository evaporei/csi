section .text
global fib
fib:
   mov eax, edi
   cmp edi, 1
   jle .end

   push rbx
   mov ebx, edi
   sub edi, 1
   call fib

   push r12
   mov r12d, eax
   lea edi, [rbx - 2]
   call fib
   add eax, r12d

   pop r12
   pop rbx
.end:
   ret

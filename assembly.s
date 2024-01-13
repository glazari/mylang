
global _start
section .text
_start:
    call main
    ; exit syscall
    mov rax, 60
    xor rdi, rdi ; exit code 0
    syscall


main:
	mov rdi, 0; rdi is the offset for num_to_string
	mov r12, 2; r12 is the number to check for primality
loop_0:
	cmp r12, 100
	jg loop_0_end
	mov rax, r12
	call check_prime
	cmp rax, 1
	jne if_else_1_else
	mov rax, r12
	call num_to_string
	add rdi, rdx ; increment offset by length of number
	cmp rdi, 1000  ; print only once buffer has many bytes
	jl if_else_2_else
	mov rsi, number_buffer
	mov rdx, rdi
	call print
	mov rdi, 0 ; reset offset
	jmp if_else_2_end
if_else_2_else:
if_else_2_end:
	jmp if_else_1_end
if_else_1_else:
if_else_1_end:
	inc r12
	jmp loop_0
loop_0_end:
	cmp rdi, 0  ; if remaining numbers, print them
	jle if_else_3_else
	mov rsi, number_buffer
	mov rdx, rdi
	call print
	mov rdi, 0 ; reset offset
	jmp if_else_3_end
if_else_3_else:
if_else_3_end:
	ret ; return to calling proceedure from main

; prints string in rsi with length in rdx
print:
	mov rax, 1
	mov rdi, 1
	syscall
	ret ; return to calling proceedure from print

; checks if number in rax is prime, returns 1 if prime, 0 if not
check_prime:
	mov rbx, 2     ; rbx is the divisor
	mov rcx, rax   ; rcx is the number
loop_4:
	cmp rcx, rbx
	jle loop_4_end
	mov eax, ecx
	cdq
	div dword ebx ; dword is 32 bit, much faster than 64 bit
	inc rbx
	cmp rdx, 0
	jne if_else_5_else
	mov rax, 0
	ret
	jmp if_else_5_end
if_else_5_else:
if_else_5_end:
	jmp loop_4
loop_4_end:
	ret ; return to calling proceedure from check_prime

; converts number in rax to string in number_buffer+rdi, returns address in rsi and length in rdx
num_to_string:
	mov r10, 0 ; r10 is the length of the number
	mov rcx, rax ; rcx is the number
	mov r8, number_buffer; r8 is the address to write to
	add r8, rdi ; r8 is the address to write to
do_while_6:
	mov rax, rcx
	cdq
	mov rbx, 10
	div dword ebx ; rax = eax:edx / ebx, rdx = eax:edx % ebx
	add rdx, '0' ; convert to ascii
	mov byte [r8+r10], dl ; store in buffer
	inc r10 ; increment length
	mov rcx, rax ;
	cmp rcx, 0
	jne do_while_6
do_while_6_end:
	mov rcx, r10 ; rcx will be the end pointer
	dec rcx 
	mov rsi, 0 ; rsi will be the start pointer
do_while_7:
	mov byte dl, [r8+rsi]
	mov byte al, [r8+rcx]
	mov [r8+rcx], dl
	mov [r8+rsi], al
	inc rsi
	dec rcx
	cmp rsi, rcx
	jle do_while_7
do_while_7_end:
	mov byte [r8+r10], 10 ; add newline
	inc r10 ; increment length
	mov rsi, r8
	mov rdx, r10
	ret ; return to calling proceedure from num_to_string



section .bss
number_buffer: resb 1024

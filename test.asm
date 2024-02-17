
global _start
section .text
_start:
    call main
    ; exit syscall
    mov rax, 60
    xor rdi, rdi ; exit code 0
    syscall


main:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, [x]
	push rax
	sub rsp, 8
	call print_numberln
	mov rax, [rsp]
	add rsp, 16
	mov rax, [y]
	push rax
	sub rsp, 8
	call print_numberln
	mov rax, [rsp]
	add rsp, 16
	mov rax, [z]
	push rax
	sub rsp, 8
	call print_numberln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 32
	mov [x], rax
	mov rax, [x]
	push rax
	sub rsp, 8
	call print_numberln
	mov rax, [rsp]
	add rsp, 16
	; epilogue
	add rsp, 0
	pop rbp
	ret
print_numberln:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; body
	mov rax, 0
	mov [rbp - 8], rax
	mov rax, [rbp + 24]
	mov [rbp - 16], rax
while_condition_0:
	mov rax, [rbp - 16]
	push rax
	mov rax, 0
	pop rbx
	cmp rbx, rax
	jle while_end_0
while_body_0:
	mov rax, [rbp - 16]
	push rax
	mov rax, 10
	mov rbx, rax
	pop rax
	cdq
	div rbx
	mov [rbp - 16], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 8], rax
	jmp while_condition_0
while_end_0:
	mov rax, 0
	mov [rbp - 24], rax
	mov rax, 0
	mov [rbp - 32], rax
while_condition_1:
	mov rax, [rbp + 24]
	push rax
	mov rax, 0
	pop rbx
	cmp rbx, rax
	jle while_end_1
while_body_1:
	mov rax, 10
	push rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	sub rax, rbx
	push rax
	sub rsp, 8
	call pow
	mov rax, [rsp]
	add rsp, 24
	mov [rbp - 32], rax
	mov rax, [rbp + 24]
	push rax
	mov rax, [rbp - 32]
	mov rbx, rax
	pop rax
	cdq
	div rbx
	push rax
	mov rax, 48
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 24], rax
	mov rax, [rbp - 24]
	push rax
	sub rsp, 8
	call print_chr
	mov rax, [rsp]
	add rsp, 16
	mov rax, [rbp + 24]
	push rax
	mov rax, [rbp - 32]
	mov rbx, rax
	pop rax
	cdq
	div rbx
	mov rax, rdx
	mov [rbp + 24], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp - 8], rax
	jmp while_condition_1
while_end_1:
	mov rax, 10
	push rax
	sub rsp, 8
	call print_chr
	mov rax, [rsp]
	add rsp, 16
	; epilogue
	add rsp, 32
	pop rbp
	ret
pow:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 8
	; body
	mov rax, 1
	mov [rbp - 8], rax
while_condition_2:
	mov rax, [rbp + 24]
	push rax
	mov rax, 0
	pop rbx
	cmp rbx, rax
	jle while_end_2
while_body_2:
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp + 32]
	mov rbx, rax
	pop rax
	mul rbx
	mov [rbp - 8], rax
	mov rax, [rbp + 24]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp + 24], rax
	jmp while_condition_2
while_end_2:
	mov rax, [rbp - 8]
	mov [rbp + 16], rax
	add rsp, 8
	pop rbp
	ret
	; epilogue
	add rsp, 8
	pop rbp
	ret
print_chr:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rsi, rbp
	add rsi, 24 ; point to address of a
	mov rdx, 1 ; length
	mov rax, 1 ; write syscall
	mov rdi, 1 ; stdout file handle
	syscall
	; epilogue
	add rsp, 0
	pop rbp
	ret


section .data
x dq 12
y dq 59
z dq 71

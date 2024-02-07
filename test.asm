
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
	sub rsp, 8
	; body
	mov rax, 10
	push rax
	sub rsp, 8
	call add_all
	mov rax, [rsp]
	add rsp, 16
	mov [rbp - 8], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 1
	pop rbx
	add rax, rbx
	mov [rbp - 8], rax
	; epilogue
	add rsp, 8
	pop rbp
	ret
add_all:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; body
	mov rax, 0
	mov [rbp - 8], rax
	mov rax, 1
	mov [rbp - 16], rax
while_condition_0:
	mov rax, [rbp - 16]
	push rax
	mov rax, [rbp - -24]
	pop rbx
	cmp rbx, rax
	jge while_end_0
while_body_0:
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp - 16]
	pop rbx
	add rax, rbx
	mov [rbp - 8], rax
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	pop rbx
	add rax, rbx
	mov [rbp - 16], rax
	jmp while_condition_0
while_end_0:
	mov rax, [rbp - 8]
	mov [rbp + 16], rax
	add rsp, 16
	pop rbp
	ret
	; epilogue
	add rsp, 16
	pop rbp
	ret


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
	sub rsp, 16
	; body
	mov rax, 1
	push rax
	mov rax, 2
	push rax
	mov rax, 3
	pop rbx
	add rax, rbx
	pop rbx
	add rax, rbx
	mov [rbp - 0], rax
	mov rax, [rbp - 0]
	push rax
	mov rax, 4
	pop rbx
	add rax, rbx
	mov [rbp - 8], rax
	; epilogue
	add rsp, 16
	pop rbp
	ret
add:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 24
	; body
	mov rax, 1
	mov [rbp - 0], rax
	mov rax, 2
	mov [rbp - 8], rax
	mov rax, [rbp - 0]
	push rax
	mov rax, [rbp - 8]
	pop rbx
	add rax, rbx
	mov [rbp - 16], rax
	; epilogue
	add rsp, 24
	pop rbp
	ret

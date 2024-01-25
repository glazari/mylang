
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
	mov rax, 1
	push rax
	mov rax, 2
	push rax
	mov rax, 3
	push rax
	mov rax, 1
	pop rbx
	sub rax, rbx
	pop rbx
	add rax, rbx
	pop rbx
	add rax, rbx
	mov [rbp - 0], rax
	; epilogue
	add rsp, 8
	pop rbp
	ret

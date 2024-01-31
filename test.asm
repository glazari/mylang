
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
	mov rax, 2
	mov [rbp - 8], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 1
	pop rbx
	add rax, rbx
	mov [rbp - 8], rax
	mov rax, [rbp - 8]
	push rax
	sub rsp, 8
	call add
	add rsp, 8
	mov rax, [rsp - 8]
	pop rbx
	add rax, rbx
	mov [rbp - 8], rax
	; epilogue
	add rsp, 8
	pop rbp
	ret
add:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; body
	mov rax, 2
	push rax
	mov rax, 1
	pop rbx
	add rax, rbx
	mov [rbp - 8], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 3
	pop rbx
	add rax, rbx
	mov [rbp - 16], rax
	mov rax, [rbp - 16]
	mov [rbp + 16], rax
	add rsp, 16
	pop rbp
	ret
	; epilogue
	add rsp, 16
	pop rbp
	ret

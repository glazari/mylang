
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
	mov [rbp - 8], rax
if_condition_0:
	mov rax, [rbp - 8]
	push rax
	mov rax, 0
	pop rbx
	cmp rbx, rax
	jle else_0
if_body_0:
	mov rax, 3
	mov [rbp - 8], rax
	jmp end_0
else_0:
	mov rax, 5
	mov [rbp - 8], rax
end_0:
	mov rax, [rbp - 8]
	push rax
	mov rax, 2
	push rax
	mov rax, 3
	push rax
	mov rax, 4
	pop rbx
	add rax, rbx
	push rax
	sub rsp, 8
	call add
	mov rax, [rsp]
	add rsp, 24
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
	sub rsp, 0
	; body
	mov rax, [rbp - -32]
	push rax
	mov rax, [rbp - -24]
	pop rbx
	add rax, rbx
	mov [rbp + 16], rax
	add rsp, 0
	pop rbp
	ret
	; epilogue
	add rsp, 0
	pop rbp
	ret

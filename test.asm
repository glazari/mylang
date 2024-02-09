
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
	sub rsp, 24
	; body
	mov rax, 50
	push rax
	sub rsp, 8
	call print_chr
	mov rax, [rsp]
	add rsp, 16
	mov [rbp - 8], rax
	mov rax, 48
	push rax
	sub rsp, 8
	call print_chr
	mov rax, [rsp]
	add rsp, 16
	mov [rbp - 8], rax
	mov rax, 10
	push rax
	sub rsp, 8
	call print_chr
	mov rax, [rsp]
	add rsp, 16
	mov [rbp - 8], rax
	mov rax, 10
	push rax
	sub rsp, 8
	call add_all
	mov rax, [rsp]
	add rsp, 16
	mov [rbp - 16], rax
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 16], rax
	mov rax, 10
	push rax
	sub rsp, 8
	call add_all_do_while
	mov rax, [rsp]
	add rsp, 16
	mov [rbp - 24], rax
	; epilogue
	add rsp, 24
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
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 8], rax
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
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
add_all_do_while:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; body
	mov rax, 0
	mov [rbp - 8], rax
	mov rax, 1
	mov [rbp - 16], rax
do_while_body_1:
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp - 16]
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 8], rax
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 16], rax
do_while_condition_1:
	mov rax, [rbp - 16]
	push rax
	mov rax, [rbp - -24]
	pop rbx
	cmp rbx, rax
	jl do_while_end_1
	jmp do_while_body_1
do_while_end_1:
	; epilogue
	add rsp, 16
	pop rbp
	ret

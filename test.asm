
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
	sub rsp, 8
	call init
	mov rax, [rsp]
	add rsp, 8
	mov rax, 1
	push rax
	mov rax, 2
	mov rbx, rax
	pop rax
	add rax, rbx
	push rax
	mov rax, 8
	mov rbx, rax
	pop rax
	add rax, rbx
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 5
	mov [rbp - 8], rax
	mov rax, 100
	mov [rbp - 16], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp - 16]
	mov rbx, rax
	pop rax
	sub rax, rbx
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 10
	mov [rbp - 24], rax
	mov rax, 0
	push rax
	mov rax, [rbp - 24]
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp - 24], rax
	mov rax, [rbp - 24]
	push rax
	mov rax, 2
	mov rbx, rax
	pop rax
	cqo
	idiv rbx
	mov [rbp - 24], rax
	mov rax, 123
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 1
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 10
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 0
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 0
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 10000
	push rax
	sub rsp, 8
	call print_nln
	mov rax, [rsp]
	add rsp, 16
	mov rax, 10000
	push rax
	sub rsp, 8
	call print_hexln
	mov rax, [rsp]
	add rsp, 16
	; epilogue
	add rsp, 24
	pop rbp
	ret
init_brk:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, 0
	push rax
	sub rsp, 8
	call brk
	mov rax, [rsp]
	add rsp, 16
	mov [current_brk], rax
	; epilogue
	add rsp, 0
	pop rbp
	ret
malloc:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; body
	mov rax, [current_brk]
	mov [rbp - 8], rax
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp + 24]
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 16], rax
	mov rax, [rbp - 16]
	push rax
	sub rsp, 8
	call brk
	mov rax, [rsp]
	add rsp, 16
	mov [current_brk], rax
if_condition_0:
	mov rax, [current_brk]
	push rax
	mov rax, [rbp - 16]
	pop rbx
	cmp rbx, rax
	je else_0
if_body_0:
	mov rax, 1
	push rax
	sub rsp, 8
	call exit
	mov rax, [rsp]
	add rsp, 16
	jmp end_0
else_0:
end_0:
	mov rax, [rbp - 8]
	mov [rbp + 16], rax
	add rsp, 16
	pop rbp
	ret
	; epilogue
	add rsp, 16
	pop rbp
	ret
init:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	sub rsp, 8
	call init_brk
	mov rax, [rsp]
	add rsp, 8
	sub rsp, 8
	call init_print_nln
	mov rax, [rsp]
	add rsp, 8
	sub rsp, 8
	call init_num_to_string
	mov rax, [rsp]
	add rsp, 8
	; epilogue
	add rsp, 0
	pop rbp
	ret
init_print_nln:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, 1024
	push rax
	sub rsp, 8
	call malloc
	mov rax, [rsp]
	add rsp, 16
	mov [print_nln_bff], rax
	; epilogue
	add rsp, 0
	pop rbp
	ret
print_nln:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 8
	; body
	mov rax, [rbp + 24]
	push rax
	mov rax, [print_nln_bff]
	push rax
	mov rax, 10
	push rax
	sub rsp, 8
	call num_to_string
	mov rax, [rsp]
	add rsp, 32
	mov [rbp - 8], rax
	mov rax, 1
	push rax
	mov rax, [print_nln_bff]
	push rax
	mov rax, [rbp - 8]
	push rax
	sub rsp, 8
	call write
	mov rax, [rsp]
	add rsp, 32
	; epilogue
	add rsp, 8
	pop rbp
	ret
print_hexln:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 8
	; body
	mov rax, [print_nln_bff]
	mov dx, 0x7830 ; 0x
	mov [rax], dx
	mov rax, [rbp + 24]
	push rax
	mov rax, [print_nln_bff]
	push rax
	mov rax, 2
	mov rbx, rax
	pop rax
	add rax, rbx
	push rax
	mov rax, 16
	push rax
	sub rsp, 8
	call num_to_string
	mov rax, [rsp]
	add rsp, 32
	mov [rbp - 8], rax
	mov rax, 1
	push rax
	mov rax, [print_nln_bff]
	push rax
	mov rax, [rbp - 8]
	push rax
	mov rax, 2
	mov rbx, rax
	pop rax
	add rax, rbx
	push rax
	sub rsp, 8
	call write
	mov rax, [rsp]
	add rsp, 32
	; epilogue
	add rsp, 8
	pop rbp
	ret
init_num_to_string:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, 50
	push rax
	sub rsp, 8
	call malloc
	mov rax, [rsp]
	add rsp, 16
	mov [num_to_string_bff], rax
	; epilogue
	add rsp, 0
	pop rbp
	ret
num_to_string:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 40
	; body
	mov rax, 0
	mov [rbp - 8], rax
	mov rax, [num_to_string_bff]
	mov [rbp - 16], rax
	mov rax, 0
	mov [rbp - 24], rax
if_condition_1:
	mov rax, [rbp + 40]
	push rax
	mov rax, 0
	pop rbx
	cmp rbx, rax
	jge else_1
if_body_1:
	mov rax, 1
	mov [rbp - 24], rax
	mov rax, 0
	push rax
	mov rax, [rbp + 40]
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp + 40], rax
	jmp end_1
else_1:
end_1:
do_while_body_2:
	mov rax, [rbp + 40]
	push rax
	mov rax, [rbp + 24]
	mov rbx, rax
	pop rax
	cqo
	idiv rbx
	mov rax, rdx
	mov [rbp - 8], rax
	mov rax, [rbp + 40]
	push rax
	mov rax, [rbp + 24]
	mov rbx, rax
	pop rax
	cqo
	idiv rbx
	mov [rbp + 40], rax
if_condition_3:
	mov rax, [rbp - 8]
	push rax
	mov rax, 11
	pop rbx
	cmp rbx, rax
	jge else_3
if_body_3:
	mov rax, [rbp - 8]
	push rax
	mov rax, 48
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 8], rax
	jmp end_3
else_3:
	mov rax, [rbp - 8]
	push rax
	mov rax, 10
	mov rbx, rax
	pop rax
	sub rax, rbx
	push rax
	mov rax, 97
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 8], rax
end_3:
	mov rdx, [rbp - 8]
	mov rax, [rbp - 16]
	mov byte [rax], dl 
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 16], rax
do_while_condition_2:
	mov rax, [rbp + 40]
	push rax
	mov rax, 0
	pop rbx
	cmp rbx, rax
	jg do_while_body_2
do_while_end_2:
if_condition_4:
	mov rax, [rbp - 24]
	push rax
	mov rax, 1
	pop rbx
	cmp rbx, rax
	jne else_4
if_body_4:
	mov rax, [rbp - 16]
	mov byte [rax], 45; -
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 16], rax
	jmp end_4
else_4:
end_4:
	mov rax, [rbp - 16]
	push rax
	mov rax, [num_to_string_bff]
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp - 32], rax
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp - 16], rax
	mov rax, 0
	mov [rbp - 40], rax
while_condition_5:
	mov rax, [rbp - 40]
	push rax
	mov rax, [rbp - 32]
	pop rbx
	cmp rbx, rax
	jge while_end_5
while_body_5:
	mov rax, [rbp - 16]
	mov byte dl, [rax]
	mov rax, [rbp + 32] ; address points to a location that has the address
	mov byte [rax], dl
	mov rax, [rbp - 40]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 40], rax
	mov rax, [rbp + 32]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp + 32], rax
	mov rax, [rbp - 16]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	sub rax, rbx
	mov [rbp - 16], rax
	jmp while_condition_5
while_end_5:
	mov rax, [rbp + 32]
	mov byte [rax], 10 ; add new line
	mov rax, [rbp - 32]
	push rax
	mov rax, 1
	mov rbx, rax
	pop rax
	add rax, rbx
	mov [rbp - 32], rax
	mov rax, [rbp - 32]
	mov [rbp + 16], rax
	add rsp, 40
	pop rbp
	ret
	; epilogue
	add rsp, 40
	pop rbp
	ret
ptr_get:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 8
	; body
	mov rax, 0
	mov [rbp - 8], rax
	mov rax, [rbp + 24]
	mov rbx, [rax]
	mov [rbp - 8], rbx
	mov rax, [rbp - 8]
	mov [rbp + 16], rax
	add rsp, 8
	pop rbp
	ret
	; epilogue
	add rsp, 8
	pop rbp
	ret
ptr_store:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, [rbp + 24]
	mov rbx, [rbp + 32] 
	mov [rbx], rax
	; epilogue
	add rsp, 0
	pop rbp
	ret
exit:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, 60     
	mov rdi, [rbp + 24]
	syscall
	; epilogue
	add rsp, 0
	pop rbp
	ret
brk:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 8
	; body
	mov rax, 0
	mov [rbp - 8], rax
	mov rax, 12 ; brk syscall
	mov rdi, [rbp + 24]
	syscall
	mov [rbp - 8], rax ; store new program break
	mov rax, [rbp - 8]
	mov [rbp + 16], rax
	add rsp, 8
	pop rbp
	ret
	; epilogue
	add rsp, 8
	pop rbp
	ret
write:
	; prologue
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; body
	mov rax, 1
	mov rdi, [rbp + 40]
	mov rsi, [rbp + 32]
	mov rdx, [rbp + 24]
	syscall
	; epilogue
	add rsp, 0
	pop rbp
	ret


section .data
a dq 0
current_brk dq 0
print_nln_bff dq 0
num_to_string_bff dq 0

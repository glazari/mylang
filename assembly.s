global _start
section .text
_start:
	call main
end:
	mov rax, 60
	xor rdi, rdi
	syscall

main:
	mov rax, 19
	call print_is_prime
	mov rax, 1234
	call num_to_string
	call print
	ret ; return to calling proceedure

print:
	mov rax, 1 ; system call for write
	mov rdi, 1 ; file handle 1 is stdout
	syscall
	ret ; return to calling proceedure

print_is_prime:
	mov rsi, isprime
	mov rdx, 9
	call print
	ret ; return to calling proceedure

num_to_string:
	mov r10, 0       ; r10 is the length of the number
	mov rcx, rax     ; rcx is the number
num_to_string_loop:
	mov rax, rcx
	mov rdx, 0
	mov rbx, 10
	div rbx          ; rax = rax / rbx, rdx = rax % rbx
	add rdx, '0'     ; convert to ascii
	mov byte [number + r10], dl   ; store in number
	inc r10          ; increment length
	mov rcx, rax
	cmp rax, 0
	jne num_to_string_loop
	mov rcx, r10     ; rcx will be the end pointer
	dec rcx   ;  length is one less than end pointer
	mov rsi, 0       ; rsi will be the start pointer
num_to_string_reverse_loop:
	nop
	mov byte dl, [number + rsi]
	mov byte al, [number + rcx]
	mov byte [number + rsi], al
	mov byte [number + rcx], dl
	inc rsi
	dec rcx
	cmp rsi, rcx
	jle num_to_string_reverse_loop
	mov byte [number + r10], 10   ; add newline
	inc r10          ; increment length
	mov rsi, number
	mov rdx, r10
	ret
	ret ; return to calling proceedure


section .data
isprime:	db	"is prime", 10

section .bss
number:	resb	64

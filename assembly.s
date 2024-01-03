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
	mov rax, 1234
	cmp rax, 1
	je main_prime
	call print_not_prime
	jmp main_end
main_prime:
	call print_is_prime
main_end:
	nop
	ret ; return to calling proceedure

; prints message at address rsi with length rdx to stdout
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

; converts number in rax to string and returns address in rsi and length in rdx
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
	ret ; return to calling proceedure

; checks if number in rax is prime, returns 1 if prime, 0 if not
check_prime:
	mov rbx, 2      ; rbx is the divisor
	mov rcx, rax    ; rcx is the number
check_prime_loop:
	cmp rcx, rbx
	jle check_prime_end_loop
	mov rax, rcx
	mov rdx, 0
	div rbx         ; rax = rax / rbx, rdx = rax % rbx
	inc rbx         ; increment divisor
	cmp rdx, 0     ; if remainder is 0, number is not prime
	jne check_prime_loop
	mov rax, 0     ; number is not prime
	ret
check_prime_end_loop:
	mov rax, 1     ; number is prime
	ret
	ret ; return to calling proceedure

print_not_prime:
	mov rsi, notprime
	mov rdx, 10
	call print
	ret ; return to calling proceedure


section .data
isprime:	db	"is prime", 10
notprime:	db	"not prime", 10

section .bss
number:	resb	64

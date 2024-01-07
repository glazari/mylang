global _start
section .text
_start:
	call main
end:
	mov rax, 60
	xor rdi, rdi
	syscall

main:
	mov rdi, 0    ; rdi is the offset for num_to_string
	mov r12, 2    ; r12 is the number to check
main_loop:
	cmp r12, 600000
	jg main_end_loop
	mov rax, r12
	call check_prime
	cmp rax, 1
	jne main_nextnum
	mov rax, r12
	call num_to_string
	add rdi, rdx ; increments offset by length of number
	cmp rdi, 10000
	jl main_nextnum ; print only once buffer has 100 bytes
	mov rsi, number
	mov rdx, rdi
	call print
	mov rdi, 0   ; reset offset
main_nextnum:
	inc r12
	jmp main_loop
main_end_loop:
	nop
	cmp rdi, 0
	je main_end
	mov rsi, number
	mov rdx, rdi
	call print   ; print remaining numbers
	mov rdi, 0   ; reset offset
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

; converts number in rax to string in :number+rdi and returns address in rsi and length in rdx
num_to_string:
	mov r10, 0       ; r10 is the length of the number
	mov rcx, rax     ; rcx is the number
	mov r8, number   ; r8 is the start address of the number
	add r8, rdi      ; r8 is the start address of the number + rdi
num_to_string_loop:
	mov rax, rcx
	mov rdx, 0
	mov rbx, 10
	div rbx          ; rax = rax / rbx, rdx = rax % rbx
	add rdx, '0'     ; convert to ascii
	mov byte [r8 + r10], dl   ; store in number + rdi
	inc r10          ; increment length
	mov rcx, rax
	cmp rax, 0
	jne num_to_string_loop
	mov rcx, r10     ; rcx will be the end pointer
	dec rcx   ;  length is one less than end pointer
	mov rsi, 0       ; rsi will be the start pointer
num_to_string_reverse_loop:
	nop
	mov byte dl, [r8 + rsi]
	mov byte al, [r8 + rcx]
	mov byte [r8 + rsi], al
	mov byte [r8 + rcx], dl
	inc rsi
	dec rcx
	cmp rsi, rcx
	jle num_to_string_reverse_loop
	mov byte [r8 + r10], 10   ; add newline
	inc r10          ; increment length
	mov rsi, r8
	mov rdx, r10
	ret ; return to calling proceedure

; checks if number in rax is prime, returns 1 if prime, 0 if not
check_prime:
	mov rbx, 2      ; rbx is the divisor
	mov rcx, rax    ; rcx is the number
check_prime_loop:
	cmp rcx, rbx
	jle check_prime_end_loop
	mov eax, rcx
	cdq ; extends number to rdx (with zeros)
	div dword ebx         ; rax = rax / rbx, rdx = rax % rbx
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
number:	resb	10024

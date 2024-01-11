
# MyLang

The objective is to have a very simple language and compiler that generates linux x86_64 assembly to help me
understand how compilers work.


At the moment it is just a rust program that writes assembly to a file directly.
I plan to write a couple of programs in assembly like this and start adding features that make it easier to write.
This way I will be working my way backwards from assembly to a higher level language.


I added the following assembly routines:
- print (prints to stdout)
- num_to_string (converts a number to a string for printing, no dynamic memory allocation, just resuse same address)

# todo
- add an is_prime procedure (takes number in rax and returns 1 in rax if prime, 0 otherwise)
- write a main that finds all primes up to 100000 and prints them
- compare performance with c version
- add facility for loops


# ideas
- write tcp primitives (echo server, http server)

## Learnings

### 1. Simple primes.c program is faster than my assembly version

Digging in to try to understand why, I found the following:


1. `test eax eax` is faster than `cmp eax 0` because it is not necessary to do a subtration first (not sure why that is faster)
2. printf might be faster because it does buffering.
	But I found references saying that when wrtting to the terminal it only buffers until the end of line, which matches what I was doing with my assembly version, so I am not sure why it is different.
3. Real culprit was doing a 64 bit division while the c version was using 32 bit. `div dword [divisor]`


# MyLang

The objective is to have a very simple language and compiler that generates linux x86_64 assembly to help me
understand how compilers work.


At the moment it is just a rust program that writes assembly to a file directly.
I plan to write a couple of programs in assembly like this and start adding features that make it easier to write.
This way I will be working my way backwards from assembly to a higher level language.


I added the following assembly routines:
- print (prints to stdout)
- num_to_string (converts a number to a string for printing, no dynamic memory allocation, just resuse same address)

# Prime in assembly

Added `my_primes.s` to calculate primes and print them out. It ran as fast as a c version.

# Abstractions over assembly

Added the concept of loops and if else statements to help handle the jumps and labels.
First version is very simple, we have a global label counter so each new loop or if gets a number
which is used to create the labels.
The user still needs to know the assembly for the cmp and the jump type (jne, jg, jle...).

# todo
- Add function paramaters (needs fixing, some bug)
- Add expression priority parsing (mult, div, greater, less, equal)
- Fix tokenizing of !=
- Add if else statements
	- conditional is an expression whose top level is a comparison	
- Add while loops


# Minimal language

A minimal language needs:
- variables
- functions
	- parameters
	- return values
	- local variables
	- calling convention
- if/else, while, doWhile
- arithmetic operations (at first no nested expressions)
- print to stdout (takes address)
- a way of declaring fixed sized string (no dynamic memory allocation)

# Calling convention

Aiming for simplicity, everything is passed on the stack, in the following order:
- return value  (if any)
- return address (return to caller)
- parameters
- local variables


# ideas
- write tcp primitives (echo server, http server)

## Learnings

### 1. Simple primes.c program is faster than my assembly version

Digging in to try to understand why, I found the following:


1. `test eax eax` is faster than `cmp eax 0` because it is not necessary to do a subtration first (not sure why that is faster)
2. printf might be faster because it does buffering.
	But I found references saying that when wrtting to the terminal it only buffers until the end of line, which matches what I was doing with my assembly version, so I am not sure why it is different.
3. Real culprit was doing a 64 bit division while the c version was using 32 bit. `div dword [divisor]`


# GDB learningd

we can define things in `~/.gdbinit` to make it easier to debug.

I defined there a layout to show registers, asm called ar.
and another layout that includes source code called sar.

```gdb
# set a breakpoint
b <function name>

layout ar

starti # starts the program and stops at the first instruction
stepi # steps one instruction
si # same as stepi
```

```asm
; access stack top
mov eax [esp]
; access stack first int after top of stack
mov eax [esp + 4]
```


# MyLang Syntax

example

```
// comments

let a = 1; // declare a variable

fn add(a, b) -> c {
	c = a + b;
	return c;
}

// if else
if  (a == 1) {
} else {}


while (a == 1) {
}

do {
} while (a == 1);

// assembly excape
asm {
  mov eax 1
  mov ebx number
}

// call
let c = add(1, 2);
```

# GRAMMAR

I initially thought that making the expressions not nested would help, but it makes paring
harder since we need to know if we are parsing a term or an expression. Also
the distinction between expression and conditional I think will make parsing more complicated.

```

program = { function }

function = "fn" identifier "(" [ parameters ] ")" [ "->" identifier ] block

parameters = identifier { "," identifier } 

block = "{" [ statement ] "}"

statement = if | while | doWhile | let | asm | return | assignment

if = "if" "(" expression ")" block [ "else" block ]

while = "while" "(" expression ")" block

doWhile = "do" block "while" "(" expression ")" ";"

let = "let" identifier "=" expression ";"

asm = "asm" "{" { assembly } "}"

assembly = line "\n"

line = ".*"

return = "return" [ expression ] ";"

// not nested expressions for now
expression = identifier	
	Int	
	Bool
	expression "+" expression
	expression "-" expression
	expression "*" expression
	expression "/" expression
	expression "%" expression
	expression "==" expression
	expression "!=" expression
	expression "<" expression
	expression ">" expression
	call

identifier = "a-zA-Z_" { "a-zA-Z0-9_" }
Int = "0-9" { "0-9" }
Bool = "true" | "false"

call = identifier "(" [ arguments ] ")"
arguments = expression { "," expression }

assignment = identifier "=" expression ";"
```


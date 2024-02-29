
# MyLang

The objective is to have a very simple language and compiler that generates linux x86_64 assembly to help me
understand how compilers work.

The language currently has the following features:
- variables
- while/doWhile/if-else control flow statements
- expressions, fully recursive and with operator precedence
- functions with parameters and return values
- global variables
- assembly escape (to allow for things that are not possible in the language, for example system calls)
- Primitive checks:
	- no repeated name declaration
	- no circular dependencies in globals
	- all name references are defined
	- if/while conditions are "compare" expressions


It still has the following important limitations:
- only 64 bit unsigned integers (type assumed, not declared)
- no distinction between pointer and value types
- no arrays (related to types)
- no string literals (related to types and pointer types)
- no structs (related to types)
- no module system (especially useful to not repeat prelude functions like print, malloc, syscalls...)
- varaible can be declared  only at the top level of a block
- variable can be declared after it is used. (just confusing, not a real limitation)
- no block scope.


# todo
- Add support for simple types (signed-i64, u64, f64), all 64 bit to not deal with size.
	- decide syntax (probably very similar to rust, but no type inference)
	- decide structures to represent types
	- type checking of expressions

# ideas
- write tcp primitives (echo server, http server)
- write multithreaded program (mutex, channels)


# Minimal language

A minimal language needs:
- variables
- functions
	- parameters
	- return values
	- local variables
	- calling convention
- if/else, while, doWhile
- arithmetic operations
- print to stdout (takes address)
	- This is delt with by having assembly escape to do the system calls
- a way of declaring fixed sized string

- arrays (fixed size)
- structs
- enums? (sum types)?

# Calling convention

Aiming for simplicity, everything is passed on the stack, in the following order:
- return value  (if any)
- return address (return to caller)
- parameters
- local variables


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

let a: u64 = 1; // declare a variable

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

globals are sintatically very similar to let statements, but their expressions will be checked at compile to be resolved as a constant.
This means that globals can only be defined in terms of literals and other globals, not in terms of functions or other variables. 
And they can not have circular dependencies.	

```

program = { topLevel }

topLevel = function | global

global = "global" identifier ":" type "=" expression ";"

function = "fn" identifier "(" [ parameters ] ")" [ "->" type ] block

parameters = identifier ":" type { "," identifier ":" type } 

block = "{" [ statement ] "}"

statement = if | while | doWhile | let | asm | return | assignment

if = "if" "(" expression ")" block [ "else" block ]

while = "while" "(" expression ")" block

doWhile = "do" block "while" "(" expression ")" ";"

let = "let" identifier ":" type "=" expression ";"

type = "u64"

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

# Prime in assembly

Added `my_primes.s` to calculate primes and print them out. It ran as fast as a c version.



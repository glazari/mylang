mod primes_v1;

use std::fs::File;
use std::io::prelude::*;

struct Program {
    procedures: Vec<Procedure>,
    global_data: Vec<Data>,
}

struct Procedure {
    name: String,
    instructions: Vec<Instructions>,
    description: String,
}

enum Instructions {
    Raw(String),
    Loop(Loop1),
    DoWhile(DoWhile),
    IfElse(IfElse1),
    Call(String),
}

struct Data {
    name: String,
    declaration: String,
}

impl Program {
    fn new() -> Program {
        Program {
            procedures: Vec::new(),
            global_data: Vec::new(),
        }
    }
    fn add_data(&mut self, name: &str, declaration: &str) {
        self.global_data.push(Data {
            name: name.to_string(),
            declaration: declaration.to_string(),
        });
    }
}

impl Instructions {
    fn raw(raw: &str) -> Instructions {
        Instructions::Raw(raw.to_string())
    }

    fn do_while(instructions: Vec<Instructions>, repeat_condition: JumpCondition) -> Instructions {
        Instructions::DoWhile(DoWhile {
            instructions,
            repeat_condition,
        })
    }

    fn loop1(exit_condition: JumpCondition, instructions: Vec<Instructions>) -> Instructions {
        Instructions::Loop(Loop1 {
            exit_condition,
            instructions,
        })
    }

    fn if_else(
        condition: JumpCondition,
        if_instructions: Vec<Instructions>,
        else_instructions: Vec<Instructions>,
    ) -> Instructions {
        Instructions::IfElse(IfElse1 {
            condition,
            if_instructions,
            else_instructions,
        })
    }

    fn call(call: &str) -> Instructions {
        Instructions::Call(call.to_string())
    }
}

struct Loop1 {
    exit_condition: JumpCondition,
    instructions: Vec<Instructions>,
}

struct DoWhile {
    instructions: Vec<Instructions>,
    repeat_condition: JumpCondition,
}

struct JumpCondition {
    condition: String,
    jump: String,
}

struct IfElse1 {
    condition: JumpCondition,
    if_instructions: Vec<Instructions>,
    else_instructions: Vec<Instructions>,
}

fn main() {
    let mut program = Program::new();

    program.add_data("number_buffer", "resb 1024");
    let main = Procedure {
        name: "main".to_string(),
        description: "".to_string(),
        instructions: vec![
            Instructions::raw("mov rdi, 0; rdi is the offset for num_to_string"),
            Instructions::raw("mov r12, 2; r12 is the number to check for primality"),
            Instructions::loop1(
                JumpCondition {
                    condition: "cmp r12, 100000".to_string(),
                    jump: "jg".to_string(),
                },
                vec![
                    Instructions::raw("mov rax, r12"),
                    Instructions::raw("call check_prime"),
                    Instructions::if_else(
                        JumpCondition {
                            condition: "cmp rax, 1".to_string(),
                            jump: "jne".to_string(),
                        },
                        vec![
                            Instructions::raw("push r12"),
                            Instructions::raw("mov r8, number_buffer"),
                            Instructions::raw("add r8, rdi ; r8 is the address to write to"),
                            Instructions::raw("push r8"),
                            Instructions::raw("call num_to_string"),
                            Instructions::raw("add rsp, 16 ; restore stack"),
                            Instructions::raw(
                                "add rdi, rdx ; increment offset by length of number",
                            ),
                            Instructions::if_else(
                                JumpCondition {
                                    condition:
                                        "cmp rdi, 1000  ; print only once buffer has many bytes"
                                            .to_string(),
                                    jump: "jl".to_string(),
                                },
                                vec![
                                    Instructions::raw("mov rsi, number_buffer"),
                                    Instructions::raw("mov rdx, rdi"),
                                    Instructions::call("print"),
                                    Instructions::raw("mov rdi, 0 ; reset offset"),
                                ],
                                vec![],
                            ),
                        ],
                        vec![],
                    ),
                    Instructions::raw("inc r12"),
                ],
            ),
            Instructions::if_else(
                JumpCondition {
                    condition: "cmp rdi, 0  ; if remaining numbers, print them".to_string(),
                    jump: "jle".to_string(),
                },
                vec![
                    Instructions::raw("mov rsi, number_buffer"),
                    Instructions::raw("mov rdx, rdi"),
                    Instructions::call("print"),
                    Instructions::raw("mov rdi, 0 ; reset offset"),
                ],
                vec![],
            ),
        ],
    };
    program.procedures.push(main);
    program.procedures.push(print());
    program.procedures.push(check_prime());
    program.procedures.push(num_to_string());

    let assembly = CodeGenerator::new().generate_code(&program);
    save_to_file("assembly.s", &assembly);
    nasm("assembly.s", "assembly.o");
    ld("assembly.o", "assembly");
    println!("Hello, world!");
}

fn print() -> Procedure {
    return Procedure {
        name: "print".to_string(),
        description: "prints string in rsi with length in rdx".to_string(),
        instructions: vec![
            Instructions::Raw("mov rax, 1".to_string()),
            Instructions::Raw("mov rdi, 1".to_string()),
            Instructions::Raw("syscall".to_string()),
        ],
    };
}

fn check_prime() -> Procedure {
    return Procedure {
        name: "check_prime".to_string(),
        description: "checks if number in rax is prime, returns 1 if prime, 0 if not".to_string(),
        instructions: vec![
            Instructions::raw("mov rbx, 2     ; rbx is the divisor"),
            Instructions::raw("mov rcx, rax   ; rcx is the number"),
            Instructions::loop1(
                JumpCondition {
                    condition: "cmp rcx, rbx".to_string(),
                    jump: "jle".to_string(),
                },
                vec![
                    Instructions::raw("mov eax, ecx"),
                    Instructions::raw("cdq"),
                    Instructions::raw("div dword ebx ; dword is 32 bit, much faster than 64 bit"),
                    Instructions::raw("inc rbx"),
                    Instructions::if_else(
                        JumpCondition {
                            condition: "cmp rdx, 0".to_string(),
                            jump: "jne".to_string(),
                        },
                        vec![Instructions::raw("mov rax, 0"), Instructions::raw("ret")],
                        vec![],
                    ),
                ],
            ),
        ],
    };
}

fn num_to_string() -> Procedure {
    // assumes existence of number_buffer
    return Procedure {
        name: "num_to_string".to_string(),
        // (number, address) -> (length)
        description: "converts number in rax to string in number_buffer+rdi, returns address in rsi and length in rdx"
            .to_string(),
        instructions: vec![
            Instructions::raw("mov rax, [rsp + 16] ; rax is the number"),
            Instructions::raw("mov r8, [rsp + 8] ; r8 the address to write to"),
            Instructions::raw("mov r10, 0 ; r10 is the length of the number"),
            Instructions::raw("mov rcx, rax ; rcx is the number"),
            //Instructions::raw("mov r8, number_buffer; r8 is the address to write to"),
            //Instructions::raw("add r8, rdi ; r8 is the address to write to"),
            Instructions::do_while(
                vec![
                    Instructions::raw("mov rax, rcx"),
                    Instructions::raw("cdq"),
                    Instructions::raw("mov rbx, 10"),
                    Instructions::raw("div dword ebx ; rax = eax:edx / ebx, rdx = eax:edx % ebx"),
                    Instructions::raw("add rdx, '0' ; convert to ascii"),
                    Instructions::raw("mov byte [r8+r10], dl ; store in buffer"),
                    Instructions::raw("inc r10 ; increment length"),
                    Instructions::raw("mov rcx, rax ;"),
                ],
                JumpCondition {
                    condition: "cmp rcx, 0".to_string(),
                    jump: "jne".to_string(),
                },
            ),
            Instructions::raw("mov rcx, r10 ; rcx will be the end pointer"),
            Instructions::raw("dec rcx "),
            Instructions::raw("mov rsi, 0 ; rsi will be the start pointer"),
            Instructions::do_while(
                vec![
                    Instructions::raw("mov byte dl, [r8+rsi]"),
                    Instructions::raw("mov byte al, [r8+rcx]"),
                    Instructions::raw("mov [r8+rcx], dl"),
                    Instructions::raw("mov [r8+rsi], al"),
                    Instructions::raw("inc rsi"),
                    Instructions::raw("dec rcx"),
                ],
                JumpCondition {
                    condition: "cmp rsi, rcx".to_string(),
                    jump: "jle".to_string(),
                },
            ),
            Instructions::raw("mov byte [r8+r10], 10 ; add newline"),
            Instructions::raw("inc r10 ; increment length"),
            Instructions::raw("mov rsi, r8"),
            Instructions::raw("mov rdx, r10"),
        ],
    };
}

fn save_to_file(filename: &str, contents: &str) {
    let mut file = File::create(filename).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
}

struct CodeGenerator {
    assembly: String,
    lable_counter: usize,
}

impl CodeGenerator {
    fn new() -> CodeGenerator {
        CodeGenerator {
            assembly: String::new(),
            lable_counter: 0,
        }
    }

    fn generate_code(&mut self, program: &Program) -> String {
        self.assembly.push_str(
            "
global _start
section .text
_start:
    call main
    ; exit syscall
    mov rax, 60
    xor rdi, rdi ; exit code 0
    syscall


",
        );

        for procedure in &program.procedures {
            self.generate_procedure(procedure);
        }

        // add uninitialized data segment
        self.assembly.push_str("\n\nsection .bss\n");
        for data in &program.global_data {
            self.assembly
                .push_str(&format!("{}: {}\n", data.name, data.declaration));
        }

        self.assembly.clone()
    }

    fn generate_procedure(&mut self, procedure: &Procedure) {
        if procedure.description.len() > 0 {
            self.assembly
                .push_str(&format!("; {}\n", procedure.description));
        }
        self.assembly.push_str(&format!("{}:\n", procedure.name));

        for instruction in &procedure.instructions {
            self.generate_instruction(instruction);
        }

        // return to calling function
        self.assembly.push_str(&format!(
            "\tret ; return to calling proceedure from {}\n\n",
            procedure.name
        ));
    }

    fn generate_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::Raw(raw) => self.assembly.push_str(&format!("\t{}\n", raw)),
            Instructions::Loop(loop1) => self.generate_loop(loop1),
            Instructions::DoWhile(do_while) => self.generate_do_while(do_while),
            Instructions::IfElse(if_else) => self.generate_if_else(if_else),
            Instructions::Call(call) => self.assembly.push_str(&format!("\tcall {}\n", call)),
        }
    }

    fn generate_do_while(&mut self, do_while: &DoWhile) {
        let loop_number = self.lable_counter;
        self.lable_counter += 1;

        self.assembly
            .push_str(&format!("do_while_{}:\n", loop_number));

        for instruction in &do_while.instructions {
            self.generate_instruction(instruction);
        }

        // add conditional jump
        self.assembly
            .push_str(&format!("\t{}\n", do_while.repeat_condition.condition));
        self.assembly.push_str(&format!(
            "\t{} do_while_{}\n",
            do_while.repeat_condition.jump, loop_number
        ));
        self.assembly
            .push_str(&format!("do_while_{}_end:\n", loop_number));
    }

    fn generate_loop(&mut self, loop1: &Loop1) {
        let loop_number = self.lable_counter;
        self.lable_counter += 1;

        self.assembly.push_str(&format!("loop_{}:\n", loop_number));

        // add conditional jump
        self.assembly
            .push_str(&format!("\t{}\n", loop1.exit_condition.condition));
        self.assembly.push_str(&format!(
            "\t{} loop_{}_end\n",
            loop1.exit_condition.jump, loop_number
        ));

        for instruction in &loop1.instructions {
            self.generate_instruction(instruction);
        }

        self.assembly
            .push_str(&format!("\tjmp loop_{}\n", loop_number));
        self.assembly
            .push_str(&format!("loop_{}_end:\n", loop_number));
    }

    fn generate_if_else(&mut self, if_else: &IfElse1) {
        let if_else_number = self.lable_counter;
        self.lable_counter += 1;

        // add conditional jump
        self.assembly
            .push_str(&format!("\t{}\n", if_else.condition.condition));
        self.assembly.push_str(&format!(
            "\t{} if_else_{}_else\n",
            if_else.condition.jump, if_else_number
        ));

        for instruction in &if_else.if_instructions {
            self.generate_instruction(instruction);
        }

        self.assembly
            .push_str(&format!("\tjmp if_else_{}_end\n", if_else_number));
        self.assembly
            .push_str(&format!("if_else_{}_else:\n", if_else_number));

        for instruction in &if_else.else_instructions {
            self.generate_instruction(instruction);
        }

        self.assembly
            .push_str(&format!("if_else_{}_end:\n", if_else_number));
    }
}

fn ld(infile: &str, outfile: &str) {
    let output = std::process::Command::new("ld")
        .arg("-o")
        .arg(outfile)
        .arg(infile)
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        println!("{:?}", output);
        panic!("ld failed");
    }
}

fn nasm(infile: &str, outfile: &str) {
    let output = std::process::Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg(outfile)
        .arg(infile)
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        println!("{:?}", output);
        println!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("nasm failed");
    }
}

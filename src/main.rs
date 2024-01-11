mod primes_v1;

use std::fs::File;
use std::io::prelude::*;

struct Program {
    procedures: Vec<Procedure>,
}

struct Procedure {
    name: String,
    instructions: Vec<Instructions>,
    description: String,
}

enum Instructions {
    Raw(String),
    Label(String),
    Loop(Loop1),
    IfElse(IfElse1),
    Call(String),
}

struct Loop1 {
    exit_condition: JumpCondition,
    instructions: Vec<Instructions>,
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
    let program = Program {
        procedures: vec![Procedure {
            name: "main".to_string(),
            instructions: vec![
                Instructions::Raw("mov rdi, 10".to_string()),
            ],
            description: "main function".to_string(),
        }],
    };
    let assembly = CodeGenerator::new().generate_code(&program);
    save_to_file("assembly.s", &assembly);
    nasm("assembly.s", "assembly.o");
    ld("assembly.o", "assembly");
    println!("Hello, world!");
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

        self.assembly.clone()
    }

    fn generate_procedure(&mut self, procedure: &Procedure) {
        self.assembly.push_str(&format!("{}:\n", procedure.name));

        for instruction in &procedure.instructions {
            self.generate_instruction(instruction);
        }

        // return to calling function
        self.assembly.push_str(&format!(
            "\tret ; return to calling proceedure from {}\n",
            procedure.name
        ));
    }

    fn generate_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::Raw(raw) => self.assembly.push_str(&format!("\t{}\n", raw)),
            Instructions::Label(label) => self.assembly.push_str(&format!("{}:\n", label)),
            Instructions::Loop(loop1) => self.generate_loop(loop1),
            Instructions::IfElse(if_else) => self.generate_if_else(if_else),
            Instructions::Call(call) => self.assembly.push_str(&format!("\tcall {}\n", call)),
        }
    }

    fn generate_loop(&mut self, loop1: &Loop1) {
        let loop_number = self.lable_counter;
        self.lable_counter += 1;

        self.assembly.push_str(&format!("loop_{}:\n", loop_number));

        // add conditional jump
        self.assembly
            .push_str(&format!("\t{}\n", loop1.exit_condition.condition));
        self.assembly.push_str(&format!(
            "\t{} loop_{}_end",
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
            "\t{} if_else_{}_else",
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

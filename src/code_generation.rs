use crate::ast::*;
use crate::checked_program::*;
use std::fs::File;
use std::io::prelude::*;



pub struct CodeGenerator {
    assembly: String,
    lable_counter: u32,
}

impl CodeGenerator {
    pub fn generate_code(prog: CheckedProgram) -> String {
        let mut code_generator = CodeGenerator {
            assembly: String::new(),
            lable_counter: 0,
        };
        code_generator.generate_program(prog);
        code_generator.assembly
    }

    fn generate_program(&mut self, prog: CheckedProgram) {
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

        // enumerate functions 
        for (i, function) in prog.prog.functions.iter().enumerate() {
            self.generate_function(function, &prog.program_env, &prog.function_envs[i]);
        }

        // TODO: uninitialized data section (global variables)
        // TODO: initialized data section (global variables)
    }

    fn generate_function(&mut self, function: &Function, _prog_env: &ProgEnv, _func_env: &FuncEnv) {
        self.assembly.push_str(&format!("{}:\n", function.name));
        self.assembly.push_str("\t; prologue\n");
        //self.assembly.push_str("    push rbp\n");
        //self.assembly.push_str("    mov rbp, rsp\n");
        //self.assembly.push_str(&format!(
        //    "    sub rsp, {}\n",
        //    func_env.local_variables.len() * 8
        //));

        // TODO: handle stack for local variables

        // TODO: generate code for function body
        self.assembly.push_str("\t; body\n");
        
        self.assembly.push_str("\t; epilogue\n");
        // TODO: recover stack from local variables
        self.assembly.push_str("\tret\n");
    }

}

pub fn compile(prog: Program, out_file: &str) -> Result<(), String> {
    let checked_prog = CheckedProgram::check(prog)?;
    let assembly = CodeGenerator::generate_code(checked_prog);

    let assembly_file = format!("{}.asm", out_file);
    let object_file = format!("{}.o", out_file);
    let executable_file = out_file;

    save_to_file(&assembly_file, &assembly);
    nasm(&assembly_file, &object_file);
    ld(&object_file, &executable_file);

    delete_file(&object_file);
    Ok(())
}

fn delete_file(filename: &str) {
    std::fs::remove_file(filename).expect("remove failed");
}


pub fn save_to_file(filename: &str, contents: &str) {
    let mut file = File::create(filename).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
}

pub fn ld(infile: &str, outfile: &str) {
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

pub fn nasm(infile: &str, outfile: &str) {
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

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

    fn generate_function(&mut self, function: &Function, prog_env: &ProgEnv, func_env: &FuncEnv) {
        self.assembly.push_str(&format!("{}:\n", function.name));
        self.assembly.push_str("\t; prologue\n");
        self.assembly.push_str("\tpush rbp\n");
        self.assembly.push_str("\tmov rbp, rsp\n");
        // update stack pointer for local variables
        let bytes_local_variables = func_env.local_variables.len() * 8;
        self.assembly.push_str(format!("\tsub rsp, {}\n", bytes_local_variables).as_str());

        // TODO: generate code for function body
        self.assembly.push_str("\t; body\n");
        for statement in &function.body {
            self.generate_statement(statement, prog_env, func_env);
        }
        
        self.assembly.push_str("\t; epilogue\n");
        // TODO: recover stack from local variables
        self.assembly.push_str(format!("\tadd rsp, {}\n", bytes_local_variables).as_str());
        self.assembly.push_str("\tpop rbp\n");
        self.assembly.push_str("\tret\n");
    }

    fn generate_statement(&mut self, statement: &Statement, p_env: &ProgEnv, f_env: &FuncEnv) {
        match statement {
            Statement::Let(let_statement) => {
                self.generate_let_statement(let_statement, p_env, f_env);
            }
            Statement::If(if_statement) => {
               self.generate_if_statement(if_statement, p_env, f_env); 
            }
            Statement::Assign(assign_statement) => {
                self.generate_assign_statement(assign_statement, p_env, f_env);
            }
            stmt => { panic!("{}", format!("generate_statement: unimplemented {:?}", stmt)); }
            
        }
    }

    fn generate_assign_statement(&mut self, assign_statement: &Assign, p_env: &ProgEnv, f_env: &FuncEnv) {
        self.generate_expression(&assign_statement.value, p_env, f_env);
        let var_num = f_env.local_variables.iter().position(|x| *x == assign_statement.name).unwrap();
        self.assembly.push_str(format!("\tmov [rbp - {}], rax\n", var_num * 8).as_str());
    }

    fn generate_if_statement(&mut self, if_statement: &If, p_env: &ProgEnv, f_env: &FuncEnv) {
        let label_count = self.lable_counter;
        self.lable_counter += 1;

        let if_condition_label = format!("if_condition_{}", label_count);
        let if_body_label = format!("if_body_{}", label_count);
        let else_label = format!("else_{}", label_count);
        let end_label = format!("end_{}", label_count);

        self.assembly.push_str(format!("{}:\n", if_condition_label).as_str());
        match if_statement.condition {
            Conditional::LT(ref t1, ref t2) => {
                self.generate_compare_terms(&t1, &t2, p_env, f_env);
                self.assembly.push_str(format!("\tjge {}\n", else_label).as_str());
            }
            Conditional::GT(ref t1, ref t2) => {
                self.generate_compare_terms(&t1, &t2, p_env, f_env);
                self.assembly.push_str(format!("\tjle {}\n", else_label).as_str());
            }
            Conditional::NE(ref t1, ref t2) => {
                self.generate_compare_terms(&t1, &t2, p_env, f_env);
                self.assembly.push_str(format!("\tje {}\n", else_label).as_str());
            }
            Conditional::Eq(ref t1, ref t2) => {
                self.generate_compare_terms(&t1, &t2, p_env, f_env);
                self.assembly.push_str(format!("\tjne {}\n", else_label).as_str());
            }
        }
        self.assembly.push_str(format!("{}:\n", if_body_label).as_str());
        for statement in &if_statement.body {
            self.generate_statement(statement, p_env, f_env);
        }

        self.assembly.push_str(format!("\tjmp {}\n", end_label).as_str());
        self.assembly.push_str(format!("{}:\n", else_label).as_str());

        for statement in &if_statement.else_body {
            self.generate_statement(statement, p_env, f_env);
        }

        self.assembly.push_str(format!("{}:\n", end_label).as_str());
    }

    fn generate_compare_terms(&mut self, term1: &Term, term2: &Term, p_env: &ProgEnv, f_env: &FuncEnv) {
        self.generate_term(term1, p_env, f_env);
        self.assembly.push_str("\tpush rax\n");
        self.generate_term(term2, p_env, f_env);
        self.assembly.push_str("\tpop rbx\n");
        self.assembly.push_str("\tcmp rbx, rax\n");
    }

    fn generate_let_statement(&mut self, let_statement: &Let, p_env: &ProgEnv, f_env: &FuncEnv) {
        self.generate_expression(&let_statement.value, p_env, f_env);
        let var_num = f_env.local_variables.iter().position(|x| *x == let_statement.name).unwrap();
        self.assembly.push_str(format!("\tmov [rbp - {}], rax\n", var_num * 8).as_str());
    }

    fn generate_expression(&mut self, expression: &Expression, p_env: &ProgEnv, f_env: &FuncEnv) {
        match expression {
            Expression::Int(number) => {
                self.assembly.push_str(format!("\tmov rax, {}\n", number).as_str());
            }
            Expression::Term(term) => {
                self.generate_term(term, p_env, f_env);
            }
            Expression::Add(term1, term2) => {
                self.generate_term(term1, p_env, f_env);
                self.assembly.push_str("\tpush rax\n");
                self.generate_term(term2, p_env, f_env);
                self.assembly.push_str("\tpop rbx\n");
                self.assembly.push_str("\tadd rax, rbx\n");
            }
            expr => { panic!("{}", format!("generate_expression: unimplemented {:?}", expr)); }
        }
    }

    fn generate_term(&mut self, term: &Term, _p_env: &ProgEnv, f_env: &FuncEnv) {
        match term {
            Term::Number(number) => {
                self.assembly.push_str(format!("\tmov rax, {}\n", number).as_str());
            }
            Term::Variable(name) => {
                let var_num = f_env.local_variables.iter().position(|x| *x == *name).unwrap();
                self.assembly.push_str(format!("\tmov rax, [rbp - {}]\n", var_num * 8).as_str());
            }
        }
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

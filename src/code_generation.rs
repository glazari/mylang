use crate::ast::*;
use crate::checked_program::*;
use crate::parser::parse_program;
use crate::tokenizer;
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

        self.assembly.push_str("\n\nsection .data\n");
        for (i, global) in prog.program_env.globals_def.iter().enumerate() {
            let value = &prog.program_env.global_values[i];
            self.assembly
                .push_str(&format!("{} dq {}\n", global.name, value));
        }
    }

    fn add_asm(&mut self, s: &str) {
        self.assembly.push_str(&format!("\t{}\n", s));
    }
    fn add_label(&mut self, s: &str) {
        self.assembly.push_str(&format!("{}:\n", s));
    }

    fn generate_function(&mut self, function: &Function, prog_env: &ProgEnv, func_env: &FuncEnv) {
        self.add_label(&function.name);
        self.add_asm("; prologue");
        self.add_asm("push rbp");
        self.add_asm("mov rbp, rsp");
        // update stack pointer for local variables
        let bytes_local_variables = func_env.local_variables.len() * 8;
        self.add_asm(&format!("sub rsp, {}", bytes_local_variables));

        self.add_asm("; body");
        for stmt in &function.body {
            self.generate_statement(stmt, prog_env, func_env);
        }

        self.add_asm("; epilogue");
        self.generate_function_epilogue(func_env);
    }

    fn generate_function_epilogue(&mut self, func_env: &FuncEnv) {
        let bytes_local_variables = func_env.local_variables.len() * 8;
        self.add_asm(&format!("add rsp, {}", bytes_local_variables));
        self.add_asm("pop rbp");
        self.add_asm("ret");
    }

    fn generate_statement(&mut self, stmt: &Stmt, p_env: &ProgEnv, f_env: &FuncEnv) {
        match stmt {
            Stmt::Let(let_stmt) => self.generate_let_statement(let_stmt, p_env, f_env),
            Stmt::If(if_stmt) => self.generate_if_statement(if_stmt, p_env, f_env),
            Stmt::While(while_stmt) => self.generate_while_stmt(while_stmt, p_env, f_env),
            Stmt::DoWhile(do_while) => self.generate_do_while_stmt(do_while, p_env, f_env),
            Stmt::Assign(assign_stmt) => self.generate_assign_statement(assign_stmt, p_env, f_env),
            Stmt::Return(return_stmt) => {
                self.generate_expression(&return_stmt.value, p_env, f_env);
                self.add_asm("mov [rbp + 16], rax");
                self.generate_function_epilogue(f_env);
            }
            Stmt::Asm(asm) => self.generate_asm_block(asm, p_env, f_env),
            Stmt::Call(call) => self.generate_call(call, p_env, f_env),
        }
    }

    fn generate_asm_block(&mut self, asm: &Asm, p_env: &ProgEnv, f_env: &FuncEnv) {
        let mut line = String::new();

        for segment in &asm.segments {
            match segment {
                ASMSegment::String(s) => {
                    line.push_str(s);
                }
                ASMSegment::Variable(var) => {
                    let var_address = Self::get_var_address(var, f_env, p_env);
                    line.push_str(&var_address);
                }
                ASMSegment::Newline => {
                    self.add_asm(&line);
                    line.clear();
                }
            }
        }
    }

    fn get_var_address(var_name: &str, f_env: &FuncEnv, p_env: &ProgEnv) -> String {
        let var_num = f_env.get_local_pos(var_name);
        if let Some(var_num) = var_num {
            let offset = (var_num as i64 + 1) * 8;
            return format!("[rbp - {}]", offset);
        }
        let param_num = f_env.get_param_pos(var_name);
        if let Some(param_num) = param_num {
            let num_rev = f_env.function_params.len() - param_num - 1;
            let const_offset = 24; // rbp, return address, return value
            let offset = (num_rev as i64) * 8 + const_offset;
            return format!("[rbp + {}]", offset);
        }
        if let Some(_) = p_env.get_global_def(var_name) {
            return format!("[{}]", var_name);
        }
        panic!("Variable not found: {}, {:?}", var_name, p_env.globals_def);
    }

    fn generate_assign_statement(&mut self, assign: &Assign, p_env: &ProgEnv, f_env: &FuncEnv) {
        self.generate_expression(&assign.value, p_env, f_env);
        let var_address = Self::get_var_address(&assign.name, f_env, p_env);
        self.add_asm(&format!("mov {}, rax", var_address));
    }

    fn generate_do_while_stmt(&mut self, do_while: &DoWhile, p_env: &ProgEnv, f_env: &FuncEnv) {
        let label_count = self.lable_counter;
        self.lable_counter += 1;

        let body_label = format!("do_while_body_{}", label_count);
        let condition_label = format!("do_while_condition_{}", label_count);
        let end_label = format!("do_while_end_{}", label_count);

        self.add_label(&body_label);
        for stmt in &do_while.body {
            self.generate_statement(stmt, p_env, f_env);
        }

        self.add_label(&condition_label);
        match &do_while.condition {
            Exp::BinOp(ref e1, op, ref e2, _) => {
                self.generate_expression(e1, p_env, f_env);
                self.add_asm("push rax");
                self.generate_expression(e2, p_env, f_env);
                self.add_asm("pop rbx");
                self.add_asm("cmp rbx, rax");
                let jmp = match op {
                    Op::LT => "jl",
                    Op::GT => "jg",
                    Op::Ne => "jne",
                    Op::Eq => "je",
                    _ => panic!("unimplemented"),
                };
                self.add_asm(&format!("{} {}", jmp, body_label));
            }
            _ => {
                panic!("unimplemented");
            }
        }
        self.add_label(&end_label);
    }

    fn generate_while_stmt(&mut self, while_stmt: &While, p_env: &ProgEnv, f_env: &FuncEnv) {
        let label_count = self.lable_counter;
        self.lable_counter += 1;

        let condition_label = format!("while_condition_{}", label_count);
        let body_label = format!("while_body_{}", label_count);
        let end_label = format!("while_end_{}", label_count);

        self.add_label(&condition_label);
        match &while_stmt.condition {
            Exp::BinOp(ref e1, op, ref e2, _) => {
                self.generate_expression(e1, p_env, f_env);
                self.add_asm("push rax");
                self.generate_expression(e2, p_env, f_env);
                self.add_asm("pop rbx");
                self.add_asm("cmp rbx, rax");
                let jmp = match op {
                    Op::LT => "jge",
                    Op::GT => "jle",
                    Op::Ne => "je",
                    Op::Eq => "jne",
                    _ => panic!("unimplemented"),
                };
                self.add_asm(&format!("{} {}", jmp, end_label));
            }
            _ => panic!("unimplemented"),
        }

        self.add_label(&body_label);
        for stmt in &while_stmt.body {
            self.generate_statement(stmt, p_env, f_env);
        }
        self.add_asm(&format!("jmp {}", condition_label));
        self.add_label(&end_label);
    }

    fn generate_if_statement(&mut self, if_stmt: &If, p_env: &ProgEnv, f_env: &FuncEnv) {
        let label_count = self.lable_counter;
        self.lable_counter += 1;

        let if_condition_label = format!("if_condition_{}", label_count);
        let if_body_label = format!("if_body_{}", label_count);
        let else_label = format!("else_{}", label_count);
        let end_label = format!("end_{}", label_count);

        self.add_label(&if_condition_label);
        match &if_stmt.condition {
            Exp::BinOp(ref e1, op, ref e2, _) => {
                self.generate_expression(e1, p_env, f_env);
                self.add_asm("push rax");
                self.generate_expression(e2, p_env, f_env);
                self.add_asm("pop rbx");
                self.add_asm("cmp rbx, rax");
                let jmp = match op {
                    Op::LT => "jge",
                    Op::GT => "jle",
                    Op::Ne => "je",
                    Op::Eq => "jne",
                    _ => panic!("unimplemented"),
                };
                self.add_asm(&format!("{} {}", jmp, else_label));
            }
            _ => panic!("unimplemented"),
        }
        self.add_label(&if_body_label);
        for stmt in &if_stmt.body {
            self.generate_statement(stmt, p_env, f_env);
        }

        self.add_asm(&format!("jmp {}", end_label));
        self.add_label(&else_label);

        for stmt in &if_stmt.else_body {
            self.generate_statement(stmt, p_env, f_env);
        }

        self.add_label(&end_label);
    }

    fn generate_let_statement(&mut self, let_stmt: &Let, p_env: &ProgEnv, f_env: &FuncEnv) {
        self.generate_expression(&let_stmt.value, p_env, f_env);
        let var_address = Self::get_var_address(&let_stmt.name, f_env, p_env);
        self.add_asm(&format!("mov {}, rax", var_address));
    }

    fn generate_expression(&mut self, exp: &Exp, p_env: &ProgEnv, f_env: &FuncEnv) {
        match exp {
            Exp::U64(number, _) => {
                self.add_asm(&format!("mov rax, {}", number));
            }
            Exp::Var(name, _) => {
                let var_address = Self::get_var_address(name, f_env, p_env);
                self.add_asm(&format!("mov rax, {}", var_address));
            }
            Exp::BinOp(e1, op, e2, _) => {
                self.generate_expression(e1, p_env, f_env);
                self.add_asm("push rax");
                self.generate_expression(e2, p_env, f_env);
                self.add_asm("mov rbx, rax");
                self.add_asm("pop rax");
                match op {
                    Operator::Add => self.add_asm("add rax, rbx"),
                    Operator::Sub => self.add_asm("sub rax, rbx"),
                    Operator::Mul => self.add_asm("mul rbx"), // mul => RDX:RAX := RAX * r/m64
                    Operator::Div => {
                        self.add_asm("cdq"); // sign extend rax to rdx:rax
                        self.add_asm("div rbx"); // rax := rdx:rax / rbx
                    }
                    Operator::Mod => {
                        self.add_asm("cdq"); // sign extend rax to rdx:rax
                        self.add_asm("div rbx"); // rdx := rdx:rax % rbx
                        self.add_asm("mov rax, rdx");
                    }
                    _ => panic!("unimplemented"),
                };
            }
            Exp::Call(call) => {
                self.generate_call(call, p_env, f_env);
            }
        }
    }

    fn generate_call(&mut self, call: &Call, p_env: &ProgEnv, f_env: &FuncEnv) {
        for arg in &call.args {
            self.generate_expression(arg, p_env, f_env);
            self.add_asm("push rax");
        }
        // save space for return value
        self.add_asm("sub rsp, 8");
        self.add_asm(&format!("call {}", call.name));
        // move return value to rax
        self.add_asm("mov rax, [rsp]");
        let stack_offset = call.args.len() * 8 + 8;
        // remove arguments from stack
        self.add_asm(&format!("add rsp, {}", stack_offset));
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

pub fn compile_file(filename: &str) -> Result<(), String> {
    let input = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
    let tokens = tokenizer::tokenize(&input);
    let prog = parse_program(tokens);
    if let Err(e) = prog {
        e.pretty_print(&input);
        return Err(format!("parse error: {:?}", e));
    }
    let prog = prog.unwrap();

    let out_filename = filename.replace(".mylang", "");
    compile(prog, &out_filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_compile() {
        for file in get_all_files("test_cases") {
            if !file.ends_with(".mylang") {
                continue;
            }

            compile_file(&file).expect("compile error");

            let prog_name = file.replace(".mylang", "");
            let output = std::process::Command::new(&prog_name)
                .output()
                .expect("failed to execute process");

            let expected_out_file = file.replace("_code.mylang", "_out.txt");
            let expected_output = std::fs::read_to_string(expected_out_file).expect("read failed");
            let output = String::from_utf8_lossy(&output.stdout);
            assert_eq!(output, expected_output, "file: {}", file);

            delete_file(&prog_name);
            delete_file(&format!("{}.asm", prog_name));
        }
    }

    fn get_all_files(dir: &str) -> Vec<String> {
        let paths = std::fs::read_dir(dir).expect("read_dir failed");
        let mut files = Vec::new();
        for path in paths {
            let path = path.expect("path failed").path();
            let path = path.to_str().expect("to_str failed").to_string();
            files.push(path);
        }
        // sort files to have deterministic order
        files.sort();
        files
    }
}

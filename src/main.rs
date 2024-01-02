use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut assembly = Assembly::new();
    let mut main = Procedure::new("main");
    
    main.add_instruction("mov rax, 19");
    main.add_instruction("mov rbx, 20");
    main.add_instruction("add rax, rbx");
    main.add_instruction("add rax, 40");
    main.add_instruction("mov rax, 1");
    main.add_instruction("mov rax, 2");
    main.add_instruction("mov rax, 3");
    main.add_instruction("mov rax, 4");
    main.add_instruction("mov rax, 5");
    main.add_instruction("mov rax, 6");


    assembly.add_procedure(main);

    let assembly_string = assembly.to_string();

    println!("{}", assembly_string);
    // save to file
    
    assembly.to_file("assembly.s");
    nasm("assembly.s", "assembly.o");
    ld("assembly.o", "assembly");
}



struct Assembly {
    procedures: Vec<Procedure>,
    data_section: Vec<String>,
}

struct Procedure {
    name: String,
    instructions: Vec<String>,
}

impl Procedure {
    fn new(name: &str) -> Procedure {
        Procedure {
            name: name.to_string(),
            instructions: Vec::new(),
        }
    }

    fn add_instruction(&mut self, instruction: &str) {
        self.instructions.push(instruction.to_string());
    }

    fn to_string(&self) -> String {
        let mut procedure_string = String::new();

        procedure_string.push_str(&format!("{}:\n", self.name));

        for instruction in &self.instructions {
            procedure_string.push_str(&format!("\t{}\n", instruction));
        }

        // return to calling function
        // by convention the return value is on the top of the stack
        procedure_string.push_str("\tret ; return to calling proceedure\n");



        procedure_string
    }
}


impl Assembly {
    fn new() -> Assembly {
        Assembly {
            procedures: Vec::new(),
            data_section: Vec::new(),
        }
    }

    fn add_procedure(&mut self, procedure: Procedure) {
        // check if procedure already exists
        for p in &self.procedures {
            if p.name == procedure.name {
                panic!("Procedure {} already exists", procedure.name);
            }
        }

        self.procedures.push(procedure);
    }

    fn to_string(&self) -> String {
        let mut assembly_string = String::new();

        // start
        assembly_string.push_str("global _start\n");
        assembly_string.push_str("section .text\n");
        assembly_string.push_str("_start:\n");
        // jump to main
        assembly_string.push_str("\tcall main\n");
        // end with exit syscall
        assembly_string.push_str("end:\n");
        assembly_string.push_str("\tmov rax, 60\n");
        assembly_string.push_str("\txor rdi, rdi\n");
        assembly_string.push_str("\tsyscall\n\n");

        // add procedures 
        for procedure in &self.procedures {
            assembly_string.push_str(&procedure.to_string());
        }

        

        assembly_string.push_str("\nsection .data\n");
        for line in &self.data_section {
            assembly_string.push_str(&line);
            assembly_string.push_str("\n");
        }

        assembly_string
    }

    fn to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(self.to_string().as_bytes())
            .expect("Unable to write data");
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
        panic!("nasm failed");
    }
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut assembly = Assembly::new();
    assembly.add_message("isprime", "is prime");

    let mut main = Procedure::new("main");
    main.add("mov rax, 19");
    main.add("call print_is_prime");
    main.add("mov rax, 1234");
    main.add("call num_to_string");
    main.add("call print");
    assembly.add_procedure(main);

    // prints message at address rsi with length rdx to stdout
    let mut print = Procedure::new("print");
    print.add("mov rax, 1 ; system call for write");
    print.add("mov rdi, 1 ; file handle 1 is stdout");
    print.add("syscall");
    assembly.add_procedure(print);

    // print isprime
    let mut print_is_prime = Procedure::new("print_is_prime");
    print_is_prime.add("mov rsi, isprime");
    print_is_prime.add("mov rdx, 9");
    print_is_prime.add("call print");
    assembly.add_procedure(print_is_prime);

    // receives number in rax, returns addres of string in rsi and length in rdx
    assembly.reserve_mem("number", 64);
    let mut num_to_string = Procedure::new("num_to_string");
    num_to_string.add("mov r10, 0       ; r10 is the length of the number");
    num_to_string.add("mov rcx, rax     ; rcx is the number");
    num_to_string.add_label("loop", "mov rax, rcx");
    num_to_string.add("mov rdx, 0");
    num_to_string.add("mov rbx, 10");
    num_to_string.add("div rbx          ; rax = rax / rbx, rdx = rax % rbx");
    num_to_string.add("add rdx, '0'     ; convert to ascii");
    num_to_string.add("mov byte [number + r10], dl   ; store in number");
    num_to_string.add("inc r10          ; increment length");
    num_to_string.add("mov rcx, rax");
    num_to_string.add("cmp rax, 0");
    num_to_string.add("jne num_to_string_loop");
    // reverse string

    num_to_string.add("mov rcx, r10     ; rcx will be the end pointer");
    num_to_string.add("dec rcx   ;  length is one less than end pointer");
    num_to_string.add("mov rsi, 0       ; rsi will be the start pointer");
    num_to_string.add_label("reverse_loop", "nop");
    num_to_string.add("mov byte dl, [number + rsi]");
    num_to_string.add("mov byte al, [number + rcx]");
    num_to_string.add("mov byte [number + rsi], al");
    num_to_string.add("mov byte [number + rcx], dl");
    num_to_string.add("inc rsi");
    num_to_string.add("dec rcx");
    num_to_string.add("cmp rsi, rcx");
    num_to_string.add("jle num_to_string_reverse_loop");

    num_to_string.add("mov byte [number + r10], 10   ; add newline");
    num_to_string.add("inc r10          ; increment length");

    num_to_string.add("mov rsi, number");
    num_to_string.add("mov rdx, r10");
    num_to_string.add("ret");


    assembly.add_procedure(num_to_string);

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
    bss_section: Vec<String>,
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

    fn add(&mut self, instruction: &str) {
        let line = format!("\t{}\n", instruction);
        self.instructions.push(line);
    }

    fn add_label(&mut self, label: &str, instruction: &str) {
        // add procedure name as prefix to avoid name collisions
        let label = format!("{}_{}:\n", self.name, label); 
        self.instructions.push(label);
        self.add(instruction);
    }

    fn to_string(&self) -> String {
        let mut procedure_string = String::new();

        procedure_string.push_str(&format!("{}:\n", self.name));

        for instruction in &self.instructions {
            procedure_string.push_str(instruction);
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
            bss_section: Vec::new(),
        }
    }

    fn add_message(&mut self, label: &str, text: &str) {
        // the 10 at the end is a new line
        let line = format!("{}:\tdb\t\"{}\", 10", label, text);
        self.data_section.push(line);
    }

    fn reserve_mem(&mut self, label: &str, size: usize) {
        let line = format!("{}:\tresb\t{}", label, size);
        self.bss_section.push(line);
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
            assembly_string.push_str("\n");
        }

        

        assembly_string.push_str("\nsection .data\n");
        for line in &self.data_section {
            assembly_string.push_str(&line);
            assembly_string.push_str("\n");
        }

        assembly_string.push_str("\nsection .bss\n");
        for line in &self.bss_section {
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
        println!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("nasm failed");
    }
}

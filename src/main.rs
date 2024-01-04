use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut assembly = Assembly::new();
    assembly.add_message("isprime", "is prime");
    assembly.add_message("notprime", "not prime");

    let mut main = Procedure::new("main");
    main.add("mov rdi, 0    ; rdi is the offset for num_to_string");
    main.add("mov r12, 2    ; r12 is the number to check");
    main.add_label("loop", "cmp r12, 200000");
    main.jump("jg", "end_loop");
    main.add("mov rax, r12");
    main.add("call check_prime");
    main.add("cmp rax, 1");
    main.jump("jne", "nextnum");
    main.add("mov rax, r12");
    main.add("call num_to_string");
    main.add("add rdi, rdx ; increments offset by length of number");
    main.add("cmp rdi, 10000");
    main.jump("jl", "nextnum ; print only once buffer has 100 bytes");
    main.add("mov rsi, number");
    main.add("mov rdx, rdi");
    main.add("call print");
    main.add("mov rdi, 0   ; reset offset");
    main.add_label("nextnum", "inc r12");
    main.jump("jmp", "loop");
    main.add_label("end_loop", "nop");
    main.add("cmp rdi, 0");
    main.jump("je", "end");
    main.add("mov rsi, number");
    main.add("mov rdx, rdi");
    main.add("call print   ; print remaining numbers");
    main.add("mov rdi, 0   ; reset offset");
    main.add_label("end", "nop");
    assembly.add_procedure(main);

    assembly.add_procedure(print());
    assembly.add_procedure(print_is_prime());
    assembly.reserve_mem("number", 10024);
    assembly.add_procedure(num_to_string());
    assembly.add_procedure(check_prime());
    assembly.add_procedure(print_not_prime());

    let assembly_string = assembly.to_string();

    println!("{}", assembly_string);

    // save to file
    assembly.to_file("assembly.s");
    nasm("assembly.s", "assembly.o");
    ld("assembly.o", "assembly");
}

fn check_prime() -> Procedure {
    let mut p = Procedure::new("check_prime");
    p.description("checks if number in rax is prime, returns 1 if prime, 0 if not");
    p.add("mov rbx, 2      ; rbx is the divisor");
    p.add("mov rcx, rax    ; rcx is the number");
    p.add_label("loop", "cmp rcx, rbx");
    p.jump("jle", "end_loop");
    p.add("mov rax, rcx");
    p.add("mov rdx, 0");
    p.add("div rbx         ; rax = rax / rbx, rdx = rax % rbx");
    p.add("inc rbx         ; increment divisor");
    p.add("cmp rdx, 0     ; if remainder is 0, number is not prime");
    p.jump("jne", "loop");
    p.add("mov rax, 0     ; number is not prime");
    p.add("ret");
    p.add_label("end_loop", "mov rax, 1     ; number is prime");
    p.add("ret");
    p

}

fn print() -> Procedure {
    let mut p = Procedure::new("print");
    p.description("prints message at address rsi with length rdx to stdout");
    p.add("mov rax, 1 ; system call for write");
    p.add("mov rdi, 1 ; file handle 1 is stdout");
    p.add("syscall");
    p
}

fn print_is_prime() -> Procedure {
    // assumes existence of isprime label
    // assumes existence of print procedure
    let mut p = Procedure::new("print_is_prime");
    p.add("mov rsi, isprime");
    p.add("mov rdx, 9");
    p.add("call print");
    p
}

fn print_not_prime() -> Procedure {
    // assumes existence of notprime label
    // assumes existence of print procedure
    let mut p = Procedure::new("print_not_prime");
    p.add("mov rsi, notprime");
    p.add("mov rdx, 10");
    p.add("call print");
    p
}

fn num_to_string() -> Procedure {
    // assumes existence of number label
    let mut p = Procedure::new("num_to_string");
    p.description("converts number in rax to string in :number+rdi and returns address in rsi and length in rdx");
    p.add("mov r10, 0       ; r10 is the length of the number");
    p.add("mov rcx, rax     ; rcx is the number");
    p.add("mov r8, number   ; r8 is the start address of the number");
    p.add("add r8, rdi      ; r8 is the start address of the number + rdi");
    p.add_label("loop", "mov rax, rcx");
    p.add("mov rdx, 0");
    p.add("mov rbx, 10");
    p.add("div rbx          ; rax = rax / rbx, rdx = rax % rbx");
    p.add("add rdx, '0'     ; convert to ascii");
    p.add("mov byte [r8 + r10], dl   ; store in number + rdi");
    p.add("inc r10          ; increment length");
    p.add("mov rcx, rax");
    p.add("cmp rax, 0");
    p.jump("jne", "loop");

    // reverse string
    p.add("mov rcx, r10     ; rcx will be the end pointer");
    p.add("dec rcx   ;  length is one less than end pointer");
    p.add("mov rsi, 0       ; rsi will be the start pointer");
    p.add_label("reverse_loop", "nop");
    p.add("mov byte dl, [r8 + rsi]");
    p.add("mov byte al, [r8 + rcx]");
    p.add("mov byte [r8 + rsi], al");
    p.add("mov byte [r8 + rcx], dl");
    p.add("inc rsi");
    p.add("dec rcx");
    p.add("cmp rsi, rcx");
    p.jump("jle", "reverse_loop");

    p.add("mov byte [r8 + r10], 10   ; add newline");
    p.add("inc r10          ; increment length");

    p.add("mov rsi, r8");
    p.add("mov rdx, r10");
    p
}

struct Assembly {
    procedures: Vec<Procedure>,
    data_section: Vec<String>,
    bss_section: Vec<String>,
}

struct Procedure {
    name: String,
    instructions: Vec<String>,
    description: String,
}

impl Procedure {
    fn new(name: &str) -> Procedure {
        Procedure {
            name: name.to_string(),
            instructions: Vec::new(),
            description: String::new(),
        }
    }

    fn description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    fn add(&mut self, instruction: &str) {
        let line = format!("\t{}\n", instruction);
        self.instructions.push(line);
    }

    fn jump(&mut self, jmp: &str, label: &str) {
        let line = format!("\t{} {}_{}\n", jmp, self.name, label);
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

        if self.description.len() > 0 {
            procedure_string.push_str(&format!("; {}\n", self.description));
        }

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

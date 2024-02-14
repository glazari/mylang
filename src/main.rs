mod ast;
mod checked_program;
mod code_generation;
mod tokenizer;
mod parser;

use crate::code_generation::*;

fn main() {
    let mut file_name = "test.mylang";
    // check command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        file_name = &args[1];
    }
    
    compile_file(file_name).expect("compile error");
}

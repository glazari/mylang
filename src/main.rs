mod ast;
mod checked_program;
mod code_generation;
mod tokenizer;
mod parser;

use crate::code_generation::*;
use crate::parser::parse_program;

fn main() {
    compile_file("test.mylang").expect("compile error");
}

fn compile_file(filename: &str) -> Result<(), String> {
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

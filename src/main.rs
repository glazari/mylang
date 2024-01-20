mod ast;
mod checked_program;
mod code_generation;
mod primes_v1;
mod primes_v2_flow_control;

use crate::ast::*;
use crate::code_generation::*;

fn main() {
    let prog = primes();
    compile(prog, "primes").unwrap();
}

fn primes() -> Program {
    let mut p = Program {
        functions: Vec::new(),
    };

    p.functions.push(Function {
        name: "main".to_string(),
        params: Vec::new(),
        body: vec![],
    });

    p
}

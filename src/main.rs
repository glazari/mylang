mod ast;
mod checked_program;
mod code_generation;
mod tokenizer;
mod parser;
mod primes_v1;
mod primes_v2_flow_control;

use crate::ast::*;
use crate::code_generation::*;
use crate::parser::parse_program;

fn main() {
    compile_file("test.mylang").expect("compile error");
}

fn compile_file(filename: &str) -> Result<(), String> {
    let input = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
    let tokens = tokenizer::tokenize(&input);
    let prog = parse_program(tokens).expect("parse error");

    let out_filename = filename.replace(".mylang", "");
    compile(prog, &out_filename)
}

fn primes() -> Program {
    let mut p = Program {
        functions: Vec::new(),
    };

    p.functions.push(Function {
        name: "main".to_string(),
        params: Vec::new(),
        body: vec![
            Statement::Let(Let {
                name: "n".to_string(),
                value: Expression::Term(Term::Number(100)),
            }),
            Statement::Let(Let {
                name: "i".to_string(),
                value: Expression::Term(Term::Number(2)),
            }),
            Statement::Let(Let {
                name: "c".to_string(),
                value: Expression::Add(Term::Variable("i".to_string()), Term::Number(1)), 
            }),
            Statement::If(If {
                condition: Conditional::LT(Term::Variable("i".to_string()), Term::Variable("n".to_string())),
                body: vec![Statement::Assign(Assign {
                    name: "i".to_string(),
                    value:  Expression::Term(Term::Number(42)),
                })],
                else_body: vec![Statement::Assign(Assign {
                    name: "i".to_string(),
                    value:  Expression::Term(Term::Number(42)),
                })],
            }),
        ],
    });

    p
}

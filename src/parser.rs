use crate::ast::*;
use crate::tokenizer::{FI, KW, Token, TT};
use std::iter::Peekable;
use std::slice::Iter;

// aliases to make code more consise
type TI<'a> = Peekable<Iter<'a, Token>>;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    expected: String,
    token: Token,
}

fn error<T>(expected: &str, token: &Token) -> Result<T, ParseError> {
    Err(ParseError {
        expected: expected.to_string(),
        token: token.clone(),
    })
}

fn error_eof(expected: &str) -> ParseError {
    ParseError {
        expected: expected.to_string(),
        token: Token::new(TT::EOF, FI::zero()),
    }
}

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut p = Program {
        functions: Vec::new(),
    };

    let mut tokens = tokens.iter().peekable();

    loop {
        skip_whitespace(&mut tokens);
        if let None = tokens.peek() {
            break;
        }
        if let Some(TT::EOF) = tokens.peek().map(|t| &t.token_type) {
            break;
        }

        let f = parse_function(&mut tokens)?;
        p.functions.push(f);
    }

    Ok(p)
}

fn parse_function(ti: &mut TI<'_>) -> Result<Function, ParseError> {
    let t = ti.next().ok_or(error_eof("fn"))?;
    if t.token_type != TT::Keyword(KW::Fn) {
        return error("fn", t);
    }

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("function name"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("function name", t),
    };

    skip_whitespace(ti);
    let params = parse_params(ti)?;
    skip_whitespace(ti);
    let body = parse_block(ti)?;

    Ok(Function { name, params, body })
}

fn parse_block(ti: &mut TI<'_>) -> Result<Vec<Statement>, ParseError> {
    let mut statements = Vec::new();
    let t = ti.next().ok_or(error_eof("{"))?;
    if t.token_type != TT::LBrace {
        return error("{", t);
    }

    loop {
        skip_whitespace(ti);
        let t = ti.peek().ok_or(error_eof("statement or }"))?;
        match t.token_type {
            TT::Keyword(KW::Let) => {
                let let_statement = parse_let(ti)?;
                statements.push(Statement::Let(let_statement));
            }
            TT::Keyword(KW::Return) => {
                let return_statement = parse_return(ti)?;
                statements.push(Statement::Return(return_statement));
            }
            TT::Keyword(KW::If) => {
                let if_statement = parse_if(ti)?;
                statements.push(Statement::If(if_statement));
            }
            TT::Ident(_) => {
                let stmt = parse_ident_start_statement(ti)?;
                statements.push(stmt);
            }
            TT::RBrace => break,
            _ => return error("statement", t),
        }

    }

    let t = ti.next().ok_or(error_eof("}"))?;
    if t.token_type != TT::RBrace {
        return error("}", t);
    }

    Ok(statements)
}

fn parse_if(ti: &mut TI<'_>) -> Result<If, ParseError> {
    let t = ti.next().ok_or(error_eof("if"))?;
    if t.token_type != TT::Keyword(KW::If) {
        return error("if", t);
    }

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("("))?;
    if t.token_type != TT::LParen {
        return error("(", t);
    }

    skip_whitespace(ti);
    let condition = parse_expression(ti, Precedence::Lowest)?;

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof(")"))?;
    if t.token_type != TT::RParen {
        return error(")", t);
    }

    skip_whitespace(ti);
    let body = parse_block(ti)?;

    skip_whitespace(ti);
    if let Some(t) = ti.peek() {
        if t.token_type == TT::Keyword(KW::Else) {
            ti.next();
            skip_whitespace(ti);
            let else_body = parse_block(ti)?;
            return Ok(If { condition, body, else_body });
        }
    }

    let else_body = Vec::new();
    Ok(If { condition, body, else_body })
}

fn parse_return(ti: &mut TI<'_>) -> Result<Return, ParseError> {
    let t = ti.next().ok_or(error_eof("return"))?;
    if t.token_type != TT::Keyword(KW::Return) {
        return error("return", t);
    }

    skip_whitespace(ti);
    let value = parse_expression(ti, Precedence::Lowest)?;

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof(";"))?;
    if t.token_type != TT::Semicolon {
        return error(";", t);
    }

    Ok(Return { value })
}

fn parse_ident_start_statement(ti: &mut TI<'_>) -> Result<Statement, ParseError> {
    let t = ti.next().ok_or(error_eof("identifier"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("identifier", t),
    };

    skip_whitespace(ti);
    let t = ti.peek().ok_or(error_eof("assignment or call"))?;
    match t.token_type {
        TT::Assign => {
            ti.next();
            skip_whitespace(ti);
            let value = parse_expression(ti, Precedence::Lowest)?;
            skip_whitespace(ti);
            let t = ti.next().ok_or(error_eof(";"))?;
            if t.token_type != TT::Semicolon {
                return error(";", t);
            }
            Ok(Statement::Assign(Assign {
                name,
                value,
            }))
        }
        TT::LParen => {
            panic!("call not implemented");
        }
        _ => error("assignment or call", t),
    }
}


fn parse_let(ti: &mut TI<'_>) -> Result<Let, ParseError> {
    let t = ti.next().ok_or(error_eof("let"))?;
    if t.token_type != TT::Keyword(KW::Let) {
        return error("let", t);
    }

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("identifier"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("identifier", t),
    };

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("="))?;
    if t.token_type != TT::Assign {
        return error("=", t);
    }

    skip_whitespace(ti);
    let value = parse_expression(ti, Precedence::Lowest)?;

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof(";"))?;
    if t.token_type != TT::Semicolon {
        return error(";", t);
    }

    Ok(Let {
        name,
        value,
    })
}

fn parse_expression(ti: &mut TI<'_>, prec: Precedence) -> Result<Exp, ParseError> {
    let t = ti.next().ok_or(error_eof("expression"))?;
    let mut exp = match t.token_type {
        TT::Int(n) => { Exp::Int(n) }
        TT::Ident(ref s) => { parse_ident_start_expression(ti, s.clone())? }
        _ => return error("expression", t),
    };

    skip_whitespace(ti);
    while let Some(t) = ti.peek() {
        let next_prec = precedence(t);
        if prec >= next_prec {
            break;
        }
        match t.token_type {
            TT::Plus | TT::Minus | TT::Asterisk | TT::Slash | TT::Eq | TT::NotEq | TT::Lt | TT::Gt => {
                let t = ti.next().unwrap();
                skip_whitespace(ti);
                let right_exp = parse_expression(ti, next_prec)?;
                exp = match t.token_type {
                    TT::Plus => Exp::add(exp, right_exp),
                    TT::Minus => Exp::sub(exp, right_exp),
                    TT::Asterisk => Exp::mul(exp, right_exp),
                    TT::Slash => Exp::div(exp, right_exp),
                    TT::Eq => Exp::Eq(Box::new(exp), Box::new(right_exp)),
                    TT::NotEq => Exp::Ne(Box::new(exp), Box::new(right_exp)),
                    TT::Lt => Exp::LT(Box::new(exp), Box::new(right_exp)),
                    TT::Gt => Exp::GT(Box::new(exp), Box::new(right_exp)),
                    _ => unreachable!(),
                };
            }
            TT::Semicolon | TT::EOF => break,
            TT::Comma | TT::RParen => break, // expressions can appear as arguments to function calls
            _ => return error("operator or ;", t),
        }
    }


    Ok(exp)
}

fn parse_ident_start_expression(ti: &mut TI<'_>, name: String) -> Result<Exp, ParseError> {
    skip_whitespace(ti);
    let t = ti.peek().ok_or(error_eof("variable or call"))?;
    match t.token_type {
        TT::LParen => {
            let call = parse_call(ti, name)?;
            Ok(Exp::Call(call))

        }
        _ => Ok(Exp::Var(name)),
    }
}

fn parse_call(ti: &mut TI<'_>, name: String) -> Result<Call, ParseError> {
    let mut args = Vec::new();
    let t = ti.next().ok_or(error_eof("("))?;
    if t.token_type != TT::LParen {
        return error("(", t);
    }

    loop {
        skip_whitespace(ti);
        let t = ti.peek().ok_or(error_eof("argument or )"))?;
        let arg = match t.token_type {
            TT::RParen => {
                ti.next();
                break;
            },
            _ => parse_expression(ti, Precedence::Lowest)?,
        };
        args.push(arg);

        let t = ti.next().ok_or(error_eof(")"))?;
        match t.token_type {
            TT::RParen => break,
            TT::Comma => continue,
            _ => return error("comma or )", t),
        }
    }

    Ok(Call { name, args })
}


fn parse_params(ti: &mut TI<'_>) -> Result<Vec<Parameter>, ParseError> {
    let mut params = Vec::new();
    // starts with (
    let t = ti.next().ok_or(error_eof("("))?;
    if t.token_type != TT::LParen {
        return error("(", t);
    }

    loop {
        skip_whitespace(ti);
        let t = ti.next().ok_or(error_eof("parameter name or )"))?;
        let par = match t.token_type {
            TT::Ident(ref s) => Parameter { name: s.clone() },
            TT::RParen => break,
            _ => return error("parameter name", t),
        };
        params.push(par);

        let t = ti.next().ok_or(error_eof(")"))?;
        match t.token_type {
            TT::RParen => break,
            TT::Comma => continue,
            _ => return error("comma or )", t),
        }
    }

    Ok(params)
}

fn skip_whitespace(ti: &mut TI<'_>) {
    while let Some(t) = ti.peek() {
        match t.token_type {
            TT::Whitespace | TT::Comment(_) | TT::Newline => {}
            _ => break,
        }
        ti.next();
    }
}

impl ParseError {
    pub fn pretty_print(&self, input: &str) {
        let mut lines = input.lines();
        let mut line = lines.next().unwrap();
        let mut line_num = 1;

        while line_num < self.token.fi.line {
            line = lines.next().unwrap();
            line_num += 1;
        }


        println!("Error at line {}", self.token.fi.line);
        // fixed space for line number 3 digits
        println!("{:3}: {}", self.token.fi.line, line);
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        println!("     {}{}^{}", red,"-".repeat(self.token.fi.column - 1), reset);
        println!("Error: expected `{}` but found {:?}", self.expected, self.token.token_type);
        println!();
    }


}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Comparison,
    Sum,
    Product,
    Prefix,
    Call,
}

fn precedence(t: &Token) -> Precedence {
    match t.token_type {
        TT::Eq | TT::NotEq | TT::Lt | TT::Gt => Precedence::Comparison,
        TT::Plus | TT::Minus => Precedence::Sum,
        TT::Asterisk | TT::Slash => Precedence::Product,
        TT::LParen => Precedence::Call,
        _ => Precedence::Lowest,
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenizer::*;
    use pretty_assertions::assert_eq;

    type Exp = Expression;

    #[test]
    fn test_parse_program() {
        let tokens = tokenize("fn main() { let x = 42 + 1; }");
        let expected = Program {
            functions: vec![Function {
                name: "main".to_string(),
                params: Vec::new(),
                body: vec![Statement::Let(Let {
                    name: "x".to_string(),
                    value: Exp::add(Expression::Int(42), Expression::Int(1)),
                })],
            }],
        };

        let p = parse_program(tokens);

        assert_eq!(p, Ok(expected));
    }

    // helper functions to make the tests more concise
    fn add(x: Exp, y: Exp) -> Exp {
        Exp::add(x, y)
    }
    fn sub(x: Exp, y: Exp) -> Exp {
        Exp::sub(x, y)
    }
    fn int(n: i64) -> Exp {
        Exp::Int(n)
    }
    fn mul(x: Exp, y: Exp) -> Exp {
        Exp::mul(x, y)
    }
    fn div(x: Exp, y: Exp) -> Exp {
        Exp::div(x, y)
    }
    fn eq(x: Exp, y: Exp) -> Exp {
        Exp::eq(x, y)
    }
    fn ne(x: Exp, y: Exp) -> Exp {
        Exp::ne(x, y)
    }
    fn lt(x: Exp, y: Exp) -> Exp {
        Exp::lt(x, y)
    }
    fn gt(x: Exp, y: Exp) -> Exp {
        Exp::gt(x, y)
    }
    

    #[test]
    fn test_parse_expression() {
        struct Test {
            input: &'static str,
            expected: Exp,
        }

        let cases = vec![
            Test {
                input: "42 + 1 + 2 - 3",
                expected: sub(add(add(int(42), int(1)), int(2)), int(3)),
            },
            Test {
                input: "42 + 1 * 2 - 3",
                expected: sub(add(int(42), mul(int(1), int(2))), int(3)),
            },
            Test {
                input: "42 + 1 * 2 / 3",
                expected: add(int(42), div(mul(int(1), int(2)), int(3))),
            },
            Test {
                input: "42 + 1 * 2 / 3 == 4 + 5 * 6",
                expected: eq(
                    add(int(42), div(mul(int(1), int(2)), int(3))),
                    add(int(4), mul(int(5), int(6))),
                ),
            },
            Test {
                input: "42 + 1 * 2 / 3 != 4 + 5 * 6",
                expected: ne(
                    add(int(42), div(mul(int(1), int(2)), int(3))),
                    add(int(4), mul(int(5), int(6))),
                ),
            },
            Test {
                input: "42 + 1 * 2 / 3 < 4 + 5 * 6",
                expected: lt(
                    add(int(42), div(mul(int(1), int(2)), int(3))),
                    add(int(4), mul(int(5), int(6))),
                ),
            },
            Test {
                input: "42 + 1 * 2 / 3 > 4 + 5 * 6",
                expected: gt(
                    add(int(42), div(mul(int(1), int(2)), int(3))),
                    add(int(4), mul(int(5), int(6))),
                ),
            },

        ];

        for t in cases {
            let tokens = tokenize(t.input);
            let mut ti = tokens.iter().peekable();
            println!("input: {}", t.input);
            let e = parse_expression(&mut ti, Precedence::Lowest);

            if let Err(e) = e {
                e.pretty_print(t.input);
                panic!("parse error");
            }

            assert_eq!(e, Ok(t.expected), "input: {}", t.input);
        }
    }

    #[test]
    fn test_parse_parameters() {
        let input = "(x, y, z)";
        let tokens = tokenize(input);
        let expected = vec![
            Parameter { name: "x".to_string() },
            Parameter { name: "y".to_string() },
            Parameter { name: "z".to_string() },
        ];

        let mut ti = tokens.iter().peekable();
        let p = parse_params(&mut ti);

        if let Err(e) = p {
            e.pretty_print(input);
            panic!("parse error");
        }

        assert_eq!(p, Ok(expected));
    }
}

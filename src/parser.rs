use crate::ast::*;
use crate::tokenizer::{FileInfo, Keyword, Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

// aliases to make code more consise
type TT = TokenType;
type TI<'a> = Peekable<Iter<'a, Token>>;
type KW = Keyword;

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
        token: Token::new(TT::EOF, FileInfo::zero()),
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
    let value = parse_expression(ti)?;

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

fn parse_expression(ti: &mut TI<'_>) -> Result<Expression, ParseError> {
    let t = ti.next().ok_or(error_eof("expression"))?;
    let mut exp = match t.token_type {
        TT::Int(n) => { Ok(Expression::Int(n)) }
        _ => error("expression", t),
    };

    skip_whitespace(ti);
    while let Some(t) = ti.peek() {
        match t.token_type {
            TT::Plus => {
                ti.next();
                skip_whitespace(ti);
                let right_exp = parse_expression(ti)?;
                exp = Ok(Expression::Addition(Box::new(exp?), Box::new(right_exp)));
            }
            TT::Minus => {
                ti.next();
                skip_whitespace(ti);
                let right_exp = parse_expression(ti)?;
                exp = Ok(Expression::Subtraction(Box::new(exp?), Box::new(right_exp)));
            }
            TT::Semicolon | TT::EOF => break,
            _ => return error("operator or ;", t),
        }
    }


    exp
}


fn parse_params(ti: &mut TI<'_>) -> Result<Vec<Parameter>, ParseError> {
    let mut params = Vec::new();
    // starts with (
    let t = ti.next().ok_or(error_eof("("))?;
    if t.token_type != TT::LParen {
        return error("(", t);
    }

    loop {
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

fn addition(left: Expression, right: Expression) -> Expression {
    Expression::Addition(Box::new(left), Box::new(right))
}

fn subtraction(left: Expression, right: Expression) -> Expression {
    Expression::Subtraction(Box::new(left), Box::new(right))
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
                    value: addition(Expression::Int(42), Expression::Int(1)),
                })],
            }],
        };

        let p = parse_program(tokens);

        assert_eq!(p, Ok(expected));
    }

    #[test]
    fn test_parse_expression() {
        let input = "42 + 1 + 2 - 3";
        let tokens = tokenize(input);
        let expected = addition(Exp::Int(42), addition(Exp::Int(1), subtraction(Exp::Int(2), Exp::Int(3))));

        let mut ti = tokens.iter().peekable();
        let e = parse_expression(&mut ti);

        if let Err(e) = e {
            e.pretty_print(input);
            panic!("parse error");
        }

        assert_eq!(e, Ok(expected));
    }
}

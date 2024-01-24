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

    Ok(Function {
        name: "main".to_string(),
        params: Vec::new(),
        body: Vec::new(),
    })
}

fn parse_block(ti: &mut TI<'_>) -> Result<Vec<Statement>, ParseError> {
    let mut statements = Vec::new();
    let t = ti.next().ok_or(error_eof("{"))?;
    if t.token_type != TT::LBrace {
        return error("{", t);
    }

    loop {
        skip_whitespace(ti);
        let t = ti.next().ok_or(error_eof("statement"))?;
        if t.token_type == TT::RBrace {
            break;
        }
    }
    Ok(statements)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenizer::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_program() {
        let mut tokens = tokenize("fn main() { let x = 42; }");
        let expected = Program {
            functions: vec![Function {
                name: "main".to_string(),
                params: Vec::new(),
                body: vec![Statement::Let(Let {
                    name: "x".to_string(),
                    value: Expression::Term(Term::Number(42)),
                })],
            }],
        };

        //let p = parse_program(&mut tokens);

        //assert_eq!(p, Ok(expected));
    }
}

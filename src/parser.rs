use crate::ast::*;
use crate::tokenizer::{FileInfo, Keyword, Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

// aliases to make code more consise
type TT = TokenType;
type TI<'a> = Peekable<Iter<'a, Token>>;
type KW = Keyword;

#[derive(Debug, PartialEq)]
struct ParseError {
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

fn parse_program(tokens: &mut Vec<Token>) -> Result<Program, ParseError> {
    let mut p = Program {
        functions: Vec::new(),
    };

    let mut ti = tokens.iter().peekable();

    while let Some(t) = ti.peek() {
        match t.token_type {
            TT::Newline | TT::Whitespace => {
                ti.next();
            }
            TT::Fn => {
                let f = parse_function(&mut ti)?;
                p.functions.push(f);
            }
            _ => return error("fn", t),
        }
    }
    Ok(p)
}

fn parse_function(ti: &mut TI<'_>) -> Result<Function, ParseError> {
    let token = ti.next().ok_or(error_eof("fn"))?;
    if token.token_type != TT::Fn {
        return error("fn", token);
    }
    skip_ws(ti);
    let t = ti.next().ok_or(error_eof("function name"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("function name", t),
    };
    skip_ws(ti);

    let params = parse_params(ti)?;
    skip_ws(ti);
    let token = ti.next().ok_or(error_eof("{"))?;

    if token.token_type != TT::LBrace {
        return error("{", token);
    }

    let body = parse_statements(ti)?;

    Ok(Function { name, params, body })
}

fn parse_statements(ti: &mut TI<'_>) -> Result<Vec<Statement>, ParseError> {
    let mut stmts = Vec::new();
    loop {
        skip_ws(ti);
        let t = ti.next().ok_or(error_eof("}"))?;
        match t.token_type {
            TT::RBrace => break,
            TT::Keyword(ref k) => {
                let s = match k {
                    KW::Let => s = parse_let(ti)?,
                    KW::If => s = parse_if(ti)?,
                    KW::While => s = parse_while(ti)?,
                    KW::Do => s = parse_do_while(ti)?,
                    KW::Return => s = parse_return(ti)?,
                    KW::ASM => s = parse_asm(ti)?,
                    _ => return error("Expected statement keyword", t),
                };
                stmts.push(s);
            }
            _ => return error("Expected keyword", t),
        }
    }
    Ok(stmts)
}

fn parse_if(ti: &mut TI<'_>) -> Result<Statement, ParseError> {
    let t = ti.next().ok_or(error_eof("if"))?;
    if t.token_type != TT::Keyword(KW::If) {
        return error("Expected if", t);
    }
    skip_ws(ti);
    let condition = parse_conditional(ti)?;
    skip_ws(ti);
    let t = ti.next().ok_or(error_eof("{"))?;
    if t.token_type != TT::LBrace {
        return error("{", t);
    }
    let body = parse_statements(ti)?;
    skip_ws(ti);
    let t = ti.next().ok_or(error_eof("} or else"))?;
    let else_body = if t.token_type == TT::Keyword(KW::Else) {
        skip_ws(ti);
        let t = ti.next().ok_or(error_eof("{"))?;
        if t.token_type != TT::LBrace {
            return error("{", t);
        }
        let body = parse_statements(ti)?;
        skip_ws(ti);
        let t = ti.next().ok_or(error_eof("}"))?;
        if t.token_type != TT::RBrace {
            return error("}", t);
        }
        body
    } else {
        Vec::new()
    };
    Ok(Statement::If(If {
        condition,
        body,
        else_body,
    }))
}

fn parse_let(ti: &mut TI<'_>) -> Result<Statement, ParseError> {
    let t = ti.next().ok_or(error_eof("let"))?;
    if t.token_type != TT::Keyword(KW::Let) {
        return error("Expected let", t);
    }
    skip_ws(ti);
    let t = ti.next().ok_or(error_eof("Expected ident, got EOF"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("Expected ident", t),
    };
    skip_ws(ti);
    let t = ti.next().ok_or(error_eof("Expected =, got EOF"))?;
    if t.token_type != TT::Assign {
        return error("Expected =", t);
    }
    skip_ws(ti);
    let value = parse_expression(ti)?;
    skip_ws(ti);
    let t = ti.next().ok_or(error_eof("Expected ;, got EOF"))?;
    if t.token_type != TT::Semicolon {
        return error("Expected ;", t);
    }
    Ok(Statement::Let(Let { name, value }))
}

fn parse_expression(ti: &mut TI<'_>) -> Result<Expression, ParseError> {
    let t = ti.next().ok_or(error_eof("Expected expression, got EOF"))?;
    match t.token_type {
        TT::Int(n) => {
            let t1 = Term::Number(n);
            if let Some(token) = ti.peek() {
                match token.token_type {
                    TT::Plus | TT::Minus | TT::Asterisk | TT::Slash => {
                        ti.next();
                        let term2 = parse_term(ti)?;
                        let expression = match token.token_type {
                            TT::Plus => Expression::Add(t1, term2),
                            TT::Minus => Expression::Sub(t1, term2),
                            TT::Asterisk => Expression::Mul(t1, term2),
                            TT::Slash => Expression::Div(t1, term2),
                            _ => unreachable!(),
                        };
                        Ok(expression)
                    }
                    _ => Ok(Expression::Term(t1)),
                }
            } else {
                Ok(Expression::Term(t1))
            }
        }
        TT::Ident(ref s) => {
            let t1 = Term::Variable(s.clone());
            if let Some(token) = ti.peek() {
                match token.token_type {
                    TT::Plus | TT::Minus | TT::Asterisk | TT::Slash => {
                        ti.next();
                        let term2 = parse_term(ti)?;
                        let expression = match token.token_type {
                            TT::Plus => Expression::Add(t1, term2),
                            TT::Minus => Expression::Sub(t1, term2),
                            TT::Asterisk => Expression::Mul(t1, term2),
                            TT::Slash => Expression::Div(t1, term2),
                            _ => unreachable!(),
                        };
                        Ok(expression)
                    }
                    TT::LParen => {
                        ti.next();
                        let expression = Expression::Call(Call {
                            name: s.clone(),
                            args: parse_arguments(ti)?,
                        });
                        Ok(expression)
                    }
                    _ => Ok(Expression::Term(t1)),
                }
            } else {
                Ok(Expression::Term(t1))
            }
        }
        _ => error("Expected expression", t),
    }
}

fn parse_arguments(ti: &mut TI<'_>) -> Result<Vec<Term>, ParseError> {
    let mut args = Vec::new();
    loop {
        skip_ws(ti);
        let token = ti
            .next()
            .ok_or(error_eof("Expected ) or expression, got EOF"))?;
        match token.token_type {
            TT::RParen => break,
            TT::Int(n) => {
                args.push(Term::Number(n));
            }
            TT::Ident(ref s) => {
                args.push(Term::Variable(s.clone()));
            }
            _ => return error("Expected ) or expression", token),
        }
        skip_ws(ti);
        let token = ti.next().ok_or(error_eof("Expected , or ), got EOF"))?;
        match token.token_type {
            TT::RParen => break,
            TT::Comma => continue,
            _ => return error("Expected , or )", token),
        }
    }

    Ok(args)
}

fn parse_term(ti: &mut TI<'_>) -> Result<Term, ParseError> {
    let token = ti.next().ok_or(error_eof("Expected term, got EOF"))?;
    match token.token_type {
        TT::Int(n) => Ok(Term::Number(n)),
        TT::Ident(ref s) => Ok(Term::Variable(s.clone())),
        _ => error("Expected term", token),
    }
}

fn parse_params(ti: &mut TI<'_>) -> Result<Vec<Parameter>, ParseError> {
    let token = ti.next().ok_or(error_eof("Expected (, got EOF"))?;
    if token.token_type != TT::LParen {
        return error("Expected (", token);
    }

    let mut params = Vec::new();
    loop {
        skip_ws(ti);
        let token = ti.next().ok_or(error_eof("Expected ) or ident, got EOF"))?;
        match token.token_type {
            TT::RParen => break,
            TT::Ident(ref s) => {
                params.push(Parameter { name: s.clone() });
            }
            _ => return error("Expected ) or ident", token),
        }
        skip_ws(ti);
        let token = ti.next().ok_or(error_eof("Expected , or ), got EOF"))?;
        match token.token_type {
            TT::RParen => break,
            TT::Comma => continue,
            _ => return error("Expected , or )", token),
        }
    }

    Ok(params)
}

fn skip_ws(ti: &mut TI<'_>) {
    while let Some(token) = ti.peek() {
        match token.token_type {
            TT::Newline | TT::Whitespace => {
                ti.next();
            }
            _ => break,
        }
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

        let p = parse_program(&mut tokens);

        assert_eq!(p, Ok(expected));
    }
}

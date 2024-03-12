use crate::ast::*;
use crate::tokenizer::{Token, KW, TT};
use crate::file_info::{FI, underline_error};
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
        globals: Vec::new(),
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

        let t = tokens.peek().ok_or(error_eof("function or EOF"))?;
        match t.token_type {
            TT::Keyword(KW::Fn) => {
                let f = parse_function(&mut tokens)?;
                p.functions.push(f);
            }
            TT::Keyword(KW::Global) => {
                let g = parse_global(&mut tokens)?;
                p.globals.push(g);
            }
            _ => return error("function or global", t),
        }
    }

    Ok(p)
}

fn parse_global(ti: &mut TI<'_>) -> Result<Global, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::Global), "global")?;

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("variable name"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("variable name", t),
    };

    expect_sk_ws(ti, TT::Colon, ":")?;
    let ttype = parse_type(ti)?;

    expect_sk_ws(ti, TT::Assign, "=")?;
    let value = parse_expression(ti, Precedence::Lowest)?;

    let efi = expect_sk_ws(ti, TT::Semicolon, ";")?;
    Ok(Global { name, value, ttype, fi: sfi.merge(&efi)})
}

fn parse_function(ti: &mut TI<'_>) -> Result<Function, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::Fn), "fn")?;

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("function name"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("function name", t),
    };

    let params = parse_params(ti)?;

    expect_sk_ws(ti, TT::ReturnArrow, "->")?;
    let ret_type = parse_type(ti)?;

    let (body, efi) = parse_block(ti)?;

    Ok(Function {
        name,
        params,
        body,
        ret_type,
        fi: sfi.merge(&efi),
    })
}

fn parse_block(ti: &mut TI<'_>) -> Result<(Vec<Statement>, FI), ParseError> {
    let mut stmts = Vec::new();
    let sfi = expect_sk_ws(ti, TT::LBrace, "{")?;

    loop {
        skip_whitespace(ti);
        let t = ti.peek().ok_or(error_eof("statement or }"))?;
        match t.token_type {
            TT::Keyword(KW::Let) => stmts.push(Stmt::Let(parse_let(ti)?)),
            TT::Keyword(KW::Return) => stmts.push(Stmt::Return(parse_return(ti)?)),
            TT::Keyword(KW::If) => stmts.push(Stmt::If(parse_if(ti)?)),
            TT::Keyword(KW::While) => stmts.push(Stmt::While(parse_while(ti)?)),
            TT::Keyword(KW::Do) => stmts.push(Stmt::DoWhile(parse_do_while(ti)?)),
            TT::Ident(_) => stmts.push(parse_ident_start_statement(ti)?),
            TT::Keyword(KW::ASM) => stmts.push(Stmt::Asm(parse_asm(ti)?)),
            TT::RBrace => break,
            _ => return error("statement", t),
        }
    }

    let efi = expect(ti, TT::RBrace, "}")?;

    Ok((stmts, sfi.merge(&efi)))
}

fn parse_asm(ti: &mut TI<'_>) -> Result<Asm, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::ASM), "asm")?;
    expect_sk_ws(ti, TT::LBrace, "{")?;
    let mut segments = Vec::new();

    skip_whitespace(ti);
    let mut segment = String::new();
    loop {
        let t = ti.peek().ok_or(error_eof("asm segment or }"))?;
        match t.token_type {
            TT::RBrace => {
                if !segment.is_empty() {
                    segments.push(ASMSegment::String(segment));
                }
                break;
            }
            TT::LBrace => {
                if !segment.is_empty() {
                    segments.push(ASMSegment::String(segment));
                    segment = String::new();
                }
                ti.next();
                // expect a variable name
                let t = ti.next().ok_or(error_eof("variable"))?;
                let name = match t.token_type {
                    TT::Ident(ref s) => s.clone(),
                    _ => return error("variable", t),
                };
                segments.push(ASMSegment::Variable(name));
                expect_sk_ws(ti, TT::RBrace, "}")?;
            }
            TT::Newline => {
                if !segment.is_empty() {
                    segments.push(ASMSegment::String(segment));
                    segment = String::new();
                }
                segments.push(ASMSegment::Newline);
                skip_whitespace(ti);
            }
            _ => {
                segment.push_str(&t.token_type.string());
                ti.next();
            }
        }
    }

    let efi = expect_sk_ws(ti, TT::RBrace, "}")?;
    Ok(Asm { segments, fi: sfi.merge(&efi) })
}

fn parse_do_while(ti: &mut TI<'_>) -> Result<DoWhile, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::Do), "do")?;

    let (body, _) = parse_block(ti)?;

    expect_sk_ws(ti, TT::Keyword(KW::While), "while")?;
    expect_sk_ws(ti, TT::LParen, "(")?;

    let condition = parse_expression(ti, Precedence::Lowest)?;

    expect_sk_ws(ti, TT::RParen, ")")?;
    let efi = expect_sk_ws(ti, TT::Semicolon, ";")?;

    Ok(DoWhile { condition, body, fi: sfi.merge(&efi)})
}

fn parse_while(ti: &mut TI<'_>) -> Result<While, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::While), "while")?;
    expect_sk_ws(ti, TT::LParen, "(")?;

    let condition = parse_expression(ti, Precedence::Lowest)?;

    expect_sk_ws(ti, TT::RParen, ")")?;

    let (body, efi) = parse_block(ti)?;

    Ok(While { condition, body, fi: sfi.merge(&efi)})
}

fn parse_if(ti: &mut TI<'_>) -> Result<If, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::If), "if")?;
    expect_sk_ws(ti, TT::LParen, "(")?;

    let condition = parse_expression(ti, Precedence::Lowest)?;

    expect_sk_ws(ti, TT::RParen, ")")?;

    let (body, mut efi) = parse_block(ti)?;

    let mut else_body = Vec::new();

    skip_whitespace(ti);
    if let Some(t) = ti.peek() {
        if t.token_type == TT::Keyword(KW::Else) {
            ti.next();
            (else_body, efi) = parse_block(ti)?;
        }
    }

    Ok(If {
        condition,
        body,
        else_body,
        fi: sfi.merge(&efi),
    })
}

fn parse_return(ti: &mut TI<'_>) -> Result<Return, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::Return), "return")?;

    let value = parse_expression(ti, Precedence::Lowest)?;

    let efi = expect_sk_ws(ti, TT::Semicolon, ";")?;
    Ok(Return { value, fi: sfi.merge(&efi)})
}

fn parse_ident_start_statement(ti: &mut TI<'_>) -> Result<Statement, ParseError> {
    let t = ti.next().ok_or(error_eof("identifier"))?;
    let sfi = t.fi;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("identifier", t),
    };

    skip_whitespace(ti);
    let t = ti.peek().ok_or(error_eof("assignment or call"))?;
    match t.token_type {
        TT::Assign => {
            ti.next();
            let value = parse_expression(ti, Precedence::Lowest)?;
            let efi = expect_sk_ws(ti, TT::Semicolon, ";")?;
            Ok(Stmt::Assign(Assign { name, value, fi: sfi.merge(&efi)}))
        }
        TT::LParen => {
            let efi = t.fi;
            let call = parse_call(ti, name, efi)?;
            expect_sk_ws(ti, TT::Semicolon, ";")?;
            Ok(Stmt::Call(call))
        }
        _ => error("assignment or call", t),
    }
}

fn parse_let(ti: &mut TI<'_>) -> Result<Let, ParseError> {
    let sfi = expect(ti, TT::Keyword(KW::Let), "let")?;

    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("identifier"))?;
    let name = match t.token_type {
        TT::Ident(ref s) => s.clone(),
        _ => return error("identifier", t),
    };

    expect_sk_ws(ti, TT::Colon, ":")?;
    let ttype = parse_type(ti)?;

    expect_sk_ws(ti, TT::Assign, "=")?;
    let value = parse_expression(ti, Precedence::Lowest)?;

    let efi = expect_sk_ws(ti, TT::Semicolon, ";")?;
    Ok(Let { name, ttype, value, fi: sfi.merge(&efi)})
}

fn parse_type(ti: &mut TI<'_>) -> Result<Type_, ParseError> {
    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("type"))?;
    let ttype = match t.token_type {
        TT::Keyword(KW::U64) => Type_::U64(t.fi),
        TT::Keyword(KW::I64) => Type_::I64(t.fi),
        _ => return error("type", t),
    };
    Ok(ttype)
}

fn parse_expression(ti: &mut TI<'_>, prec: Precedence) -> Result<Exp, ParseError> {
    skip_whitespace(ti);
    let t = ti.next().ok_or(error_eof("expression"))?;
    let sfi = t.fi;
    let mut exp = match t.token_type {
        TT::U64(n) => Exp::U64(n, sfi),
        TT::I64(n) => Exp::I64(n, sfi),
        TT::Ident(ref s) => parse_ident_start_expression(ti, s.clone(), sfi)?,
        _ => return error("expression", t),
    };

    skip_whitespace(ti);
    while let Some(t) = ti.peek() {
        let next_prec = precedence(t);
        if prec >= next_prec {
            break;
        }
        let t = ti.next().ok_or(error_eof("expression"))?;
        match t.token_type {
            TT::Plus => exp = binop(exp, Op::Add, parse_expression(ti, next_prec)?),
            TT::Minus => exp = binop(exp, Op::Sub, parse_expression(ti, next_prec)?),
            TT::Asterisk => exp = binop(exp, Op::Mul, parse_expression(ti, next_prec)?),
            TT::Slash => exp = binop(exp, Op::Div, parse_expression(ti, next_prec)?),
            TT::Percent => exp = binop(exp, Op::Mod, parse_expression(ti, next_prec)?),
            TT::Eq => exp = binop(exp, Op::Eq, parse_expression(ti, next_prec)?),
            TT::NotEq => exp = binop(exp, Op::Ne, parse_expression(ti, next_prec)?),
            TT::Lt => exp = binop(exp, Op::LT, parse_expression(ti, next_prec)?),
            TT::Gt => exp = binop(exp, Op::GT, parse_expression(ti, next_prec)?),
            TT::Semicolon | TT::EOF => break,
            TT::Comma | TT::RParen => break, // expressions can appear as arguments to function calls
            _ => return error("operator or ;", t),
        }
    }

    Ok(exp)
}

fn parse_ident_start_expression(ti: &mut TI<'_>, name: String, sfi: FI) -> Result<Exp, ParseError> {
    skip_whitespace(ti);
    let t = ti.peek().ok_or(error_eof("variable or call"))?;
    match t.token_type {
        TT::LParen => {
            let call = parse_call(ti, name, sfi)?;
            Ok(Exp::Call(call))
        }
        _ => Ok(Exp::Var(name, sfi)),
    }
}

fn parse_call(ti: &mut TI<'_>, name: String, sfi: FI) -> Result<Call, ParseError> {
    let mut args = Vec::new();
    expect(ti, TT::LParen, "(")?;
    
    let efi = loop {
        skip_whitespace(ti);
        let t = ti.peek().ok_or(error_eof("argument or )"))?;
        let arg = match t.token_type {
            TT::RParen => {
                let efi = t.fi;
                ti.next();
                break efi;
            }
            _ => parse_expression(ti, Precedence::Lowest)?,
        };
        args.push(arg);

        let t = ti.next().ok_or(error_eof(")"))?;
        match t.token_type {
            TT::RParen => break t.fi,
            TT::Comma => continue,
            _ => return error("comma or )", t),
        }
    };

    Ok(Call { name, args, fi: sfi.merge(&efi)})
}

fn parse_params(ti: &mut TI<'_>) -> Result<Vec<Parameter>, ParseError> {
    let mut params = Vec::new();
    expect_sk_ws(ti, TT::LParen, "(")?;

    loop {
        skip_whitespace(ti);
        let t = ti.next().ok_or(error_eof("parameter name or )"))?;
        let par = match t.token_type {
            TT::Ident(ref s) => parse_param(ti, s.clone(), t.fi)?,
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

fn parse_param(ti: &mut TI<'_>, name: String, sfi: FI) -> Result<Parameter, ParseError> {
    expect_sk_ws(ti, TT::Colon, ":")?;

    let ttype = parse_type(ti)?;

    Ok(Parameter { name, ttype, fi: sfi.merge(&ttype.fi())})
}

fn expect(ti: &mut TI<'_>, expected: TT, msg: &str) -> Result<FI, ParseError> {
    let t = ti.next().ok_or(error_eof(msg))?;
    if t.token_type != expected {
        return error(msg, t);
    }
    Ok(t.fi)
}

fn expect_sk_ws(ti: &mut TI<'_>, expected: TT, msg: &str) -> Result<FI, ParseError> {
    skip_whitespace(ti);
    expect(ti, expected, msg)
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
        let out_str = underline_error(input, &self.token.fi);
        println!("{}", out_str);
        println!(
            "Error: expected `{}` but found {:?}",
            self.expected, self.token.token_type
        );
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
        TT::Asterisk | TT::Slash | TT::Percent => Precedence::Product,
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
        let tokens = tokenize("fn main() -> u64 { let x: u64 = 42 + 1; }");
        let expected = Program {
            globals: Vec::new(),
            functions: vec![Function {
                name: "main".to_string(),
                params: Vec::new(),
                body: vec![Stmt::Let(Let {
                    name: "x".to_string(),
                    ttype: Type_::U64(FI::new(3, 26)),
                    value: add(int(42, FI::new(2, 32)), int(1, FI::new(1, 37))),
                    fi: FI::new(20, 19), 
                })],
                ret_type: Type_::U64(FI::new(3, 13)),
                fi: FI::new(41, 0),
            }],
        };

        let p = parse_program(tokens);

        assert_eq!(p, Ok(expected));
    }

    // helper functions to make the tests more concise
    fn add(x: Exp, y: Exp) -> Exp {
        binop(x, Op::Add, y)
    }
    fn sub(x: Exp, y: Exp) -> Exp {
        binop(x, Op::Sub, y)
    }
    fn int(n: u64, fi: FI) -> Exp {
        Exp::U64(n, fi)
    }
    fn intz(n: u64) -> Exp {
        Exp::U64(n, FI::zero())
    }
    fn mul(x: Exp, y: Exp) -> Exp {
        binop(x, Op::Mul, y)
    }
    fn div(x: Exp, y: Exp) -> Exp {
        binop(x, Op::Div, y)
    }
    fn eq(x: Exp, y: Exp) -> Exp {
        binop(x, Op::Eq, y)
    }
    fn ne(x: Exp, y: Exp) -> Exp {
        binop(x, Op::Ne, y)
    }
    fn lt(x: Exp, y: Exp) -> Exp {
        binop(x, Op::LT, y)
    }
    fn gt(x: Exp, y: Exp) -> Exp {
        binop(x, Op::GT, y)
    }

    fn zero_out(exp: &mut Exp) {
        match exp {
            Exp::U64(_, ref mut fi) => *fi = FI::zero(),
            Exp::I64(_, ref mut fi) => *fi = FI::zero(),
            Exp::Var(_, ref mut fi) => *fi = FI::zero(),
            Exp::BinOp(le, _, re, ref mut fi) => {
                *fi = FI::zero();
                zero_out(le);
                zero_out(re);
            },
            Exp::Call(ref mut call) => {
                call.fi = FI::zero();
                call.args.iter_mut().for_each(zero_out);
            },
        }
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
                expected: sub(add(add(intz(42), intz(1)), intz(2)), intz(3)),
            },
            Test {
                input: "42 + 1 * 2 - 3",
                expected: sub(add(intz(42), mul(intz(1), intz(2))), intz(3)),
            },
            Test {
                input: "42 + 1 * 2 / 3",
                expected: add(intz(42), div(mul(intz(1), intz(2)), intz(3))),
            },
            Test {
                input: "42 + 1 * 2 / 3 == 4 + 5 * 6",
                expected: eq(
                    add(intz(42), div(mul(intz(1), intz(2)), intz(3))),
                    add(intz(4), mul(intz(5), intz(6))),
                ),
            },
            Test {
                input: "42 + 1 * 2 / 3 != 4 + 5 * 6",
                expected: ne(
                    add(intz(42), div(mul(intz(1), intz(2)), intz(3))),
                    add(intz(4), mul(intz(5), intz(6))),
                ),
            },
            Test {
                input: "42 + 1 * 2 / 3 < 4 + 5 * 6",
                expected: lt(
                    add(intz(42), div(mul(intz(1), intz(2)), intz(3))),
                    add(intz(4), mul(intz(5), intz(6))),
                ),
            },
            Test {
                input: "42 + 1 * 2 / 3 > 4 + 5 * 6",
                expected: gt(
                    add(intz(42), div(mul(intz(1), intz(2)), intz(3))),
                    add(intz(4), mul(intz(5), intz(6))),
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
            let mut e = e.unwrap();
            zero_out(&mut e);

            assert_eq!(e, t.expected, "input: {}", t.input);
        }
    }

    #[test]
    fn test_parse_parameters() {
        let input = "(x: u64, y: u64, z: u64)";
        let tokens = tokenize(input);
        let expected = vec![
            Parameter {
                name: "x".to_string(),
                ttype: Type_::U64(FI::zero()),
                fi: FI::zero(),
            },
            Parameter {
                name: "y".to_string(),
                ttype: Type_::U64(FI::zero()),
                fi: FI::zero(),
            },
            Parameter {
                name: "z".to_string(),
                ttype: Type_::U64(FI::zero()),
                fi: FI::zero(),
            },
        ];

        let mut ti = tokens.iter().peekable();
        let p = parse_params(&mut ti);

        if let Err(e) = p {
            e.pretty_print(input);
            panic!("parse error");
        }
        let mut p = p.unwrap();
        for par in p.iter_mut() {
            par.fi = FI::zero();
            par.ttype = par.ttype.zero();
        }

        assert_eq!(p, expected);
    }

    #[test]
    fn test_parse_asm() {
        let input = r#"asm {
                mov rax, 42
                mov rdi, {a}
                syscall
            }
        "#;

        let expected = Asm {
            segments: vec![
                ASMSegment::String("mov rax, 42".to_string()),
                ASMSegment::Newline,
                ASMSegment::String("mov rdi, ".to_string()),
                ASMSegment::Variable("a".to_string()),
                ASMSegment::Newline,
                ASMSegment::String("syscall".to_string()),
                ASMSegment::Newline,
            ],
            fi: FI::new(100, 0),
        };

        let tokens = tokenize(input);
        let mut ti = tokens.iter().peekable();
        let a = parse_asm(&mut ti);

        if let Err(e) = a {
            e.pretty_print(input);
            panic!("parse error");
        }

        assert_eq!(a, Ok(expected));
    }
}

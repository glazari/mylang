use std::iter::Peekable;
use std::str::Chars;
use crate::file_info::{FileInfo, FI, underline_error};

// aliases to make code more consise
pub type TT = TokenType;
pub type KW = Keyword;


#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub fi: FileInfo,
    pub token_type: TokenType,
}
impl Token {
    pub fn new(token_type: TokenType, fi: FileInfo) -> Token {
        Token { fi, token_type }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    Illegal,
    Ident(String),
    Keyword(Keyword),
    Int(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    ReturnArrow,
    Comma,
    Semicolon,
    Colon,
    Assign,
    Eq,
    NotEq,
    Lt,
    Gt,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Newline,
    Whitespace,
    Comment(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Fn,
    Global,
    If,
    Else,
    While,
    Do,
    Return,
    Let,
    ASM,

    // primitive types
    U64,
    I64,
}

fn keyword_or_ident(ident: &str) -> TokenType {
    match ident {
        // keywords
        "fn" => TT::Keyword(KW::Fn),
        "if" => TT::Keyword(KW::If),
        "else" => TT::Keyword(KW::Else),
        "while" => TT::Keyword(KW::While),
        "do" => TT::Keyword(KW::Do),
        "return" => TT::Keyword(KW::Return),
        "let" => TT::Keyword(KW::Let),
        "asm" => TT::Keyword(KW::ASM),
        "global" => TT::Keyword(KW::Global),
        // primitive types
        "u64" => TT::Keyword(KW::U64),
        "i64" => TT::Keyword(KW::I64),
        _ => TT::Ident(ident.to_string()),
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut fi = FI::zero();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => tokenize_whitespace(&mut chars, &mut fi, &mut tokens),
            '0'..='9' => {
                let start_fi = fi.clone();
                let int = tokenize_int(&mut chars, &mut fi);
                tokens.push(Token::new(int, fi.len_diff(&start_fi)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = fi.clone();
                let ident = tokenize_ident(&mut chars, &mut fi);
                tokens.push(Token::new(ident, fi.len_diff(&start)));
            }
            '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | '=' | '+' | '-' | '*' | '/' | '%'
            | '<' | '>' | '!' | ':' => {
                let start = fi.clone();
                let simbol = tokenize_simbol(&mut chars, &mut fi);
                tokens.push(Token::new(simbol, fi.len_diff(&start)));
            }
            _ => {
                println!("{}", underline_error(input, &fi));
                println!("fi: {:?}", fi);
                panic!("Unknown character: {}", c);
            }
        }
    }

    tokens.push(Token::new(TT::EOF, FI::new(0, input.len())));

    tokens
}

fn tokenize_simbol(chars: &mut Peekable<Chars>, fi: &mut FileInfo) -> TokenType {
    let c = chars.next().unwrap();
    fi.inc();
    match c {
        '(' => TT::LParen,
        ')' => TT::RParen,
        '{' => TT::LBrace,
        '}' => TT::RBrace,
        '[' => TT::LBracket,
        ']' => TT::RBracket,
        ';' => TT::Semicolon,
        ':' => TT::Colon,
        ',' => TT::Comma,
        '=' => {
            if let Some('=') = chars.peek() {
                chars.next();
                fi.inc();
                TT::Eq
            } else {
                TT::Assign
            }
        }
        '!' => {
            if let Some('=') = chars.peek() {
                chars.next();
                fi.inc();
                TT::NotEq
            } else {
                TT::Illegal
            }
        }
        '+' => TT::Plus,
        '-' => {
            if let Some('>') = chars.peek() {
                chars.next();
                fi.inc();
                TT::ReturnArrow
            } else {
                TT::Minus
            }
        }
        '*' => TT::Asterisk,
        '/' => {
            if let Some('/') = chars.peek() {
                chars.next();
                fi.inc();
                let mut comment = String::new();
                while let Some(c) = chars.peek() {
                    match c {
                        '\n' => break,
                        _ => {
                            comment.push(*c);
                            chars.next();
                            fi.inc();
                        }
                    }
                }
                TT::Comment(comment)
            } else {
                TT::Slash
            }
        }
        '%' => TT::Percent,
        '<' => TT::Lt,
        '>' => TT::Gt,
        _ => TT::Illegal,
    }
}

fn tokenize_int(chars: &mut Peekable<Chars>, fi: &mut FileInfo) -> TokenType {
    let mut int = String::new();
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => {
                int.push(*c);
                chars.next();
                fi.inc();
            }
            _ => break,
        }
    }
    TT::Int(int.parse::<i64>().unwrap())
}

fn tokenize_ident(chars: &mut Peekable<Chars>, fi: &mut FileInfo) -> TokenType {
    let mut ident = String::new();
    while let Some(c) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                ident.push(*c);
                chars.next();
                fi.inc();
            }
            _ => break,
        }
    }
    keyword_or_ident(&ident)
}

fn tokenize_whitespace(chars: &mut Peekable<Chars>, fi: &mut FileInfo, tokens: &mut Vec<Token>) {
    while let Some(c) = chars.peek() {
        let tt = match c {
            ' ' | '\t' => TT::Whitespace,
            '\n' => TT::Newline,
            _ => break,
        };
        tokens.push(Token::new(tt, FI::new(1, fi.offset)));
        chars.next();
        fi.inc();
    }
}

impl TokenType {
    pub fn string(&self) -> String {
        let tmp: String;
        let out = match self {
            TT::EOF => "",
            TT::Illegal => "Illegal",
            TT::Ident(s) => s,
            TT::Keyword(kw) => match kw {
                KW::Fn => "fn",
                KW::Global => "global",
                KW::If => "if",
                KW::Else => "else",
                KW::While => "while",
                KW::Do => "do",
                KW::Return => "return",
                KW::Let => "let",
                KW::ASM => "asm",
                KW::U64 => "u64",
                KW::I64 => "i64",
            },
            TT::Int(i) => {
                tmp = i.to_string();
                &tmp
            }
            TT::LParen => "(",
            TT::RParen => ")",
            TT::LBrace => "{",
            TT::RBrace => "}",
            TT::LBracket => "[",
            TT::RBracket => "]",
            TT::ReturnArrow => "->",
            TT::Comma => ",",
            TT::Semicolon => ";",
            TT::Colon => ":",
            TT::Assign => "=",
            TT::Eq => "==",
            TT::NotEq => "!=",
            TT::Lt => "<",
            TT::Gt => ">",
            TT::Plus => "+",
            TT::Minus => "-",
            TT::Asterisk => "*",
            TT::Slash => "/",
            TT::Percent => "%",
            TT::Newline => "\n",
            TT::Whitespace => " ",
            TT::Comment(s) => {
                tmp = format!("//{}", s);
                &tmp
            }
        };
        out.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn token(tt: TokenType, length: usize, offset: usize) -> Token {
        Token::new(tt, FI::new(length, offset))
    }

    #[test]
    fn test_tokenize() {
        let input = "fn add(x, y) { x + y }";
        let expected = vec![
            token(TT::Keyword(KW::Fn), 2, 0),
            token(TT::Whitespace, 1, 2),
            token(TT::Ident("add".to_string()), 3, 3),
            token(TT::LParen, 1, 6),
            token(TT::Ident("x".to_string()), 1, 7),
            token(TT::Comma, 1, 8),
            token(TT::Whitespace, 1, 9),
            token(TT::Ident("y".to_string()), 1, 10),
            token(TT::RParen, 1, 11),
            token(TT::Whitespace, 1, 12),
            token(TT::LBrace, 1, 13),
            token(TT::Whitespace, 1, 14),
            token(TT::Ident("x".to_string()),  1, 15),
            token(TT::Whitespace, 1, 16),
            token(TT::Plus, 1, 17),
            token(TT::Whitespace, 1, 18),
            token(TT::Ident("y".to_string()), 1, 19),
            token(TT::Whitespace, 1, 20),
            token(TT::RBrace, 1, 21),
            token(TT::EOF, 0, 22),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_neq() {
        let input = "x != y";
        let expected = vec![
            token(TT::Ident("x".to_string()), 1, 0),
            token(TT::Whitespace, 1, 1),
            token(TT::NotEq, 2, 2),
            token(TT::Whitespace, 1, 4),
            token(TT::Ident("y".to_string()), 1, 5),
            token(TT::EOF, 0, 6),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_comment() {
        let input = "x // comment\n y";
        let expected = vec![
            token(TT::Ident("x".to_string()), 1, 0),
            token(TT::Whitespace, 1, 1),
            token(TT::Comment(" comment".to_string()), 10, 2),
            token(TT::Newline, 1, 12),
            token(TT::Whitespace, 1, 13),
            token(TT::Ident("y".to_string()), 1, 14),
            token(TT::EOF, 0, 15),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }
}

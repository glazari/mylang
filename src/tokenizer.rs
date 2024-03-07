use std::iter::Peekable;
use std::str::Chars;
use crate::file_info::FileInfo;

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
    let mut fi = FileInfo {
        line: 1,
        column: 1,
        length: 0,
        offset: 0,
    };
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
            _ => panic!("Unknown character: {}", c),
        }
    }

    tokens.push(Token::new(
        TokenType::EOF,
        FileInfo::new(fi.line, fi.column, 0, input.len()),
    ));

    tokens
}

fn tokenize_simbol(chars: &mut Peekable<Chars>, fi: &mut FileInfo) -> TokenType {
    let c = chars.next().unwrap();
    fi.col_inc();
    match c {
        '(' => TokenType::LParen,
        ')' => TokenType::RParen,
        '{' => TokenType::LBrace,
        '}' => TokenType::RBrace,
        '[' => TokenType::LBracket,
        ']' => TokenType::RBracket,
        ';' => TokenType::Semicolon,
        ':' => TokenType::Colon,
        ',' => TokenType::Comma,
        '=' => {
            if let Some('=') = chars.peek() {
                chars.next();
                fi.col_inc();
                TokenType::Eq
            } else {
                TokenType::Assign
            }
        }
        '!' => {
            if let Some('=') = chars.peek() {
                chars.next();
                fi.col_inc();
                TokenType::NotEq
            } else {
                TokenType::Illegal
            }
        }
        '+' => TokenType::Plus,
        '-' => {
            if let Some('>') = chars.peek() {
                chars.next();
                fi.col_inc();
                TokenType::ReturnArrow
            } else {
                TokenType::Minus
            }
        }
        '*' => TokenType::Asterisk,
        '/' => {
            if let Some('/') = chars.peek() {
                chars.next();
                fi.col_inc();
                let mut comment = String::new();
                while let Some(c) = chars.peek() {
                    match c {
                        '\n' => break,
                        _ => {
                            comment.push(*c);
                            chars.next();
                            fi.col_inc();
                        }
                    }
                }
                TokenType::Comment(comment)
            } else {
                TokenType::Slash
            }
        }
        '%' => TokenType::Percent,
        '<' => TokenType::Lt,
        '>' => TokenType::Gt,
        _ => TokenType::Illegal,
    }
}

fn tokenize_int(chars: &mut Peekable<Chars>, fi: &mut FileInfo) -> TokenType {
    let mut int = String::new();
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => {
                int.push(*c);
                chars.next();
                fi.col_inc();
            }
            _ => break,
        }
    }
    TokenType::Int(int.parse::<i64>().unwrap())
}

fn tokenize_ident(chars: &mut Peekable<Chars>, fi: &mut FileInfo) -> TokenType {
    let mut ident = String::new();
    while let Some(c) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                ident.push(*c);
                chars.next();
                fi.col_inc();
            }
            _ => break,
        }
    }
    keyword_or_ident(&ident)
}

fn tokenize_whitespace(chars: &mut Peekable<Chars>, fi: &mut FileInfo, tokens: &mut Vec<Token>) {
    while let Some(c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                tokens.push(Token::new(
                    TokenType::Whitespace,
                    FileInfo::new(fi.line, fi.column, 1, fi.offset),
                ));
                chars.next();
                fi.col_inc();
            }
            '\n' => {
                tokens.push(Token::new(
                    TokenType::Newline,
                    FileInfo::new(fi.line, fi.column, 1, fi.offset),
                ));
                chars.next();
                fi.line_inc();
            }
            _ => break,
        }
    }
}

impl TokenType {
    pub fn string(&self) -> String {
        let tmp: String;
        let out = match self {
            TokenType::EOF => "",
            TokenType::Illegal => "Illegal",
            TokenType::Ident(s) => s,
            TokenType::Keyword(kw) => match kw {
                Keyword::Fn => "fn",
                Keyword::Global => "global",
                Keyword::If => "if",
                Keyword::Else => "else",
                Keyword::While => "while",
                Keyword::Do => "do",
                Keyword::Return => "return",
                Keyword::Let => "let",
                Keyword::ASM => "asm",
                Keyword::U64 => "u64",
                Keyword::I64 => "i64",
            },
            TokenType::Int(i) => {
                tmp = i.to_string();
                &tmp
            }
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",
            TokenType::ReturnArrow => "->",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Colon => ":",
            TokenType::Assign => "=",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Percent => "%",
            TokenType::Newline => "\n",
            TokenType::Whitespace => " ",
            TokenType::Comment(s) => {
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

    fn token(tt: TokenType, line: usize, column: usize, length: usize, offset: usize) -> Token {
        Token::new(tt, FileInfo::new(line, column, length, offset))
    }

    #[test]
    fn test_tokenize() {
        let input = "fn add(x, y) { x + y }";
        let expected = vec![
            token(TokenType::Keyword(Keyword::Fn), 1, 1, 2, 0),
            token(TokenType::Whitespace, 1, 3, 1, 2),
            token(TokenType::Ident("add".to_string()), 1, 4, 3, 3),
            token(TokenType::LParen, 1, 7, 1, 6),
            token(TokenType::Ident("x".to_string()), 1, 8, 1, 7),
            token(TokenType::Comma, 1, 9, 1, 8),
            token(TokenType::Whitespace, 1, 10, 1, 9),
            token(TokenType::Ident("y".to_string()), 1, 11, 1, 10),
            token(TokenType::RParen, 1, 12, 1, 11),
            token(TokenType::Whitespace, 1, 13, 1, 12),
            token(TokenType::LBrace, 1, 14, 1, 13),
            token(TokenType::Whitespace, 1, 15, 1, 14),
            token(TokenType::Ident("x".to_string()), 1, 16, 1, 15),
            token(TokenType::Whitespace, 1, 17, 1, 16),
            token(TokenType::Plus, 1, 18, 1, 17),
            token(TokenType::Whitespace, 1, 19, 1, 18),
            token(TokenType::Ident("y".to_string()), 1, 20, 1, 19),
            token(TokenType::Whitespace, 1, 21, 1, 20),
            token(TokenType::RBrace, 1, 22, 1, 21),
            token(TokenType::EOF, 1, 23, 0, 22),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_neq() {
        let input = "x != y";
        let expected = vec![
            token(TokenType::Ident("x".to_string()), 1, 1, 1, 0),
            token(TokenType::Whitespace, 1, 2, 1, 1),
            token(TokenType::NotEq, 1, 3, 2, 2),
            token(TokenType::Whitespace, 1, 5, 1, 4),
            token(TokenType::Ident("y".to_string()), 1, 6, 1, 5),
            token(TokenType::EOF, 1, 7, 0, 6),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_comment() {
        let input = "x // comment\n y";
        let expected = vec![
            token(TokenType::Ident("x".to_string()), 1, 1, 1, 0),
            token(TokenType::Whitespace, 1, 2, 1, 1),
            token(TokenType::Comment(" comment".to_string()), 1, 3, 10, 2),
            token(TokenType::Newline, 1, 13, 1, 12),
            token(TokenType::Whitespace, 2, 1, 1, 13),
            token(TokenType::Ident("y".to_string()), 2, 2, 1, 14),
            token(TokenType::EOF, 2, 3, 0, 15),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }
}

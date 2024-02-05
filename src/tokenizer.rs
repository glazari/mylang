use std::iter::Peekable;
use std::str::Chars;

// aliases to make code more consise
pub type TT = TokenType;
pub type FI = FileInfo;
pub type KW = Keyword;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FileInfo {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl FileInfo {
    fn new(line: usize, column: usize, length: usize) -> FileInfo {
        FileInfo {line, column, length}
    }
    pub fn zero() -> FileInfo {
        FileInfo {line: 0, column: 0, length: 0}
    }

    fn col_inc(&mut self) {
        self.column += 1;
        self.length += 1;
    }
    fn line_inc(&mut self) {
        self.line += 1;
        self.length += 1;
        // reset column
        self.column = 1;
    }

    fn len_diff(&self, start: &FileInfo) -> FileInfo {
        FileInfo::new(start.line, start.column, self.length - start.length)
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub fi: FileInfo,
    pub token_type: TokenType,
}
impl Token {
    
    pub fn new(token_type: TokenType, fi: FileInfo) -> Token {
        Token {fi, token_type}
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
    If,
    Else,
    While,
    Do,
    Return,
    Let,
    ASM,
}

// keywords
fn keywordOrIdent(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Keyword(Keyword::Fn),
        "if" => TokenType::Keyword(Keyword::If),
        "else" => TokenType::Keyword(Keyword::Else),
        "while" => TokenType::Keyword(Keyword::While),
        "do" => TokenType::Keyword(Keyword::Do),
        "return" => TokenType::Keyword(Keyword::Return),
        "let" => TokenType::Keyword(Keyword::Let),
        "asm" => TokenType::Keyword(Keyword::ASM),
        _ => TokenType::Ident(ident.to_string()),
    }
}

pub fn tokenize(input: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut fi = FileInfo {line: 1, column: 1, length: 0};
    let mut chars = input.chars().peekable();


    while let Some(c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => tokenize_whitespace(&mut chars, &mut fi, &mut tokens),
            '0'..='9' => {
                let start_fi = fi.clone();
                let int = tokenize_int(&mut chars, &mut fi);
                tokens.push(Token::new(int, fi.len_diff(&start_fi)));
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = fi.clone();
                let ident = tokenize_ident(&mut chars, &mut fi);
                tokens.push(Token::new(ident, fi.len_diff(&start)));
            },
            '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | '=' | '+' | '-' | '*' | '/' | '%' | '<' | '>' => {
                let start = fi.clone();
                let simbol = tokenize_simbol(&mut chars, &mut fi);
                tokens.push(Token::new(simbol, fi.len_diff(&start)));
            },
            _ => {},
        }
        
    }

    tokens.push(Token::new(TokenType::EOF, FileInfo::new(fi.line, fi.column, 0)));

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
        ',' => TokenType::Comma,
        '=' => {
            if let Some('=') = chars.peek() {
                chars.next();
                fi.col_inc();
                TokenType::Eq
            } else {
                TokenType::Assign
            }
        },
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Asterisk,
        '/' => TokenType::Slash,
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
            },
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
            },
            _ => break,
        }
    }
    keywordOrIdent(&ident)
}

fn tokenize_whitespace(chars: &mut Peekable<Chars>, fi: &mut FileInfo, tokens: &mut Vec<Token>) {
    while let Some(c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                tokens.push(Token::new(TokenType::Whitespace, FileInfo::new(fi.line, fi.column, 1)));
                chars.next();
                fi.col_inc();
            },
            '\n' => {
                tokens.push(Token::new(TokenType::Newline, FileInfo::new(fi.line, fi.column, 1)));
                chars.next();
                fi.line_inc();
            },
            _ => break,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn token(tt: TokenType, line: usize, column: usize, length: usize) -> Token {
        Token::new(tt, FileInfo::new(line, column, length))
    }

    #[test]
    fn test_tokenize() {
        let input = "fn add(x, y) { x + y }";
        let expected = vec![
            token(TokenType::Keyword(Keyword::Fn), 1, 1, 2),
            token(TokenType::Whitespace, 1, 3, 1),
            token(TokenType::Ident("add".to_string()), 1, 4, 3),
            token(TokenType::LParen, 1, 7, 1),
            token(TokenType::Ident("x".to_string()), 1, 8, 1),
            token(TokenType::Comma, 1, 9, 1),
            token(TokenType::Whitespace, 1, 10, 1),
            token(TokenType::Ident("y".to_string()), 1, 11, 1),
            token(TokenType::RParen, 1, 12, 1),
            token(TokenType::Whitespace, 1, 13, 1),
            token(TokenType::LBrace, 1, 14, 1),
            token(TokenType::Whitespace, 1, 15, 1),
            token(TokenType::Ident("x".to_string()), 1, 16, 1),
            token(TokenType::Whitespace, 1, 17, 1),
            token(TokenType::Plus, 1, 18, 1),
            token(TokenType::Whitespace, 1, 19, 1),
            token(TokenType::Ident("y".to_string()), 1, 20, 1),
            token(TokenType::Whitespace, 1, 21, 1),
            token(TokenType::RBrace, 1, 22, 1),
            token(TokenType::EOF, 1, 23, 0),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    #[ignore]
    fn test_tokenize_neq() {
        let input = "x != y";
        let expected = vec![
            token(TokenType::Ident("x".to_string()), 1, 1, 1),
            token(TokenType::Whitespace, 1, 2, 1),
            token(TokenType::NotEq, 1, 3, 2),
            token(TokenType::Whitespace, 1, 5, 1),
            token(TokenType::Ident("y".to_string()), 1, 6, 1),
            token(TokenType::EOF, 1, 7, 0),
        ];

        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }
}

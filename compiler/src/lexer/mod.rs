mod token;

use std::num::ParseIntError;

pub use token::Token;

#[derive(Debug)]
pub enum LexerError {
    MissingStringQuote,
    NumberParseError(ParseIntError)
}

pub struct Lexer {
    source_stream: Vec<char>,
    position: usize
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Self { source_stream: source, position: 0 }
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c = self.source_stream.get(self.position);
        self.position += 1;

        /* In case stream is ended */
        let mut c = if let Some(c) = c {
            c
        } else {
            return None;
        };

        /* Ignore whitespace */
        while c.is_whitespace() {
            c = match self.source_stream.get(self.position) {
                Some(v) => v,
                None => return None
            };
            self.position += 1;
        }
        
        /* In case there is a ident */
        if c.is_alphabetic() || *c == '_' {
            let mut s = String::new();
            s.push(*c);
            loop {
                let n = self.source_stream.get(self.position);
                if let Some(x) = n {
                    if x.is_alphanumeric() || *x == '_' {
                        s.push(*x);
                        self.position += 1;
                    } else {
                        break
                    }
                } else {
                    break;
                };
            }
            return Some(Ok(Token::Ident(s)));
        };

        /* In case there is a string */
        if *c == '"' {
            let mut s = String::new();
            loop {
                let n = self.source_stream.get(self.position);
                if let Some(x) = n {
                    if *x != '\"' {
                        s.push(*x);
                        self.position += 1;
                    } else {
                        self.position += 1;
                        break
                    }
                } else {
                    return Some(Err(LexerError::MissingStringQuote));
                };
            }
            return Some(Ok(Token::Literal(s)));
        };

        /* In case there is a positive number */
        if c.is_numeric() {
            let mut s = String::new();
            s.push(*c);
            loop {
                let n = self.source_stream.get(self.position);
                if let Some(x) = n {
                    if x.is_numeric() {
                        s.push(*x);
                        self.position += 1;
                    } else {
                        break
                    }
                } else {
                    break
                };
            }
            let n = match s.parse::<u64>() {
                Ok(v) => { v },
                Err(e) => return Some(Err(LexerError::NumberParseError(e)))
            };
            return Some(Ok(Token::Number(n)));
        };

        /* In case a comment starts */
        if *c == '/' {
            if let Some('/') = self.source_stream.get(self.position + 1) {
                self.position += 1;
                return Some(Ok(Token::CommentStart))
            }
        }

        /* In case of other chars */
        Some(Ok(match *c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '=' => Token::Eq,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '^' => Token::Caret,
            '`' => Token::GraveAccent,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '|' => Token::VerticalBar,
            '~' => Token::Tilde,
            '(' => Token::OpenParentesis,
            ')' => Token::CloseParenthesis,
            '@' => Token::At,
            '\n' => Token::EndOfLine,
            _ =>   Token::Unknown
        }))
    }
}

#[test]
fn test_lexer_basic() {
    let source = r#"
test=123
    "#;

    let lexer = Lexer::new(source.chars().collect());
    fn flat<X>(v: Vec<Result<Token, X>>) -> Result<Vec<Token>, X> {
        let mut v1 = Vec::with_capacity(v.len());
        for i in v {
            v1.push(match i { Ok(x) => x, Err(e) => return Err(e) });
        }

        Ok(v1)
    }

    let r = flat(lexer.collect());
    assert!(r.is_ok());
    
    let r = r.ok().unwrap();
    assert_eq!(Token::Ident(String::from("test")), r[0]);
    assert_eq!(Token::Eq, r[1]);
    assert_eq!(Token::Number(123), r[2]);
}

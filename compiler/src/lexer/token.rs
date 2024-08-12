#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /* basic */
    Ident(String),
    Literal(String),
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Period,
    Colon,
    Semicolon,
    Lt,
    Gt,
    Eq,
    OpenBracket,
    CloseBracket,
    Caret,
    GraveAccent,
    OpenBrace,
    CloseBrace,
    VerticalBar,
    Tilde,
    OpenParentesis,
    CloseParenthesis,
    At,
    
    /* special */
    CommentStart,
    EndOfLine,

    Unknown
}

use super::source::SourcePosition;

pub struct TokenData {
    pub token: Token,
    pub start: SourcePosition,
    pub end: SourcePosition,
}

pub enum Token {
    Comment(Comment),
    EndOfInput,
    Identifier(Identifier),
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    Punctuator(Punctuator),
    Whitespace(Whitespace),
}

pub struct Comment {
    pub text: String,
}

pub struct Identifier {
    pub name: String,
}

pub enum Keyword {
    Else,
    Export,
    For,
    From,
    If,
    In,
    Import,
    Let,
    Mut,
}

pub enum Literal {
    Boolean,
    Char,
    Integer,
    Float,
    String,
}

pub enum Operator {
    And,
    AndAssign,
    Arrow,
    Assign,
    Divide,
    DivideAssign,
    Dot,
    Equality,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    LShift,
    LShiftAssign,
    Minus,
    MinusAssign,
    Modulo,
    ModuloAssign,
    Not,
    NotEquals,
    Or,
    OrAssign,
    Plus,
    PlusAssign,
    Power,
    PowerAssign,
    Ternary,
    RShift,
    RShiftAssign,
    Spread,
    Times,
    TimesAssign,
    Xor,
    XorAssign,
}

pub enum Punctuator {
    Colon,
    Comma,
    LBrace,
    LBracket,
    LParens,
    RBrace,
    RBracket,
    RParens,
}

pub struct Whitespace {
    pub text: String,
}
pub enum Token {
    /* Reversed words */
    While,
    For,
    To,
    Break,
    Let,
    In,
    End,
    Function,
    Var,
    Type,
    Array,
    If,
    Then,
    Else,
    Do,
    Of,
    Nil,

    /* punctuation symbols */
    Comma,
    Colon,
    Semicolon,
    // ()
    Lparen,
    Rparen,
    // []
    Lbracket,
    Rbracket,
    // {}
    Lbrace,
    Rbrace,
    Period,
    Plus,
    Minus,
    Times,
    Div,
    Equal,
    Neq,
    Less,
    Leq,
    Greater,
    Geq,
    And, // &
    Or, // |
    Def, // :=

    // literals
    Id(String),
    Comment,
    Num(u64),
    Str(String),
    Eof,
}

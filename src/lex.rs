use logos::{Lexer, Logos};
use std::fmt::Write;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"\s+", logos::skip)]
    #[error]
    Error,

    #[regex("[a-zA-Z][a-zA-Z0-9_]*")]
    IdentOrKeyword,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    LiteralString,

    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,

    #[token(":")]
    TypeQuery,
    #[token("!")]
    Bang,

    #[token("{")]
    BlockOpen,
    #[token("}")]
    BlockClose,
    #[token(";")]
    BlockDelimiter,
}

pub fn get_debug_string_from_tokens(mut lexer: Lexer<Token>) -> String {
    let mut string = String::new();
    loop {
        match lexer.next() {
            Some(token) => writeln!(
                string,
                "{:15}  @  {:2?}  =   {}",
                format!("{:?}", token),
                lexer.span(),
                lexer.slice()
            )
            .unwrap(),
            None => break string,
        }
    }
}

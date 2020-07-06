use lalrpop_util::lexer::Token;

pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, &'a str>;

pub struct File(pub i32);

use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            r"
                func main() -> U32 {{
                    return {};
                }}
            ",
            self.0
        )
    }
}

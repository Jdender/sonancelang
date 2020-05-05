use logos::Logos;
use sonancelang::lex::*;

static PROGRAM: &str = r#"
    func main(): void {
        print!("Hello world!");
    }
"#;

fn main() {
    let mut lex = Token::lexer(PROGRAM);
    println!("{}", get_debug_string_from_tokens(lex));
}

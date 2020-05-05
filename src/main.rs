use logos::Logos;
use sonancelang::Token;

static PROGRAM: &str = r#"
    func main(): void {
        print!("Hello world!");
    }
"#;

fn main() {
    let mut lex = Token::lexer(PROGRAM);
    println!("{}", sonancelang::get_debug_string_from_tokens(lex));
}

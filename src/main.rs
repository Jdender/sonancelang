macro_rules! generate_parser_repl {
    ( $kind:ident, $( $name:ident ),* ) => {
        loop {
            $(paste::expr! {
                if $kind == stringify!([<$name:snake>]) {
                    match &read_line()[..] {
                        "/exit" => return,
                        "/new" => $kind = read_line(),
                        x => {
                            let _ = dbg!(
                                sonancelang::parser::[<$name Parser>]::new().parse(x)
                            );
                        }
                    }
                    break;
                }
            })*
            eprintln!("Kind not found.");
            $kind = read_line();
            break;
        }
    };
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut kind = read_line();

    loop {
        generate_parser_repl!(
            kind,
            Literal,
            Identifier,
            Module,
            Item,
            FunctionItem,
            Arguments,
            Type,
            Pattern,
            Block,
            Statement,
            Expression
        );
    }
}

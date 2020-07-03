macro_rules! generate_parser_repl {
    ( $kind:ident, $( $name:ident ),* ) => {
        loop {
            $(paste::expr! {
                if $kind == stringify!([<$name:snake>]) {

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    let result = sonancelang::parser::[<$name Parser>]::new().parse(input.trim());

                    let _ = dbg!(result);
                    break;
                }
                })*
            eprintln!("Kind not found.");
            break;
        }
    };
}

fn main() {
    let mut previous_kind = String::new();
    loop {
        let mut kind = String::new();
        std::io::stdin().read_line(&mut kind).unwrap();
        let mut kind = kind.trim();

        match kind {
            "exit" => break,
            "" => kind = &previous_kind,
            _ => previous_kind = kind.to_string(),
        }

        generate_parser_repl!(
            kind,
            Literal,
            Identifier,
            Module,
            Item,
            Path,
            StarterPath,
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

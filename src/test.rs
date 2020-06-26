use crate::parse::*;
use crate::parser::*;

fn ident(input: &str) -> Identifier {
    Identifier(input.to_string())
}

#[test]
fn identifier_test() {
    let parser = IdentifierParser::new();

    assert_eq!(parser.parse("foobar").unwrap(), ident("foobar"));
    assert_eq!(parser.parse("asa_adw").unwrap(), ident("asa_adw"));
    assert_eq!(parser.parse("a1234").unwrap(), ident("a1234"));

    assert!(parser.parse("123asa").is_err());
    assert!(parser.parse("1").is_err());
    assert!(parser.parse("[]+=-").is_err());
}

#[test]
fn function_test() {
    let parser = FunctionItemParser::new();

    assert_eq!(
        parser.parse("func foobar() {}").unwrap(),
        FunctionItem {
            name: ident("foobar"),
            arguments: vec![],
            return_type: None,
            body: Block {
                body: vec![],
                trailing: None
            },
        }
    );

    assert_eq!(
        parser.parse("func foobar() -> Type {}").unwrap(),
        FunctionItem {
            name: ident("foobar"),
            arguments: vec![],
            return_type: Some(Type {
                name: ident("Type"),
                arguments: vec![]
            }),
            body: Block {
                body: vec![],
                trailing: None
            },
        }
    );
}

#[test]
fn arguments_test() {
    let parser = ArgumentsParser::new();

    assert_eq!(
        parser.parse("(a: A, b: B)").unwrap(),
        vec![
            Argument {
                pattern: Pattern::Identifier(ident("a")),
                declared_type: Type {
                    name: ident("A"),
                    arguments: vec![]
                },
            },
            Argument {
                pattern: Pattern::Identifier(ident("b")),
                declared_type: Type {
                    name: ident("B"),
                    arguments: vec![]
                },
            }
        ]
    );
}

#[test]
fn type_test() {
    let parser = TypeParser::new();

    assert_eq!(
        parser.parse("Foo").unwrap(),
        Type {
            name: ident("Foo"),
            arguments: vec![]
        }
    );

    assert_eq!(
        parser.parse("Foo[Bar[Baz], Buz[]]").unwrap(),
        Type {
            name: ident("Foo"),
            arguments: vec![
                Type {
                    name: ident("Bar"),
                    arguments: vec![Type {
                        name: ident("Baz"),
                        arguments: vec![]
                    }]
                },
                Type {
                    name: ident("Buz"),
                    arguments: vec![]
                }
            ]
        }
    );
}

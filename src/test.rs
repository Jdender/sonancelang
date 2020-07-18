fn compile_and_run(_: &str) -> i32 {
    unimplemented!()
}

fn process_int_helper(cases: Vec<(i32, &str)>) {
    for (num, input) in cases {
        assert_eq!(
            num,
            compile_and_run(&format!(
                "func main() -> I32 {{
                    return {};
                }}",
                input
            ))
        );
    }
}

fn process_statements_helper(num: i32, input: &str) {
    assert_eq!(
        num,
        compile_and_run(&format!(
            "func main() -> I32 {{
                    {}
                }}",
            input
        ))
    );
}

#[test]
fn process_numeric_expressions() {
    process_int_helper(vec![
        (12345, "12345"),
        (-308, "20 + 56 - 32 * 72 / 6"),
        (151201, "(231 * (1321 - 12) + 23) / 2"),
    ]);
}

#[test]
fn process_boolean_expressions() {
    process_int_helper(vec![
        (1, "123 || 0"),
        (0, "!0 && !1"),
        (456, "123 && 456"),
        (1, "123 || (return 456)"),
        (456, "0 || (return 456)"),
        (0, "0 && 456"),
        (456, "1 && (return 456)"),
    ])
}

#[test]
fn process_variable_statements() {
    process_statements_helper(123, "let foo = 123; let bar = foo; return bar;");
    process_statements_helper(124, "let foo = 123; foo = foo + 1; return foo;");
}

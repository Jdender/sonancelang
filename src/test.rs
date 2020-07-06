fn compile(_input: &str) {
    todo!();
}

#[test]
fn return_int_test() {
    compile(
        r"
        func main() -> I32 {
            return 1234;
        }
    ",
    );
}

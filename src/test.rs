fn compile(_input: &str) {
    todo!();
}

#[test]
fn return_int_test() {
    compile(r"
        func main() -> U32 {
            return 1234;
        }
    ");
}

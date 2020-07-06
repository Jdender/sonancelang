use crate::compile;

#[test]
fn return_int_test() {
    let _ = compile(
        r"
        func main() -> I32 {
            return 1234;
        }
    ",
    )
    .unwrap();

    todo!("Figure out how to test this properly");
}

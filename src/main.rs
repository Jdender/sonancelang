#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub test);

fn main() {
    let program = include_str!("../test/hello_world.so");

    dbg!(test::ModuleParser::new().parse(program)).unwrap();
}

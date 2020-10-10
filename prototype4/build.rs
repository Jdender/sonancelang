fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("src")
        .always_use_colors()
        .process()
        .unwrap();
}

macro_rules! id {
    ($($vis: vis $ident: ident => $string: expr),* $(,)?) => {$(

        #[derive(Clone, PartialEq)]
        $vis struct $ident($vis usize);

        impl std::fmt::Debug for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}{}", $string, self.0)
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}{}", $string, self.0)
            }
        }
    )*};
}

id! {
    pub BlockId => "block",
    pub VariableId => "var",
    pub FunctionId => "func",
}

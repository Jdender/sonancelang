#[derive(Debug, Clone)]
pub struct Module<'a>(pub Vec<Item<'a>>);
// https://michael-f-bryan.github.io/calc/book/html/parse/visit.html
#[derive(Debug, Clone)]
pub enum Item<'a> {
    Function(FunctionItem<'a>),
}

#[derive(Debug, Clone)]
pub struct FunctionItem<'a> {
    pub name: &'a str,
    pub return_type: &'a str,
    pub body: Block<'a>,
}

#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub body: Vec<Statement<'a>>,
    pub trailing: Option<Exper<'a>>,
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Exper(Exper<'a>),
}

#[derive(Debug, Clone)]
pub enum Exper<'a> {
    Parens(Box<Exper<'a>>),
    Block(Box<Block<'a>>),
    FunctionCall { name: &'a str, args: Vec<Exper<'a>> },
    StringLiteral(&'a str),
}

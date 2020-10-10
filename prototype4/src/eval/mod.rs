mod value;

use {
    crate::{ast, Result},
    value::{UnwrapValue, Value},
};

pub fn eval(input: ast::File) -> Result<()> {
    input.eval()?;
    Ok(())
}

impl ast::File {
    fn eval(self) -> Result<()> {
        for expr in self.body {
            expr.eval()?;
        }
        Ok(())
    }
}

impl ast::Literal {
    fn eval(self) -> Value {
        match self {
            Self::Number(num) => Value::Number(num),
            Self::String(string) => Value::String(string),
        }
    }
}

impl ast::Expression {
    fn eval(self) -> Result<Value> {
        Ok(match self {
            Self::Literal(literal) => literal.eval(),
            Self::Call(name, args) => {
                let mut args = args
                    .into_iter()
                    .map(|arg| arg.eval())
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .peekable();

                match name.as_str() {
                    "print" => {
                        while let Some(arg) = args.next() {
                            if args.peek().is_some() {
                                print!("{} ", arg);
                            } else {
                                println!("{}", arg);
                            }
                        }

                        Value::Nil
                    }
                    "add" => {
                        let x = args
                            .next()
                            .unwrap_number()
                            .ok_or("add() takes two numbers")?;

                        let y = args
                            .next()
                            .unwrap_number()
                            .ok_or("add() takes two numbers")?;

                        Value::Number(x + y)
                    }
                    "type_of" => {
                        let arg = args.next().ok_or("type_of() takes a argument")?;
                        Value::String(arg.type_of().to_owned())
                    }
                    "to_string" => {
                        let arg = args.next().ok_or("to_string() takes a argument")?;
                        Value::String(arg.to_string())
                    }
                    _ => unimplemented!("function doesn't exist"),
                }
            }
        })
    }
}

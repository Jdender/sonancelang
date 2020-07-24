use super::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

const INDENT: &str = "    ";

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Name and id
        write!(f, "{} {}(", self.id, self.name)?;

        // Arguments
        for (i, arg) in self.arguments.iter().enumerate() {
            write!(f, "{}", arg)?;
            if i != self.arguments.len().saturating_sub(1) {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")?;

        // Return type
        if let Some(ref result) = self.result {
            write!(f, " -> {}", result)?;
        }

        writeln!(f, " {{")?;

        // Body
        for block in self.body.iter() {
            write!(f, "{}{}", INDENT, block)?;
        }

        write!(f, "}}")
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::I32 => "I32",
            }
        )
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Id
        write!(f, "{}", self.id)?;

        if !self.arguments.is_empty() {
            write!(f, "(")?;
        }

        // Arguments
        for (i, arg) in self.arguments.iter().enumerate() {
            write!(f, "{}: {}", arg.0, arg.1)?;
            if i != self.arguments.len().saturating_sub(1) {
                write!(f, ", ")?;
            }
        }

        if !self.arguments.is_empty() {
            write!(f, ")")?;
        }

        writeln!(f, ":")?;

        // Instructions
        for inst in self.instructions.iter() {
            writeln!(f, "{}{}{}", INDENT, INDENT, inst)?;
        }

        // Terminator
        writeln!(f, "{}{}=> {}", INDENT, INDENT, self.terminator)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Dead(var) => write!(f, "-> dead {}", var),
            Self::UnaryOperation {
                result,
                operator,
                operand,
            } => write!(f, "{} = {}{}", result, operator, operand),
            Self::BinaryOperation {
                result,
                x_operand,
                operator,
                y_operand,
            } => write!(f, "{} = {} {} {}", result, x_operand, operator, y_operand),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Negate => "-",
            }
        )
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Subtract => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
            }
        )
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Jump { block, arguments } => {
                write!(f, "jump {}", block)?;

                if !arguments.is_empty() {
                    write!(f, "(")?;
                }

                // Arguments
                for (i, arg) in arguments.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i != arguments.len().saturating_sub(1) {
                        write!(f, ",")?;
                    }
                }

                if !arguments.is_empty() {
                    write!(f, ")")?;
                }
            }
            Self::Return { argument } => {
                write!(f, "return")?;
                if let Some(arg) = argument {
                    write!(f, " {}", arg)?;
                }
            }
        }
        Ok(())
    }
}

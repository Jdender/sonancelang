use super::{
    super::semantic,
    symbol_table::{SymbolInfo, SymbolTable},
    SemResult, SemanticError,
};
use crate::ast;

pub fn semantic_pass(input: crate::ast::File) -> SemResult<semantic::File> {
    input.visit_ast(&AstContext {
        symbol_table: &SymbolTable::new(),
        arg: (),
    })
}

#[derive(Debug, Clone)]
pub struct AstContext<'a, T> {
    symbol_table: &'a SymbolTable<'a>,
    arg: T,
}

pub trait AstVisitor {
    type Argument;
    type Return;

    fn visit_ast(&self, ctx: &AstContext<Self::Argument>) -> Self::Return;
}

impl AstVisitor for ast::File {
    type Argument = ();
    type Return = SemResult<semantic::File>;

    fn visit_ast(&self, ctx: &AstContext<Self::Argument>) -> Self::Return {
        Ok(semantic::File {
            name: self.name.visit_ast(ctx),
            body: self.body.visit_ast(ctx)?,
        })
    }
}

impl AstVisitor for ast::Identifier {
    type Argument = ();
    type Return = semantic::Identifier;

    fn visit_ast(&self, _ctx: &AstContext<Self::Argument>) -> Self::Return {
        semantic::Identifier(self.0.clone())
    }
}

impl AstVisitor for ast::Block {
    type Argument = ();
    type Return = SemResult<semantic::Block>;

    fn visit_ast(&self, ctx: &AstContext<Self::Argument>) -> Self::Return {
        let mut symbol_table = ctx.symbol_table.fork();
        let mut statements = Vec::new();

        for stmt in self.0.iter() {
            statements.push(match stmt {
                ast::Statement::LetBinding(ident, expr) => {
                    symbol_table.set(ident.0.clone(), SymbolInfo);

                    let ctx = &AstContext {
                        arg: (),
                        symbol_table: &symbol_table,
                    };

                    semantic::Statement::LetBinding(ident.visit_ast(ctx), expr.visit_ast(ctx)?)
                }
                ast::Statement::Expression(expr) => {
                    let ctx = &AstContext {
                        arg: (),
                        symbol_table: &symbol_table,
                    };

                    semantic::Statement::Expression(expr.visit_ast(ctx)?)
                }
            });
        }

        dbg!(symbol_table);

        Ok(semantic::Block(statements))
    }
}

impl AstVisitor for ast::Expression {
    type Argument = ();
    type Return = SemResult<semantic::Expression>;

    fn visit_ast(&self, ctx: &AstContext<Self::Argument>) -> Self::Return {
        Ok(match self {
            Self::Literal(num) => semantic::Expression::Literal(*num),
            Self::Lookup(ident) => {
                ctx.symbol_table
                    .get(&ident.0)
                    .ok_or_else(|| SemanticError::VariableNotDeclared(ident.0.clone()))?;
                semantic::Expression::Lookup(ident.visit_ast(ctx))
            }
            Self::Assignment(ident, expr) => semantic::Expression::Assignment(
                ident.visit_ast(ctx),
                Box::new(expr.visit_ast(ctx)?),
            ),
            Self::ReturnValue(expr) => {
                semantic::Expression::ReturnValue(Box::new(expr.visit_ast(ctx)?))
            }
            Self::PrefixCall(op, expr) => {
                semantic::Expression::PrefixCall(op.visit_ast(ctx), Box::new(expr.visit_ast(ctx)?))
            }
            Self::InfixCall(x, op, y) => semantic::Expression::InfixCall(
                Box::new(x.visit_ast(ctx)?),
                op.visit_ast(ctx),
                Box::new(y.visit_ast(ctx)?),
            ),
        })
    }
}

impl AstVisitor for ast::PrefixOp {
    type Argument = ();
    type Return = semantic::PrefixOp;

    fn visit_ast(&self, _ctx: &AstContext<Self::Argument>) -> Self::Return {
        match self {
            Self::Negate => semantic::PrefixOp::Negate,
            Self::BooleanNot => semantic::PrefixOp::BooleanNot,
        }
    }
}

impl AstVisitor for ast::InfixOp {
    type Argument = ();
    type Return = semantic::InfixOp;

    fn visit_ast(&self, _ctx: &AstContext<Self::Argument>) -> Self::Return {
        match self {
            Self::Add => semantic::InfixOp::Add,
            Self::Subtract => semantic::InfixOp::Subtract,
            Self::Multiply => semantic::InfixOp::Multiply,
            Self::Divide => semantic::InfixOp::Divide,

            Self::Equal => semantic::InfixOp::Equal,
            Self::NotEqual => semantic::InfixOp::NotEqual,
            Self::GreaterThan => semantic::InfixOp::GreaterThan,
            Self::LessThan => semantic::InfixOp::LessThan,
            Self::GreaterOrEqual => semantic::InfixOp::GreaterOrEqual,
            Self::LessOrEqual => semantic::InfixOp::LessOrEqual,

            Self::BooleanOr => semantic::InfixOp::BooleanOr,
            Self::BooleanAnd => semantic::InfixOp::BooleanAnd,
        }
    }
}

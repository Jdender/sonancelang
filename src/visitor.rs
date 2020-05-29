use super::parser::*;

pub trait AstNode {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> V::Result;
}

pub trait AstVisitor {
    type Result;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result;
    fn visit_opcode(&mut self, opcode: &Opcode) -> Self::Result;
    fn visit_number_literal(&mut self, number: &NumberLiteral) -> Self::Result;
}

impl AstNode for Expr {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_expr(self)
    }
}

impl AstNode for Opcode {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_opcode(self)
    }
}

impl AstNode for NumberLiteral {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_number_literal(self)
    }
}

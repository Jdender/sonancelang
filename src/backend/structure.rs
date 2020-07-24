use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub id: FunctionId,
    pub name: String,
    pub arguments: Vec<Type>,
    pub result: Option<Type>,
    pub body: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub enum Type {
    I32,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub arguments: Vec<(VariableId, Type)>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Dead(VariableId),
    UnaryOperation {
        result: VariableId,
        operator: UnaryOperator,
        operand: VariableId,
    },
    BinaryOperation {
        result: VariableId,
        operator: BinaryOperator,
        x_operand: VariableId,
        y_operand: VariableId,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Jump {
        block: BlockId,
        arguments: Vec<VariableId>,
    },
    Return {
        argument: Option<VariableId>,
    },
}

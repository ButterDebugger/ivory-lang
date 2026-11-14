use crate::analyzer::VarId;
use ivory_parser::{FunctionParam, types::DataType};

#[derive(Clone, Debug)]
pub struct Tir {
    /// Indexed by [`VarId`]
    pub(crate) variables: Vec<Variable>,
    /// Indexed by [`FuncId`]
    pub(crate) functions: Vec<Function>,
}

#[derive(Clone, Debug)]
pub enum TirStmt {}

#[derive(Clone, Debug)]
pub enum TirExpr {}

// Variables

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
    pub mutable: bool,
    pub used: bool,
    pub initialized: bool,
}

// Functions

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Option<DataType>,
    pub param_vars: Vec<VarId>,
    /// Indexed by [`BlockId`]
    pub blocks: Vec<BasicBlock>,
}

/// [https://en.wikipedia.org/wiki/Basic_block]
#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub label: String,
    pub stmts: Vec<TirStmt>,
    pub terminator: Terminator,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockId(pub usize);

#[derive(Clone, Debug)]
pub enum Terminator {
    /// Return from the function
    Return(Option<TirExpr>),
    /// Goto another block
    Jump(BlockId),
    /// Goto another block based on a condition
    Branch {
        condition: TirExpr,
        then_block: BlockId,
        else_block: BlockId,
    },
}

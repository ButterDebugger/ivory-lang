use crate::{errors::{SematicError, SematicWarning}, scope::{Function, Scope, Variable}};
use ivory_parser::{FunctionDeclaration, Program};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VarId(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FuncId(pub usize);

#[derive(Debug)]
pub(crate) struct Analyzer {
    pub(crate) scopes: Vec<Scope>,

    /// Indexed by [`VarId`]
    pub(crate) variables: Vec<Variable>,
    /// Indexed by [`FuncId`]
    pub(crate) functions: Vec<Function>,

    pub(crate) errors: Vec<SematicError>,
    pub(crate) warnings: Vec<SematicWarning>,
}

impl Analyzer {
    pub fn analyze(ast: Program) {
        let mut analyzer = Analyzer {
            scopes: vec![Scope::default()],

            variables: Vec::new(),
            functions: Vec::new(),

            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Register all function signatures
        for FunctionDeclaration {
            name,
            params,
            return_type,
            ..
        } in ast.functions.clone()
        {
            analyzer.define_function(
                name,
                Function {
                    return_type,
                    params,
                },
            );
        }

        // Analyze each function
        todo!();
    }
}

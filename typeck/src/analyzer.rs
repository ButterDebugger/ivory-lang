use crate::{
    errors::{SematicError, SematicWarning},
    scope::{FunctionSignature, Scope},
    tir::{Function, Tir, Variable},
};
use ivory_parser::{Block, FunctionDeclaration, Program};

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
    pub(crate) functions: Vec<FunctionSignature>,

    pub(crate) errors: Vec<SematicError>,
    pub(crate) warnings: Vec<SematicWarning>,
}

impl Analyzer {
    pub fn analyze(ast: Program) -> Tir {
        let mut analyzer = Analyzer {
            scopes: vec![Scope::default()],

            variables: Vec::new(),
            functions: Vec::new(),

            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Register all function signatures
        let registered_funcs: Vec<(FuncId, Box<Block>)> = ast
            .functions
            .iter()
            .cloned()
            .map(|decl| {
                (
                    analyzer.define_function(
                        decl.name.clone(),
                        FunctionSignature {
                            name: decl.name,
                            params: decl.params,
                            return_type: decl.return_type,
                        },
                    ),
                    decl.block,
                )
            })
            .collect();

        // Analyze each function
        let functions: Vec<Function> = registered_funcs
            .iter()
            .map(|(id, body)| {
                let signature = analyzer.functions[id.0].clone();

                analyzer.analyze_function(*id, &signature, body.as_ref().clone())
            })
            .collect();

        // Return the resulting Tir
        Tir {
            variables: analyzer.variables,
            functions,
        }
    }
}

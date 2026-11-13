use ivory_parser::{FunctionParam, types::DataType};
use std::collections::HashMap;

use crate::analyzer::{Analyzer, FuncId, VarId};

#[derive(Clone, Debug, Default)]
pub struct Scope {
    variables: HashMap<String, VarId>,
    functions: HashMap<String, FuncId>,
}

// Traversing scope

impl Analyzer {
    pub(crate) fn enter_new_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub(crate) fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}

// Variables

#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) data_type: DataType,
    pub(crate) mutable: bool,
    pub(crate) used: bool,
    pub(crate) initialized: bool,
}

impl Analyzer {
    pub(crate) fn define_variable(&mut self, name: String, variable: Variable) -> VarId {
        // Add variable to global symbols list
        let id = VarId(self.variables.len());

        self.variables.push(variable);

        // Save the variable in the local scope
        self.scopes.last_mut().unwrap().variables.insert(name, id);

        id
    }

    pub(crate) fn get_variable(&self, name: &String) -> Option<VarId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.variables.get(name).copied())
    }
}

// Functions

#[derive(Clone, Debug)]
pub struct Function {
    pub(crate) return_type: Option<DataType>,
    pub(crate) params: Vec<FunctionParam>,
}

impl Analyzer {
    pub(crate) fn define_function(&mut self, name: String, function: Function) -> FuncId {
        // Add function to global symbols list
        let id = FuncId(self.functions.len());

        self.functions.push(function);

        // Save the function in the local scope
        self.scopes.last_mut().unwrap().functions.insert(name, id);

        id
    }

    pub(crate) fn get_function(&self, name: &String) -> Option<FuncId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.functions.get(name).copied())
    }
}

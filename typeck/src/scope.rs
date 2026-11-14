use crate::{
    analyzer::{Analyzer, FuncId, VarId},
    tir::{Function, Variable},
};
use ivory_parser::{FunctionParam, types::DataType};
use std::collections::HashMap;

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

// Variable methods

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

// Function methods

#[derive(Clone, Debug)]
pub(crate) struct FunctionSignature {
    pub(crate) name: String,
    pub(crate) params: Vec<FunctionParam>,
    pub(crate) return_type: Option<DataType>,
}

impl Analyzer {
    pub(crate) fn define_function(&mut self, name: String, signature: FunctionSignature) -> FuncId {
        // Add function to global symbols list
        let id = FuncId(self.functions.len());

        self.functions.push(signature);

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

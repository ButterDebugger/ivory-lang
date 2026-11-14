use crate::{
    analyzer::{Analyzer, FuncId, VarId},
    scope::FunctionSignature,
    tir::{Function, Variable},
};
use ivory_parser::{Block, types::DataType};

mod block;

#[derive(Debug)]
struct FunctionContext {
    id: FuncId,
    return_type: Option<DataType>,
    // loop_depth: u32,
}

impl Analyzer {
    pub(crate) fn analyze_function(
        &mut self,
        id: FuncId,
        signature: &FunctionSignature,
        body: Block,
    ) -> Function {
        self.enter_new_scope();

        // Define parameters as variables
        let param_vars: Vec<VarId> = signature
            .params
            .iter()
            .map(|param| {
                self.define_variable(
                    param.name.clone(),
                    Variable {
                        name: param.name.clone(),
                        data_type: param.data_type.clone(),
                        mutable: param.mutable,
                        used: false,
                        initialized: param.default_value.is_some(),
                    },
                )
            })
            .collect();

        // Analyze the function body
        let mut ctx = FunctionContext {
            id,
            return_type: signature.return_type.clone(),
        };

        let blocks = self.analyze_block(&mut ctx, body);

        // TODO: add missing return check

        self.exit_scope();

        // Return the resulting Function
        Function {
            name: signature.name.clone(),
            return_type: signature.return_type.clone(),
            param_vars,
            blocks,
        }
    }
}

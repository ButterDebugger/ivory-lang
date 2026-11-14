use crate::{analyzer::Analyzer, function::FunctionContext, tir::BasicBlock};
use ivory_parser::Block;

impl Analyzer {
    pub(crate) fn analyze_block(
        &mut self,
        ctx: &mut FunctionContext,
        block: Block,
    ) -> Vec<BasicBlock> {
        self.enter_new_scope();

        // TODO: analyze the block

        self.exit_scope();

        todo!()
    }
}

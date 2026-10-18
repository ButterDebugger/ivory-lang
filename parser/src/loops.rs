use crate::{LoopStmt, WhileStmt, block::parse_block, expressions::parse_expression};
use anyhow::{Result, anyhow};
use ivory_lexer::{TokenStream, tokens::Token};

pub fn parse_loop(tokens: &mut TokenStream) -> Result<LoopStmt> {
    // Consume the Loop token
    match tokens.next() {
        Some((Token::Loop, _)) => {}
        _ => return Err(anyhow!("Expected if statement")),
    }

    // Return a loop statement with the parsed block
    Ok(LoopStmt {
        block: Box::new(parse_block(tokens)?),
    })
}

pub fn parse_while_loop(tokens: &mut TokenStream) -> Result<WhileStmt> {
    // Consume the While token
    match tokens.next() {
        Some((Token::While, _)) => {}
        _ => return Err(anyhow!("Expected if statement")),
    }

    // Parse the condition expression
    let condition = parse_expression(tokens)?;

    // Parse the loop body block
    let loop_body = parse_block(tokens)?;

    Ok(WhileStmt {
        condition: Box::new(condition),
        block: Box::new(loop_body),
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        Block, BooleanLit, Expression, FunctionCall, FunctionDeclaration, LoopStmt, Program, Stmt,
        StringLit, WhileStmt, parse,
    };
    use ivory_lexer::lex;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &'static str) -> Program {
        let tokens = lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    fn a_loop() {
        let source = include_str!("../tests/loops/a_loop.iv");
        let program = lex_then_parse(source);

        assert_eq!(
            program,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    annotations: vec![],
                    block: Box::new(Block {
                        body: vec![Stmt::Loop(LoopStmt {
                            block: Box::new(Block {
                                body: vec![Stmt::FunctionCall(FunctionCall {
                                    name: "println".to_string(),
                                    args: vec![Expression::String(StringLit {
                                        value: "Hello, world!".to_string(),
                                        span: 41..56
                                    })],
                                    span: 33..57
                                })],
                                allow_implicit_return: false
                            }),
                        })],
                        allow_implicit_return: true
                    })
                }]
            }
        );
    }

    #[test]
    fn break_and_continue_statements() {
        let source = include_str!("../tests/loops/break_and_continue_statements.iv");
        let program = lex_then_parse(source);

        assert_eq!(
            program,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    annotations: vec![],
                    block: Box::new(Block {
                        body: vec![Stmt::Loop(LoopStmt {
                            block: Box::new(Block {
                                body: vec![Stmt::Break, Stmt::Continue],
                                allow_implicit_return: true,
                            }),
                        })],
                        allow_implicit_return: true
                    })
                }]
            }
        );
    }

    #[test]
    fn while_loop() {
        let source = include_str!("../tests/loops/while_loop.iv");
        let program = lex_then_parse(source);

        assert_eq!(
            program,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    annotations: vec![],
                    block: Box::new(Block {
                        body: vec![Stmt::While(WhileStmt {
                            condition: Box::new(Expression::Boolean(BooleanLit {
                                value: true,
                                span: 25..29
                            })),
                            block: Box::new(Block {
                                body: vec![Stmt::FunctionCall(FunctionCall {
                                    name: "println".to_string(),
                                    args: vec![Expression::String(StringLit {
                                        value: "Hello, world!".to_string(),
                                        span: 49..64
                                    })],
                                    span: 41..65
                                })],
                                allow_implicit_return: false
                            }),
                        })],
                        allow_implicit_return: true
                    })
                }]
            }
        );
    }
}

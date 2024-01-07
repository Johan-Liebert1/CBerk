use std::rc::Rc;

use crate::{
    ast::{
        abstract_syntax_tree::AST, declaration_statement::DeclarationStatement,
        variable::VariableAST,
    },
    interpreter::interpreter::{VarScope, VariableHashMapValue},
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// VARIABLE_DECLARATION -> def VAR_NAME: VAR_TYPE (= COMPARISON_EXPRESSION)*
    pub fn parse_variable(&mut self) -> VariableAST {
        let token = self.get_next_token();

        match token.token {
            TokenEnum::Variable(var_name) => {
                let token = self.get_next_token();

                match token.token {
                    // : after variable name, so can only be VAR_NAME: VAR_TYPE
                    TokenEnum::Colon => {
                        let token = self.peek_next_token();

                        match &token.token {
                            TokenEnum::Type(var_type) => {
                                let token = self.get_next_token();

                                return VariableAST::new(
                                    Box::new(token),
                                    var_type.to_string(),
                                    var_name,
                                );
                            }

                            _ => panic!("Expected type found {:?}", token),
                        }
                    }

                    _ => panic!("Expected : found {:?}", token),
                }
            }

            _ => panic!("Expected a variable found {:?}", token),
        }
    }

    pub fn parse_declaration_statement(&mut self) -> Rc<Box<dyn AST>> {
        // we get here after consuming 'def'
        let left = self.parse_variable();

        match self.function_name {
            Some(_) => {
                self.function_variables.borrow_mut().insert(
                    left.var_name.clone(),
                    VariableHashMapValue {
                        var: left.get_var_enum_from_type(),
                        scope: VarScope::Global,
                        index: 0,
                    },
                );
            }

            None => {
                self.variables.insert(
                    left.var_name.clone(),
                    VariableHashMapValue {
                        var: left.get_var_enum_from_type(),
                        scope: VarScope::Global,
                        index: 0,
                    },
                );
            }
        }

        match self.get_next_token().token {
            TokenEnum::Equals => {
                // fine just consume the token
            }

            _ => {
                panic!("Expected assignment")
            }
        };

        // TODO: handle function calls and strings and stuff here
        return Rc::new(Box::new(DeclarationStatement::new(
            left,
            self.parse_logical_expression(),
        )));
    }
}

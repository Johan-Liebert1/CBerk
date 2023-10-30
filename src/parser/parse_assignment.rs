use crate::{
    ast::{
        abstract_syntax_tree::AST, assignment_statement::AssignmentStatement, variable::Variable,
    },
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// VARIABLE_DECLARATION -> def VAR_NAME: VAR_TYPE (= COMPARISON_EXPRESSION)*
    pub fn parse_variable(&mut self) -> Variable {
        let token = self.get_next_token();

        match token.token {
            TokenEnum::Variable(var_name) => {
                let token = self.get_next_token();

                match token.token {
                    TokenEnum::Colon => {
                        let token = self.peek_next_token();

                        match &token.token {
                            TokenEnum::Type(var_type) => {
                                let token = self.get_next_token();

                                return Variable::new(
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

    pub fn parse_assignment_statement(&mut self) -> Box<dyn AST> {
        // we get here after consuming 'def'

        let left = self.parse_variable();

        match self.get_next_token().token {
            TokenEnum::Equals => {
                // fine just consume the token
            }

            _ => {
                panic!("Expected assignment")
            }
        };

        // TODO: handle function calls and strings and stuff here
        return Box::new(AssignmentStatement::new(
            left,
            self.parse_comparison_expression(),
        ));
    }
}

use crate::{
    helpers::unexpected_token, interpreter::interpreter::FunctionHashMapValue, lexer::types::VarType, trace,
    types::ASTNode,
};

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    ast::{function_def::FunctionDefinition, variable::Variable},
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::{Parser, ParserFunctions};

impl Parser {
    fn parse_function_definition_parameters(&mut self) -> Vec<Variable> {
        let mut parameters = vec![];

        loop {
            let token = self.peek_next_token();

            match &token.token {
                TokenEnum::Bracket(b) => match b {
                    Bracket::RParen => {
                        self.get_next_token();
                        break;
                    }

                    _ => {
                        unexpected_token(&token, None);
                    }
                },

                TokenEnum::Comma => {
                    self.get_next_token();
                    continue;
                }

                _ => {
                    let variable = self.parse_variable();
                    parameters.push(variable);
                }
            };
        }

        return parameters;
    }

    /// FUNCTION_DEF -> fun VAR_NAME LPAREN (VAR_NAME : VAR_TYPE)* RPAREN (-> VarType)* LCURLY (STATEMENT[] - FUNCTION_DEF) RCURLY
    pub fn parse_function_definition(&mut self, f: ParserFunctions) -> ASTNode {
        // we get here after consuming 'fun'
        let token = self.get_next_token();

        let function_name = match token.token {
            TokenEnum::Variable(n) => n,

            _ => {
                unexpected_token(&token, None);
                exit(1);
            }
        };

        self.current_function_being_parsed = Some(function_name.clone());

        self.validate_token(TokenEnum::Bracket(Bracket::LParen));

        // we validate closing ')' in the following function
        let parameters = self.parse_function_definition_parameters();

        let mut return_type = VarType::Unknown;

        if let TokenEnum::FunctionReturnIndicator = self.peek_next_token().token {
            self.get_next_token();

            let peeked_token = self.peek_next_token();

            match peeked_token.token {
                TokenEnum::Type(var_type) => {
                    self.get_next_token();
                    return_type = var_type;
                }

                _ => {
                    unexpected_token(&peeked_token, None);
                    exit(1);
                }
            }
        };

        self.validate_token(TokenEnum::Bracket(Bracket::LCurly));

        // As we can fit an entire program inside a function
        // TODO: This introduces function and variable scoping issues
        self.inside_function_depth += 1;
        let block = self.parse_program();
        self.inside_function_depth -= 1;

        // println!("next token after parse_statements in parse_function_definition {:?}", self.peek_next_token().token);

        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

        let ff = function_name.clone();

        // Create an Rc from the Box
        let function_def = FunctionDefinition::new(function_name, parameters, block, return_type.clone());

        let fdef: ASTNode = Rc::new(RefCell::new(Box::new(function_def)));

        // Use Rc::clone to get a reference-counted clone of the Rc, not the inner value
        f.borrow_mut().insert(
            ff,
            FunctionHashMapValue {
                func: Rc::clone(&fdef),
                return_type,
            },
        );

        self.current_function_being_parsed = None;

        // Convert Rc back to Box for the return value
        return Rc::clone(&fdef);
    }
}

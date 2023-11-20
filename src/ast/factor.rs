use crate::{constants, interpreter::interpreter::{Variables, Functions}, lexer::lexer::Token};

use super::abstract_syntax_tree::{VisitResult, AST};

/// FACTOR -> INTEGER | FLOAT
#[derive(Debug)]
pub struct Factor {
    token: Box<Token>,
}

impl Factor {
    pub fn new(token: Box<Token>) -> Self {
        Self { token }
    }
}

impl AST for Factor {
    fn visit(&self, _: &mut Variables, _: &mut Functions) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:?}", &self);
        }

        VisitResult {
            token: Box::new(self.token.token.clone()),
        }
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}

use crate::lexer::tokens::AllOperations;
use crate::lexer::types::VarType;
use crate::trace;
use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use crate::{
    asm::asm::ASM,
    constants,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Number, Operand, Operations, TokenEnum, VariableEnum},
    },
};
use core::panic;
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct BinaryOP {
    left: ASTNode,
    operator: Box<Token>,
    right: ASTNode,
    pub times_dereferenced: usize,
    pub result_type: VarType,
}

impl BinaryOP {
    pub fn new(left: ASTNode, operator: Box<Token>, right: ASTNode, times_dereferenced: usize) -> Self {
        Self {
            left,
            operator,
            right,
            times_dereferenced,
            result_type: VarType::Unknown,
        }
    }

    fn evaluate_int<T>(&self, l: T, r: T) -> T
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: std::ops::Div<Output = T>,
        T: std::ops::Shl<Output = T>,
        T: std::ops::Shr<Output = T>,
        T: std::ops::Rem<Output = T>,
        T: std::fmt::Debug,
    {
        match &self.operator.token {
            TokenEnum::Op(op) => match op {
                Operations::Plus => l + r,
                Operations::Minus => l - r,
                Operations::Divide => l / r,
                Operations::Multiply => l * r,
                Operations::ShiftLeft => l << r,
                Operations::ShiftRight => l >> r,
                Operations::Modulo => l % r,
            },

            _ => {
                unreachable!("WTF!!")
            }
        }
    }

    fn evaluate_float<T>(&self, l: T, r: T) -> T
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: std::ops::Div<Output = T>,
        T: std::fmt::Debug,
    {
        match &self.operator.token {
            TokenEnum::Op(op) => match op {
                Operations::Plus => l + r,
                Operations::Minus => l - r,
                Operations::Divide => l / r,
                Operations::Multiply => l * r,
                Operations::ShiftLeft => panic!("Op << not implemented for floating point numbers"),
                Operations::ShiftRight => {
                    panic!("Op >> not implemented for floating point numbers")
                }
                Operations::Modulo => panic!("Op % not implemented for floating point numbers"),
            },

            _ => {
                unreachable!("WTF!!")
            }
        }
    }

    fn eval_number_number(&self, left_op: &Number, right_op: &Number) -> VisitResult {
        match (left_op, right_op) {
            (Number::Integer(l), Number::Integer(r)) => {
                return VisitResult {
                    token: Box::new(TokenEnum::new_integer(self.evaluate_int(*l, *r))),
                };
            }

            (Number::Float(l), Number::Float(r)) => {
                return VisitResult {
                    token: Box::new(TokenEnum::new_float(self.evaluate_float(*l, *r))),
                };
            }

            _ => {
                panic!("Cannot add Float and Integer");
            }
        };
    }

    fn eval_var_num(&self, number: &Number, variable: &String, i: &mut Variables) -> VisitResult {
        let result = i.get(variable);

        match result {
            Some(var_num) => match var_num {
                VariableEnum::Number(var_num) => self.eval_number_number(number, var_num),
                VariableEnum::String(_) => todo!(),
                VariableEnum::Pointer(_) => todo!(),
            },

            None => panic!("Variable {} is not defined", variable),
        }
    }

    fn eval_var_var(&self, var1: &String, var2: &String, i: &mut Variables) -> VisitResult {
        let r1 = i.get(var1);
        let r2 = i.get(var2);

        match (r1, r2) {
            (Some(var1), Some(var2)) => match (var1, var2) {
                (VariableEnum::Number(var1), VariableEnum::Number(var2)) => self.eval_number_number(var1, var2),

                (VariableEnum::Number(_), VariableEnum::String(_)) => todo!(),
                (VariableEnum::String(_), VariableEnum::Number(_)) => todo!(),
                (VariableEnum::String(_), VariableEnum::String(_)) => todo!(),
                (VariableEnum::Number(_), VariableEnum::Pointer(_)) => todo!(),
                (VariableEnum::String(_), VariableEnum::Pointer(_)) => todo!(),
                (VariableEnum::Pointer(_), VariableEnum::Number(_)) => todo!(),
                (VariableEnum::Pointer(_), VariableEnum::String(_)) => todo!(),
                (VariableEnum::Pointer(_), VariableEnum::Pointer(_)) => todo!(),
            },

            (None, Some(_)) => panic!("Variable {} is not defined", var2),
            (Some(_), None) => panic!("Variable {} is not defined", var1),
            (None, None) => panic!("Variable {} and {} is not defined", var1, var2),
        }
    }

    fn evaluate_operands(&self, left_op: &Operand, right_op: &Operand, i: &mut Variables) -> VisitResult {
        match (left_op, right_op) {
            (Operand::Number(left_op), Operand::Number(right_op)) => self.eval_number_number(left_op, right_op),

            (Operand::Number(n), Operand::Variable(v)) => self.eval_var_num(n, v, i),
            (Operand::Variable(v), Operand::Number(n)) => self.eval_var_num(n, v, i),

            (Operand::Variable(v1), Operand::Variable(v2)) => self.eval_var_var(v1, v2, i),
        }
    }
}

impl AST for BinaryOP {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        self.left.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

        self.right.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

        match &self.operator.token {
            TokenEnum::Op(c) => {
                // if matches!(c, Operations::Plus) {
                //     trace!("line: {}, times_dereferenced: {}, op: {c}", self.get_token().line_number, self.times_dereferenced);
                // }

                asm.binary_op_nums(c.clone(), self.times_dereferenced, &self.get_type().1);
            }

            _ => unreachable!("Found non operator for a Binary Expression"),
        }
    }

    fn visit(&self, i: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:#?}", &self);
            println!("===============================================");
        }

        let visit_left = self.left.borrow().visit(i, Rc::clone(&f), call_stack);
        let visit_right = self.right.borrow().visit(i, Rc::clone(&f), call_stack);

        let left_operand = visit_left.token.get_operand();
        let right_operand = visit_right.token.get_operand();

        match (&left_operand, &right_operand) {
            (Ok(lop), Ok(rop)) => {
                // Handle the case where both operands are Ok

                return self.evaluate_operands(lop, rop, i);
            }

            (Err(err), _) => {
                // Handle the case where left_operand is an error
                panic!("{}", err);
            }

            (_, Err(err)) => {
                // Handle the case where right_operand is an error
                panic!("{}", err);
            }
        };
    }

    fn get_token(&self) -> &Token {
        return &self.operator;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.left.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));
        self.right.borrow_mut().semantic_visit(call_stack, f);

        if let TokenEnum::Op(op) = &self.operator.token {
            self.result_type = self
                .left
                .borrow()
                .get_node()
                .figure_out_type(&self.right.borrow().get_node(), AllOperations::Op(op.clone()));

            // trace!("left: {:#?}", self.left.borrow());
            // trace!("right: {:#?}", self.right.borrow());

            // if self.operator.line_number == 12 {
            //     trace!("self.result_type: {:#?}", self.result_type);
            // }
        } else {
            unreachable!("Found Operation '{:?}' which is not defined for a binary operation. This must be a bug in the parsing step", self.operator.token)
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::BinaryOp(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::BinaryOp(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type
                .get_actual_type(self.times_dereferenced, &self.operator),
            self.result_type.clone(),
        );
    }
}

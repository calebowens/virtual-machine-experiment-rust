use crate::instruction::{Runnable, InstructionResult, InstructionError};
use crate::value::Value;
use crate::stack::Stack;

#[derive(Debug)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    Eql,
}

fn push_to_stack(value: Option<Value>, current_stack: &mut Vec<Value>) -> InstructionResult {
    if let Some(value) = value {
        current_stack.push(value);

        InstructionResult::None
    } else {
        InstructionResult::Error(InstructionError::new("Oprands not of matching numeric sub-types"))
    }
}

impl Runnable for MathOp {

    fn run(&self, stack: &mut Stack) -> InstructionResult {
        let current_stack = stack.current();

        let mut current_stack = current_stack.borrow_mut();

        let b = current_stack.pop().unwrap();
        let a = current_stack.pop().unwrap();

        if let (Value::Numeric(a), Value::Numeric(b)) = (a, b) {
            match self {
                MathOp::Add           => push_to_stack(a.add(&b), &mut current_stack),
                MathOp::Sub           => push_to_stack(a.sub(&b), &mut current_stack),
                MathOp::Mul           => push_to_stack(a.mul(&b), &mut current_stack),
                MathOp::Div           => push_to_stack(a.div(&b), &mut current_stack),
                MathOp::GreaterThan   => push_to_stack(a.greater_than(&b), &mut current_stack),
                MathOp::LessThan      => push_to_stack(a.less_than(&b), &mut current_stack),
                MathOp::GreaterThanEq => push_to_stack(a.greater_than_eq(&b), &mut current_stack),
                MathOp::LessThanEq    => push_to_stack(a.less_than_eq(&b), &mut current_stack),
                MathOp::Eql           => push_to_stack(a.eq(&b), &mut current_stack),
                
            }
        } else {
            InstructionResult::Error(InstructionError::new("An oprand is not of type numeric"))
        }

    }
}

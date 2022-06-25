use crate::instruction::Runnable;
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


impl Runnable for MathOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        let mut current_stack = current_stack.borrow_mut();

        let b = current_stack.pop().unwrap();
        let a = current_stack.pop().unwrap();

        if let (Value::Numeric(a), Value::Numeric(b)) = (a, b) {
            match self {
                MathOp::Add           => current_stack.push(Value::Numeric(a.add(&b))),
                MathOp::Sub           => current_stack.push(Value::Numeric(a.sub(&b))),
                MathOp::Mul           => current_stack.push(Value::Numeric(a.mul(&b))),
                MathOp::Div           => current_stack.push(Value::Numeric(a.div(&b))),
                MathOp::GreaterThan   => current_stack.push(a.greater_than(&b)),
                MathOp::LessThan      => current_stack.push(a.less_than(&b)),
                MathOp::GreaterThanEq => current_stack.push(a.greater_than_eq(&b)),
                MathOp::LessThanEq    => current_stack.push(a.less_than_eq(&b)),
                MathOp::Eql           => current_stack.push(a.eq(&b)),
                
            };
        } else {
            panic!("A or B of MathOp is not a number");
        }

    }
}

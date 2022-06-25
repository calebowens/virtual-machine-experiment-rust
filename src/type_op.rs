use crate::stack::Stack;
use crate::instruction::Runnable;
use crate::numeric::NumericType;
use crate::value::Value;

#[derive(Debug)]
pub enum TypeOp {
    NumericCast(NumericType)
}


impl Runnable for TypeOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        let mut current_stack = current_stack.borrow_mut();

        match self {
            TypeOp::NumericCast(to) => {
                let end = current_stack.pop().unwrap();

                if let Value::Numeric(end) = end {
                    current_stack.push(Value::Numeric(end.cast(to)))
                } else {
                    panic!("Can't perform cast on non-numeric type");
                }
            }
        }
    }
}

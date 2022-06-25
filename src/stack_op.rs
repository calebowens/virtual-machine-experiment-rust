use crate::numeric::Numeric;
use crate::instruction::Runnable;
use crate::value::{ValueType, Value};
use crate::ptr::Ptr;
use crate::stack::Stack;
use crate::cast_to_value;

#[derive(Debug)]
pub enum StackOp {
    Swap,
    Duplicate,
    Pop(Ptr),
    Push(ValueType),
    PushPtr(Ptr),
    DeRef,
    SubStack(ValueType),
    Destack(ValueType),
    Len
}


impl Runnable for StackOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        match self {
            StackOp::Swap => {
                let mut current_stack = current_stack.borrow_mut();

                let len = current_stack.len();
                let tmp = current_stack[len - 1].clone();
                current_stack[len - 1] = current_stack[len - 2].clone();
                current_stack[len - 2] = tmp;
            },
            StackOp::Duplicate => {
                current_stack.borrow_mut().push(current_stack.borrow().last().unwrap().clone());
            },
            StackOp::Pop(ptr) => {
                ptr.value.replace(current_stack.borrow_mut().pop().unwrap());
            },
            StackOp::Push(value) => {
                let value = value.to_value(current_stack.borrow().last().unwrap().clone());

                current_stack.borrow_mut().push(value);
            },
            StackOp::PushPtr(ptr) => {
                current_stack.borrow_mut().push(Value::Ptr(ptr.clone()))
            },
            StackOp::DeRef => {
                let mut current_stack = current_stack.borrow_mut();

                let value = current_stack.pop().unwrap();

                if let Value::Ptr(value) = value {
                    current_stack.push(value.value.borrow().clone());
                } else {
                    panic!("Can't DeRef a non-ptr");
                }
            },
            StackOp::SubStack(value) => { 
                let value = value.to_value(current_stack.borrow().last().unwrap().clone());

                if let Value::Numeric(value) = value {
                    stack.substack(cast_to_value!(value, usize)); 
                } else {
                    panic!("SubStack argument can't be non-Numeric")
                }
                
            },
            StackOp::Destack(value) => { 
                let value = value.to_value(current_stack.borrow().last().unwrap().clone());

                if let Value::Numeric(value) = value {
                    stack.destack(cast_to_value!(value, usize)); 
                } else {
                    panic!("SubStack argument can't be non-Numeric")
                }
            },
            StackOp::Len => { 
                current_stack.borrow_mut().push(Value::Numeric(Numeric::USize(current_stack.borrow().len()))) 
            }
        };
    }
}

use crate::numeric::Numeric;
use crate::instruction::{Runnable, InstructionResult, InstructionError};
use crate::value::{ValueType, Value};
use crate::ptr::Ptr;
use crate::stack::Stack;
use crate::cast_to_value;

#[derive(Debug)]
pub enum StackOp {
    Swap,
    Duplicate,
    Drop,
    Pop(Ptr),
    Push(ValueType),
    PushPtr(Ptr),
    DeRef,
    SubStack(ValueType),
    Destack(ValueType),
    Len,
    Inspect,
}


impl Runnable for StackOp {
    fn run(&self, stack: &mut Stack) -> InstructionResult {
        let current_stack = stack.current();

        match self {
            StackOp::Swap => {
                let mut current_stack = current_stack.borrow_mut();

                let len = current_stack.len();

                if len < 2 {
                    InstructionResult::Error(InstructionError::new("Stack too short to perform swap"))
                } else {
                    let tmp = current_stack[len - 1].clone();
                    current_stack[len - 1] = current_stack[len - 2].clone();
                    current_stack[len - 2] = tmp;

                    InstructionResult::None
                }
            },
            StackOp::Duplicate => {
                let mut current_stack = current_stack.borrow_mut();

                let last_element = current_stack.last().cloned();

                if let Some(last_element) = last_element {
                    current_stack.push(last_element.clone());

                    InstructionResult::None
                } else {
                    InstructionResult::Error(InstructionError::new("No item on stack to duplicate"))
                }
            },
            StackOp::Pop(ptr) => {
                let popped_element = current_stack.borrow_mut().pop();

                if let Some(popped_element) = popped_element {
                    ptr.value.replace(popped_element);

                    InstructionResult::None
                } else {
                    InstructionResult::Error(InstructionError::new("No item on stack to pop"))
                }

            },
            StackOp::Drop => {
                current_stack.borrow_mut().pop();

                InstructionResult::None
            },
            StackOp::Push(value) => {
                let value = value.to_value(current_stack.borrow().last());

                if let Some(value) = value {
                    current_stack.borrow_mut().push(value);
                    InstructionResult::None
                } else {
                    InstructionResult::Error(InstructionError::new("Failed to get value"))
                }
            },
            StackOp::PushPtr(ptr) => {
                current_stack.borrow_mut().push(Value::Ptr(ptr.clone()));

                InstructionResult::None
            },
            StackOp::DeRef => {
                let mut current_stack = current_stack.borrow_mut();

                let value = current_stack.pop();

                if let Some(value) = value {
                    if let Value::Ptr(value) = value {
                        current_stack.push(value.value.borrow().clone());

                        InstructionResult::None
                    } else {
                        InstructionResult::Error(InstructionError::new("Last item on stack not ptr"))
                    }
                } else {
                    InstructionResult::Error(InstructionError::new("No item on stack to DeRef"))
                }
            },
            StackOp::SubStack(value) => { 
                let value = value.to_value(current_stack.borrow().last());

                if let Some(Value::Numeric(value)) = value {
                    stack.substack(cast_to_value!(value, usize)); 

                    InstructionResult::None
                } else {
                    InstructionResult::Error(InstructionError::new("Failed to read numeric value"))
                }
                
            },
            StackOp::Destack(value) => { 
                let value = value.to_value(current_stack.borrow().last());

                if let Some(Value::Numeric(value)) = value {
                    stack.destack(cast_to_value!(value, usize)); 

                    InstructionResult::None
                } else {
                    InstructionResult::Error(InstructionError::new("Failed to read numeric value"))
                }
            },
            StackOp::Len => { 
                current_stack.borrow_mut().push(Value::Numeric(Numeric::USize(current_stack.borrow().len())));

                InstructionResult::None
            },
            StackOp::Inspect => {
                println!("Inspect: {:?}", current_stack);

                InstructionResult::None
            }
        }
    }
}

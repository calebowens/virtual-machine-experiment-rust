use crate::value::{Value, ValueType};
use crate::numeric::Numeric;
use crate::instruction::{Runnable, InstructionResult, InstructionError};
use crate::control::InstructionControl;
use crate::stack::Stack;


#[derive(Debug)]
pub enum ControlOp {
    Call(ValueType)
}


impl Runnable for ControlOp {
    fn run(&self, stack: &mut Stack) -> InstructionResult {
        match self {
            ControlOp::Call(value) => {
                let value = value.to_value(stack.current().borrow().last());

                if let Some(value) = value {
                    if let Value::Numeric(Numeric::USize(value)) = value {
                        InstructionResult::Control(InstructionControl::Call(value))
                    } else {
                        InstructionResult::Error(InstructionError::new("Oprand must be numeric usize"))
                    }
                } else {
                    InstructionResult::Error(InstructionError::new("Failed to obtian value"))
                }
            }
        }
    }
}

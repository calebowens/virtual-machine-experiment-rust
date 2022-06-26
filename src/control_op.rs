use crate::value::{Value, ValueType};
use crate::numeric::Numeric;
use crate::instruction::{Runnable, InstructionResult, InstructionError};
use crate::control::InstructionControl;
use crate::stack::Stack;


#[derive(Debug)]
pub enum ControlOp {
    Call(ValueType),
    CallIf(ValueType, ValueType),
    CallElse(ValueType, ValueType),
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
                    InstructionResult::Error(InstructionError::new("Failed to obtian function value"))
                }
            },
            ControlOp::CallIf(ptr, value) => {
                let ptr = ptr.to_value(stack.current().borrow().last());

                if let Some(ptr) = ptr {
                    if let Value::Numeric(Numeric::USize(ptr)) = ptr {
                        let value = value.to_value(stack.current().borrow().last());

                        if let Some(value) = value {
                            if let Value::Bool(value) = value {
                                if value {
                                    InstructionResult::Control(InstructionControl::Call(ptr))
                                } else {
                                    InstructionResult::None
                                }
                            } else {
                                InstructionResult::Error(InstructionError::new("Predicate value must be boolean"))
                            }
                        } else {
                            InstructionResult::Error(InstructionError::new("Failed to obtain predicate value"))
                        }
                    } else {
                        InstructionResult::Error(InstructionError::new("Oprand must be numeric usize"))
                    }
                } else {
                    InstructionResult::Error(InstructionError::new("Failed to obtian function value"))
                }
            },
            ControlOp::CallElse(ptr, value) => {
                let ptr = ptr.to_value(stack.current().borrow().last());

                if let Some(ptr) = ptr {
                    if let Value::Numeric(Numeric::USize(ptr)) = ptr {
                        let value = value.to_value(stack.current().borrow().last());

                        if let Some(value) = value {
                            if let Value::Bool(value) = value {
                                if !value {
                                    InstructionResult::Control(InstructionControl::Call(ptr))
                                } else {
                                    InstructionResult::None
                                }
                            } else {
                                InstructionResult::Error(InstructionError::new("Predicate value must be boolean"))
                            }
                        } else {
                            InstructionResult::Error(InstructionError::new("Failed to obtain predicate value"))
                        }
                    } else {
                        InstructionResult::Error(InstructionError::new("Oprand must be numeric usize"))
                    }
                } else {
                    InstructionResult::Error(InstructionError::new("Failed to obtian function value"))
                }
            }
        }
    }
}

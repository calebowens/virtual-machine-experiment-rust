use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::value::Value;
use crate::stack::Stack;
use crate::instruction::{Instruction, Runnable, InstructionResult};
use crate::control::InstructionControl;


#[derive(Debug)]
pub struct Function {
    pub ptr_recipie: Vec<Value>,
    pub instructions: Vec<Instruction>,
    pub param_count: usize,
    pub return_count: usize
}


#[derive(Debug, Clone)]
struct RuntimeContext {
    current_fn: usize,
    current_instruction: usize,
}


impl RuntimeContext {
    pub fn new(current_fn: usize) -> RuntimeContext {
        RuntimeContext {
            current_fn,
            current_instruction: 0,
        }
    }
}


pub struct FunctionController {
    functions: HashMap<usize, Function>,
    context: Vec<Rc<RefCell<RuntimeContext>>>,
    stack: Stack
}


impl FunctionController {
    pub fn new(functions: HashMap<usize, Function>, start: usize) -> FunctionController {
        FunctionController {
            functions,
            context: vec![Rc::new(RefCell::new(RuntimeContext::new(start)))],
            stack: Stack::new()
        }
    }

    pub fn run(&mut self) {
        'runtime_loop: loop {
            if self.context.len() == 0 {
                break 'runtime_loop;
            }

            let cloned_context = self.context.clone();

            let mut current_context = cloned_context[self.context.len() - 1].borrow_mut();

            let current_fn = self.functions.get(&current_context.current_fn)
                .expect("Invalid function address used");
            
            while current_context.current_instruction != current_fn.instructions.len() {
                let result = current_fn.instructions[current_context.current_instruction].run(&mut self.stack);

                match result {
                    InstructionResult::None => {
                        current_context.current_instruction += 1;
                    },
                    InstructionResult::Control(control) => {
                        match control {
                            InstructionControl::Call(address) => {
                                current_context.current_instruction += 1;

                                self.stack.substack(self.functions
                                                    .get(&address)
                                                    .expect("Invalid function address used")
                                                    .param_count);

                                self.context.push(Rc::new(RefCell::new(RuntimeContext::new(address))));

                                continue 'runtime_loop;
                            }
                        }
                    },
                    InstructionResult::Error(error) => {
                        panic!("Error: '{}' occoured at instruction {}, {:?}", error.message, current_context.current_instruction, current_fn.instructions[current_context.current_instruction])
                    }
                }
            }

            if self.context.len() != 1 {
                self.stack.destack(current_fn.return_count);
            }

            self.context.pop();
        }
    }
}

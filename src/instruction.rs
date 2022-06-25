use crate::stack::Stack;
use crate::math_op::MathOp;
use crate::type_op::TypeOp;
use crate::stack_op::StackOp;
use crate::control_op::ControlOp;
use crate::control::InstructionControl;

pub trait Runnable {
    fn run(&self, stack: &mut Stack) -> InstructionResult;
}


#[derive(Debug)]
pub enum Instruction {
    Math(MathOp),
    Stack(StackOp),
    Type(TypeOp),
    Control(ControlOp)
}


pub struct InstructionError {
    pub message: String
}


impl InstructionError {
    pub fn new(message: &str) -> InstructionError {
        InstructionError {
            message: message.to_string()
        }
    }
}


pub enum InstructionResult {
    None,
    Error(InstructionError),
    Control(InstructionControl)
}


impl Runnable for Instruction {
    fn run(&self, stack: &mut Stack) -> InstructionResult {
        match self {
            Instruction::Math(instr) => instr.run(stack),
            Instruction::Stack(instr) => instr.run(stack),
            Instruction::Type(instr) => instr.run(stack),
            Instruction::Control(instr) => instr.run(stack),
        }
    }
}

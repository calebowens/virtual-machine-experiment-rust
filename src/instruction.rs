use crate::stack::Stack;
use crate::math_op::MathOp;
use crate::type_op::TypeOp;
use crate::stack_op::StackOp;

pub trait Runnable {
    fn run(&self, stack: &mut Stack);
}


#[derive(Debug)]
pub enum Instruction {
    Math(MathOp),
    Stack(StackOp),
    Type(TypeOp)
}


impl Runnable for Instruction {
    fn run(&self, stack: &mut Stack) {
        match self {
            Instruction::Math(instr) => instr.run(stack),
            Instruction::Stack(instr) => instr.run(stack),
            Instruction::Type(instr) => instr.run(stack),
        }
    }
}

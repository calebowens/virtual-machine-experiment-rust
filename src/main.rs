#![allow(dead_code)]
mod data_type;
mod numeric;
mod value;
mod ptr;
mod stack;
mod instruction;
mod math_op;
mod type_op;
mod stack_op;

use crate::numeric::Numeric;
use crate::value::Value;
use crate::ptr::Ptr;
use crate::stack::Stack;
use crate::instruction::{Instruction, Runnable};
use crate::stack_op::StackOp;

fn main() {
    let ptr = Ptr::new(Value::Numeric(Numeric::Int8(0)));
    let ptr_ptr = Ptr::new(Value::Ptr(Ptr::new(Value::Numeric(Numeric::Int8(1)))));

    let instructions: Vec<Instruction> = vec![
        Instruction::Stack(StackOp::PushPtr(ptr)),
        Instruction::Stack(StackOp::Pop(ptr_ptr.clone()))
    ];

    let mut stack = Stack::new();


    for instruction in instructions {
        instruction.run(&mut stack);
    }

    println!("ptr output value: {:?}", ptr_ptr);

}

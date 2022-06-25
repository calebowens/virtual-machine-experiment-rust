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
mod function;
mod control;
mod control_op;


use std::collections::HashMap;

use crate::numeric::Numeric;
use crate::value::{Value, ValueType};
use crate::ptr::Ptr;
use crate::instruction::Instruction;
use crate::stack_op::StackOp;
use crate::math_op::MathOp;
use crate::control_op::ControlOp;
use crate::function::{Function, FunctionController};


fn main() {
    let ptr = Ptr::new(Value::Numeric(Numeric::Int8(2)));


    let start = Function {
        ptr_recipie: vec![],
        param_count: 0,
        return_count: 0,
        instructions: vec![
            Instruction::Stack(StackOp::PushPtr(ptr.clone())),
            Instruction::Control(ControlOp::Call(ValueType::Value(Value::Numeric(Numeric::USize(2))))),
            Instruction::Stack(StackOp::Pop(ptr.clone()))
        ],
    };


    let sub_fn = Function {
        ptr_recipie: vec![],
        param_count: 1,
        return_count: 1,
        instructions: vec![
            Instruction::Stack(StackOp::DeRef),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Int8(3))))),
            Instruction::Math(MathOp::Add)
        ]
    };


    let mut functions = HashMap::new();

    functions.insert(1, start);
    functions.insert(2, sub_fn);


    let mut fn_controller = FunctionController::new(functions, 1);

    
    fn_controller.run();


    println!("ptr output value: {:?}", ptr);

}

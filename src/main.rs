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
    let ptr = Ptr::new(Value::Numeric(Numeric::Int32(14)));
    
    let fibonacci_fn_id = ValueType::Value(Value::Numeric(Numeric::USize(2)));

    let start = Function {
        ptr_recipie: vec![],
        param_count: 0,
        return_count: 0,
        instructions: vec![
            Instruction::Stack(StackOp::PushPtr(ptr.clone())),
            Instruction::Stack(StackOp::DeRef),
            Instruction::Control(ControlOp::Call(fibonacci_fn_id.clone())),
            Instruction::Stack(StackOp::Swap),
            Instruction::Stack(StackOp::Drop),
            Instruction::Stack(StackOp::Pop(ptr.clone()))
        ],
    };


    let fibonacci_fn_sub_one_id = ValueType::Value(Value::Numeric(Numeric::USize(3)));
    let fibonacci_fn_sub_two_id = ValueType::Value(Value::Numeric(Numeric::USize(4)));
    let fibonacci_fn_sub_exit_id = ValueType::Value(Value::Numeric(Numeric::USize(5)));


    let fibonacci_fn = Function { // fib n -> fib n-1 + fib n-2 unless n = 1 or 2 when -> 1
        ptr_recipie: vec![],
        param_count: 1,
        return_count: 1,
        instructions: vec![ // Starting on the stack there is the input value
            Instruction::Stack(StackOp::Duplicate),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Int32(2))))),
            Instruction::Math(MathOp::GreaterThan),
            Instruction::Control(ControlOp::CallIf(fibonacci_fn_sub_one_id, ValueType::StackValue)),
            Instruction::Control(ControlOp::CallIf(fibonacci_fn_sub_two_id, ValueType::StackValue)),
            Instruction::Control(ControlOp::CallElse(fibonacci_fn_sub_exit_id, ValueType::StackValue)),
            Instruction::Stack(StackOp::Drop),
            Instruction::Math(MathOp::Add)
        ]
    };


    let fibonacci_fn_sub_one = Function {
        ptr_recipie: vec![],
        param_count: 2,
        return_count: 3,
        instructions: vec![ // Starting on the stack there is the input value
            Instruction::Stack(StackOp::Drop),
            Instruction::Stack(StackOp::Duplicate),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Int32(1))))),
            Instruction::Math(MathOp::Sub),
            Instruction::Control(ControlOp::Call(fibonacci_fn_id.clone())),
            Instruction::Stack(StackOp::Swap),
            Instruction::Stack(StackOp::Drop),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Bool(true)))),
        ]
    };


    let fibonacci_fn_sub_two = Function {
        ptr_recipie: vec![],
        param_count: 3,
        return_count: 3,
        instructions: vec![ // Starting on the stack there is the input value
            Instruction::Stack(StackOp::Drop),
            Instruction::Stack(StackOp::Swap),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Int32(2))))),
            Instruction::Math(MathOp::Sub),
            Instruction::Control(ControlOp::Call(fibonacci_fn_id.clone())),
            Instruction::Stack(StackOp::Swap),
            Instruction::Stack(StackOp::Drop),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Bool(true)))),
        ]
    };

    let fibonacci_fn_sub_exit = Function {
        ptr_recipie: vec![],
        param_count: 0,
        return_count: 3,
        instructions: vec![ // Starting on the stack there is the input value
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Int32(1))))),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Int32(0))))),
            Instruction::Stack(StackOp::Push(ValueType::Value(Value::Bool(false)))),
        ]
    };


    let mut functions = HashMap::new();

    functions.insert(1, start);
    functions.insert(2, fibonacci_fn);
    functions.insert(3, fibonacci_fn_sub_one);
    functions.insert(4, fibonacci_fn_sub_two);
    functions.insert(5, fibonacci_fn_sub_exit);


    let mut fn_controller = FunctionController::new(functions, 1);

    
    fn_controller.run();


    println!("ptr output value: {:?}", ptr);

}

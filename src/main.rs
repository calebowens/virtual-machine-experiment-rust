#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone, Copy, PartialEq, Debug)]
enum DataType {
    Str,
    Int32,
    Bool
}


trait Typed {
    fn get_type(&self) -> DataType;
}


#[derive(Clone, Debug)]
enum Value {
    Str(String),
    Int32(i32),
    Bool(bool)
}


impl Typed for Value {
    fn get_type(&self) -> DataType {
        match self {
            Value::Str(_) => DataType::Str,
            Value::Int32(_) => DataType::Int32,
            Value::Bool(_) => DataType::Bool
        }
    }
}


#[derive(Clone, Debug)]
struct Ptr {
    value: Rc<RefCell<Value>>
}


impl Ptr {
    fn new(value: Value) -> Ptr {
        Ptr {
            value: Rc::new(RefCell::new(value))
        }
    }
}


impl Typed for Ptr {
    fn get_type(&self) -> DataType {
        self.value.borrow().get_type()
    }
}


#[derive(Debug)]
enum ValueType {
    Ptr(Ptr),
    Value(Value),
    StackValue
}


#[derive(Debug)]
struct Stack {
    stacks: Vec<Vec<Value>>
}


impl Stack {
    fn new() -> Stack {
        Stack { stacks: vec![vec![]] }
    }

    fn current(&mut self) -> &mut Vec<Value> {
        self.stacks.last_mut().unwrap()
    }

    fn substack(&mut self) {
        self.stacks.push(vec![]);
    }

    fn destack(&mut self) {
        self.stacks.pop();
    }
}


trait Runnable {
    fn run(&self, stack: &mut Stack);
}


#[derive(Debug)]
enum Instruction {
    Math(MathOp),
    Stack(StackOp)
}


#[derive(Debug)]
enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    Eql
}


impl Runnable for MathOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        let b = current_stack.pop().unwrap();
        let a = current_stack.pop().unwrap();

        if let (Value::Int32(a), Value::Int32(b)) = (a, b) {
            match self {
                MathOp::Add           => current_stack.push(Value::Int32(a + b)),
                MathOp::Sub           => current_stack.push(Value::Int32(a - b)),
                MathOp::Mul           => current_stack.push(Value::Int32(a * b)),
                MathOp::Div           => current_stack.push(Value::Int32(a / b)),
                MathOp::GreaterThan   => current_stack.push(Value::Bool(a < b)),
                MathOp::LessThan      => current_stack.push(Value::Bool(a > b)),
                MathOp::GreaterThanEq => current_stack.push(Value::Bool(a <= b)),
                MathOp::LessThanEq    => current_stack.push(Value::Bool(a >= b)),
                MathOp::Eql           => current_stack.push(Value::Bool(a == b))
            };
        } else {
            panic!("A or B of MathOp is not a number");
        }

    }
}


#[derive(Debug)]
enum StackOp {
    Swap,
    Duplicate,
    Pop(Ptr),
    Push(ValueType),
    SubStack,
    Destack
}


impl Runnable for StackOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        match self {
            StackOp::Swap => {
                let len = current_stack.len();
                let tmp = current_stack[len - 1].clone();
                current_stack[len - 1] = current_stack[len - 2].clone();
                current_stack[len - 2] = tmp;
            },
            StackOp::Duplicate => {
                current_stack.push(current_stack.last().unwrap().clone());
            },
            StackOp::Pop(ptr) => {
                ptr.value.replace(current_stack.pop().unwrap());
            },
            StackOp::Push(value) => {
                match value {
                    ValueType::Ptr(ptr) => current_stack.push(ptr.value.borrow().clone()),
                    ValueType::Value(value) => current_stack.push(value.clone()),
                    ValueType::StackValue => current_stack.push(current_stack.last().unwrap().clone())
                };
            },
            StackOp::SubStack => { stack.substack(); },
            StackOp::Destack => { stack.destack(); },
        };
    }
}


impl Runnable for Instruction {
    fn run(&self, stack: &mut Stack) {
        match self {
            Instruction::Math(instr) => instr.run(stack),
            Instruction::Stack(instr) => instr.run(stack),
        }
    }
}


trait Valuable {
    fn to_value(self) -> Value;
}


trait Ptrable {
    fn to_pointer(self) -> Ptr;
}


impl Valuable for i32 {
    fn to_value(self) -> Value {
        Value::Int32(self)
    }
}


impl Valuable for bool {
    fn to_value(self) -> Value {
        Value::Bool(self)
    }
}


impl Valuable for String {
    fn to_value(self) -> Value {
        Value::Str(self)
    }
}

impl Ptrable for i32 {
    fn to_pointer(self) -> Ptr {
        Ptr::new(Value::Int32(self))
    }
}


impl Ptrable for bool {
    fn to_pointer(self) -> Ptr {
        Ptr::new(Value::Bool(self))
    }
}


impl Ptrable for String {
    fn to_pointer(self) -> Ptr {
        Ptr::new(Value::Str(self))
    }
}


fn main() {
    let ptr = 0.to_pointer();

    let instructions: Vec<Instruction> = vec![
        Instruction::Stack(StackOp::Push(ValueType::Value(Value::Int32(2)))),
        Instruction::Stack(StackOp::Push(ValueType::Value(Value::Int32(3)))),
        Instruction::Math(MathOp::Add),
        Instruction::Stack(StackOp::Pop(ptr.clone()))
    ];

    let mut stack = Stack::new();


    for instruction in instructions {
        instruction.run(&mut stack);
    }

    println!("ptr output value: {:?}", ptr);

}

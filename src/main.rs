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
enum Numeric {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    USize(usize),
    ISize(isize),
}


macro_rules! impl_math {
    ($name:ident, $op:tt) => {
        pub fn $name(&self, rhs: &Self) -> Self {
            match (self, rhs) {
                (Numeric::UInt8(a),   Numeric::UInt8(b)  ) => Numeric::UInt8(a $op b),
                (Numeric::UInt16(a),  Numeric::UInt16(b) ) => Numeric::UInt16(a $op b),
                (Numeric::UInt32(a),  Numeric::UInt32(b) ) => Numeric::UInt32(a $op b),
                (Numeric::UInt64(a),  Numeric::UInt64(b) ) => Numeric::UInt64(a $op b),
                (Numeric::UInt128(a), Numeric::UInt128(b)) => Numeric::UInt128(a $op b),
                (Numeric::Int8(a),    Numeric::Int8(b)   ) => Numeric::Int8(a $op b),
                (Numeric::Int16(a),   Numeric::Int16(b)  ) => Numeric::Int16(a $op b),
                (Numeric::Int32(a),   Numeric::Int32(b)  ) => Numeric::Int32(a $op b),
                (Numeric::Int64(a),   Numeric::Int64(b)  ) => Numeric::Int64(a $op b),
                (Numeric::Float32(a), Numeric::Float32(b)) => Numeric::Float32(a $op b),
                (Numeric::Float64(a), Numeric::Float64(b)) => Numeric::Float64(a $op b),
                (Numeric::USize(a),   Numeric::USize(b)  ) => Numeric::USize(a $op b),
                (Numeric::ISize(a),   Numeric::ISize(b)  ) => Numeric::ISize(a $op b),
                _ => panic!("LHS and RHS not matching")
            }
        }
    }
}


macro_rules! impl_cmp {
    ($name:ident, $op:tt) => {
        pub fn $name(&self, rhs: &Self) -> Value {
            match (self, rhs) {
                (Numeric::UInt8(a),   Numeric::UInt8(b)  ) => Value::Bool(a $op b),
                (Numeric::UInt16(a),  Numeric::UInt16(b) ) => Value::Bool(a $op b),
                (Numeric::UInt32(a),  Numeric::UInt32(b) ) => Value::Bool(a $op b),
                (Numeric::UInt64(a),  Numeric::UInt64(b) ) => Value::Bool(a $op b),
                (Numeric::UInt128(a), Numeric::UInt128(b)) => Value::Bool(a $op b),
                (Numeric::Int8(a),    Numeric::Int8(b)   ) => Value::Bool(a $op b),
                (Numeric::Int16(a),   Numeric::Int16(b)  ) => Value::Bool(a $op b),
                (Numeric::Int32(a),   Numeric::Int32(b)  ) => Value::Bool(a $op b),
                (Numeric::Int64(a),   Numeric::Int64(b)  ) => Value::Bool(a $op b),
                (Numeric::Float32(a), Numeric::Float32(b)) => Value::Bool(a $op b),
                (Numeric::Float64(a), Numeric::Float64(b)) => Value::Bool(a $op b),
                (Numeric::USize(a),   Numeric::USize(b)  ) => Value::Bool(a $op b),
                (Numeric::ISize(a),   Numeric::ISize(b)  ) => Value::Bool(a $op b),
                _ => panic!("LHS and RHS not matching")
            }
        }
    }
}


#[derive(Debug, Clone)]
enum NumericType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
    USize,
    ISize,
}


macro_rules! cast {
    ($to:ident, $value:ident) => {
        match $to {
            NumericType::UInt8   => Numeric::UInt8($value as u8),
            NumericType::UInt16  => Numeric::UInt16($value as u16),
            NumericType::UInt32  => Numeric::UInt32($value as u32),
            NumericType::UInt64  => Numeric::UInt64($value as u64),
            NumericType::UInt128 => Numeric::UInt128($value as u128),
            NumericType::Int8    => Numeric::Int8($value as i8),
            NumericType::Int16   => Numeric::Int16($value as i16),
            NumericType::Int32   => Numeric::Int32($value as i32),
            NumericType::Int64   => Numeric::Int64($value as i64),
            NumericType::Int128  => Numeric::Int128($value as i128),
            NumericType::Float32 => Numeric::Float32($value as f32),
            NumericType::Float64 => Numeric::Float64($value as f64),
            NumericType::USize   => Numeric::USize($value as usize),
            NumericType::ISize   => Numeric::ISize($value as isize),
        }
    }
}


macro_rules! cast_to_value {
    ($from:ident, $to:tt) => {
        match $from {
            Numeric::UInt8(a)   => a as $to,
            Numeric::UInt16(a)  => a as $to,
            Numeric::UInt32(a)  => a as $to,
            Numeric::UInt64(a)  => a as $to,
            Numeric::UInt128(a) => a as $to,
            Numeric::Int8(a)    => a as $to,
            Numeric::Int16(a)   => a as $to,
            Numeric::Int32(a)   => a as $to,
            Numeric::Int64(a)   => a as $to,
            Numeric::Int128(a)  => a as $to,
            Numeric::Float32(a) => a as $to,
            Numeric::Float64(a) => a as $to,
            Numeric::USize(a)   => a as $to,
            Numeric::ISize(a)   => a as $to,
        }
    }
}


impl Numeric {
    impl_math!(add, +);
    impl_math!(sub, -);
    impl_math!(mul, *);
    impl_math!(div, /);
    impl_cmp!(greater_than, >);
    impl_cmp!(greater_than_eq, >=);
    impl_cmp!(less_than, <);
    impl_cmp!(less_than_eq, <=);
    impl_cmp!(eq, ==);

    fn cast(self, to: &NumericType) -> Numeric {
        match self {
            Numeric::UInt8(a)   => { return cast!(to, a); },
            Numeric::UInt16(a)  => { return cast!(to, a); },
            Numeric::UInt32(a)  => { return cast!(to, a); },
            Numeric::UInt64(a)  => { return cast!(to, a); },
            Numeric::UInt128(a) => { return cast!(to, a); },
            Numeric::Int8(a)    => { return cast!(to, a); },
            Numeric::Int16(a)   => { return cast!(to, a); },
            Numeric::Int32(a)   => { return cast!(to, a); },
            Numeric::Int64(a)   => { return cast!(to, a); },
            Numeric::Int128(a)  => { return cast!(to, a); },
            Numeric::Float32(a) => { return cast!(to, a); },
            Numeric::Float64(a) => { return cast!(to, a); },
            Numeric::USize(a)   => { return cast!(to, a); },
            Numeric::ISize(a)   => { return cast!(to, a); },
        };
    }
}


#[derive(Clone, Debug)]
enum Value {
    Str(String),
    Numeric(Numeric),
    Bool(bool)
}


impl Typed for Value {
    fn get_type(&self) -> DataType {
        match self {
            Value::Str(_) => DataType::Str,
            Value::Numeric(_) => DataType::Int32,
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

impl ValueType {
    fn to_value(&self, stack: &mut Stack) -> Value {
        match self {
            ValueType::Ptr(ptr) => ptr.value.borrow().clone(),
            ValueType::Value(value) => value.clone(),
            ValueType::StackValue => stack.current().borrow().last().unwrap().clone()
        }
    }
}


type SubStack = Rc<RefCell<Vec<Value>>>;


#[derive(Debug)]
struct Stack {
    stacks: Vec<SubStack>
}


impl Stack {
    fn new() -> Stack {
        Stack { stacks: vec![Rc::new(RefCell::new(vec![]))] }
    }

    fn current(&mut self) -> SubStack {
        self.stacks.last_mut().unwrap().clone()
    }

    fn substack(&mut self, carry_count: usize) {
        self.stacks.push(Rc::new(RefCell::new(vec![])));
    }

    fn destack(&mut self, carry_count: usize) {
        self.stacks.pop();
    }
}


trait Runnable {
    fn run(&self, stack: &mut Stack);
}


#[derive(Debug)]
enum Instruction {
    Math(MathOp),
    Stack(StackOp),
    Type(TypeOp)
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
    Eql,
}


impl Runnable for MathOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        let mut current_stack = current_stack.borrow_mut();

        let b = current_stack.pop().unwrap();
        let a = current_stack.pop().unwrap();

        if let (Value::Numeric(a), Value::Numeric(b)) = (a, b) {
            match self {
                MathOp::Add           => current_stack.push(Value::Numeric(a.add(&b))),
                MathOp::Sub           => current_stack.push(Value::Numeric(a.sub(&b))),
                MathOp::Mul           => current_stack.push(Value::Numeric(a.mul(&b))),
                MathOp::Div           => current_stack.push(Value::Numeric(a.div(&b))),
                MathOp::GreaterThan   => current_stack.push(a.greater_than(&b)),
                MathOp::LessThan      => current_stack.push(a.less_than(&b)),
                MathOp::GreaterThanEq => current_stack.push(a.greater_than_eq(&b)),
                MathOp::LessThanEq    => current_stack.push(a.less_than_eq(&b)),
                MathOp::Eql           => current_stack.push(a.eq(&b)),
                
            };
        } else {
            panic!("A or B of MathOp is not a number");
        }

    }
}


#[derive(Debug)]
enum TypeOp {
    NumericCast(NumericType)
}


impl Runnable for TypeOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        let mut current_stack = current_stack.borrow_mut();

        match self {
            TypeOp::NumericCast(to) => {
                let end = current_stack.pop().unwrap();

                if let Value::Numeric(end) = end {
                    current_stack.push(Value::Numeric(end.cast(to)))
                } else {
                    panic!("Can't perform cast on non-numeric type");
                }
            }
        }
    }
}


#[derive(Debug)]
enum StackOp {
    Swap,
    Duplicate,
    Pop(Ptr),
    Push(ValueType),
    SubStack(ValueType),
    Destack(ValueType),
    Len
}


impl Runnable for StackOp {
    fn run(&self, stack: &mut Stack) {
        let current_stack = stack.current();

        match self {
            StackOp::Swap => {
                let mut current_stack = current_stack.borrow_mut();

                let len = current_stack.len();
                let tmp = current_stack[len - 1].clone();
                current_stack[len - 1] = current_stack[len - 2].clone();
                current_stack[len - 2] = tmp;
            },
            StackOp::Duplicate => {
                current_stack.borrow_mut().push(current_stack.borrow().last().unwrap().clone());
            },
            StackOp::Pop(ptr) => {
                ptr.value.replace(current_stack.borrow_mut().pop().unwrap());
            },
            StackOp::Push(value) => {
                current_stack.borrow_mut().push(value.to_value(stack));
            },
            StackOp::SubStack(value) => { 
                let value = value.to_value(stack);

                if let Value::Numeric(value) = value {
                    stack.substack(cast_to_value!(value, usize)); 
                } else {
                    panic!("SubStack argument can't be non-Numeric")
                }
                
            },
            StackOp::Destack(value) => { 
                let value = value.to_value(stack);

                if let Value::Numeric(value) = value {
                    stack.destack(cast_to_value!(value, usize)); 
                } else {
                    panic!("SubStack argument can't be non-Numeric")
                }
            },
            StackOp::Len => { 
                current_stack.borrow_mut().push(Value::Numeric(Numeric::USize(current_stack.borrow().len()))) 
            }
        };
    }
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


fn main() {
    let ptr = Ptr::new(Value::Numeric(Numeric::Int8(0)));

    let instructions: Vec<Instruction> = vec![
        Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Float32(3.0))))),
        Instruction::Stack(StackOp::Push(ValueType::Value(Value::Numeric(Numeric::Float32(0.5))))),
        Instruction::Math(MathOp::Mul),
        Instruction::Type(TypeOp::NumericCast(NumericType::Int8)),
        Instruction::Stack(StackOp::Pop(ptr.clone()))
    ];

    let mut stack = Stack::new();


    for instruction in instructions {
        instruction.run(&mut stack);
    }

    println!("ptr output value: {:?}", ptr);

}

use crate::numeric::Numeric;
use crate::ptr::Ptr;
use crate::data_type::{DataType, Typed};


#[derive(Clone, Debug)]
pub enum Value {
    Str(String),
    Numeric(Numeric),
    Bool(bool),
    Ptr(Ptr)
}


impl Typed for Value {
    fn get_type(&self) -> DataType {
        match self {
            Value::Str(_) => DataType::Str,
            Value::Numeric(_) => DataType::Int32,
            Value::Bool(_) => DataType::Bool,
            Value::Ptr(_) => DataType::Ptr
        }
    }
}


#[derive(Debug)]
pub enum ValueType {
    Ptr(Ptr),
    Value(Value),
    StackValue,
}

impl ValueType {
    pub fn to_value(&self, last_stack_value: Value) -> Value {
        match self {
            ValueType::Ptr(ptr) => ptr.value.borrow().clone(),
            ValueType::Value(value) => value.clone(),
            ValueType::StackValue => last_stack_value
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DataType {
    Str,
    Int32,
    Bool,
    Ptr
}


pub trait Typed {
    fn get_type(&self) -> DataType;
}



use std::rc::Rc;
use std::cell::RefCell;

use crate::value::Value;
use crate::data_type::{DataType, Typed};

#[derive(Clone, Debug)]
pub struct Ptr {
    pub value: Rc<RefCell<Value>>
}


impl Ptr {
    pub fn new(value: Value) -> Ptr {
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

use std::rc::Rc;
use std::cell::RefCell;

use crate::value::Value;


pub type SubStack = Rc<RefCell<Vec<Value>>>;


#[derive(Debug)]
pub struct Stack {
    stacks: Vec<SubStack>
}


impl Stack {
    pub fn new() -> Stack {
        Stack { stacks: vec![Rc::new(RefCell::new(vec![]))] }
    }

    pub fn current(&self) -> SubStack {
        self.stacks.last().unwrap().clone()
    }

    pub fn substack(&mut self, carry_count: usize) {
        let last_items = self.current().borrow().iter().rev().take(carry_count).rev().cloned().collect();
        self.stacks.push(Rc::new(RefCell::new(last_items)));
    }

    pub fn destack(&mut self, carry_count: usize) {
        let mut last_items = self.current().borrow().iter().rev().take(carry_count).rev().cloned().collect();
        self.stacks.pop();
        self.current().borrow_mut().append(&mut last_items);
    }
}

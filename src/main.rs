use std::cell::RefCell;
use std::rc::Rc;

pub struct Test {
    x: i32,
}

impl Test {
    pub fn println(&self) {
        println!("{}", self.x);
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Test { x: 11 }));
    a.borrow().println();
    let a1 = a.clone();
    let a2 = a.clone();

    a1.borrow_mut().x = 12;
    a2.borrow().println();

    let a3 = a1.borrow_mut();
    let a4 = a2.borrow();
}
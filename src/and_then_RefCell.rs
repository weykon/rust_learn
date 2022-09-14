// A mutable memory location with dynamically checked borrow rules

pub fn main() {
    // example:

    use std::cell::RefCell;

    let c = RefCell::new(5);

    // 创建一个对堆值为5的引用计数，动态检查借用的类型。，
    
}

// 对于引用的命题，还有
// Option<&T>: 这是一种经典的技巧，可以通过使用Option<&T>来处理引用可能不存在的问题。但是这个方法对内存管理的需求是非常强烈的，不太容易使用。
// Rc<T>: 这是一个 Rust 标准库中的类型，可以实现引用计数，从而实现在多个地方共享同一个值。
// Arc<T>: 这也是一个 Rust 标准库中的类型，与Rc<T>类似，可以实现引用计数，但是它是线程安全的，可以在多个线程中共享

// 为什么引用计数
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));    // 此处(a)这里报错，

    // 首先，将a放入了Box，它不是说把引用放进去，是整个a放进去了，所以后续去使用就已经被b拿去了所有权
    // 意思是说，在Box::new的处理中，a的情况是未知的，所以不能在外部就去调用。
    // 那么这个时候就使用引用，也就是说可以改变 Cons 的定义来存放一个引用，不过接着必须指定生命周期参数。

    // 当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>，
    // 这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权。
    // 创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。
    // 每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。
}

// 我觉得这个命题很大程度地重点是，使用 Rc<T> 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在其值也保持有效。
enum Rc_List {
    Cons(i32, Rc<Rc_List>),
    Nil,
}
use std::rc::Rc;
fn use_Rc() {
    let a = Rc::new(Rc_List::Cons(1, Rc::new(Rc_List::Nil)));
    let b = Rc_List::Cons(3, Rc::clone(&a));
    let c = Rc_List::Cons(4, Rc::clone(&a));

    // 用a的引用了，在b、c的使用当中相当动态了
}

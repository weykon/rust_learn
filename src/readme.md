智能指针跟引用的不同

手写一些Box这个标准库来看看里面有什么内容

Box就是用来在对上储存数据，Box自身在栈上，指向的数据在堆上. 

## 第一个： Deref

## 第二个： Drop

    用法比较简单，相当于在析构前的代码
    Drop trait 包含在 prelude 中，所以无需导入

### 还有 std::mem::drop 提早丢弃值

 原本是不允许显式调用 drop 的，这里考虑是 main 的结尾是自动调用drop，这会导致 double free错误，因为Rust会尝试清理相同的值两次。
 这里使用 std:mem::drop 和 Drop trait 不同

## 使用Rc<T> 共享数据

引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。
注意 Rc<T> 只能用于单线程场景，多线程有多线程的方案

### 之前用下面这个报错 , (b用了堆a的数据，c又来抢，肯定报错嘛) 

```rust 
enum List {

    Cons(i32, Box<List>),
    Nil,

}
use crate:: List::{Cons, Nil}; 
fn main() {

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); 

}

```

### 现在  

```rust 
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

克隆、引用计数 ++ 
克隆 Rc<T> 会增加引用计数，所以调用clone就加

然后就接上之前学的 强指针 和 弱指针 。 

Rc::strong_count 
Rc::weak_count 

调用Rc::strong_count 可以获得引用计数是多少

use std::ops::Deref;
struct MyCrazyBox<T>(T);
// 这个结构体一开始有一些疑惑
// type MyCrazyBox = <T> (x: T) =>  MyCrazyBox(x) => MyCrazyBox
// 从根本上说，Box<T> 被定义为包含一个元素的元组结构体，
// 类似 struct Color (i32,i32,i32);

impl<T> Deref for MyCrazyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0    // 0 是访问元组的第一个元素
    }
}

impl<T> MyCrazyBox<T> {
    fn new(x: T) -> MyCrazyBox<T> {
        MyCrazyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyCrazyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);   // 这里 *y 报错，因为不知道如何应对 对于MyCrazyBox 的解引用
    // (写完 Deref for MyCrazyBox 就不报错了)
    // 为了启用 * 运算符的解引用功能，需要实现 Deref trait。
    // Used for immutable dereferencing operations, like *v.
    // 用于不可变的解引用操作，例如*v.
}

// 当输入 *y 时，Rust 事实上在底层运行了如下代码：
// *(y.deref())

// 上面那个是应对不可变的类型
// 以后应对mut的
// 当 T: Deref<Target=U> 时从 &T 到 &U。
// 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
// 当 T: Deref<Target=U> 时从 &mut T 到 &U。
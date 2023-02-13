mod practice;
mod Box_answer;
mod ref_linklist;
// 这个commit删除了之前的内容，作为一个重新开始的commit来重新认识练习
fn main() {
    create_pointer_which_everyone_can_modify();
    check_ref();

    practice::拓展一下();

    Box_answer::main();
}


// 构建一个指针指向一处，并可以各方去修改他
fn create_pointer_which_everyone_can_modify() {
    let pos = 8;

    let mut pointer = Box::new(pos);

    // 在这里使用的时候
    *pointer += 1;

    println!("pointer: {}", pointer);

    // 将这个指针用到很多其他的地方
    // let mut pointer = use_pointer_modify_to_one(pointer);
    // println!("pointer: {}", pointer);
    // let mut pointer = use_pointer_modify_to_two(pointer);
    // println!("pointer: {}", pointer);

    // 使用权的问题让我很困惑去按照自己的习惯去写。
    // 这里有一个将这个指针的引用的使用
    let pointer_ref = &mut pointer;

    use_ref_from_pointer(pointer_ref);
    use_ref_from_pointer_0(pointer_ref);
    println!("pointer: {}", pointer);
}

fn use_pointer_modify_to_one(mut pointer: Box<i32>) -> Box<i32> {
    *pointer += 1;
    pointer
}

fn use_pointer_modify_to_two(mut pointer: Box<i32>) -> Box<i32> {
    *pointer += 2;
    pointer
}

fn use_ref_from_pointer(pointer_ref: &mut Box<i32>) {
    **pointer_ref += 1;
}

fn use_ref_from_pointer_0(pointer_ref: &mut Box<i32>) {
    **pointer_ref += 1;
}

// Box 是 Rust 中的一种常见智能指针类型，它用于管理在堆上分配的内存。
// 1. 动态内存分配： Box 可以在运行时动态分配内存，这使得您可以在需要时创建新的对象。
// 2. 简化内存管理： Box 简化了内存管理，因为它将资源分配和释放自动封装在一个类型中，无需显式地清理内存。
// 3. 实现继承： Box 可以被用来实现继承，因为它允许您将父对象的引用存储在子对象中。
// 4. 提高可移植性： Box 使用了独立于平台的内存分配，因此可以在不同的操作系统和体系结构上使用相同的代码。

// 例子代码from gpt
use std::ops::Deref;

fn main_from_gpt() {
    let b = Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("b[0]: {}", b[0]);

    // 使用 `deref` 解引用
    let deref_b = b.deref();
    println!("deref_b[0]: {}", deref_b[0]);
} // 这跟let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]有什么不同

// When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// 当你有一个在编译时无法知道其大小的类型并且你想在需要精确大小的上下文中使用该类型的值时
// When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// 当您拥有大量数据并且想要转移所有权但要确保在您这样做时数据不会被复制
// When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
// 当你想拥有一个值并且你只关心它是一个实现特定特征的类型而不是特定类型时

// 那么我应该常常用到第二点的应用，在处理所有权的时候，不用使用clone来完成，而是确保原来那个地方的引用
fn check_ref() {
    let b = 3;

    println!("what &b : {}", &b); // 这里 b 和 &b 都是3 ，那么这里有无&有什么不同，可能是println的解析容差了。

    // 不过好像是在传参上的不同

    translate_ref(&b);
    translate_ref(&b);
    translate_ref(&b);
    // 这里可以多次使用了，我等下试试mut去改变看看，我申请多一个c.

    let mut c = 4;
    mut_c(&mut c);
    mut_c(&mut c);
    mut_c(&mut c);
    println!("look look c : {} ", c);
    // 通过传入引用，加数改变自己都没问题。所以在Box的使用中并不是我想的那个场景。
}

fn translate_ref(number_ref: &i32) {
    println!("translate_ref: {}", number_ref);
    // 试试拿去用
    let mut here_num = 1;
    println!("result: {}", here_num + number_ref);
    // 没问题
}

fn mut_c(c_ref: &mut i32) {
    *c_ref += 1;
}

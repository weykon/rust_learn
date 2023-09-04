use std::vec;

struct Test {
    a: u32,
    b: u32,
}

impl Test {
    fn increase(&mut self) {
        let mut a = &mut self.a;
        let mut b = &mut self.b;
        *b += 1;
        *a += 1;
    }
}

pub fn main() {
    let mut t = Test { a: 1, b: 2 };
    t.increase();
    println!("a: {}, b: {}", t.a, t.b);
}

fn borrow_vec() {
    // 常常在一些 mut vec 的iter下会出现borrow repeat 问题
    let mut vec = vec![1, 2, 3];
    // for i in vec.iter_mut() {
    //     // if vec[0] != 0 {  这里的vec就会报错了，
    //     //我看到一个方案说将vec.iter_mut改为0..vec.len();
    //     // 貌似确实是那么回事，如果只是计数的话。
    //     // }
    //     *i += 1;
    // }
}

fn another() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 此处背诵
// 可变引用同时只能存在一个
// 关于所有权的一些规则必须熟记
// 可变引用与不可变引用不能同时存在
// NLL - Non-Lexical Lifetimes(NLL)
// 专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。

// # 多个可变引用
fn multi_mutable_reference() {
    let mut a = String::from("hello");
    let b = &mut a;
    let c = &mut a;
    // println!("{}, {}", b, c);
    // 上面这条打印如果去掉注释，在c的那行的 &mut a 就会报错
    // 大概是说println使用了a的引用，而使用了的话，也就是
    // 这个作用域中，即被b借用了，所以c就不能再借用了
    // 所以在我平时的代码中，vec.iter_mut(),的同一作用域下的，for当中，
    // vec就不能再被借用了，所以就会报错
}

// # 可变引用和不可变引用不能同时存在
fn same_time_at_mutable_ref_and_unmutable_ref() {
    let mut a = String::from("hello");
    let b = &a;
    let c = &mut a;
    // println!("{}, {}", b, c);
}

fn while_iter_done() {
    let vec = vec![1, 2, 3];
    let mut iter_vec = vec.iter().enumerate();
    // 做算法题目的时候常常有需要while直到iter结束，那么可以这样写
    while let Some((i, &value)) = iter_vec.next() {
        println!("{}: {}", i, value);
    }

    // 比如上面的 iter() . enumerate() 如果我延迟去放进 while 那里
    // 意思就会变了while的下一次再去计算一遍 vec.iter().enumerate()
    // 所以一定要记得把操作前置
}

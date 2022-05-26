#[path = "./切片的理解/mod.rs"]
mod slice_learn;

fn main() {
    slice_learn::slice_learn();

    let _not_used_var = 2; // 加了下划线的prefix可以告诉编译器这个变量我想保留尽管它没被使用，编译器就不会报错警告。
}

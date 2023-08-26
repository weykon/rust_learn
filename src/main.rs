// https://github.com/weykon/mini-grep.git
// 作为了一个练习的库，里面包括了 Option, ?的应用和解释

// 我记得说"?"这个是一个语法糖，是用宏来做了一个trait之类，应该是我记错的。
// 不过我是想在这里看看，实现一遍。

// 在之前都是try!这个宏来使用，他大概是：

// macro_rules! try {
//     ($expr:expr) => {
//         match $expr {
//             Ok(val) => val,
//             Err(err) => return Err(err),
//         }
//     };
// }

use std::error::Error;

fn main() {}

fn divide(x: i32, y: i32) -> Option<i32> {
    // let result = try!(x.checked_div(y));
    let result = x.checked_div(y)?;
    Some(result)
}

fn bound( arr: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    let index = 10;
    // let result = arr.get(index)?;
    // 上面这条代码说the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
    // ？是不支持使用在option上的
    let result= arr.get(index).unwrap();
    // 他们说？这个是为了应对Result的，如果是Option的话，可以使用unwrap。
    // 类似我们在别的语言中写trycatch，都是为了应对错误的情况，大概这里也是这个意思去使用的。
    // 那么在于是否None的问题中，我们的方向一方面是确实不报错，还确保他拿不到值的时候是None的情况。
    Ok(*result)
}
